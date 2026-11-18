// RP Renames for Horizon - a port of HiWord9's RP Renames.
// Original: https://github.com/HiWord9/RPRenames (Fabric).
// Collects CIT renames from resource packs and shows them at the anvil.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

pub const ID: &str = "rp_renames";
pub const VERSION: &str = "0.1.0";

pub const OPTION_KEYS: [(&str, &str); 1] = [("enabled", "Enabled")];

pub fn option(key: &str, value: Option<&String>) -> bool {
    let _ = key;
    value.map(|v| v == "true").unwrap_or(true)
}

// --- session state -----------------------------------------------------------

static STORE: OnceLock<Mutex<Vec<Rename>>> = OnceLock::new();
pub fn store() -> &'static Mutex<Vec<Rename>> {
    STORE.get_or_init(|| Mutex::new(Vec::new()))
}

static RECT: OnceLock<Mutex<Option<[f32; 4]>>> = OnceLock::new();
pub fn rect() -> &'static Mutex<Option<[f32; 4]>> {
    RECT.get_or_init(|| Mutex::new(None))
}

static PANEL: OnceLock<Mutex<Option<[f32; 4]>>> = OnceLock::new();
pub fn panel() -> &'static Mutex<Option<[f32; 4]>> {
    PANEL.get_or_init(|| Mutex::new(None))
}

static SCROLL: OnceLock<Mutex<usize>> = OnceLock::new();
pub fn scroll() -> &'static Mutex<usize> {
    SCROLL.get_or_init(|| Mutex::new(0))
}

static SELECTED: OnceLock<Mutex<Option<usize>>> = OnceLock::new();
pub fn selected() -> &'static Mutex<Option<usize>> {
    SELECTED.get_or_init(|| Mutex::new(None))
}

static PENDING_CHAT: OnceLock<Mutex<Option<String>>> = OnceLock::new();
pub fn pending_chat() -> &'static Mutex<Option<String>> {
    PENDING_CHAT.get_or_init(|| Mutex::new(None))
}

pub static ANVIL_OPEN: AtomicBool = AtomicBool::new(false);
static SCANNED: AtomicBool = AtomicBool::new(false);

// --- the rename data -----------------------------------------------------------

#[derive(Clone)]
pub struct Rename {
    /// e.g. "minecraft:diamond_sword", empty when unknown.
    pub item: String,
    /// The display name (may contain & colour codes).
    pub name: String,
}

pub fn short_item(item: &str) -> String {
    let n = item.rsplit(':').next().unwrap_or(item);
    if n.is_empty() {
        "any".into()
    } else {
        n.replace('_', " ")
    }
}

pub fn parse_properties(text: &str) -> Option<Rename> {
    let mut name: Option<String> = None;
    let mut item = String::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        if key == "nbt.display.Name" {
            let v = value
                .strip_prefix("iregex:")
                .or_else(|| value.strip_prefix("regex:"))
                .or_else(|| value.strip_prefix("pattern:"))
                .unwrap_or(value);
            if !v.is_empty() && v != ".*" && v != ".+" {
                name = Some(v.to_string());
            }
        } else if key == "items" {
            item = value.split_whitespace().next().unwrap_or("").to_string();
        }
    }
    name.map(|n| Rename { item, name: n })
}

pub fn parse_renames_txt(text: &str) -> Vec<Rename> {
    text.lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (item, name) = match line.split_once(';') {
                Some((a, b)) => (a.trim().to_string(), b.trim().to_string()),
                None => (String::new(), line.to_string()),
            };
            if name.is_empty() {
                None
            } else {
                Some(Rename { item, name })
            }
        })
        .collect()
}

// --- pack scanning (unzipped packs; zip support is a later step) ---------------

pub fn scan_packs(packs_dir: &Path, out: &mut Vec<Rename>) {
    let Ok(entries) = std::fs::read_dir(packs_dir) else {
        return;
    };
    for entry in entries.flatten() {
        let assets = entry.path().join("assets");
        if assets.is_dir() {
            scan_pack_assets(&assets, out);
        }
    }
}

fn scan_pack_assets(assets: &Path, out: &mut Vec<Rename>) {
    let Ok(namespaces) = std::fs::read_dir(assets) else {
        return;
    };
    for ns in namespaces.flatten() {
        let cit = ns.path().join("optifine").join("cit");
        walk_dir(&cit, out);
    }
}

fn walk_dir(dir: &Path, out: &mut Vec<Rename>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            walk_dir(&p, out);
        } else if p.extension().is_some_and(|e| e == "properties") {
            if let Ok(text) = std::fs::read_to_string(&p) {
                if let Some(r) = parse_properties(&text) {
                    out.push(r);
                }
            }
        }
    }
}

/// One scan per session: packs next to the config dir, plus the
/// config/rp_renames/renames.txt fallback.
pub fn scan(config_dir: &Path) {
    if SCANNED.swap(true, Ordering::Relaxed) {
        return;
    }
    let mut out = Vec::new();
    if let Some(game_dir) = config_dir.parent() {
        scan_packs(&game_dir.join("resourcepacks"), &mut out);
    }
    let fallback = config_dir.join("rp_renames").join("renames.txt");
    if let Ok(text) = std::fs::read_to_string(&fallback) {
        out.extend(parse_renames_txt(&text));
    }
    out.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    out.dedup_by(|a, b| a.name == b.name);
    *store().lock().unwrap() = out;
}

// --- panel geometry ------------------------------------------------------------

pub const PANEL_W: f32 = 110.0;
pub const LINE_H: f32 = 11.0;

/// Panel to the right of the anvil window, or to its left if there is
/// no room. `screen_w` is the GUI width probed from Corners.
pub fn panel_rect(anvil: [f32; 4], screen_w: f32) -> [f32; 4] {
    let h = anvil[3].min(170.0);
    let mut x = anvil[0] + anvil[2] + 8.0;
    if x + PANEL_W > screen_w {
        x = (anvil[0] - PANEL_W - 8.0).max(4.0);
    }
    [x, anvil[1], PANEL_W, h]
}

pub fn inside(rect: [f32; 4], at: [f32; 2]) -> bool {
    at[0] >= rect[0] && at[0] <= rect[0] + rect[2] && at[1] >= rect[1] && at[1] <= rect[1] + rect[3]
}
// Click inside the panel selects a rename; its name goes to the chat
// for the player to type into the anvil's name field.
fn rp_renames_screen_click(block: &str, data: &[i32], at: [f32; 2], right: bool) {
    use std::sync::atomic::Ordering;
    let m = crate::game::rp_renames;
    let _ = (block, data, right);
    if !m::ANVIL_OPEN.load(Ordering::Relaxed) {
        return;
    }
    let Some(panel) = *m::panel() else {
        return;
    };
    if !m::inside(panel, at) {
        return;
    }
    let renames = m::store().lock().unwrap().clone();
    if renames.is_empty() {
        return;
    }
    let list_top = panel[1] + 16.0;
    let visible = ((panel[3] - 20.0) / m::LINE_H).max(1.0) as usize;
    let max_scroll = renames.len().saturating_sub(visible);
    let scroll = (*m::scroll()).min(max_scroll);
    let row = ((at[1] - list_top) / m::LINE_H) as usize;
    let idx = scroll + row;
    if idx >= renames.len() {
        return;
    }
    *m::selected() = Some(idx);
    let r = &renames[idx];
    *m::pending_chat() = Some(format!(
        "[RP Renames] {} -> '{}' (type it in the anvil)",
        m::short_item(&r.item),
        r.name
    ));
}
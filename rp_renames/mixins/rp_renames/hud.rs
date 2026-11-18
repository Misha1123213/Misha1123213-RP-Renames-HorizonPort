// The renames panel, drawn over the anvil screen (the hud hook fires over
// menus too, in the same GUI pixel space the screen reports).
fn rp_renames_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::mod_api::HudBatch,
    corners: &mut horizon_client::mod_api::Corners,
) {
    use std::sync::atomic::Ordering;
    let m = crate::game::rp_renames;

    if !game.native_on(m::ID) || !m::ANVIL_OPEN.load(Ordering::Relaxed) {
        return;
    }

    if let Some(msg) = m::pending_chat().lock().unwrap().take() {
        game.native_chat(&msg);
    }

    let Some(anvil) = *m::rect() else {
        return;
    };
    let screen_w = corners
        .place(horizon_client_lib::extras_hud::Corner::TopRight, 0.0, 0.0)
        .0
        + 8.0;
    let panel = m::panel_rect(anvil, screen_w);
    *m::panel() = Some(panel);

    // Panel background and border.
    game.native_fill(batch, panel, 0xE0_11_14_1B);
    let b = 0xFF_3A_43_56;
    game.native_fill(batch, [panel[0], panel[1], panel[2], 1.0], b);
    game.native_fill(batch, [panel[0], panel[1] + panel[3] - 1.0, panel[2], 1.0], b);
    game.native_fill(batch, [panel[0], panel[1], 1.0, panel[3]], b);
    game.native_fill(batch, [panel[0] + panel[2] - 1.0, panel[1], 1.0, panel[3]], b);

    let renames = m::store().lock().unwrap().clone();
    game.native_text(
        batch,
        &format!("Renames ({})", renames.len()),
        [panel[0] + 5.0, panel[1] + 4.0],
        0xFF_FF_FF_FF,
        true,
    );

    if renames.is_empty() {
        game.native_text(
            batch,
            "No renames found.",
            [panel[0] + 5.0, panel[1] + 18.0],
            0xFF_9A_A4_B8,
            false,
        );
        game.native_text(
            batch,
            "See config/rp_renames/",
            [panel[0] + 5.0, panel[1] + 30.0],
            0xFF_9A_A4_B8,
            false,
        );
        game.native_text(
            batch,
            "renames.txt",
            [panel[0] + 5.0, panel[1] + 41.0],
            0xFF_9A_A4_B8,
            false,
        );
        return;
    }

    let list_top = panel[1] + 16.0;
    let visible = ((panel[3] - 20.0) / m::LINE_H).max(1.0) as usize;
    let max_scroll = renames.len().saturating_sub(visible);
    let scroll = (*m::scroll()).min(max_scroll);
    let sel = *m::selected();

    for (i, r) in renames.iter().enumerate().skip(scroll).take(visible) {
        let y = list_top + (i - scroll) as f32 * m::LINE_H;
        if Some(i) == sel {
            game.native_fill(batch, [panel[0] + 2.0, y - 1.0, panel[2] - 4.0, m::LINE_H], 0x40_FF_FF_FF);
        }
        let line = format!("{}: {}", m::short_item(&r.item), r.name);
        game.native_text(batch, &line, [panel[0] + 5.0, y], 0xFF_CF_D6_E4, false);
    }

    if max_scroll > 0 {
        game.native_text(
            batch,
            &format!("{}..{} of {}", scroll + 1, (scroll + visible).min(renames.len()), renames.len()),
            [panel[0] + 5.0, panel[1] + panel[3] - 10.0],
            0xFF_7E_88_9C,
            false,
        );
    }
}
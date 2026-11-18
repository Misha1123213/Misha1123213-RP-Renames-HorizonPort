// Wheel over the panel scrolls the list and swallows the event.
fn rp_renames_scroll(
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
    scroll: [f64; 2],
    canceled: &mut bool,
) {
    let m = crate::game::rp_renames;
    if screen.menu != Some("minecraft:anvil") {
        return;
    }
    let Some(panel) = *m::panel() else {
        return;
    };
    if !m::inside(panel, screen.mouse) {
        return;
    }
    let mut s = m::scroll().lock().unwrap();
    if scroll[1] > 0.0 {
        *s = s.saturating_sub(1);
    } else if scroll[1] < 0.0 {
        *s += 1;
    }
    *canceled = true;
}
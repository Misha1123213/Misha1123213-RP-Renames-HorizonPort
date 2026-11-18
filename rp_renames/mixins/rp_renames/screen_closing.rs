fn rp_renames_screen_closing(screen: &horizon_client::mod_api::NativeScreenView<'_>) {
    use std::sync::atomic::Ordering;
    let m = crate::game::rp_renames;
    if screen.menu == Some("minecraft:anvil") {
        m::ANVIL_OPEN.store(false, Ordering::Relaxed);
        *m::panel() = None;
    }
}
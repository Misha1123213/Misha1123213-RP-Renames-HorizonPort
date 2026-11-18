fn rp_renames_screen_opening(screen: &horizon_client::mod_api::NativeScreenView<'_>) {
    use std::sync::atomic::Ordering;
    let m = crate::game::rp_renames;
    if screen.menu == Some("minecraft:anvil") {
        m::ANVIL_OPEN.store(true, Ordering::Relaxed);
        *m::rect() = Some(screen.rect);
        *m::scroll() = 0;
        if let Ok(config_dir) = crate::game::Game::native_config_dir() {
            m::scan(&config_dir);
        }
    }
}
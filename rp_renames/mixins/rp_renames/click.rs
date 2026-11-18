fn rp_renames_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::rp_renames as m;
    if mod_id != m::ID {
        return;
    }
    let Some((key, _)) = m::OPTION_KEYS.get(button) else {
        return;
    };
    let on = m::option(key, game.native_setting(m::ID, key).as_ref());
    game.native_set_setting(m::ID, key, if on { "false" } else { "true" });
}
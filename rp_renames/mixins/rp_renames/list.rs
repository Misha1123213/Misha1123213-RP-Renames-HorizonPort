fn rp_renames_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::rp_renames as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("RP Renames {}", m::VERSION),
        description: "CIT renames from resource packs at the anvil. Port of RP Renames.".into(),
        keys: Vec::new(),
    });
}
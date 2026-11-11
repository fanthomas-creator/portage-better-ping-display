fn list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "portage_better_ping_display".into(),
        title: "portage-better-ping-display 0.1.1".into(),
        description: "Unofficial Rust port of Vlad Marica's mod: colored numeric ping in Tab."
            .into(),
        keys: Vec::new(),
    });
}

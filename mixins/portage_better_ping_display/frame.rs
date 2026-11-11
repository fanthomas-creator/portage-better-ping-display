fn frame(game: &mut crate::game::Game<'_>) {
    if game.native_window() {
        horizon_client::tab_list::portage_better_ping_display::set_enabled(
            game.native_on("portage_better_ping_display"),
        );
    }
}

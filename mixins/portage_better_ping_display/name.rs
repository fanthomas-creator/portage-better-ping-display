// Original name formatting preserved from Horizon tab_list.rs (MIT).
fn name(
    player: &crate::tab_list::TabPlayer,
    board: &crate::scoreboard::Scoreboard,
) -> ::core::ops::ControlFlow<Text> {
    use crate::{scoreboard::comp_text, tab_list::portage_better_ping_display as m, text::Text};
    if !m::enabled() {
        return std::ops::ControlFlow::Continue(());
    }
    let spectator = player.game_mode == crate::tab_list::SPECTATOR;
    let original = match &player.display_name {
        Some(display) if spectator => comp_text(&crate::tab_list::italic_root(display)),
        Some(display) => comp_text(display),
        None => {
            let text = match board.players_team(&player.name) {
                Some(team) => team.formatted_name(Text::literal(player.name.clone())),
                None => Text::literal(player.name.clone()),
            };
            if spectator { text.italic() } else { text }
        }
    };
    std::ops::ControlFlow::Break(original.append(Text::colored(
        format!(" {}", m::label(player.latency)),
        m::color(player.latency),
    )))
}

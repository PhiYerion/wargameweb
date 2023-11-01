use leptos::*;

use super::Board;

/// GamePage that contains the board to play the game.
/// Passes the players to the board component.
/// * `players` - number of players in the game
#[component]
pub fn GamePage(players: usize) -> impl IntoView {
    view! {
    <div class="flex self-center items-center p-8">
        <Board players={players} />
    </div>
    }
}

use leptos::*;

use super::Board;

#[component]
pub fn GamePage(players: usize) -> impl IntoView {
    view! {
    <div class="flex self-center items-center p-8">
        <Board players={players} />
    </div>
    }
}

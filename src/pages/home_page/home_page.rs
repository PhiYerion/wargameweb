use leptos::*;

use super::Board;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex self-center items-center p-8">
            <p>test</p>
            <Board players={4} />
        </div>
    }
}

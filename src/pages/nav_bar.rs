use leptos::*;

/// Simple NavBar that displays the title as a link to /home
#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <div class="flex self-center items-center justify-between w-full p-8 bg-gray-800">
            <a href="/home">
                <h1 class="mx-auto justify-between self-center text-4xl">War Game</h1>
            </a>
        </div>
    }
}

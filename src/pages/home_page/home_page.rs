use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
    <div class="flex flex-col items-center justify-center">
        <h1 class="text-4xl text-bold text-grey-200 p-8">Welcome to the WarGameWeb</h1>
        <h2 class="text-2xl text-grey-300 p-2">Please choose how many players</h2>
        <form class="p-2" method="GET" action="/game">
            <label class="p-2" for="players">Number of players</label>
            <input class="bg-slate-300 hover:bg-blue-100 text-slate-900 font-bold py-1 px-2 rounded" type="number"
                id="players" name="players" min=2 max=4 />
        </form>
    </div>
    }
}

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
    // injects a stylesheet into the document

    <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/wargameweb.css" />

        // sets the document title
        <Title text="War Game" />

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="/home" view=HomePage />
                    <Route path="/game" view=GamePage />
                    <Route path="/*any" view=NotFound />
                </Routes>
            </main>
        </Router>
    </head>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    use crate::pages::{HomePage, NavBar};
    // Creates a reactive value to update the button

    view! {

    <body class="text-slate-200 bg-slate-700 divide-y-8 divide-stone-100 divide-solid">
        <NavBar />
        <HomePage />
    </body>
    }
}

#[derive(Params, PartialEq, Copy, Clone)]
struct GamePagePlayers {
    players: usize,
}

fn GamePage() -> impl IntoView {
    use crate::pages::{GamePage, NavBar};
    let query = use_query::<GamePagePlayers>();
    let players = move || {
        query.with(|params| {
            params
                .clone()
                .map(|params| params.players)
                .unwrap_or_default()
        })
    };

    view! {

    <body class="text-slate-200 bg-slate-700 divide-y-8 divide-stone-100 divide-solid">
        <NavBar />
        <GamePage players=players() />
    </body>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
    <h1>"Not Found"</h1>
    }
}

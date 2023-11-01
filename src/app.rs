use leptos::*;
use leptos_meta::*;
use leptos_router::*;

/// Basic entry point that provides routing
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
    <head>
        <Stylesheet id="leptos" href="/pkg/wargameweb.css" />
        <Title text="War Game" />
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

/// Basic Home Page
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

/// Get request query struct for GamePage
/// * `players` - number of players for get request
#[derive(Params, PartialEq, Copy, Clone)]
struct GamePageQuery {
    players: usize,
}

/// Game page with number of players as query parameter in get request
fn GamePage() -> impl IntoView {
    use crate::pages::{GamePage, NavBar};

    // Get the query parameters from the URL
    let query = use_query::<GamePageQuery>();
    // Extract the players from the query parameters and put it into reactive value
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

/// 404 - Not Found, taken from examples
#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
    <h1>"Not Found"</h1>
    }
}

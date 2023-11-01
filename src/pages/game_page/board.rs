use super::*;
use leptos::logging;
use leptos::*;

#[component]
fn ViewCard(card: Card, #[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
    <div class={class}>
        {match card.suit {
        Suit::Spades => "♠ ",
        Suit::Hearts => "♥ ",
        Suit::Diamonds => "♦ ",
        Suit::Clubs => "♣ ",
        }}
        {
        match card.rank {
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "Jack",
        11 => "Queen",
        12 => "King",
        13 => "Ace",
        _ => {
        panic!("Unknown rank");
        }
        }}
    </div>
    }
}

#[component]
pub fn Board(players: usize) -> impl IntoView {
    // Error handling
    if players < 2 {
        logging::error!("Too few players");
        return view! {<div>
        </div>};
    } else if players > 4 {
        logging::error!("Too many players");
        return view! {<div></div>};
    }

    // Create deck
    let cards_per_player = 52 / players as usize;
    let mut decks: Vec<Deck> = Vec::with_capacity(players);
    for i in 0..players {
        logging::log!("Creating deck");
        let deck = Deck::new(cards_per_player, i);
        decks.push(deck);
    }

    // View for deck list
    let (decks_view, set_deck_view) = create_signal(decks.clone());
    // View for header above deck list
    let (header_view, set_header_view) = create_signal("Round 1".to_string());
    // Round counter
    let mut rounds_count = 1;

    // Round button handler
    let next_round = move |_| {
        logging::log!("Round button clicked");
        let (decks_result, result) = round(decks_view());
        let decks = decks_result;
        rounds_count += 1;

        match result {
            RoundResult::Continue => {
                logging::log!("Round continues");
                set_header_view(format!("Round {}", rounds_count));
                set_deck_view(decks);
            }
            RoundResult::Winner => {
                logging::log!("Round winner");
                set_header_view(format!("Player {} wins!", decks[0].player));
                set_deck_view(decks);
            }
            RoundResult::Tie => {
                logging::log!("Round tie");
                set_header_view("WAR!".to_string());
                set_deck_view(decks);
            }
        }
    };

    // Go through whole game button handler

    view! {
    <div class="w-full">
        <div class="flex flex-wrap self-center justify-center items-center">
            <h1 class="px-10 text-xl font-bold"> { move || header_view() } </h1>
            <button
                class="bg-transparent text-blue-300 font-semibold hover:text-blue-200 py-1 px-2 border border-blue-500 hover:border-blue-400 rounded"
                on:click=next_round>Next Round</button>
        </div>
        <div class="flex flex-wrap self-center justify-center">
            { move ||
            decks_view().iter().map(|deck| {
            view! {
            <div class="p-8 mx-10">
                <h2 class="font-bold">Player {deck.player}</h2>
                {deck.cards.iter().map(|card| {
                view! {
                <ViewCard card={card.clone()} />
                }}).collect_view()}
                {deck.captured_cards.iter().map(|card| {
                view! {
                <ViewCard class="text-red-600" card={card.clone()} />
                }}).collect_view()}
            </div>
            }
            }).collect_view()
            }
        </div>
    </div>
    }
}

use super::*;
use leptos::logging;
use leptos::*;

/// Transforms a card into a div element with the suit and rank
/// * `card` - Card to Transform
/// * `class` - Class to be injected into top level div for this component
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

/// The board of the game. This is the top-level component for the game.
/// * `players` - Number of players in the game
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

    // Create the decks
    let cards_per_player = 52 / players as usize;
    let mut decks: Vec<Deck> = Vec::with_capacity(players);
    //   Populate the decks
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

        // Run the round and collect the results
        // Due to this function being sent into outside due to the closure (which is
        // needed for a button handler), we need to use the signal this is in garbage
        // collection.
        let (decks_result, result) = round(decks_view());
        rounds_count += 1;

        match result {
            RoundResult::Continue => {
                logging::log!("Round continues");
                // Change the header appropriately to show the round number
                set_header_view(format!("Round {}", rounds_count));
                // Update the deck view
                set_deck_view(decks_result);
            }
            RoundResult::Winner => {
                logging::log!("Round winner");
                // And we keep doing the same as above
                set_header_view(format!("Player {} wins!", decks[0].player));
                set_deck_view(decks_result);
            }
            RoundResult::Tie => {
                logging::log!("Round tie");
                set_header_view("WAR!".to_string());
                set_deck_view(decks_result);
            }
        }
    };

    view! {
        <div class="w-full">
            // Header and button section
            <div class="flex flex-wrap self-center justify-center items-center">
                // Header (E.G. Round 1)
                <h1 class="px-10 text-xl font-bold"> { move || header_view() } </h1>
                // Button to continue to next round
                <button
                    class="bg-transparent text-blue-300 font-semibold hover:text-blue-200 py-1 px-2 border border-blue-500 hover:border-blue-400 rounded"
                    on:click=next_round>Next Round</button>
            </div>
            // Deck section
            <div class="flex flex-wrap self-center justify-center">
                // For each deck, create a div with the player number and the cards
                // move || is a closure so we can pass this function through the mystical
                //   realm of wasm-JS bindings
                // .iter().map(...) is analagous to a for each
                { move || decks_view().iter().map(|deck| {
                    view! {
                        <div class="p-8 mx-10">
                            <h2 class="font-bold">Player {deck.player}</h2>
                            // iterate over the cards in the deck and create a ViewCard for each
                            {deck.cards.iter().map(|card| {
                                view! {
                                    <ViewCard card={card.clone()} />
                                }}).collect_view()}
                            // iterate over the captured cards in the deck and create a ViewCard for each
                            {deck.captured_cards.iter().map(|card| {
                                view! {
                                    <ViewCard class="text-red-600" card={card.clone()} />
                                }}).collect_view()}
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

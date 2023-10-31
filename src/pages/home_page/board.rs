use crate::pages::home_page::{card::Card, card::Rank, card::Suit, deck::Deck};
use leptos::{leptos_dom::logging, *};

#[component]
fn ViewCard(card: Card) -> impl IntoView {
    view! {
        <div class="flex">
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

enum RoundResult<'a> {
    RoundWinner(&'a mut Deck),
    Winner(Deck),
}

#[inline]
fn round<'a>(decks: &'a mut Vec<&'a mut Deck>) -> RoundResult<'a> {
    let mut cards: Vec<Card> = Vec::with_capacity(decks.len());

    // dummy card as placeholder
    let mut highest_card = Card {
        suit: Suit::Spades,
        rank: 0,
    };

    let mut highest_deck: Option<&mut Deck> = None;
    for deck in decks {
        match deck.cards.pop() {
            Some(card) => {
                if highest_deck.is_none() || card.rank > highest_card.rank {
                    highest_card = card.clone();
                    highest_deck = Some(deck);
                } else if card.rank == highest_card.rank {
                    let tie_winner = round(&mut vec![deck, highest_deck.unwrap()]);
                    match tie_winner {
                        RoundResult::RoundWinner(mut tie_winner) => {
                            highest_deck = Some(&mut tie_winner);
                        }
                        RoundResult::Winner(tie_winner) => {
                            return RoundResult::Winner(tie_winner);
                        }
                    }
                }
                cards.push(card);
            }
            None => {
                return RoundResult::Winner(deck.clone());
            }
        }
    }
    highest_deck.unwrap().cards.append(&mut cards);

    RoundResult::RoundWinner(highest_deck.unwrap())
}

#[component]
pub fn Board(players: usize) -> impl IntoView {
    // Error handling
    if players < 2 {
        logging::console_error("Too few players");
        return view! {<div></div>};
    } else if players > 4 {
        logging::console_error("Too many players");
        return view! {<div></div>};
    }

    // Create deck
    let cards_per_player = 52 / players as usize;
    let mut decks: Vec<Deck> = Vec::with_capacity(players);
    for _ in 0..players {
        logging::console_log("Creating deck");
        let deck = Deck::new(cards_per_player);
        decks.push(deck);
    }

    view! {
        <div>
            {decks.into_iter().map(|deck| {
                deck.cards.into_iter().map(|card| {
                    view! {
                        <ViewCard card=card/>
                    }
                }
            ).collect_view()}).collect_view()}
        </div>
    }
}

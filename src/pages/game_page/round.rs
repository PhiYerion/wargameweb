use leptos::logging;

use super::*;

pub enum RoundResult {
    Winner,
    Tie,
    Continue,
}

pub fn round(mut decks: Vec<Deck>) -> (Vec<Deck>, RoundResult) {
    logging::log!("Round started");

    //
    let mut pot: Vec<Card> = Vec::with_capacity(decks.len());
    let mut result = RoundResult::Continue;

    'outer: loop {
        // dummy card as placeholder
        let mut highest_card = Card {
            suit: Suit::Spades,
            rank: 0,
        };
        let mut highest_deck_index: Option<usize> = None;

        for (idx, deck) in decks.iter_mut().enumerate() {
            // Cleanup captured_cards
            deck.cards.append(&mut deck.captured_cards);
            deck.captured_cards.clear();

            match deck.cards.pop() {
                Some(card) => {
                    if highest_deck_index.is_none() || card.rank > highest_card.rank {
                        highest_card = card.clone();
                        highest_deck_index = Some(idx);
                    } else if card.rank == highest_card.rank {
                        result = RoundResult::Tie;
                        continue 'outer;
                    }
                    pot.push(card);
                }
                None => {}
            }
        }

        decks[highest_deck_index.unwrap()].captured_cards = pot;

        if decks[highest_deck_index.unwrap()].cards.len() == 52 {
            result = RoundResult::Winner;
        }

        return (decks, result);
    }
}

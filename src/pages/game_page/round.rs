use super::*;
use leptos::logging;

/// The result of a round
pub enum RoundResult {
    Winner,
    Tie,
    Continue,
}

/// Runs a round of the game
/// * `decks` - The decks to run the round on
/// * Returns the decks after the round and the result of the round
pub fn round(mut decks: Vec<Deck>) -> (Vec<Deck>, RoundResult) {
    logging::log!("Round started");

    // Pot of cards to be won
    let mut pot: Vec<Card> = Vec::with_capacity(decks.len());
    // Default result is continue
    let mut result = RoundResult::Continue;

    // Loop in case we need to do a tiebreaker. This is preferred over
    // recursion to avoid rust's RAII issues and preformance concerns.
    'outer: loop {
        // dummy card as placeholder
        let mut highest_card = Card {
            suit: Suit::Spades,
            rank: 0,
        };

        // Index of the deck with the highest card
        let mut highest_deck_index: Option<usize> = None;

        // Loop through the decks
        for (idx, deck) in decks.iter_mut().enumerate() {
            // Cleanup captured_cards
            deck.cards.append(&mut deck.captured_cards);
            deck.captured_cards.clear();

            // Get the card and handle the case of no cards left.
            match deck.cards.pop() {
                Some(card) => {
                    // If this is the first iteration or if the card is higher,
                    if highest_deck_index.is_none() || card.rank > highest_card.rank {
                        // Create a clone of the card because we are not taking
                        // ownership. We can most likely avoid this clone though, but we
                        // accidentally deleted the linux box in DOECF because I added a fake
                        // parition in /dev to mount /tmp for security but forgot that /dev was
                        // temporary and opensuse does not have safe guards to time out on remount
                        // attempts. I may have caught this but we needed to reboot the machine
                        // suddenly. We cannot pull the image from amazon (because amazon just says
                        // no) and we backed up the task box instead of the linux box so we had to
                        // start from square one and I have to get up at 2:30 AM to fly out on
                        // thrusday.
                        //
                        // I will not be doing any more optimizations for now.
                        highest_card = card.clone();
                        // Set the highest deck index
                        highest_deck_index = Some(idx);
                    } else if card.rank == highest_card.rank {
                        // If the card is the same rank, we have a tie
                        result = RoundResult::Tie;
                        // Continue the outer loop to do another round but keep the pot intact.
                        // This is in accordance to specifications.
                        continue 'outer;
                    }
                    // Add the card to the pot
                    pot.push(card);
                }
                // If there are no cards left for the plauer, we just skip.
                None => {}
            }
        }

        // If we get here, we have a winner and we will add the cards to the winner's
        decks[highest_deck_index.unwrap()].captured_cards = pot;

        // if the winner has all the cards, we have a winner.
        // The devision and multiplication is factor in truncation.
        if decks[highest_deck_index.unwrap()].cards.len() == (52 / decks.len()) * decks.len() {
            result = RoundResult::Winner;
        }

        return (decks, result);
    }
}

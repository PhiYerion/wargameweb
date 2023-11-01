use super::*;

/// Deck struct
/// * `cards` - Cards in the deck
/// * `captured_cards` - Cards captured by the player
/// * `player` - Player number
#[derive(Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub captured_cards: Vec<Card>,
    pub player: usize,
}

impl Deck {
    // No linining to reduce the size of the binary
    /// Creates a new deck with a given size and player number
    /// * `deck_size` - Size of the deck
    /// * `player` - Player number
    #[inline(never)]
    pub fn new(deck_size: usize, player: usize) -> Deck {
        // Preallocate max possible capacity. Minimal memory impact,
        // no additional allocations.
        let mut cards: Vec<Card> = Vec::with_capacity(52);

        // Populate the deck
        for _ in 0..deck_size {
            cards.push(Card::new());
        }

        // Return the deck
        Deck {
            cards,
            captured_cards: Vec::new(),
            player,
        }
    }
}

#[cfg(test)]
mod deck_tests {
    use super::*;

    #[test]
    fn test_deck() {
        let deck = Deck::new(52, 1);
        assert_eq!(deck.cards.len(), 52);
    }
}

use crate::pages::home_page::card::Card;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    // No linining to reduce the size of the binary
    #[inline(never)]
    pub fn new(deck_size: usize) -> Deck {
        // Preallocation with small capacity to improve performance
        let mut cards: Vec<Card> = Vec::with_capacity(52);
        for _ in 0..deck_size {
            cards.push(Card::new());
        }
        Deck { cards }
    }
}

#[cfg(test)]
mod deck_tests {
    use super::*;

    #[test]
    fn test_deck() {
        let deck = Deck::new(52);
        assert_eq!(deck.deck.len(), 52);
    }
}

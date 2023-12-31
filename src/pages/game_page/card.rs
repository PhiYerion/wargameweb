/// Card suits
#[derive(Clone)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

/// Card ranks
#[derive(Clone)]
pub enum Rank {
    Base(u8),
    Ace,
    Jack,
    Queen,
    King,
}

/// Card struct
/// * `suit` - Suit of the card
/// * `rank` - Rank of the card
#[derive(Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: u8,
}
impl Card {
    // No linining to reduce the size of the binary
    /// Creates a new card with a random suit and rank
    #[inline(never)]
    pub fn new() -> Card {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();

        Card {
            suit: match rng.gen_range(0..3) {
                0 => Suit::Spades,
                1 => Suit::Hearts,
                2 => Suit::Diamonds,
                3 => Suit::Clubs,
                _ => {
                    panic!("Unkown number");
                }
            },
            rank: rng.gen_range(2..13),
        }
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn test_card() {
        for _ in 0..10000 {
            let card = Card::new();
        }
    }
}

/// Cards, and methods to determine their relative values.

/// The card suits.
///
/// In poker, suits are not ordered, but we need to be able to compare them.
#[derive(Debug, Eq, PartialEq)]
pub enum Suits {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// The card ranks.
///
/// The items can be compared using ==, < and > to determine their relative values as used in poker.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Ranks {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

/// An individual card.
///
/// Cards can be compared using ==, but not with < or >.
/// To make a relative comparison, use the `rank` field.
#[derive(Debug, Eq)]
pub struct Card {
    pub rank: Ranks,
    pub suit: Suits,
}

impl Card {
    pub fn new(rank: Ranks, suit: Suits) -> Self {
        Self { rank, suit }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.suit == other.suit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranks() {
        assert!(Ranks::Ten == Ranks::Ten);
        assert!(Ranks::Ten < Ranks::Jack);
        assert!(Ranks::Ten > Ranks::Nine);
    }

    #[test]
    fn test_suits() {
        assert!(Suits::Clubs == Suits::Clubs);
    }

    #[test]
    fn test_cards() {
        let card1 = Card::new(Ranks::Ten, Suits::Clubs);
        let card2 = Card::new(Ranks::Nine, Suits::Clubs);
        let card3 = Card::new(Ranks::Ten, Suits::Hearts);
        assert!(card1 == card1);
        assert!(card1 != card2);
        assert!(card1 != card3);
    }
}

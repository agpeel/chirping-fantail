/// Cards, and methods to determine their relative values.

/// The card suits.
///
/// In poker, suits are not ordered, but we need to be able to compare them.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Suits {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// The card ranks.
///
/// The items can be compared using ==, < and > to determine their relative values as used in poker.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Copy, Clone)]
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
/// Cards can be compared using ==, > and <.
#[derive(Debug, Eq, Copy, Clone)]
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

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank.partial_cmp(&other.rank)
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
        assert!(Suits::Clubs != Suits::Hearts);
    }

    #[test]
    fn test_cards() {
        let card1 = Card::new(Ranks::Ten, Suits::Clubs);
        let card2 = Card::new(Ranks::Ten, Suits::Clubs);
        let card3 = Card::new(Ranks::Nine, Suits::Clubs);
        let card4 = Card::new(Ranks::Ten, Suits::Hearts);
        assert!(card1 == card2);
        assert!(card1 != card3);
        assert!(card1 != card4);
        assert!(card1 > card3);
        // TODO: check whether PartialOrd and PartialEq are in conflict.
        // This is behaviour that I want, but there is a discrepancy between the results of
        // comparisons by PartialEq and PartialOrd.
        // Should they be the same?
        // In PartialOrd, card1 and card4 are equal because their ranks are the same.
        assert!(card1 != card4);
    }

    #[test]
    fn test_cards_sort() {
        // Confirm that PartialEq is doing the correct behaviour.
        let card1 = Card::new(Ranks::Jack, Suits::Diamonds);
        let card2 = Card::new(Ranks::Ten, Suits::Clubs);
        let card3 = Card::new(Ranks::Nine, Suits::Clubs);
        let card4 = Card::new(Ranks::Ten, Suits::Hearts);
        let mut cards = vec![card1, card2, card3, card4];
        cards.sort();
        assert!(cards[0] == card3);
        assert!(cards[3] == card1);
    }
}

use crate::cards::{Card, Ranks, Suits};
use crate::error::PokerHandError;
use std::cmp::Ordering;

/// Poker hand types, in the order of their relative value.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PokerHandRanks {
    HighCard = 1,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

/// A poker hand.
///
/// PartialOrd is supported to allow sorting of hands.
/// A reference to the hand string from the calling environment is stored so that it can
/// be returned as a reference to the winning hand(s).
#[derive(Debug)]
pub struct PokerHand<'a> {
    pub hand_handle: &'a str,
    pub hand_rank: PokerHandRanks,
    // A vector of the cards in the hand, in "scoring order".
    // Scoring order means the order in which the cards are compared to determine the winner.
    // For example, a pair is decided first on the rank of the cards in the pair, then the
    // rank of the remaining cards if the pairs are equal.
    // So an example pair will be stored in the order [4H, 4C, AD, 10S, 3C]
    pub cards: Vec<Card>,
}

impl PokerHand<'_> {
    pub fn new(hand: &str) -> Result<PokerHand, PokerHandError> {
        let mut cards: Vec<Card> = Vec::with_capacity(5);

        // Parse the hand string.
        todo!();

        // Classify the hand.
        let hand_rank: PokerHandRanks = PokerHandRanks::HighCard;
        todo!();

        Ok(PokerHand {
            hand_handle: hand,
            hand_rank,
            cards,
        })
    }
}

impl PartialEq for PokerHand<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_rank != other.hand_rank {
            return false;
        }
        // Poker hand rank does not depend on the suit, only the rank of the cards.
        for i in 0..5 {
            if self.cards[i].rank != other.cards[i].rank {
                return false;
            }
        }
        true
    }
}

impl PartialOrd for PokerHand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_rank < other.hand_rank {
            Some(Ordering::Less)
        } else if self.hand_rank > other.hand_rank {
            Some(Ordering::Greater)
        } else {
            // Compare the card ranks.
            for i in 0..5 {
                if self.cards[i].rank < other.cards[i].rank {
                    return Some(Ordering::Less);
                } else if self.cards[i].rank > other.cards[i].rank {
                    return Some(Ordering::Greater);
                }
            }
            Some(Ordering::Equal)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poker_hand_ranks() {
        assert!(PokerHandRanks::FullHouse == PokerHandRanks::FullHouse);
        assert!(PokerHandRanks::HighCard < PokerHandRanks::Pair);
    }

    #[test]
    fn test_poker_hand_comparisons() {
        let hand1 = PokerHand {
            hand_handle: "4D 4H JD 6C 2S",
            hand_rank: PokerHandRanks::Pair,
            cards: vec![
                Card::new(Ranks::Four, Suits::Diamonds),
                Card::new(Ranks::Four, Suits::Hearts),
                Card::new(Ranks::Jack, Suits::Diamonds),
                Card::new(Ranks::Six, Suits::Clubs),
                Card::new(Ranks::Two, Suits::Spades),
            ],
        };
        // 6C is less than 7C.
        let hand2 = PokerHand {
            hand_handle: "4D 4H JD 7C 2S",
            hand_rank: PokerHandRanks::Pair,
            cards: vec![
                Card::new(Ranks::Four, Suits::Diamonds),
                Card::new(Ranks::Four, Suits::Hearts),
                Card::new(Ranks::Jack, Suits::Diamonds),
                Card::new(Ranks::Seven, Suits::Clubs),
                Card::new(Ranks::Two, Suits::Spades),
            ],
        };
        assert!(hand1 < hand2);
        // Same ranks but different suits.
        let hand3 = PokerHand {
            hand_handle: "4C 4S JH 6S 2S",
            hand_rank: PokerHandRanks::Pair,
            cards: vec![
                Card::new(Ranks::Four, Suits::Clubs),
                Card::new(Ranks::Four, Suits::Spades),
                Card::new(Ranks::Jack, Suits::Hearts),
                Card::new(Ranks::Six, Suits::Spades),
                Card::new(Ranks::Two, Suits::Spades),
            ],
        };
        assert!(hand3 == hand1);
    }
}

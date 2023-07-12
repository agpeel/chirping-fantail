use std::cmp::Ordering;

#[derive(Debug)]
pub enum PokerHandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

// Record a reference to the input string slice and derived information about the hand.
#[derive(Debug)]
pub struct PokerHandHandle<'a> {
    pub hand_handle: &'a str,
    pub hand_rank: PokerHandRank,
    // A vector of the card ranks (the number, not the suit) in the hand, sorted in "scoring order".
    // Scoring order means the order in which the cards are compared to determine the winner.
    // For example "9H 2S 3C 3D 5H" would be [3, 3, 9, 5, 2]
    pub card_ranks: Vec<u8>,
}

pub fn build_poker_hand_handle(hand: &str) -> Result<PokerHandHandle, &'static str> {
    let mut card_ranks: Vec<u8> = Vec::with_capacity(5);
    let mut hand_rank: PokerHandRank = PokerHandRank::HighCard;

    // TODO: Parse the hand string.  Return None for invalid hand strings.

    Ok(PokerHandHandle {
        hand_handle: hand,
        hand_rank,
        card_ranks,
    })
}

impl PartialEq for PokerHandHandle<'_> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Compare the hand rank, not the original string.
        *self.hand_handle == *other.hand_handle
    }
}

impl PartialOrd for PokerHandHandle<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // TODO: Compare the hand rank, not the original string.
        if *self.hand_handle == *other.hand_handle {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

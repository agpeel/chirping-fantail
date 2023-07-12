use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl From<PokerHandRank> for u8 {
    fn from(rank: PokerHandRank) -> Self {
        match rank {
            PokerHandRank::HighCard => 1,
            PokerHandRank::Pair => 2,
            PokerHandRank::TwoPair => 3,
            PokerHandRank::ThreeOfAKind => 4,
            PokerHandRank::Straight => 5,
            PokerHandRank::Flush => 6,
            PokerHandRank::FullHouse => 7,
            PokerHandRank::FourOfAKind => 8,
            PokerHandRank::StraightFlush => 9,
        }
    }
}

// Record a reference to the input string slice and derived information about the hand.
#[derive(Debug)]
pub struct PokerHandHandle<'a> {
    pub hand_handle: &'a str,
    pub hand_rank: PokerHandRank,
    // A vector of the card ranks (the number, not the suit) in the hand, sorted in "scoring order".
    // Scoring order means the order in which the cards are compared to determine the winner.
    // For example "9H 2S 3C 3D 5H" would be [3, 3, 9, 5, 2].
    pub card_ranks: Vec<u8>,
}

pub fn build_poker_hand_handle(hand: &str) -> Result<PokerHandHandle, &'static str> {
    let mut card_ranks: Vec<u8> = Vec::with_capacity(5);
    let mut card_suits: Vec<char> = Vec::with_capacity(5);
    let mut hand_rank: PokerHandRank = PokerHandRank::HighCard;

    // Parse the hand string.
    let mut expect_num: bool = true;
    let mut expect_suit: bool = false;
    let mut card_num: u8 = 0;
    for c in hand.chars() {
        if expect_num {
            if card_num == 5 {
                return Err("Too many cards in hand");
            }
            match c {
                // Note the card_rank value range is 2 to 14 (for Ace).
                '0' => card_ranks.push(10),
                '1' => continue, // Skip the 1 in 10.
                '2'..='9' => card_ranks.push(c.to_digit(10).unwrap() as u8),
                'J' => card_ranks.push(11),
                'Q' => card_ranks.push(12),
                'K' => card_ranks.push(13),
                'A' => card_ranks.push(14),
                _ => return Err("Invalid card number"),
            }
            expect_num = false;
            expect_suit = true;
        } else if expect_suit {
            match c {
                'H' | 'S' | 'C' | 'D' => card_suits.push(c),
                _ => return Err("Invalid card suit"),
            }
            expect_suit = false;
            card_num += 1;
        } else {
            match c {
                ' ' => {
                    expect_num = true;
                }
                _ => return Err("Missing ' ' char in hand string."),
            }
        }
    }
    if card_num != 5 {
        return Err("Too few cards in hand");
    }

    // TODO: Classify the hand.

    Ok(PokerHandHandle {
        hand_handle: hand,
        hand_rank,
        card_ranks,
    })
}

impl PartialEq for PokerHandHandle<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_rank == other.hand_rank && self.card_ranks == other.card_ranks
    }
}

impl PartialOrd for PokerHandHandle<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if u8::from(self.hand_rank) < u8::from(other.hand_rank) {
            Some(Ordering::Less)
        } else if u8::from(self.hand_rank) > u8::from(other.hand_rank) {
            Some(Ordering::Greater)
        } else {
            // Compare the card ranks.
            for i in 0..5 {
                if self.card_ranks[i] < other.card_ranks[i] {
                    return Some(Ordering::Less);
                } else if self.card_ranks[i] > other.card_ranks[i] {
                    return Some(Ordering::Greater);
                }
            }
            Some(Ordering::Equal)
        }
    }
}

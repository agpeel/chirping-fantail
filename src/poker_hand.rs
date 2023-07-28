use crate::cards::{Card, Ranks, Suits};
use crate::error::PokerHandError;
use regex::Regex;
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
/// 'hand_handle' is a reference to the hand string from the calling environment so that it can
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
    // Construct a new PokerHand from the hand string.
    pub fn new(hand: &str) -> Result<PokerHand, PokerHandError> {
        let mut cards: Vec<Card>;

        // Parse the hand string.
        match PokerHand::parse_hand_str(hand) {
            Some(parsed_cards) => {
                cards = parsed_cards;
            }
            None => return Err(PokerHandError::new("Invalid poker hand")),
        }

        // Sort the cards from highest rank to lowest.
        cards.sort();
        cards.reverse();

        if PokerHand::check_for_duplicate_cards(&cards) {
            return Err(PokerHandError::new("Duplicate cards in hand"));
        }

        // Classify the hand.
        // The hand is already sorted into the correct order for a HighCard hand.
        let mut hand_rank: PokerHandRanks = PokerHandRanks::HighCard;

        // Check for a flush
        if cards[0].suit == cards[1].suit
            && cards[0].suit == cards[2].suit
            && cards[0].suit == cards[3].suit
            && cards[0].suit == cards[4].suit
        {
            hand_rank = PokerHandRanks::Flush;
        }
        // Check for a straight.
        if (cards[0].rank as isize == cards[1].rank as isize + 1
            && cards[0].rank as isize == cards[2].rank as isize + 2
            && cards[0].rank as isize == cards[3].rank as isize + 3
            && cards[0].rank as isize == cards[4].rank as isize + 4)
            // Check for an Ace-low straight.
            || (cards[0].rank == Ranks::Ace
                && cards[1].rank == Ranks::Five
                && cards[2].rank == Ranks::Four
                && cards[3].rank == Ranks::Three
                && cards[4].rank == Ranks::Two)
        {
            if hand_rank == PokerHandRanks::Flush {
                hand_rank = PokerHandRanks::StraightFlush;
            } else {
                hand_rank = PokerHandRanks::Straight;
            }
            if cards[0].rank == Ranks::Ace && cards[1].rank == Ranks::Five {
                // Move the Ace to the end of the hand.
                let ace = cards.remove(0);
                cards.push(ace);
            }
        }
        // Check four of a kind.
        else if cards[1].rank == cards[2].rank
            && cards[1].rank == cards[3].rank
            && (cards[1].rank == cards[0].rank || cards[1].rank == cards[4].rank)
        {
            hand_rank = PokerHandRanks::FourOfAKind;
            // Move the four of a kind to the front of the hand.
            if cards[4].rank == cards[1].rank {
                cards.swap(0, 4);
            }
        }
        // Check three of a kind and full house.
        else if cards[0].rank == cards[1].rank && cards[0].rank == cards[2].rank {
            if cards[3].rank == cards[4].rank {
                hand_rank = PokerHandRanks::FullHouse;
            } else {
                hand_rank = PokerHandRanks::ThreeOfAKind;
            }
        } else if cards[1].rank == cards[2].rank && cards[1].rank == cards[3].rank {
            hand_rank = PokerHandRanks::ThreeOfAKind;
            // Move the three of a kind to the front of the hand.
            cards.swap(0, 3);
        } else if cards[2].rank == cards[3].rank && cards[2].rank == cards[4].rank {
            if cards[0].rank == cards[1].rank {
                hand_rank = PokerHandRanks::FullHouse;
            } else {
                hand_rank = PokerHandRanks::ThreeOfAKind;
            }
            // Move the three of a kind to the front of the hand.
            cards.swap(0, 3);
            cards.swap(1, 4);
        }
        // Handle one pair and two pairs
        else if cards[0].rank == cards[1].rank {
            if cards[2].rank == cards[3].rank {
                hand_rank = PokerHandRanks::TwoPair;
                if cards[2].rank > cards[0].rank {
                    // Move the second pair to the front.
                    cards.swap(0, 2);
                    cards.swap(1, 3);
                }
            } else if cards[3].rank == cards[4].rank {
                hand_rank = PokerHandRanks::TwoPair;
                // Move the pairs to the front of the hand.
                if cards[3].rank > cards[0].rank {
                    cards.swap(0, 3);
                    cards.swap(1, 4);
                }
                cards.swap(2, 4);
            } else {
                // Pair is already at the front.
                hand_rank = PokerHandRanks::Pair;
            }
        } else if cards[1].rank == cards[2].rank {
            if cards[3].rank == cards[4].rank {
                hand_rank = PokerHandRanks::TwoPair;
                if cards[3].rank > cards[1].rank {
                    // Swap the pairs
                    cards.swap(1, 3);
                    cards.swap(2, 4);
                }
                // Move the pairs to the front
                cards.swap(0, 2);
                cards.swap(2, 4);
            } else {
                hand_rank = PokerHandRanks::Pair;
                // Move the pair to the front.
                cards.swap(0, 2);
            }
        } else if cards[2].rank == cards[3].rank {
            hand_rank = PokerHandRanks::Pair;
            cards.swap(0, 2);
            cards.swap(1, 3);
        } else if cards[3].rank == cards[4].rank {
            hand_rank = PokerHandRanks::Pair;
            // Move the pair to the front.
            cards.swap(2, 4);
            cards.swap(1, 3);
            cards.swap(0, 2);
        }

        Ok(PokerHand {
            hand_handle: hand,
            hand_rank,
            cards,
        })
    }

    fn convert_strings_to_card(rank: &str, suit: &str) -> Card {
        let card_rank: Ranks = match rank {
            "2" => Ranks::Two,
            "3" => Ranks::Three,
            "4" => Ranks::Four,
            "5" => Ranks::Five,
            "6" => Ranks::Six,
            "7" => Ranks::Seven,
            "8" => Ranks::Eight,
            "9" => Ranks::Nine,
            "10" => Ranks::Ten,
            "J" => Ranks::Jack,
            "Q" => Ranks::Queen,
            "K" => Ranks::King,
            "A" => Ranks::Ace,
            _ => panic!("Invalid card rank"),
        };
        let card_suit: Suits = match suit {
            "H" => Suits::Hearts,
            "S" => Suits::Spades,
            "C" => Suits::Clubs,
            "D" => Suits::Diamonds,
            _ => panic!("Invalid card suit"),
        };
        Card::new(card_rank, card_suit)
    }

    /// Check for duplicate cards in a hand.
    fn check_for_duplicate_cards(cards: &Vec<Card>) -> bool {
        // NOTE: even though the cards are sorted, we still need to check every pair
        // as the cards are only sorted by rank, so duplicates may not be adjacent.
        // For example, "4C 4S 4C 3S 2H".
        for i in 0..(cards.len() - 1) {
            for j in i + 1..cards.len() {
                if cards[i] == cards[j] {
                    return true;
                }
            }
        }
        false
    }

    /// Parse the hand string into a vector of cards.
    fn parse_hand_str(hand: &str) -> Option<Vec<Card>> {
        let mut cards: Vec<Card> = Vec::with_capacity(5);

        let re = Regex::new(r"^(?<rank1>[2-9]|10|[JQKA])(?<suit1>[HSCD]) (?<rank2>[2-9]|10|[JQKA])(?<suit2>[HSCD]) (?<rank3>[2-9]|10|[JQKA])(?<suit3>[HSCD]) (?<rank4>[2-9]|10|[JQKA])(?<suit4>[HSCD]) (?<rank5>[2-9]|10|[JQKA])(?<suit5>[HSCD])$").unwrap();
        let Some(caps) = re.captures(hand) else { return None; };
        cards.push(PokerHand::convert_strings_to_card(
            &caps["rank1"],
            &caps["suit1"],
        ));
        cards.push(PokerHand::convert_strings_to_card(
            &caps["rank2"],
            &caps["suit2"],
        ));
        cards.push(PokerHand::convert_strings_to_card(
            &caps["rank3"],
            &caps["suit3"],
        ));
        cards.push(PokerHand::convert_strings_to_card(
            &caps["rank4"],
            &caps["suit4"],
        ));
        cards.push(PokerHand::convert_strings_to_card(
            &caps["rank5"],
            &caps["suit5"],
        ));

        Some(cards)
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

    #[test]
    fn test_parse_hand_str() {
        let hand_str = "9H AS JC 10D 5H";
        let cards = PokerHand::parse_hand_str(hand_str).unwrap();
        assert_eq!(cards[0].rank, Ranks::Nine);
        assert_eq!(cards[0].suit, Suits::Hearts);
        assert_eq!(cards[1].rank, Ranks::Ace);
        assert_eq!(cards[1].suit, Suits::Spades);
        assert_eq!(cards[2].rank, Ranks::Jack);
        assert_eq!(cards[2].suit, Suits::Clubs);
        assert_eq!(cards[3].rank, Ranks::Ten);
        assert_eq!(cards[3].suit, Suits::Diamonds);
        assert_eq!(cards[4].rank, Ranks::Five);
        assert_eq!(cards[4].suit, Suits::Hearts);
    }
    #[test]
    fn test_parse_hand_str_invalid_rank() {
        let hand_str = "9H AS JC 12D 5H";
        let result = PokerHand::parse_hand_str(hand_str);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_hand_str_invalid_suit() {
        let hand_str = "9H AS JK 10D 5H";
        let result = PokerHand::parse_hand_str(hand_str);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_hand_str_not_enough_cards() {
        let hand_str = "9H AS JC 10D";
        let result = PokerHand::parse_hand_str(hand_str);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_hand_str_too_many_cards() {
        let hand_str = "9H AS JC 10D 5H QS";
        let result = PokerHand::parse_hand_str(hand_str);
        assert_eq!(result, None);
    }

    #[test]
    fn test_duplicate_cards() {
        let card1 = PokerHand::parse_hand_str("9H AS JC 10D 5H").unwrap();
        assert!(!PokerHand::check_for_duplicate_cards(&card1));
        let card2 = PokerHand::parse_hand_str("9H AS JC JC 5H").unwrap();
        assert!(PokerHand::check_for_duplicate_cards(&card2));
    }
}

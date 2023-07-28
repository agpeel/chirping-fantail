use crate::cards::{Card, Ranks, Suits};
use crate::error::PokerHandError;
use core::num::dec2flt::parse;
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
        let mut cards: Vec<Card>;

        // Parse the hand string.
        match PokerHand::parse_hand_str(hand) {
            Some(parsed_cards) => {
                cards = parsed_cards;
            }
            None => return Err(PokerHandError::new("Invalid poker hand")),
        }
        // TODO: Check for the same card in the hard.

        // Classify the hand.
        let hand_rank: PokerHandRanks = PokerHandRanks::HighCard;
        todo!();

        Ok(PokerHand {
            hand_handle: hand,
            hand_rank,
            cards,
        })
    }

    fn convert_strings_to_card(rank: &str, suit: &str) -> Card {
        let card_rank: Ranks;
        let card_suit: Suits;
        match rank {
            "2" => card_rank = Ranks::Two,
            "3" => card_rank = Ranks::Three,
            "4" => card_rank = Ranks::Four,
            "5" => card_rank = Ranks::Five,
            "6" => card_rank = Ranks::Six,
            "7" => card_rank = Ranks::Seven,
            "8" => card_rank = Ranks::Eight,
            "9" => card_rank = Ranks::Nine,
            "10" => card_rank = Ranks::Ten,
            "J" => card_rank = Ranks::Jack,
            "Q" => card_rank = Ranks::Queen,
            "K" => card_rank = Ranks::King,
            "A" => card_rank = Ranks::Ace,
            _ => panic!("Invalid card rank"),
        }
        match suit {
            "H" => card_suit = Suits::Hearts,
            "S" => card_suit = Suits::Spades,
            "C" => card_suit = Suits::Clubs,
            "D" => card_suit = Suits::Diamonds,
            _ => panic!("Invalid card suit"),
        }
        Card::new(card_rank, card_suit)
    }

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
}

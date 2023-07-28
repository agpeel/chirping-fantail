mod cards;
mod error;
mod poker_hand;

// use error::PokerHandError;
use poker_hand::PokerHand;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    // Record the hand handles in a mutable vector that will be sorted.
    let mut hand_handles: Vec<PokerHand> = Vec::with_capacity(hands.len());
    for hand in hands {
        let hand_handle = match PokerHand::new(hand) {
            Ok(hand_handle) => hand_handle,
            // Ignore invalid hands and process the rest of the list.
            // TODO:  Only catch error::PokerHandError and propogate other errors.
            //        I tried to do that but could not get it to work.
            Err(_) => continue,
        };
        hand_handles.push(hand_handle);
    }

    hand_handles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    hand_handles.reverse(); // Highest hand first.

    match hand_handles.len() {
        0 => None,
        1 => Some(vec![hand_handles[0].hand_handle]),
        _ => {
            let mut top_hands = vec![hand_handles[0].hand_handle];
            for i in 1..hand_handles.len() {
                if hand_handles[i] == hand_handles[0] {
                    top_hands.push(hand_handles[i].hand_handle);
                } else {
                    break;
                }
            }
            // Return an immutable vector of references to the winning hands.
            // TODO: There must be a way to do this without cloning.
            let return_val = top_hands.clone();
            Some(return_val)
        }
    }
}

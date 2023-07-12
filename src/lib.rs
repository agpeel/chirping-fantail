mod poker_hand;

use poker_hand::build_poker_hand_handle;
use poker_hand::PokerHandHandle;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    // Record the hand handles in a mutable vector that will be sorted.
    let mut hand_handles: Vec<PokerHandHandle> = Vec::with_capacity(hands.len());
    for hand in hands {
        let hand_handle = match build_poker_hand_handle(hand) {
            Ok(hand_handle) => hand_handle,
            // Ignore invalid hands.
            Err(_) => continue,
        };
        hand_handles.push(hand_handle);
    }

    hand_handles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    hand_handles.reverse(); // Highest hand first.

    // TODO: Check for multiple winning hands.
    match hand_handles.len() {
        0 => None,
        _ => Some(vec![hand_handles[0].hand_handle]),
    }
}

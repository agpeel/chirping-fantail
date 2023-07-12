use std::cmp::Ordering;

// Record a reference to the input string slice and derived information about the hand.
#[derive(Debug)]
pub struct PokerHandHandle<'a> {
    pub hand_handle: &'a str,
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

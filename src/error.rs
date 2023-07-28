use std::error::Error;
use std::fmt;

/// An error type for invalid poker hands.
///
/// The message field is a string describing the error.
/// For example, PokerHandError::new("Invalid poker hand")
#[derive(Debug)]
pub struct PokerHandError {
    message: String,
}

impl PokerHandError {
    pub fn new(message: &str) -> PokerHandError {
        PokerHandError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for PokerHandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PokerHandError: {}", self.message)
    }
}

impl Error for PokerHandError {
    fn description(&self) -> &str {
        &self.message
    }
}

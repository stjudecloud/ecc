//! Sentences.

use serde::Serialize;
use serde_with::DeserializeFromStr;
use thiserror::Error;

/// A parse error related to a [`Sentence`].
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    /// The first letter was not capitalized.
    #[error("the first letter in this sentence was not capitalized: {0}")]
    Capitalization(String),

    /// The sentence, with whitespace removed, is empty.
    #[error("the sentence was empty")]
    Empty,

    /// The sentence had surrounding whitespace.
    #[error("the sentence had surrounding whitespace")]
    Whitespace,
}

/// A sentence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, DeserializeFromStr)]
pub struct Sentence(String);

impl std::str::FromStr for Sentence {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Err(ParseError::Empty);
        }

        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let error = "".parse::<Sentence>().unwrap_err();
        assert_eq!(error, ParseError::Empty);

        let error = "   ".parse::<Sentence>().unwrap_err();
        assert_eq!(error, ParseError::Empty);
    }
}

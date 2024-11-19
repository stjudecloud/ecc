//! Characteristic identifiers.

use std::num::NonZeroU64;

use serde::Deserialize;
use serde::Serialize;
use serde::de::Visitor;

/// The prefix of any serialized identifier.
const PREFIX: &str = "ECC";

/// The join character for parts of an identifier.
const JOIN_CHAR: char = '-';

/// A composable characteristic identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Identifier {
    /// A numbered molecular characteristic.
    Molecular(NonZeroU64),

    /// A numbered morphological characteristic.
    Morphological(NonZeroU64),
}

impl Identifier {
    /// Creates a molecular identifier.
    ///
    /// If `n` is 0, [`None`] is returned, as identifiers start at 1.
    pub fn molecular(n: u64) -> Option<Self> {
        if n == 0 {
            return None;
        }

        // SAFETY: we just checked to ensure `n` is not zero, so this will
        // always unwrap.
        Some(Self::Molecular(NonZeroU64::try_from(n).unwrap()))
    }

    /// Creates a morphological identifier.
    ///
    /// If `n` is 0, [`None`] is returned, as identifiers start at 1.
    pub fn morphological(n: u64) -> Option<Self> {
        if n == 0 {
            return None;
        }

        // SAFETY: we just checked to ensure `n` is not zero, so this will
        // always unwrap.
        Some(Self::Morphological(NonZeroU64::try_from(n).unwrap()))
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{PREFIX}{JOIN_CHAR}")?;

        match self {
            Identifier::Molecular(n) => write!(f, "MOLEC{JOIN_CHAR}{n:06}"),
            Identifier::Morphological(n) => write!(f, "MORPH{JOIN_CHAR}{n:06}"),
        }
    }
}

/// An error when parsing an identifier.
#[derive(Debug)]
pub enum ParseError {
    /// An invalid number of parts (as split by `JOIN_CHAR`).
    IncorrectNumberOfParts {
        /// The number of parts found.
        found: usize,

        /// The number of parts expected.
        expected: usize,
    },

    /// An invalid prefix was found.
    InvalidPrefix {
        /// The prefix that was found.
        found: String,
    },

    /// An unknown type was encountered.
    UnknownType(String),

    /// A invalid number was passed.
    InvalidNumber {
        /// The number that was parsed.
        found: String,

        /// The reason the number was invalid.
        reason: String,
    },

    /// An invalid number padding was used.
    InvalidNumberPadding {
        /// The invalid number padding.
        found: String,

        /// What was expected.
        expected: String,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::IncorrectNumberOfParts { found, expected } => write!(
                f,
                "invalid number of parts separated by `{JOIN_CHAR}`: found `{found}` parts, \
                 expected `{expected}` parts"
            ),
            ParseError::InvalidPrefix { found } => {
                write!(f, "invalid prefix: found `{found}`, expected `{PREFIX}`")
            }
            ParseError::UnknownType(r#type) => write!(f, "unknown type: `{type}`"),
            ParseError::InvalidNumber { found, reason } => {
                write!(f, "invalid number: found `{found}`, {reason}")
            }
            ParseError::InvalidNumberPadding { found, expected } => write!(
                f,
                "invalid number padding: found `{found}` but `{expected}` was expected"
            ),
        }
    }
}

impl std::error::Error for ParseError {}

/// The number of expected parts in an identifier.
const EXPECTED_PARTS: usize = 3;

impl std::str::FromStr for Identifier {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(JOIN_CHAR).collect::<Vec<_>>();

        if parts.len() != EXPECTED_PARTS {
            return Err(ParseError::IncorrectNumberOfParts {
                found: parts.len(),
                expected: EXPECTED_PARTS,
            });
        }

        let mut parts = parts.into_iter();

        // SAFETY: we just checked that exactly this many parts exists, so these
        // will always unwrap.
        let prefix = parts.next().unwrap();
        let r#type = parts.next().unwrap();
        let number_as_str = parts.next().unwrap();

        if prefix != PREFIX {
            return Err(ParseError::InvalidPrefix {
                found: prefix.to_string(),
            });
        }

        let number = number_as_str
            .parse::<u64>()
            .map_err(|e| ParseError::InvalidNumber {
                found: number_as_str.to_string(),
                reason: e.to_string(),
            })?;

        let number = NonZeroU64::try_from(number).map_err(|_| ParseError::InvalidNumber {
            found: number_as_str.to_string(),
            reason: String::from("the number in an identifier cannot be zero"),
        })?;

        if number_as_str.len() != 6 {
            return Err(ParseError::InvalidNumberPadding {
                found: number_as_str.to_string(),
                expected: format!("{number_as_str:0>6}"),
            });
        }

        match r#type {
            "MOLEC" => Ok(Self::Molecular(number)),
            "MORPH" => Ok(Self::Morphological(number)),
            v => Err(ParseError::UnknownType(v.to_string())),
        }
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// A visitor for deserializing identifiers.
pub struct IdentifierVisitor;

impl Visitor<'_> for IdentifierVisitor {
    type Value = Identifier;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a valid characteristic identifier")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse::<Identifier>()
            .map_err(|e| E::custom(format!("invalid identifier: {e}")))
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(IdentifierVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::Identifier;

    #[test]
    fn morphological_zero_is_none() {
        assert!(Identifier::morphological(0).is_none());
    }

    #[test]
    fn molecular_zero_is_none() {
        assert!(Identifier::molecular(0).is_none());
    }

    #[test]
    fn display() {
        assert_eq!(
            Identifier::molecular(1).unwrap().to_string(),
            "ECC-MOLEC-000001"
        );
        assert_eq!(
            Identifier::morphological(1).unwrap().to_string(),
            "ECC-MORPH-000001"
        );
    }

    #[test]
    fn parsing() {
        // Valid identifiers.
        "ECC-MORPH-000001".parse::<Identifier>().unwrap();
        "ECC-MOLEC-999999".parse::<Identifier>().unwrap();

        // An nnvalid number of parts.
        let result = "MORPH-000001".parse::<Identifier>().unwrap_err();
        assert_eq!(
            result.to_string().as_str(),
            "invalid number of parts separated by `-`: found `2` parts, expected `3` parts"
        );

        // An invalid prefix.
        let result = "ECV-MORPH-000001".parse::<Identifier>().unwrap_err();
        assert_eq!(
            result.to_string().as_str(),
            "invalid prefix: found `ECV`, expected `ECC`"
        );

        let result = "ecc-MORPH-000001".parse::<Identifier>().unwrap_err();
        assert_eq!(
            result.to_string().as_str(),
            "invalid prefix: found `ecc`, expected `ECC`"
        );

        // An unknown type.
        let result = "ECC-FOO-000001".parse::<Identifier>().unwrap_err();
        assert_eq!(result.to_string().as_str(), "unknown type: `FOO`");

        // Invalid number.
        let result = "ECC-MOLEC-abcdef".parse::<Identifier>().unwrap_err();
        assert_eq!(
            result.to_string().as_str(),
            "invalid number: found `abcdef`, invalid digit found in string"
        );

        let result = "ECC-MOLEC-000".parse::<Identifier>().unwrap_err();
        assert_eq!(
            result.to_string().as_str(),
            "invalid number: found `000`, the number in an identifier cannot be zero"
        );

        // Invalid number padding.
        //
        let result = "ECC-MOLEC-1".parse::<Identifier>().unwrap_err();
        assert_eq!(
            result.to_string().as_str(),
            "invalid number padding: found `1` but `000001` was expected"
        );
    }
}

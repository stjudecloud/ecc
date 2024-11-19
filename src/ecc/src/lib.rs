//! Composable characteristics.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

mod identifier;

pub use identifier::Identifier;

/// A composable characteristic.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "lowercase")]
pub enum Characteristic {
    /// An RFC that is currently being proposed.
    Proposed {
        /// A link to the RFC within which the characteristic is being
        /// discussed.
        rfc: Url,
    },

    /// An RFC that has been provisionally accepted and has entered the settling
    /// phase of adoption.
    Provisional {
        /// A link to the RFC within which the characteristic is being
        /// discussed.
        rfc: Url,
    },

    /// A characteristic that has been adopted.
    Adopted {
        /// The identifier.
        identifier: Identifier,

        /// A link to the RFC within which the characteristic was adopted.
        rfc: Url,

        /// The date that the characteristic was adopted.
        date: DateTime<Utc>,
    },
}

impl Characteristic {
    /// Gets the characteristic's identifier, if one has been assigned.
    pub fn identifier(&self) -> Option<&Identifier> {
        match self {
            Characteristic::Proposed { .. } | Characteristic::Provisional { .. } => None,
            Characteristic::Adopted { identifier, .. } => Some(identifier),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::LazyCell;
    use std::str::FromStr;

    use url::Url;

    use super::*;

    const URL: LazyCell<Url> =
        LazyCell::new(|| Url::from_str("https://github.com/stjudecloud/ecc/issues/1").unwrap());

    #[test]
    fn identifier() {
        let char = Characteristic::Proposed { rfc: URL.clone() };
        assert!(char.identifier().is_none());

        let char = Characteristic::Provisional { rfc: URL.clone() };
        assert!(char.identifier().is_none());

        let identifier = "ECC-MORPH-000001".parse::<Identifier>().unwrap();
        let char = Characteristic::Adopted {
            identifier: identifier.clone(),
            rfc: URL.clone(),
            date: Utc::now(),
        };
        assert_eq!(char.identifier(), Some(&identifier));
    }
}

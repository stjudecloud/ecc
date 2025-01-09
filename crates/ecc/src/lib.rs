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
    /// A characteristic that is currently being proposed.
    Proposed {
        /// A link to the RFC within which the characteristic is being
        /// discussed.
        rfc: Url,
    },

    /// An characteristics that has been accepted in principle and has entered
    /// the settling phase of adoption.
    Provisional {
        /// The provisional identifier.
        identifier: Identifier,

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
        adoption_date: DateTime<Utc>,
    },
}

impl Characteristic {
    /// Gets the characteristic's identifier (if one has been assigned).
    pub fn identifier(&self) -> Option<&Identifier> {
        match self {
            Characteristic::Proposed { .. } => None,
            Characteristic::Provisional { identifier, .. }
            | Characteristic::Adopted { identifier, .. } => Some(identifier),
        }
    }

    /// Gets the URL for the associated RFC.
    pub fn rfc(&self) -> &Url {
        match self {
            Characteristic::Proposed { rfc } => rfc,
            Characteristic::Provisional { rfc, .. } => rfc,
            Characteristic::Adopted { rfc, .. } => rfc,
        }
    }

    /// Gets the adoption date (if it the charactertistic has been adopted).
    pub fn adoption_date(&self) -> Option<&DateTime<Utc>> {
        match self {
            Characteristic::Proposed { .. } | Characteristic::Provisional { .. } => None,
            Characteristic::Adopted { adoption_date, .. } => Some(adoption_date),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::sync::LazyLock;

    use url::Url;

    use super::*;

    static URL: LazyLock<Url> =
        LazyLock::new(|| Url::from_str("https://github.com/stjudecloud/ecc/issues/1").unwrap());

    #[test]
    fn identifier() {
        let char = Characteristic::Proposed { rfc: URL.clone() };
        assert!(char.identifier().is_none());

        let identifier = "ECC-MORPH-000001".parse::<Identifier>().unwrap();
        let char = Characteristic::Provisional {
            identifier: identifier.clone(),
            rfc: URL.clone(),
        };
        assert_eq!(char.identifier(), Some(&identifier));

        let identifier = "ECC-MOLEC-000001".parse::<Identifier>().unwrap();
        let char = Characteristic::Adopted {
            identifier: identifier.clone(),
            rfc: URL.clone(),
            adoption_date: Utc::now(),
        };
        assert_eq!(char.identifier(), Some(&identifier));
    }
}

//! Composable characteristics.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

mod identifier;
pub mod rfc;

pub use identifier::Identifier;
pub use rfc::Link;

/// A composable characteristic.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "lowercase")]
pub enum Characteristic {
    /// A characteristic that is currently being proposed.
    Proposed {
        /// A link to the RFC within which the characteristic is being
        /// discussed.
        rfc: rfc::Link,
    },

    /// An characteristics that has been accepted in principle and has entered
    /// the settling phase of adoption.
    Provisional {
        /// The provisional identifier.
        identifier: Identifier,

        /// A link to the RFC within which the characteristic is being
        /// discussed.
        rfc: rfc::Link,
    },

    /// A characteristic that has been adopted.
    Adopted {
        /// The identifier.
        identifier: Identifier,

        /// A link to the RFC within which the characteristic was adopted.
        rfc: rfc::Link,

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
    pub fn rfc(&self) -> &Link {
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
    use std::sync::LazyLock;

    use super::*;

    static RFC_LINK: LazyLock<Link> = LazyLock::new(|| {
        "https://github.com/stjudecloud/ecc/issues/1"
            .parse::<Link>()
            .unwrap()
    });

    #[test]
    fn identifier() {
        let char = Characteristic::Proposed {
            rfc: RFC_LINK.clone(),
        };
        assert!(char.identifier().is_none());

        let identifier = "ECC-MORPH-000001".parse::<Identifier>().unwrap();
        let char = Characteristic::Provisional {
            identifier: identifier.clone(),
            rfc: RFC_LINK.clone(),
        };
        assert_eq!(char.identifier(), Some(&identifier));

        let identifier = "ECC-MOLEC-000001".parse::<Identifier>().unwrap();
        let char = Characteristic::Adopted {
            identifier: identifier.clone(),
            rfc: RFC_LINK.clone(),
            adoption_date: Utc::now(),
        };
        assert_eq!(char.identifier(), Some(&identifier));
    }
}

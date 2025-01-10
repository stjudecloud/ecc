//! Composable characteristics.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

mod common;
mod identifier;
pub mod rfc;

use common::Common;
pub use identifier::Identifier;
pub use rfc::Link;

/// A composable characteristic.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "lowercase")]
pub enum Characteristic {
    /// A characteristic that is currently being proposed.
    Proposed {
        /// The common set of elements for any characteristic.
        #[serde(flatten)]
        common: Common,
    },

    /// An characteristics that has been accepted in principle and has entered
    /// the settling phase of adoption.
    Provisional {
        /// The provisional identifier.
        identifier: Identifier,

        /// The common set of elements for any characteristic.
        #[serde(flatten)]
        common: Common,
    },

    /// A characteristic that has been adopted.
    Adopted {
        /// The identifier.
        identifier: Identifier,

        /// The common set of elements for any characteristic.
        #[serde(flatten)]
        common: Common,

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
            Characteristic::Proposed { common } => common.rfc(),
            Characteristic::Provisional { common, .. } => common.rfc(),
            Characteristic::Adopted { common, .. } => common.rfc(),
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
            common: Common {
                rfc: RFC_LINK.clone(),
            },
        };
        assert!(char.identifier().is_none());

        let identifier = "ECC-MORPH-000001".parse::<Identifier>().unwrap();
        let char = Characteristic::Provisional {
            identifier: identifier.clone(),
            common: Common {
                rfc: RFC_LINK.clone(),
            },
        };
        assert_eq!(char.identifier(), Some(&identifier));

        let identifier = "ECC-MOLEC-000001".parse::<Identifier>().unwrap();
        let char = Characteristic::Adopted {
            identifier: identifier.clone(),
            common: Common {
                rfc: RFC_LINK.clone(),
            },
            adoption_date: Utc::now(),
        };
        assert_eq!(char.identifier(), Some(&identifier));
    }
}

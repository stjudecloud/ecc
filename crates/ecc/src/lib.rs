//! Composable characteristics.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

mod common;
mod identifier;
pub mod rfc;

use common::Common;
use common::OptionalCommon;
pub use identifier::Identifier;
pub use rfc::Link;

/// A composable characteristic.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "lowercase")]
pub enum Characteristic {
    /// A characteristic that is currently being drafted.
    Draft {
        /// The common set of elements for any characteristic.
        ///
        /// In this case, the set of features is captured by an
        /// [`OptionalCommon`], indicating that fields may or may not be present
        /// in draft phase.
        #[serde(flatten)]
        common: OptionalCommon,
    },

    /// A characteristic that is currently being proposed to be adopted.
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
            Characteristic::Draft { .. } | Characteristic::Proposed { .. } => None,
            Characteristic::Provisional { identifier, .. }
            | Characteristic::Adopted { identifier, .. } => Some(identifier),
        }
    }

    /// Gets the URL for the associated RFC.
    pub fn rfc(&self) -> Option<&Link> {
        match self {
            Characteristic::Draft { common } => common.rfc(),
            Characteristic::Proposed { common } => Some(common.rfc()),
            Characteristic::Provisional { common, .. } => Some(common.rfc()),
            Characteristic::Adopted { common, .. } => Some(common.rfc()),
        }
    }

    /// Gets the adoption date (if it the charactertistic has been adopted).
    pub fn adoption_date(&self) -> Option<&DateTime<Utc>> {
        match self {
            Characteristic::Draft { .. }
            | Characteristic::Proposed { .. }
            | Characteristic::Provisional { .. } => None,
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
    fn features() {
        //=======//
        // Draft //
        //=======//

        let draft = Characteristic::Draft {
            common: OptionalCommon {
                rfc: Some(RFC_LINK.clone()),
            },
        };
        assert!(draft.identifier().is_none());
        assert_eq!(
            draft.rfc().unwrap().as_str(),
            "https://github.com/stjudecloud/ecc/issues/1"
        );
        assert!(draft.adoption_date().is_none());

        //==========//
        // Proposed //
        //==========//

        let proposed = Characteristic::Proposed {
            common: Common {
                rfc: RFC_LINK.clone(),
            },
        };
        assert!(proposed.identifier().is_none());
        assert_eq!(
            proposed.rfc().unwrap().as_str(),
            "https://github.com/stjudecloud/ecc/issues/1"
        );
        assert!(proposed.adoption_date().is_none());

        //=============//
        // Provisional //
        //=============//

        let identifier = "ECC-MORPH-000001".parse::<Identifier>().unwrap();
        let provisional = Characteristic::Provisional {
            identifier: identifier.clone(),
            common: Common {
                rfc: RFC_LINK.clone(),
            },
        };
        assert!(provisional.identifier().is_some());
        assert_eq!(
            provisional.rfc().unwrap().as_str(),
            "https://github.com/stjudecloud/ecc/issues/1"
        );
        assert!(provisional.adoption_date().is_none());

        //=========//
        // Adopted //
        //=========//

        let identifier = "ECC-MOLEC-000001".parse::<Identifier>().unwrap();
        let adopted = Characteristic::Adopted {
            identifier: identifier.clone(),
            common: Common {
                rfc: RFC_LINK.clone(),
            },
            adoption_date: Utc::now(),
        };
        assert_eq!(adopted.identifier(), Some(&identifier));
        assert_eq!(
            adopted.rfc().unwrap().as_str(),
            "https://github.com/stjudecloud/ecc/issues/1"
        );
        assert!(adopted.adoption_date().is_some());
    }
}

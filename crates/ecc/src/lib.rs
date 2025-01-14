//! Composable characteristics.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

mod common;
pub mod field;
mod identifier;
pub mod rfc;
pub mod text;

use common::Common;
use common::OptionalCommon;
pub use identifier::Identifier;
pub use rfc::Link;

use crate::common::Reference;
use crate::common::value::Kind;

/// A composable characteristic.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "lowercase", deny_unknown_fields)]
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
        /// The common set of elements for any characteristic.
        #[serde(flatten)]
        common: Common,
    },

    /// A characteristic that has been adopted.
    Adopted {
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
            Characteristic::Draft { common, .. } => common.identifier.as_ref(),
            Characteristic::Proposed { common }
            | Characteristic::Provisional { common }
            | Characteristic::Adopted { common, .. } => Some(&common.identifier),
        }
    }

    /// Gets the name.
    pub fn name(&self) -> Option<&str> {
        match self {
            Characteristic::Draft { common } => common.name.as_deref(),
            Characteristic::Proposed { common }
            | Characteristic::Provisional { common, .. }
            | Characteristic::Adopted { common, .. } => Some(&common.name),
        }
    }

    /// Gets the URL for the associated RFC.
    pub fn rfc(&self) -> Option<&Link> {
        match self {
            Characteristic::Draft { common } => common.rfc.as_ref(),
            Characteristic::Proposed { common }
            | Characteristic::Provisional { common, .. }
            | Characteristic::Adopted { common, .. } => Some(&common.rfc),
        }
    }

    /// Gets the permissible values.
    pub fn values(&self) -> Option<&Kind> {
        match self {
            Characteristic::Draft { common } => common.values.as_ref(),
            Characteristic::Proposed { common }
            | Characteristic::Provisional { common, .. }
            | Characteristic::Adopted { common, .. } => Some(&common.values),
        }
    }

    /// Gets the description.
    pub fn description(&self) -> Option<&str> {
        match self {
            Characteristic::Draft { common } => common.description.as_deref(),
            Characteristic::Proposed { common }
            | Characteristic::Provisional { common, .. }
            | Characteristic::Adopted { common, .. } => Some(common.description.as_str()),
        }
    }

    /// Gets the references.
    pub fn references(&self) -> Option<impl Iterator<Item = &Reference>> {
        match self {
            Characteristic::Draft { common } => common
                .references
                .as_ref()
                .map(|publications| publications.iter()),
            Characteristic::Proposed { common } => common
                .references
                .as_ref()
                .map(|publications| publications.iter()),
            Characteristic::Provisional { common, .. } => common
                .references
                .as_ref()
                .map(|publications| publications.iter()),
            Characteristic::Adopted { common, .. } => common
                .references
                .as_ref()
                .map(|publications| publications.iter()),
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

    use nonempty::NonEmpty;
    use url::Url;

    use super::*;
    use crate::common::Reference;
    use crate::common::value::Kind;
    use crate::text::Sentence;

    static RFC_LINK: LazyLock<Link> = LazyLock::new(|| {
        "https://github.com/stjudecloud/ecc/issues/1"
            .parse::<Link>()
            .unwrap()
    });

    #[test]
    fn features() {
        let identifier = "ECC-MORPH-000001".parse::<Identifier>().unwrap();

        let values = Kind::Binary {
            description: crate::common::value::kind::binary::Description {
                r#true: field::Description {
                    summary: "Foo".parse::<Sentence>().unwrap(),
                    details: "Bar".parse::<Sentence>().unwrap(),
                },
                r#false: field::Description {
                    summary: "Baz".parse::<Sentence>().unwrap(),
                    details: "Quux".parse::<Sentence>().unwrap(),
                },
            },
        };

        //=======//
        // Draft //
        //=======//

        let draft = Characteristic::Draft {
            common: OptionalCommon {
                name: Some(String::from("A Characteristic Name")),
                identifier: None,
                rfc: Some(RFC_LINK.clone()),
                values: Some(values.clone()),
                description: Some(String::from("A description")),
                references: Some(NonEmpty::new(Reference::Manuscript {
                    title: String::from("The Discovery of Foo Bar"),
                    authors: String::from("Jane Smith"),
                    context: "Some context about the manuscript"
                        .parse::<Sentence>()
                        .unwrap(),
                    url: "https://nature.org/the-discovery-of-foo-bar"
                        .parse::<Url>()
                        .unwrap(),
                    highlighted: false,
                })),
            },
        };

        assert!(draft.identifier().is_none());
        assert_eq!(draft.name().unwrap(), "A Characteristic Name");
        assert_eq!(
            draft.rfc().unwrap().as_str(),
            "https://github.com/stjudecloud/ecc/issues/1"
        );
        assert_eq!(draft.description().unwrap(), "A description");
        assert_eq!(draft.values().unwrap(), &values);
        assert_eq!(draft.references().unwrap().count(), 1);
        assert!(draft.adoption_date().is_none());

        //==========//
        // Proposed //
        //==========//

        let proposed = Characteristic::Proposed {
            common: Common {
                name: String::from("A Characteristic Name"),
                identifier: identifier.clone(),
                rfc: RFC_LINK.clone(),
                values: values.clone(),
                description: String::from("A description"),
                references: Some(NonEmpty::new(Reference::Manuscript {
                    title: String::from("The Discovery of Foo Bar"),
                    authors: String::from("Jane Smith"),
                    context: "Some context about the manuscript"
                        .parse::<Sentence>()
                        .unwrap(),
                    url: "https://nature.org/the-discovery-of-foo-bar"
                        .parse::<Url>()
                        .unwrap(),
                    highlighted: false,
                })),
            },
        };

        assert_eq!(proposed.identifier().unwrap(), &identifier);
        assert_eq!(draft.name().unwrap(), "A Characteristic Name");
        assert_eq!(
            proposed.rfc().unwrap().as_str(),
            "https://github.com/stjudecloud/ecc/issues/1"
        );
        assert_eq!(draft.description().unwrap(), "A description");
        assert_eq!(draft.values().unwrap(), &values);
        assert_eq!(draft.references().unwrap().count(), 1);
        assert!(proposed.adoption_date().is_none());

        //=============//
        // Provisional //
        //=============//

        let provisional = Characteristic::Provisional {
            common: Common {
                name: String::from("A Characteristic Name"),
                identifier: identifier.clone(),
                rfc: RFC_LINK.clone(),
                values: values.clone(),
                description: String::from("A description"),
                references: Some(NonEmpty::new(Reference::Manuscript {
                    title: String::from("The Discovery of Foo Bar"),
                    authors: String::from("Jane Smith"),
                    context: "Some context about the manuscript"
                        .parse::<Sentence>()
                        .unwrap(),
                    url: "https://nature.org/the-discovery-of-foo-bar"
                        .parse::<Url>()
                        .unwrap(),
                    highlighted: false,
                })),
            },
        };

        assert_eq!(proposed.identifier().unwrap(), &identifier);
        assert_eq!(draft.name().unwrap(), "A Characteristic Name");
        assert_eq!(
            provisional.rfc().unwrap().as_str(),
            "https://github.com/stjudecloud/ecc/issues/1"
        );
        assert_eq!(draft.description().unwrap(), "A description");
        assert_eq!(draft.values().unwrap(), &values);
        assert_eq!(draft.references().unwrap().count(), 1);
        assert!(provisional.adoption_date().is_none());

        //=========//
        // Adopted //
        //=========//

        let adopted = Characteristic::Adopted {
            common: Common {
                name: String::from("A Characteristic Name"),
                identifier: identifier.clone(),
                rfc: RFC_LINK.clone(),
                values: values.clone(),
                description: String::from("A description"),
                references: Some(NonEmpty::new(Reference::Manuscript {
                    title: String::from("The Discovery of Foo Bar"),
                    authors: String::from("Jane Smith"),
                    context: "Some context about the manuscript"
                        .parse::<Sentence>()
                        .unwrap(),
                    url: "https://nature.org/the-discovery-of-foo-bar"
                        .parse::<Url>()
                        .unwrap(),
                    highlighted: false,
                })),
            },
            adoption_date: Utc::now(),
        };

        assert_eq!(proposed.identifier().unwrap(), &identifier);
        assert_eq!(draft.name().unwrap(), "A Characteristic Name");
        assert_eq!(
            adopted.rfc().unwrap().as_str(),
            "https://github.com/stjudecloud/ecc/issues/1"
        );
        assert_eq!(draft.description().unwrap(), "A description");
        assert_eq!(draft.values().unwrap(), &values);
        assert_eq!(draft.references().unwrap().count(), 1);
        assert!(adopted.adoption_date().is_some());
    }
}

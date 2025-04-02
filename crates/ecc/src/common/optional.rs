//! Optional common feature sets.
//!
//! This module contains a struct that represents a [`Common`], but with all of
//! the fields being optional. This is useful when a characteristic is in
//! `draft` phase so as to not upset the deserializer when information is
//! missing.

use nonempty::NonEmpty;
use serde::Deserialize;
use serde::Serialize;

use crate::Identifier;
use crate::common::Common;
use crate::common::Reference;
use crate::common::value;
use crate::rfc;

/// An "option common" feature set.
///
/// This represents a [`Common`] where all of the fields are optional. This
/// feature set should only be relevant for characteristics in the `draft` phase
/// where all of the information might not yet be filled in.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OptionalCommon {
    /// The name of the characteristic.
    pub name: Option<String>,

    /// The provisional identifier.
    pub identifier: Option<Identifier>,

    /// A link to the RFC within which the characteristic has been and is being
    /// discussed.
    ///
    /// Any questions regarding the characteristic after adoption should also be
    /// organized here.
    pub rfc: Option<rfc::Link>,

    /// A description.
    pub description: Option<String>,

    /// The permissible values that the characteristic takes.
    pub values: Option<value::Kind>,

    /// An optional list of publications.
    pub references: Option<NonEmpty<Reference>>,
}

impl OptionalCommon {
    /// Consumes `self` and returns a [`Common`].
    ///
    /// This method largely exists to statically verify that every field in
    /// [`Common`] also exists optionally in [`OptionalCommon`]. There is not
    /// intended use-case for it in the code.
    #[allow(dead_code)]
    pub fn into_common(self) -> Common {
        Common {
            name: self.name.expect("`name` to be present"),
            identifier: self.identifier.expect("`identifier` to be present"),
            rfc: self.rfc.expect("`rfc` to be present"),
            description: self.description.expect("`description` to be present"),
            values: self.values.expect("`values` to be present"),
            references: self.references,
        }
    }
}

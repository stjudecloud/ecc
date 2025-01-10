//! Optional common feature sets.
//!
//! This module contains a struct that represents a [`Common`], but with all of
//! the fields being optional. This is useful when a characteristic is in
//! `draft` phase so as to not upset the deserializer when information is
//! missing.

use serde::Deserialize;
use serde::Serialize;

use crate::common::Common;
use crate::rfc;

/// An "option common" feature set.
///
/// This represents a [`Common`] where all of the fields are optional. This
/// feature set should only be relevant for characteristics in the `draft` phase
/// where all of the information might not yet be filled in.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionalCommon {
    /// The name of the characteristic.
    pub name: Option<String>,

    /// A link to the RFC within which the characteristic has been and is being
    /// discussed.
    ///
    /// Any questions regarding the characteristic after adoption should also be
    /// organized here.
    pub rfc: Option<rfc::Link>,
}

impl OptionalCommon {
    /// Gets the name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Gets the RFC link.
    pub fn rfc(&self) -> Option<&rfc::Link> {
        self.rfc.as_ref()
    }

    /// Consumes `self` and returns a [`Common`].
    ///
    /// This method largely exists to statically verify that every field in
    /// [`Common`] also exists optionally in [`OptionalCommon`]. There is not
    /// intended use-case for it in the code.
    #[allow(dead_code)]
    pub fn into_common(self) -> Common {
        Common {
            name: self.name.expect("`name` to be present"),
            rfc: self.rfc.expect("`rfc` to be present"),
        }
    }
}

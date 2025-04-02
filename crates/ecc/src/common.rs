//! Common features for a composable characteristics.

use nonempty::NonEmpty;
use serde::Deserialize;
use serde::Serialize;

use crate::Identifier;
use crate::rfc;

mod optional;
mod reference;
pub mod value;

pub use optional::OptionalCommon;
pub use reference::Reference;

/// Common features for composable characteristics in any stage.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Common {
    /// The name.
    pub name: String,

    /// The provisional identifier.
    pub identifier: Identifier,

    /// A link to the RFC for the characteristic.
    ///
    /// All discussion of the characteristic, whether in the draft phase or
    /// questions after adoption, should occur within this RFC link.
    pub rfc: rfc::Link,

    /// A description.
    pub description: String,

    /// The permissible values that the characteristic takes.
    pub values: value::Kind,

    /// An optional list of publications.
    pub references: Option<NonEmpty<Reference>>,
}

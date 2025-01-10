//! Common features for a composable characteristics.

use nonempty::NonEmpty;
use serde::Deserialize;
use serde::Serialize;

use crate::rfc;

mod optional;
mod reference;

pub use optional::OptionalCommon;
pub use reference::Reference;

/// Common features for composable characteristics in any stage.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Common {
    /// The name of the characteristic.
    pub name: String,

    /// A link to the RFC within which the characteristic has been and is being
    /// discussed.
    ///
    /// Any questions regarding the characteristic after adoption should also be
    /// organized here.
    pub rfc: rfc::Link,

    /// An optional list of publications.
    pub references: Option<NonEmpty<Reference>>,
}

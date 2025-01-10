//! Common features for a composable characteristics.

use serde::Deserialize;
use serde::Serialize;

use crate::rfc;

mod optional;

pub use optional::OptionalCommon;

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
}

impl Common {
    /// Gets the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the RFC link.
    pub fn rfc(&self) -> &rfc::Link {
        &self.rfc
    }
}

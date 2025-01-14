//! Binary features.

use serde::Deserialize;
use serde::Serialize;

use crate::field;

/// The description of a binary feature kind.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Description {
    /// A description of the `true` field value.
    pub r#true: field::Description,

    /// A description of the `false` field value.
    pub r#false: field::Description,
}

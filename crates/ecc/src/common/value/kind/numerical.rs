//! Numerical features.

use serde::Deserialize;
use serde::Serialize;

/// A numerical feature type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    /// An signed integer.
    Signed,

    /// An unsigned integer.
    Unsigned,

    /// A float.
    Float,
}

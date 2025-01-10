//! Permissible values for characteristics.

use std::collections::HashSet;

use serde::Deserialize;
use serde::Serialize;

/// A numberical feature type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Numerical {
    /// An signed integer.
    Signed,

    /// An unsigned integer.
    Unsigned,

    /// A float.
    Float,
}

/// A permissible value for a characteristic.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Kind {
    /// A binary feature.
    ///
    /// Examples of binary features a "yes"/"no", "true"/"false", and
    /// "present"/"not present" determinations.
    Binary,

    /// A categorical feature.
    Categorical {
        /// The set of values that the feature can take on.
        options: HashSet<String>,
    },

    /// A numerical feature.
    Numerical {
        /// The type of numerical feature.
        r#type: Numerical,

        /// A description of the units of measurement.
        units: String,
    },
}

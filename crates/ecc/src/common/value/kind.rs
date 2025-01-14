//! Kinds of permissible values.

use std::collections::HashSet;

use serde::Deserialize;
use serde::Serialize;

pub mod binary;
pub mod numerical;

/// A permissible value for a characteristic.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Kind {
    /// A binary feature.
    ///
    /// Examples of binary features a "yes"/"no", "true"/"false", and
    /// "present"/"not present" determinations.
    Binary {
        /// The description.
        description: binary::Description,
    },

    /// A categorical feature.
    Categorical {
        /// The set of values that the feature can take on.
        options: HashSet<String>,
    },

    /// A numerical feature.
    Numerical {
        /// The type of numerical feature.
        r#type: numerical::Type,

        /// A description of the units of measurement.
        units: String,
    },
}

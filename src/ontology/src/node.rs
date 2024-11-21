//! Nodes within the ontology.

use serde::Deserialize;
use serde::Serialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;

pub mod builder;
pub mod name;

pub use builder::Builder;
pub use name::Name;

/// A node in the ontology.
#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node {
    /// The name of the node.
    #[serde_as(as = "DisplayFromStr")]
    name: Name,

    /// The name of the parent node.
    #[serde_as(as = "DisplayFromStr")]
    parent: Name,

    /// The short code for the node.
    code: String,
    // NOTE: if you add or remove fields here, you need to update the help
    // message in the `ontology init` subcommand to ensure each column is
    // documented.
}

impl Node {
    /// Gets the node name.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Consumes `self` and returns the node name.
    pub fn into_name(self) -> Name {
        self.name
    }

    /// Gets the node's parent.
    pub fn parent(&self) -> &Name {
        &self.parent
    }

    /// Consumes `self` and returns the node's parent.
    pub fn into_parent(self) -> Name {
        self.parent
    }

    /// Gets the short code of the node.
    pub fn code(&self) -> &str {
        self.code.as_str()
    }

    /// Consumes `self` and returns the node's short code.
    pub fn into_code(self) -> String {
        self.code
    }
}

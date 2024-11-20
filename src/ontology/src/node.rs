//! Nodes within the ontology.

use serde::Deserialize;
use serde::Serialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;

pub mod name;

pub use name::Name;

/// A node in the ontology.
#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node {
    /// The name of the node.
    #[serde_as(as = "DisplayFromStr")]
    name: Name,

    /// The name of the parent node.
    #[serde_as(as = "DisplayFromStr")]
    parent: Name,
}

impl Node {
    /// Creates a new node.
    pub fn new(name: Name, parent: Name) -> Self {
        Self { name, parent }
    }
}

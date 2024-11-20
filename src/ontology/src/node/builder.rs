//! Node builders.

use super::Name;
use super::Node;

/// An error when using a node builder.
#[derive(Debug)]
pub enum Error {
    /// A required field was missing.
    MissingField(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingField(field) => write!(f, "missing required field: {field}"),
        }
    }
}

impl std::error::Error for Error {}

/// A builder for a node.
#[derive(Default)]
pub struct Builder {
    /// The node.
    name: Option<Name>,

    /// The parent node.
    parent: Option<Name>,
}

impl Builder {
    /// Sets the name for the node.
    pub fn name(mut self, value: Name) -> Self {
        self.name = Some(value);
        self
    }

    /// Sets the parent for the node.
    pub fn parent(mut self, value: Name) -> Self {
        self.parent = Some(value);
        self
    }

    /// Consumes self and tries to return a built node.
    pub fn try_build(self) -> Result<Node, Error> {
        let name = self.name.ok_or(Error::MissingField("name"))?;
        let parent = self.parent.ok_or(Error::MissingField("parent"))?;

        Ok(Node { name, parent })
    }
}

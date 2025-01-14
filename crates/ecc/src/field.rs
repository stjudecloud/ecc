//! Fields within the data model.

use serde::Deserialize;
use serde::Serialize;

use crate::text::Sentence;

/// A field description.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Description {
    /// A summary.
    ///
    /// This text is formatted as a single sentence.
    pub summary: Sentence,

    /// A full set of details.
    ///
    /// This field is formatted as a Markdown rich text field.
    pub details: Sentence,
}

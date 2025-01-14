//! References.

use serde::Deserialize;
use serde::Serialize;
use url::Url;

use crate::text::Sentence;

/// A reference.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Reference {
    /// A peer-reviewed published manuscript.
    Manuscript {
        /// The title of the publication.
        title: String,

        /// The authors of the manuscript.
        authors: String,

        /// Discusses the contextual relevance of this manuscript for this ECC.
        context: Sentence,

        /// A URL where the publication can be accessed.
        url: Url,

        /// Whether or not the manuscript should be highlighted or not.
        highlighted: bool,
    },

    /// A non-peer reviewed preprint.
    Preprint {
        /// The title of the preprint.
        title: String,

        /// The authors of the manuscript.
        authors: String,

        /// Discusses the contextual relevance of this preprint for this ECC.
        context: Sentence,

        /// A URL where the preprint can be accessed.
        url: Url,

        /// Whether or not the preprint should be highlighted or not.
        highlighted: bool,
    },
}

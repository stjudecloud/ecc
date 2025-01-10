//! References.

use serde::Deserialize;
use serde::Serialize;
use url::Url;

/// A reference.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Reference {
    /// A peer-reviewed published manuscript.
    Manuscript {
        /// The title of the publication.
        title: String,

        /// A URL where the publication can be accessed.
        url: Url,
    },

    /// A non-peer reviewed preprint.
    Preprint {
        /// The title of the preprint.
        title: String,

        /// A URL where the preprint can be accessed.
        url: Url,
    },
}

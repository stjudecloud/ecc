//! Request for comments.

use std::ops::Deref;
use std::sync::LazyLock;

use regex::Regex;
use serde_with::DeserializeFromStr;
use serde_with::SerializeDisplay;
use url::Url;

/// The regex that the link needs to match to be valid.
static VALID_LINK_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("https://github.com/stjudecloud/ecc/issues/[0-9]+").unwrap());

////////////////////////////////////////////////////////////////////////////////////////
// Errors
////////////////////////////////////////////////////////////////////////////////////////

/// A parsing error for a link.
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    /// An invalid link.
    Invalid {
        /// The value that was attempted to be parsed.
        value: String,
    },

    /// A url parse error.
    Url {
        /// The value that was attempted to be parsed.
        value: String,

        /// The parse error from `url`.
        error: url::ParseError,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Invalid { value } => {
                write!(
                    f,
                    "invalid link: `{}`; must point to a GitHub issue on the `stjudecloud/ecc` \
                     repository",
                    value
                )
            }
            ParseError::Url { value, error } => {
                write!(f, "url parse error: `{}`; {}", value, error)
            }
        }
    }
}

impl std::error::Error for ParseError {}

////////////////////////////////////////////////////////////////////////////////////////
// Link
////////////////////////////////////////////////////////////////////////////////////////

/// A link to an RFC for a composable characteristic.
#[derive(Clone, Debug, PartialEq, Eq, SerializeDisplay, DeserializeFromStr)]
pub struct Link(Url);

impl Link {
    /// Returns a reference to the inner URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use ecc::rfc::Link;
    /// use url::Url;
    ///
    /// let link = "https://github.com/stjudecloud/ecc/issues/1"
    ///     .parse::<Link>()
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     link.url(),
    ///     &"https://github.com/stjudecloud/ecc/issues/1"
    ///         .parse::<Url>()
    ///         .unwrap()
    /// );
    /// ```
    pub fn url(&self) -> &Url {
        &self.0
    }

    /// Consumes `self` and returns the inner URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use ecc::rfc::Link;
    /// use url::Url;
    ///
    /// let link = "https://github.com/stjudecloud/ecc/issues/1"
    ///     .parse::<Link>()
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     link.into_url(),
    ///     "https://github.com/stjudecloud/ecc/issues/1"
    ///         .parse::<Url>()
    ///         .unwrap()
    /// );
    /// ```
    pub fn into_url(self) -> Url {
        self.0
    }
}

impl Deref for Link {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Link {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = s.parse::<Url>().map_err(|error| ParseError::Url {
            value: s.to_string(),
            error,
        })?;

        if VALID_LINK_REGEX.is_match(url.as_str()) {
            Ok(Self(url))
        } else {
            Err(ParseError::Invalid {
                value: s.to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let link = "https://github.com/stjudecloud/ecc/issues/1"
            .parse::<Link>()
            .unwrap();

        assert_eq!(
            link.to_string(),
            "https://github.com/stjudecloud/ecc/issues/1"
        )
    }

    #[test]
    fn bad_url() {
        let err = "/home/foo/bar".parse::<Link>().unwrap_err();
        assert!(matches!(err, ParseError::Url { .. }));
    }

    #[test]
    fn invalid() {
        let err = "https://github.com/stjudecloud/ecc/issues/"
            .parse::<Link>()
            .unwrap_err();
        assert_eq!(err, ParseError::Invalid {
            value: String::from("https://github.com/stjudecloud/ecc/issues/")
        });

        let err = "https://github.com/stjudecloud/another-repo/issues/1"
            .parse::<Link>()
            .unwrap_err();
        assert_eq!(err, ParseError::Invalid {
            value: String::from("https://github.com/stjudecloud/another-repo/issues/1")
        });
    }
}

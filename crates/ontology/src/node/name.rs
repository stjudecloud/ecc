//! Names for ontology nodes.

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::LazyLock;

/// The words that are expected to be lowercase.
const LOWERCASE_WORDS: &[&str] = &[
    "and",
    // The standalone word `like` is included here because there are many
    // existing references to `-like` (which is always lowercase). For visual
    // consistency, we will treat even standalone `like` as lowercase.
    "like", "the", "of", "or", "with",
];

/// After words are converted to title case, any phrases that are matched with
/// the keys are replaced with the values of the map. This allows us to do
/// things like change `Non-hodgkin` to `Non-Hodgkin` easily.
static TITLE_CASE_REPLACEMENTS: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        let mut hm = HashMap::new();

        // A list of names that should always be capitalized.
        hm.insert("hodgkin", "Hodgkin");
        hm.insert("barr", "Barr");
        hm.insert("dorfman", "Dorfman");
        hm.insert("leydig", "Leydig");

        // Intrachromosomal amplification of chromosome 21 (iAMP21) has a
        // specific nomenclature and should keep that casing.
        hm.insert("Iamp21", "iAMP21");

        hm
    });

/// A string that is validated to only contain ASCII characters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AsciiString(String);

impl AsciiString {
    /// Attempts to create a new ASCII string.
    pub fn new(value: String) -> Option<Self> {
        if value.is_ascii() {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Consumes `self` and returns the inner [`String`].
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Converts the ASCII string to lowercase.
    pub fn to_lowercase(&self) -> Self {
        Self(self.0.to_ascii_lowercase())
    }

    /// Converts the ASCII string to uppercase.
    pub fn to_uppercase(&self) -> Self {
        Self(self.0.to_ascii_uppercase())
    }

    /// Converts the ASCII string to title case.
    pub fn to_title_case(&self) -> Self {
        let mut next_letter_uppercase = true;

        let mut chars = Vec::new();

        for c in self.0.chars() {
            if next_letter_uppercase {
                chars.push(c.to_ascii_uppercase());
            } else {
                chars.push(c.to_ascii_lowercase());
            }

            next_letter_uppercase = c == '/';
        }

        let mut result = chars.into_iter().collect::<String>();

        for (k, v) in TITLE_CASE_REPLACEMENTS.iter() {
            result = result.replace(k, v);
        }

        Self(result)
    }
}

impl Deref for AsciiString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The case of a word.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Case {
    /// A lowercase word.
    Lower(AsciiString),

    /// A title case word.
    Title(AsciiString),

    /// An uppercase word.
    Upper(AsciiString),
}

impl Case {
    /// Consumes `self` and gets the inner [`AsciiString`].
    pub fn into_inner(self) -> AsciiString {
        match self {
            Case::Lower(v) => v,
            Case::Title(v) => v,
            Case::Upper(v) => v,
        }
    }

    /// Consumes `self` and gets the inner [`String`].
    pub fn into_string(self) -> String {
        self.into_inner().into_inner()
    }
}

/// An error that occurs when a word has an incorrect case.
#[derive(Debug)]
pub struct IncorrectCaseError {
    /// The word with the incorrect casing.
    found: String,

    /// The expected word.
    expected: String,

    /// The reason for this expectation.
    reason: String,
}

impl std::fmt::Display for IncorrectCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "found `{}` but expected `{}` because {}",
            self.found, self.expected, self.reason
        )
    }
}

impl std::error::Error for IncorrectCaseError {}

/// Validates the case of a word to ensure it meets the policy of the ontology
/// node names.
fn validate_word_case(input: AsciiString) -> Result<Case, IncorrectCaseError> {
    // Check if the word should be lowercased.
    let lowercased = input.to_lowercase();
    if LOWERCASE_WORDS
        .iter()
        .any(|word| word == &lowercased.as_str())
    {
        if lowercased == input {
            return Ok(Case::Lower(input));
        } else {
            return Err(IncorrectCaseError {
                found: input.into_inner(),
                expected: lowercased.into_inner(),
                reason: String::from("the word is in the lowercase list"),
            });
        }
    }

    // Check if the word is uppercase (preserve if so).
    let uppercased = input.to_uppercase();
    if uppercased == input {
        return Ok(Case::Upper(input));
    }

    // Else, assume title case.
    let title_cased = input.to_title_case();
    if title_cased == input {
        Ok(Case::Title(input))
    } else {
        Err(IncorrectCaseError {
            found: input.into_inner(),
            expected: title_cased.into_inner(),
            reason: String::from(
                "the word is neither in the lowercase list nor is fully uppercase",
            ),
        })
    }
}

/// An error when parsing a name.
#[derive(Debug)]
pub enum ParseError {
    /// One or more non-ASCII characters were included in the name.
    NonAsciiWords(Vec<String>),

    /// One or more words was incorrectly cased.
    IncorrectlyCasedWords(Vec<IncorrectCaseError>),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::NonAsciiWords(words) => write!(
                f,
                "some words include non-ASCII characters: {}",
                words.join(", ")
            ),
            ParseError::IncorrectlyCasedWords(words) => {
                let issues = words
                    .iter()
                    .map(|err| err.to_string())
                    .collect::<Vec<_>>()
                    .join("\n* ");
                write!(f, "some words are incorrectly cased:\n\n* {issues}")
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// A node name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name {
    /// The inner value.
    inner: String,

    /// The cased words that comprise the name.
    words: Vec<Case>,
}

impl Name {
    /// Consumes `self` and gets the cased words of the name.
    pub fn into_words(self) -> impl Iterator<Item = Case> {
        self.words.into_iter()
    }

    /// Gets the cased words of the name by reference.
    pub fn words(&self) -> impl Iterator<Item = &Case> {
        self.words.iter()
    }

    /// Consumes `self` and gets the inner name.
    pub fn into_inner(self) -> String {
        self.inner
    }

    /// Gets the cased inner of the name by reference.
    pub fn inner(&self) -> &str {
        self.inner.as_ref()
    }

    /// Consumes `self` and returns the constitutient parts of the name.
    pub fn into_parts(self) -> (String, impl Iterator<Item = Case>) {
        (self.inner, self.words.into_iter())
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::str::FromStr for Name {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (invalid, ascii_words): (Vec<_>, Vec<_>) = input
            .chars()
            .filter(|c| *c != ',' && *c != ';')
            .collect::<String>()
            .split_whitespace()
            .map(|s| s.to_string())
            .map(|s| (s.clone(), AsciiString::new(s)))
            .partition(|(_, result)| result.is_none());

        if !invalid.is_empty() {
            return Err(ParseError::NonAsciiWords(
                invalid
                    .into_iter()
                    .map(|(input, _)| input)
                    .collect::<Vec<_>>(),
            ));
        }

        let (invalid, cased_words): (Vec<_>, Vec<_>) = ascii_words
            .into_iter()
            // SAFETY: we just partitioned the array above to make sure only
            // [`Some`] results are included in the `ascii_words` vector.
            .map(|(_, value)| value.unwrap())
            .map(validate_word_case)
            .partition(Result::is_err);

        if !invalid.is_empty() {
            return Err(ParseError::IncorrectlyCasedWords(
                invalid
                    .into_iter()
                    .map(|result| result.unwrap_err())
                    .collect::<Vec<_>>(),
            ));
        }

        let words = cased_words
            .into_iter()
            .map(|word| word.unwrap())
            .collect::<Vec<_>>();

        Ok(Name {
            inner: input.to_string(),
            words,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_validation() {
        let (name, parts) = "Foo Bar BAZ".parse::<Name>().unwrap().into_parts();
        assert_eq!(name, "Foo Bar BAZ");
        assert_eq!(
            parts.collect::<Vec<_>>(),
            vec![
                Case::Title(AsciiString::new(String::from("Foo")).unwrap()),
                Case::Title(AsciiString::new(String::from("Bar")).unwrap()),
                Case::Upper(AsciiString::new(String::from("BAZ")).unwrap()),
            ]
        );

        let err = "Foo Bèar BAZ".parse::<Name>().unwrap_err();
        assert_eq!(
            &err.to_string(),
            "some words include non-ASCII characters: Bèar"
        );

        let err = "foo, baR, and bAZ".parse::<Name>().unwrap_err();
        assert_eq!(
            &err.to_string(),
            "some words are incorrectly cased:

* found `foo` but expected `Foo` because the word is neither in the lowercase list nor is fully \
             uppercase
* found `baR` but expected `Bar` because the word is neither in the lowercase list nor is fully \
             uppercase
* found `bAZ` but expected `Baz` because the word is neither in the lowercase list nor is fully \
             uppercase"
        );
    }

    #[test]
    fn special_cases() {
        let err = "Iamp21".parse::<Name>().unwrap_err();
        assert_eq!(
            err.to_string(),
            "some words are incorrectly cased:

* found `Iamp21` but expected `iAMP21` because the word is neither in the lowercase list nor is \
             fully uppercase"
        );

        let _ = "iAMP21".parse::<Name>().unwrap();
    }
}

use std::fmt;

use jespersen_lexicon::LexiconError;
use jespersen_phonemes::Notation;

/// Why text could not be read or written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// The language tag names no language jespersen reads (yet).
    UnsupportedLanguage(String),
    /// The language (its tag) is not written in that notation.
    UnsupportedNotation { language: &'static str, notation: Notation },
    /// Words, numbers or symbols jespersen cannot pronounce yet, each listed once, in order.
    Unpronounced(Vec<String>),
    /// An embedded lexicon is damaged.
    Lexicon(LexiconError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedLanguage(tag) => write!(f, "jespersen does not read the language {tag:?} yet"),
            Self::UnsupportedNotation { language, notation } => {
                write!(f, "jespersen does not write {language} in the {notation:?} notation")
            }
            Self::Unpronounced(words) => write!(f, "jespersen cannot pronounce yet: {}", words.join(", ")),
            Self::Lexicon(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Lexicon(error) => Some(error),
            _ => None,
        }
    }
}

impl From<LexiconError> for Error {
    fn from(error: LexiconError) -> Self {
        Self::Lexicon(error)
    }
}

use std::fmt;

/// Why a lexicon could not be compiled or opened.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LexiconError {
    /// A key was empty or contained a NUL byte.
    InvalidKey(String),
    /// A value was longer than 255 bytes.
    ValueTooLong { key: String, len: usize },
    /// More entries or bytes than the format's 32-bit counts hold.
    TooLarge,
    /// The bytes are not a compiled lexicon of a version this crate reads.
    NotALexicon,
    /// The bytes are a lexicon, but damaged: an offset or length points outside them.
    Damaged,
}

impl fmt::Display for LexiconError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidKey(key) => write!(f, "invalid lexicon key {key:?}: empty or containing NUL"),
            Self::ValueTooLong { key, len } => {
                write!(f, "a value of {key:?} is {len} bytes long; the limit is 255")
            }
            Self::TooLarge => write!(f, "the lexicon is larger than its format allows"),
            Self::NotALexicon => write!(f, "not a compiled lexicon of a known version"),
            Self::Damaged => write!(f, "the compiled lexicon is damaged"),
        }
    }
}

impl std::error::Error for LexiconError {}

//! Italian: stress and the open or closed e and o from the spelling and jespersen's own word list,
//! numbers, abbreviations and elisions, in plain IPA or eSpeak NG's notation.

mod abbreviations;
mod locale;
mod numbers;
mod phonemes;
mod sentence;
mod spelling;
mod words;

pub use abbreviations::is_abbreviation;
pub use jespersen_spoken::TokenReading;
pub use sentence::read_tokens;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;

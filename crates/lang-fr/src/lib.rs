//! French: words from their spelling and jespersen's own lists, with liaison, elision and the e that
//! drops, numbers, dates, money, abbreviations and symbols, in plain IPA or eSpeak NG's notation.

mod endings;
mod locale;
mod numbers;
mod phonemes;
mod sentence;
mod spelling;
mod words;

pub use jespersen_spoken::TokenReading;
pub use locale::is_abbreviation;
pub use sentence::read_tokens;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;

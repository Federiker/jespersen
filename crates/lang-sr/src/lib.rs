//! Serbian, in Cyrillic or Latin letters: words from their spelling, stressed on the first syllable
//! as eSpeak NG reads Croatian, with voicing, numbers, dates, money, abbreviations and symbols, in
//! plain IPA or eSpeak NG's notation.

mod cyrillic;
mod locale;
mod numbers;
mod phonemes;
mod sentence;
mod spelling;

pub use jespersen_spoken::TokenReading;
pub use locale::is_abbreviation;
pub use sentence::read_tokens;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;

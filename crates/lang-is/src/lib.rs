//! Icelandic: words from their spelling, stressed on the first syllable, with their long and
//! short vowels, preaspiration and voiceless consonants, with numbers, dates, money, abbreviations
//! and symbols, in plain IPA or eSpeak NG's notation.

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

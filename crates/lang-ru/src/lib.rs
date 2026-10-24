//! Russian: words from their spelling, with their stress from jespersen's own list and rules, hard
//! and soft consonants, the unstressed vowels reduced, voicing, numbers, dates, money,
//! abbreviations and symbols, in plain IPA or eSpeak NG's notation.

mod locale;
mod numbers;
mod phonemes;
mod sentence;
mod spelling;
mod stress;
mod words;

pub use jespersen_spoken::TokenReading;
pub use locale::is_abbreviation;
pub use sentence::read_tokens;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;

//! Somali: words from their official Latin spelling, stressed on the second-last mora, with
//! numbers, dates, money, abbreviations and symbols, in plain IPA or eSpeak NG's notation.

mod locale;
mod sentence;
mod spelling;

pub use jespersen_spoken::TokenReading;
pub use locale::is_abbreviation;
pub use sentence::read_tokens;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;

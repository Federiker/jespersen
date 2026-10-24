//! Catalan: words from their spelling in the central variety, stressed and with their open and
//! closed vowels as eSpeak NG reads them, with numbers, dates, money, abbreviations and symbols,
//! in plain IPA or eSpeak NG's notation.

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

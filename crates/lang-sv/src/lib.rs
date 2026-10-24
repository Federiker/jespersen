//! Swedish: words from their spelling, stressed and with their long and short vowels as
//! Swedish spells them, with numbers, dates, money, abbreviations and symbols, in plain IPA or
//! eSpeak NG's notation.

mod locale;
mod nst;
mod numbers;
mod phonemes;
mod sampa;
mod sentence;
mod spelling;
mod words;

pub use jespersen_spoken::TokenReading;
pub use locale::is_abbreviation;
pub use sentence::read_tokens;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;

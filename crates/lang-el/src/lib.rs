//! Greek: words from their spelling, monotonic or polytonic, stressed where the accent is written,
//! with numbers that agree with what they count, dates, money, abbreviations and symbols, in plain
//! IPA or eSpeak NG's notation.

mod letters;
mod locale;
mod sentence;
mod spelling;

pub use jespersen_spoken::TokenReading;
pub use locale::is_abbreviation;
pub use sentence::read_tokens;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;

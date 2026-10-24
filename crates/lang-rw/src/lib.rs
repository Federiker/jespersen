//! Kinyarwanda: words from their spelling, with a stress on the last syllable but one as a
//! convention for voices that need one (the language's tones are not written), with numbers,
//! dates, money, abbreviations and symbols, in plain IPA or eSpeak NG's notation.

mod locale;
mod sentence;
mod spelling;

pub use jespersen_spoken::TokenReading;
pub use locale::is_abbreviation;
pub use sentence::read_tokens;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;

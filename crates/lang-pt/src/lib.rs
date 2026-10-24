//! Portuguese, European and Brazilian: words from their spelling and jespersen's own list, stressed
//! by the accents and the endings, with numbers, dates, money, abbreviations and symbols, in plain
//! IPA or eSpeak NG's notation.

mod locale;
mod numbers;
mod phonemes;
mod sentence;
mod spelling;
mod variety;
mod words;

pub use jespersen_spoken::TokenReading;
pub use locale::is_abbreviation;
pub use sentence::read_tokens;
pub use variety::Variety;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;

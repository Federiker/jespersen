//! Irish: words from their spelling, with broad and slender consonants, the vowel groups, the
//! síneadh fada, lenition and eclipsis, and the stress on the first syllable, with numbers, dates,
//! money, abbreviations and symbols, in plain IPA or eSpeak NG's notation.

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

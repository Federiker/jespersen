//! Spanish, Castilian and Latin American.

mod abbreviations;
mod locale;
mod numbers;
mod phonemes;
mod sentence;
mod spelling;
mod variety;

pub use abbreviations::is_abbreviation;
pub use jespersen_spoken::TokenReading;
pub use sentence::read_tokens;
pub use variety::Variety;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;

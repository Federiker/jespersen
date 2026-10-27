//! English, American and British: CMUdict and jespersen's own entries, words read
//! from known ones or with rules learned from CMUdict, numbers and symbols in
//! words, function words as in running text, British English derived from the
//! American forms, and eSpeak NG-style allophones in its notation.

mod abbreviations;
mod addresses;
mod arpabet;
mod british;
mod elision;
mod function_words;
mod homographs;
mod letters;
mod lexicon;
mod morphology;
mod normalise;
mod numbers;
mod onsets;
mod phonemes;
mod rules;
mod sentence;
mod spelling;
mod spellings;
mod titles;
mod units;
mod variety;

pub use abbreviations::is_abbreviation;
pub use lexicon::american;
pub use sentence::{Reading, TokenReading, read_sentence, read_tokens};
pub use variety::{BathVowel, Variety};

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;

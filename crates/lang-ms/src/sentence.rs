use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{locale::Malay, spelling::pronounce};

/// Words said without stress in running text: prepositions, dan and yang.
const UNSTRESSED: [&str; 7] = ["di", "ke", "dan", "yang", "dari", "untuk", "pada"];
/// Words said with a lighter stress.
const LIGHTER: [&str; 3] = ["adalah", "ialah", "sebagai"];

/// How each token (text and kind) of a Malay sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols
/// as words, a reduplicated word as its two halves (kanak-kanak); function words lose their stress
/// in running text.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Malay, &Reader)
}

/// How Malay says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        pronounce(word)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }
}

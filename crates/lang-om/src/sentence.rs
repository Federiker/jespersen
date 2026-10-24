use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{locale::Oromo, spelling::pronounce};

/// Words said without stress in running text: fi (and), the verbal particles ni and hin, and the
/// linkers kan and akka.
const UNSTRESSED: [&str; 5] = ["fi", "ni", "hin", "kan", "akka"];

/// How each token (text and kind) of an Oromo sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols
/// as words; the particles lose their stress in running text.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Oromo, &Reader)
}

/// How Oromo says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        pronounce(word)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }
}

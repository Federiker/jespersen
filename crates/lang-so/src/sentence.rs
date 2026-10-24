use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{locale::Somali, spelling::pronounce};

/// Words said without stress in running text: iyo (and), the focus markers waa, baa and ayaa,
/// the preverbal particles ka, ku, u and la, the linkers oo and ee, and se.
const UNSTRESSED: [&str; 11] = ["iyo", "waa", "baa", "ayaa", "ka", "ku", "u", "la", "oo", "ee", "se"];

/// How each token (text and kind) of a Somali sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols
/// as words; the particles lose their stress in running text.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Somali, &Reader)
}

/// How Somali says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        pronounce(word)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }
}

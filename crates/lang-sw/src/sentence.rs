use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{locale::Swahili, spelling::pronounce};

/// Words said without stress in running text: na, kwa, the connective -a (ya, wa, la, za, cha,
/// vya, pa, mwa) and the copulas ni and si.
const UNSTRESSED: [&str; 11] = ["na", "kwa", "ya", "wa", "la", "za", "cha", "vya", "pa", "ni", "si"];

/// How each token (text and kind) of a Swahili sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols
/// as words; the connectives and copulas lose their stress in running text.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Swahili, &Reader)
}

/// How Swahili says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        pronounce(word)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }
}

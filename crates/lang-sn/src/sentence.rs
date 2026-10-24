use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{locale::Shona, spelling::pronounce};

/// Words said without stress in running text: na and ne, the locatives ku, mu and pa, and the
/// possessive -a (kwa, ya, wa, ra, cha, za, zva, dza, rwa, hwa, va).
const UNSTRESSED: [&str; 16] = [
    "na", "ne", "ku", "mu", "pa", "kwa", "ya", "wa", "ra", "cha", "za", "zva", "dza", "rwa", "hwa", "va",
];

/// How each token (text and kind) of a Shona sentence is read, in order; `None` for a token jespersen
/// cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols as
/// words; the connectives lose their stress in running text.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Shona, &Reader)
}

/// How Shona says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        pronounce(word)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }
}

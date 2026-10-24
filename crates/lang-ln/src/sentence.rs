use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{locale::Lingala, spelling::pronounce};

/// Words said without stress in running text: na, the connective ya, mpe, mpo, the locative o and
/// the negative te.
const UNSTRESSED: [&str; 6] = ["na", "ya", "mpe", "mpo", "o", "te"];

/// How each token (text and kind) of a Lingala sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and
/// symbols as words; the connectives lose their stress in running text.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Lingala, &Reader)
}

/// How Lingala says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        pronounce(word)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }
}

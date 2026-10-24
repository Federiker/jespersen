use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{locale::Turkish, spelling::pronounce};

/// Words said without stress in running text: ve, bir, the enclitics de and ki, the question
/// particle mi, ile, and bu, şu, ben and sen.
const UNSTRESSED: [&str; 15] = [
    "ve", "bir", "de", "da", "ki", "mi", "mı", "mu", "mü", "ile", "ya", "bu", "şu", "ben", "sen",
];

/// How each token (text and kind) of a Turkish sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling and jespersen's list of stresses,
/// numbers, abbreviations and symbols as words; function words lose their stress in running text.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Turkish, &Reader)
}

/// How Turkish says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        pronounce(word)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }
}

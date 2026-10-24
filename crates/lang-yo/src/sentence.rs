use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Speaker, TokenReading, marks::plain};
use jespersen_text::TokenKind;

use crate::{locale::Yoruba, spelling::pronounce};

/// Words said without stress in running text, without their tone marks: the prepositions ní, sí
/// and fún, àti, the particles ti, ni and ń, the subject pronouns mo, o, ó, a, ẹ and wọ́n, the
/// negation kò, kan (a, one) and yìí (this).
const UNSTRESSED: [&str; 14] = [
    "ni", "si", "fun", "ati", "ti", "n", "mo", "o", "a", "ẹ", "wọn", "ko", "kan", "yi",
];

/// How each token (text and kind) of a Yoruba sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols
/// as words; the function words lose their stress in running text.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Yoruba, &Reader)
}

/// How Yoruba says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        pronounce(word)
    }

    /// A function word is known whatever tone marks it is written with.
    fn is_unstressed(&self, written: &str) -> bool {
        UNSTRESSED.contains(&plain(written).as_str())
    }
}

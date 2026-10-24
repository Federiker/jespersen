use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Speaker, TokenReading, marks::plain};
use jespersen_text::TokenKind;

use crate::{locale::Igbo, spelling::pronounce};

/// Words said without stress in running text, without their tone marks: na (and, in), nke (of),
/// ka (that), the copula bụ, and the pronouns m, ọ, ya and ha.
const UNSTRESSED: [&str; 8] = ["na", "nke", "ka", "bụ", "m", "ọ", "ya", "ha"];

/// How each token (text and kind) of an Igbo sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols
/// as words; the function words lose their stress in running text.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Igbo, &Reader)
}

/// How Igbo says its words, for the shared reader.
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

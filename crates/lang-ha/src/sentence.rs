use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Speaker, TokenReading, marks::plain};
use jespersen_text::TokenKind;

use crate::{locale::Hausa, spelling::pronounce};

/// Words said without stress in running text, without any tone or length marks: da (and, with),
/// a (at), the linkers na and ta, the copulas ne and ce, ba (not), ga (to), wa (for), and the
/// short pronouns ya, ka, su, mu, ku and za.
const UNSTRESSED: [&str; 15] = [
    "da", "a", "na", "ta", "ne", "ce", "ba", "ga", "wa", "ya", "ka", "su", "mu", "ku", "za",
];

/// How each token (text and kind) of a Hausa sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols
/// as words; the function words lose their stress in running text.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Hausa, &Reader)
}

/// How Hausa says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        pronounce(word)
    }

    /// A function word is known whatever tone or length marks it is written with.
    fn is_unstressed(&self, written: &str) -> bool {
        UNSTRESSED.contains(&plain(written).as_str())
    }
}

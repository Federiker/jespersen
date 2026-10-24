use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Speaker, TokenReading, marks::plain};
use jespersen_text::TokenKind;

use crate::{locale::Akan, spelling::pronounce};

/// Words said without stress in running text, without any tone marks: ne and na (and), wɔ (at),
/// sɛ (that, if), no (the), a (who, which), mu (in), yɛ (is), and the short pronouns me, wo, ɔ and
/// ɛ.
const UNSTRESSED: [&str; 12] = ["ne", "na", "wɔ", "sɛ", "no", "a", "mu", "yɛ", "me", "wo", "ɔ", "ɛ"];

/// How each token (text and kind) of an Akan sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols
/// as words; the function words lose their stress in running text.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Akan, &Reader)
}

/// How Akan says its words, for the shared reader.
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

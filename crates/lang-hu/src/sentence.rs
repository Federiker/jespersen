use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{locale::Hungarian, spelling::pronounce};

/// Words said with a lighter stress in running text (eSpeak NG writes a secondary one): the
/// articles and the commonest conjunctions.
const LIGHTER: [&str; 5] = ["a", "az", "és", "de", "s"];

/// How each token (text and kind) of a Hungarian sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and
/// symbols as words. The words between punctuation marks are read together: articles and the
/// commonest conjunctions are lighter.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Hungarian, &Reader)
}

/// How Hungarian says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && letter.is_uppercase()
            && let Some(name) = Hungarian.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        pronounce(&lower)
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }
}

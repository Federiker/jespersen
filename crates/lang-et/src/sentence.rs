use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{locale::Estonian, spelling::pronounce};

/// Words said without stress in running text: conjunctions, the short pronouns and the copula.
const UNSTRESSED: [&str; 13] = [
    "ja", "on", "ning", "mis", "kes", "ma", "sa", "ta", "me", "te", "nad", "mu", "ehk",
];

/// Words said with a lighter stress (eSpeak NG writes a secondary one): the long pronouns, the
/// negation, olema and some conjunctions.
const LIGHTER: [&str; 23] = [
    "ei", "oli", "aga", "või", "mina", "sina", "tema", "meie", "teie", "see", "selle", "seda", "need", "neid", "neil",
    "nende", "ole", "oled", "olla", "ega", "olid", "olen", "olnud",
];

/// How each token (text and kind) of an Estonian sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and
/// symbols as words. The words between punctuation marks are read together: function words lose
/// their stress.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Estonian, &Reader)
}

/// How Estonian says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && letter.is_uppercase()
            && let Some(name) = Estonian.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        pronounce(word)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }
}

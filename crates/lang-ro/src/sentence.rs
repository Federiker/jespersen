use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{locale::Romanian, spelling::pronounce, words::respelling};

/// Words said without stress in running text: prepositions, articles, clitic pronouns,
/// conjunctions and the auxiliaries.
const UNSTRESSED: [&str; 43] = [
    "și", "se", "pe", "la", "în", "cu", "de", "o", "a", "ce", "să", "nu", "mai", "mă", "te", "îl", "le", "ne", "vă",
    "au", "un", "sunt", "fost", "va", "mea", "ca", "din", "al", "că", "am", "ți", "e", "dar", "când", "cel", "cea",
    "el", "eu", "tu", "ai", "lui", "sau", "voi",
];
/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 10] = [
    "este", "meu", "avea", "pentru", "erau", "dacă", "care", "una", "fie", "ori",
];

/// How each token (text and kind) of a Romanian sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings),
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Romanian, &Reader)
}

/// How Romanian says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a letter on its own that is not a word by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = lower.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && !"aoeiî".contains(letter)
            && let Some(name) = Romanian.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        pronounce(respelling(&lower).unwrap_or(&lower))
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }
}

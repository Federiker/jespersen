use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{locale::Latvian, spelling::pronounce, words::respelling};

/// Words said without stress in running text: prepositions, conjunctions, short pronouns and
/// the forms of būt.
const UNSTRESSED: [&str; 24] = [
    "un", "ar", "uz", "no", "par", "pie", "aiz", "līdz", "ir", "kā", "ka", "kad", "lai", "man", "mums", "jums", "mēs",
    "es", "tu", "šīs", "šis", "ko", "jo", "bez",
];

/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 6] = ["bija", "savu", "viņa", "viņam", "ļoti", "pirms"];

/// How each token (text and kind) of a Latvian sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings),
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Latvian, &Reader)
}

/// How Latvian says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && letter.is_uppercase()
            && let Some(name) = Latvian.letter(letter)
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

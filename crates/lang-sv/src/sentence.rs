use jespersen_phonemes::{Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Swedish,
    nst,
    numbers::{is_number_word, is_tens},
    phonemes::{O, O_FINAL, OO, legal_onset},
    spelling::pronounce,
    words::respelling,
};

/// Words said without stress in running text: articles, prepositions, pronouns, conjunctions and
/// the auxiliaries.
const UNSTRESSED: [&str; 34] = [
    "jag", "är", "var", "har", "och", "en", "det", "den", "som", "med", "i", "på", "av", "till", "för", "att", "om",
    "han", "hon", "de", "vi", "ni", "du", "då", "hans", "kan", "vad", "än", "från", "sin", "min", "din", "vid", "vill",
];

/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 8] = ["hade", "skulle", "kunde", "ville", "deras", "något", "någon", "eller"];

/// How each token (text and kind) of a Swedish sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings),
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Swedish, &Reader)
}

/// How Swedish says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && letter.is_uppercase()
            && let Some(name) = Swedish.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        match respelling(&lower) {
            Some(respelt) => pronounce(respelt),
            None => nst::pronounce(word).or_else(|| pronounce(&lower)),
        }
    }

    /// The parts of a number joined by hyphens, said as one word: the tens before their unit
    /// lighter, with a short final o (tjugo-ett ɕˌʉɡɔˈɛt), the others with their stress (ett-hundra
    /// ˈɛthˈɵndra).
    fn say_joined(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let parts: Vec<&str> = lower.split('-').filter(|part| !part.is_empty()).collect();
        if !parts.iter().all(|part| is_number_word(part)) {
            return None;
        }
        let mut joined: Vec<Segment> = Vec::new();
        for (index, part) in parts.iter().enumerate() {
            let lighter = is_tens(part) && index + 1 < parts.len();
            for syllable in &self.say(part)?.syllables {
                let stress = if lighter {
                    syllable.stress.min(Stress::Secondary)
                } else {
                    syllable.stress
                };
                for &phoneme in syllable.onset.iter().chain(&syllable.nucleus).chain(&syllable.coda) {
                    let phoneme = if lighter && [OO, O_FINAL].contains(&phoneme) {
                        O
                    } else {
                        phoneme
                    };
                    let stress = if phoneme.is_vowel() { stress } else { Stress::Unstressed };
                    joined.push(Segment::new(phoneme, stress));
                }
            }
        }
        Some(syllabify(&joined, legal_onset))
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }
}

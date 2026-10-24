use jespersen_phonemes::{Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Norwegian, numbers::is_number_word, phonemes::legal_onset, spelling::pronounce, words::respelling,
};

/// Words said without stress in running text: articles, prepositions, pronouns, conjunctions and
/// the auxiliaries.
const UNSTRESSED: [&str; 30] = [
    "var", "har", "og", "en", "et", "det", "den", "som", "med", "i", "på", "av", "til", "for", "å", "om", "han", "hun",
    "de", "vi", "dere", "du", "men", "så", "fra", "at", "min", "ham", "dem", "sin",
];

/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 12] = [
    "ikke", "hadde", "skulle", "kunne", "mine", "denne", "eller", "jeg", "er", "dette", "disse", "deres",
];

/// How each token (text and kind) of a Norwegian sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings),
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Norwegian, &Reader)
}

/// How Norwegian says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && letter.is_uppercase()
            && let Some(name) = Norwegian.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        pronounce(respelling(&lower).unwrap_or(&lower))
    }

    /// The parts of a number joined by hyphens, said as one word, each with its stress (tjue-en
    /// ʂˈʉːaˈeːn, to-hundre tˈuːhˈʉndra).
    fn say_joined(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let parts: Vec<&str> = lower.split('-').filter(|part| !part.is_empty()).collect();
        if !parts.iter().all(|part| is_number_word(part)) {
            return None;
        }
        let mut joined: Vec<Segment> = Vec::new();
        for part in parts {
            for syllable in &self.say(part)?.syllables {
                let stress = syllable.stress;
                for &phoneme in syllable.onset.iter().chain(&syllable.nucleus).chain(&syllable.coda) {
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

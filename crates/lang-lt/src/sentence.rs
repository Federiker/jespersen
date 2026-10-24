use jespersen_phonemes::{Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Lithuanian,
    numbers::is_number_word,
    phonemes::{I, I_STRESSED, legal_onset},
    spelling::pronounce,
    words::respelling,
};

/// Words said without stress in running text: prepositions, conjunctions and particles.
const UNSTRESSED: [&str; 11] = ["ir", "bet", "o", "ar", "iš", "nuo", "per", "po", "be", "jog", "ne"];

/// How each token (text and kind) of a Lithuanian sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings),
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Lithuanian, &Reader)
}

/// How Lithuanian says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && letter.is_uppercase()
            && let Some(name) = Lithuanian.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        pronounce(respelling(&lower).unwrap_or(&lower))
    }

    /// The parts of a number joined by hyphens, said as one word, each with its stress
    /// (dvidešimt-vienas dvʲˈɪdʲeɕimtvʲˈienas).
    fn say_joined(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let parts: Vec<&str> = lower.split('-').filter(|part| !part.is_empty()).collect();
        if !parts.iter().all(|part| is_number_word(part)) {
            return None;
        }
        let mut joined: Vec<Segment> = Vec::new();
        for part in parts {
            let said = self.say(part)?;
            for syllable in &said.syllables {
                for &phoneme in syllable.onset.iter().chain(&syllable.nucleus).chain(&syllable.coda) {
                    let stress = if phoneme.is_vowel() {
                        syllable.stress
                    } else {
                        Stress::Unstressed
                    };
                    joined.push(Segment::new(phoneme, stress));
                }
            }
        }
        Some(syllabify(&joined, legal_onset))
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    /// A word left without stress has a plain i, i rather than ɪ in eSpeak NG's notation (ir).
    fn run(&self, words: &mut [&mut Pronunciation]) {
        for syllable in words.iter_mut().flat_map(|said| said.syllables.iter_mut()) {
            if syllable.stress == Stress::Unstressed {
                for vowel in &mut syllable.nucleus {
                    if *vowel == I_STRESSED {
                        *vowel = I;
                    }
                }
            }
        }
    }
}

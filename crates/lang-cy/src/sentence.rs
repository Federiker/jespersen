use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Welsh,
    numbers::{is_lighter, is_number_word},
    phonemes::{A, A_LONG, D, E, E_LONG, I, I_LONG, O, O_LONG, S, SCHWA, U, U_LONG, Y, legal_onset},
    spelling::pronounce,
    words::respelling,
};

/// Words said without stress in running text: the article, prepositions, particles, pronouns
/// and conjunctions.
const UNSTRESSED: [&str; 63] = [
    "y", "yr", "'r", "yn", "yng", "ym", "a", "ac", "o", "i", "ar", "ei", "eu", "fy", "dy", "ein", "eich", "mae", "na",
    "nac", "fel", "am", "at", "dan", "gan", "wrth", "ni", "fod", "heb", "mewn", "dros", "sydd", "hyn", "a'r", "o'r",
    "i'r", "â'r", "a'i", "o'i", "i'w", "a’r", "o’r", "i’r", "a’i", "ap", "mai", "os", "oes", "cyn", "mae'r", "mae’r",
    "roedd", "ti", "hi", "fe", "dwy", "drwy", "drwy'r", "i'n", "hi'n", "ni'n", "chi'n", "fi'n",
];

/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 6] = ["wedi", "dyma", "gyda", "gyda'n", "rydw", "rydyn"];

/// How each token (text and kind) of a Welsh sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings),
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress, and y in them is ə.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Welsh, &Reader)
}

/// How Welsh says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && letter.is_uppercase()
            && let Some(name) = Welsh.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        pronounce(respelling(&lower).unwrap_or(&lower))
    }

    /// The parts of a number joined by hyphens, said as one word: deg and cant after their count
    /// lighter (dau-ddeg dˈaɨðˌeːɡ), the others with their stress (dau-ddeg-pedwar).
    fn say_joined(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let parts: Vec<&str> = lower.split('-').filter(|part| !part.is_empty()).collect();
        if !parts.iter().all(|part| is_number_word(part)) {
            return None;
        }
        let mut joined: Vec<Segment> = Vec::new();
        for (index, part) in parts.iter().enumerate() {
            let lighter = index > 0 && is_lighter(part);
            for syllable in &self.say(part)?.syllables {
                let stress = if lighter {
                    syllable.stress.min(Stress::Secondary)
                } else {
                    syllable.stress
                };
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

    /// A function word left without stress has a short vowel, and ə for a y that starts it (y ə, yn
    /// ən, a a; but fy vø, o oː, ni niː).
    fn run(&self, words: &mut [&mut Pronunciation]) {
        for said in words.iter_mut() {
            if said
                .syllables
                .iter()
                .all(|syllable| syllable.stress == Stress::Unstressed)
            {
                let starts_with_vowel = said.phonemes().next().is_some_and(Phoneme::is_vowel);
                // An o closed by d or s is short (fod vɔd, os; but o oː).
                let closed = said.phonemes().last().is_some_and(|last| [D, S].contains(&last));
                // i on its own is short (i ɨ; but ni niː).
                let alone = said.phonemes().count() == 1;
                for vowel in said
                    .syllables
                    .iter_mut()
                    .flat_map(|syllable| syllable.nucleus.iter_mut())
                {
                    *vowel = match *vowel {
                        vowel if starts_with_vowel && [Y, U, U_LONG].contains(&vowel) => SCHWA,
                        vowel if vowel == U_LONG => U,
                        vowel if vowel == A_LONG => A,
                        vowel if vowel == E_LONG => E,
                        vowel if vowel == I_LONG && alone => I,
                        vowel if vowel == O_LONG && closed => O,
                        vowel => vowel,
                    };
                }
            }
        }
    }
}

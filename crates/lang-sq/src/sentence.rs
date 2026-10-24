use jespersen_phonemes::{Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Albanian,
    numbers::is_number_word,
    phonemes::{SCHWA, SCHWA_STRESSED, legal_onset},
    spelling::pronounce,
};

/// Words said without stress in running text: the articles, prepositions, particles and
/// conjunctions.
const UNSTRESSED: [&str; 16] = [
    "e", "i", "të", "së", "në", "më", "me", "për", "nga", "dhe", "që", "u", "se", "si", "a", "ka",
];

/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 2] = ["janë", "mijë"];

/// How each token (text and kind) of an Albanian sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and
/// symbols as words. The words between punctuation marks are read together: function words lose
/// their stress.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Albanian, &Reader)
}

/// How Albanian says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && letter.is_uppercase()
            && let Some(name) = Albanian.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        pronounce(word)
    }

    /// The parts of a number joined by hyphens, said as one word: the tens and unit each with its
    /// stress but e (njëzet-e-një ɲˈʌzɛtɛɲˈʌ), a teen stressed on its unit and lighter two
    /// syllables on (pesë-mbëdhjetë pˈɛsəmbˌəðjetə).
    fn say_joined(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let parts: Vec<&str> = lower.split('-').filter(|part| !part.is_empty()).collect();
        if parts.len() < 2 || !parts.iter().all(|part| is_number_word(part)) {
            return None;
        }
        let mut joined: Vec<Segment> = Vec::new();
        for part in parts {
            let said = pronounce(part)?;
            for syllable in &said.syllables {
                let stress = if part == "e" || part == "mbëdhjetë" {
                    Stress::Unstressed
                } else {
                    syllable.stress
                };
                for &phoneme in syllable.onset.iter().chain(&syllable.nucleus).chain(&syllable.coda) {
                    let stress = if phoneme.is_vowel() { stress } else { Stress::Unstressed };
                    joined.push(Segment::new(phoneme, stress));
                }
            }
        }
        let mut said = syllabify(&joined, legal_onset);
        if lower.ends_with("mbëdhjetë")
            && let Some(syllable) = said.syllables.get_mut(2)
        {
            syllable.stress = Stress::Secondary;
        }
        Some(said)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    /// A ë left without stress in a sentence is ə (në nə, të tə).
    fn run(&self, words: &mut [&mut Pronunciation]) {
        for syllable in words.iter_mut().flat_map(|said| said.syllables.iter_mut()) {
            if syllable.stress == Stress::Unstressed {
                for vowel in &mut syllable.nucleus {
                    if *vowel == SCHWA_STRESSED {
                        *vowel = SCHWA;
                    }
                }
            }
        }
    }
}

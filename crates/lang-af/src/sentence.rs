use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Afrikaans,
    numbers::is_number_word,
    phonemes::{EE, SCHWA, legal_onset},
    spelling::pronounce,
    words::respelling,
};

/// Words said without stress in running text: articles, prepositions, pronouns and conjunctions.
const UNSTRESSED: [&str; 38] = [
    "die", "is", "het", "en", "van", "in", "op", "te", "met", "aan", "dat", "ek", "u", "om", "by", "na", "of", "tot",
    "se", "my", "jou", "hom", "haar", "vir", "dit", "toe", "soos", "gaan", "word", "kan", "sal", "'n", "ons", "dis",
    "kon", "hy", "sy", "jy",
];

/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 4] = ["hulle", "julle", "as", "nie"];

/// How each token (text and kind) of an Afrikaans sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings),
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Afrikaans, &Reader)
}

/// How Afrikaans says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && letter.is_uppercase()
            && let Some(name) = Afrikaans.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        // 'n, the indefinite article, is ə.
        if lower.trim_matches(['\'', '’']) == "n" {
            return Some(syllabify(&[Segment::new(SCHWA, Stress::Unstressed)], legal_onset));
        }
        pronounce(respelling(&lower).unwrap_or(&lower))
    }

    /// The parts of a number joined by hyphens, said as one word, each with its stress but en
    /// (een-en-twintig ˈiənɛntwˈəntəx, twee-honderd twˈiəhˈɔnərt).
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
                let stress = if part == "en" {
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
        Some(syllabify(&joined, legal_onset))
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    fn run(&self, words: &mut [&mut Pronunciation]) {
        reduce(words);
    }
}

/// Function words left without stress: an open e is ə (te tə, se sə).
fn reduce(words: &mut [&mut Pronunciation]) {
    let unstressed = |said: &Pronunciation| {
        said.syllables
            .iter()
            .all(|syllable| syllable.stress == Stress::Unstressed)
    };
    for said in words.iter_mut() {
        if !unstressed(said) {
            continue;
        }
        let phonemes: Vec<Phoneme> = said.phonemes().collect();
        if phonemes.last() == Some(&EE) {
            for vowel in said
                .syllables
                .iter_mut()
                .flat_map(|syllable| syllable.nucleus.iter_mut())
            {
                *vowel = SCHWA;
            }
        }
    }
}

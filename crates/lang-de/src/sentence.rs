use jespersen_phonemes::{Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::German,
    numbers::is_number_word,
    phonemes::{E, E_LONG, R_VOCALIC, legal_onset},
    spelling::pronounce,
    words::respelling,
};

/// Words said without stress in running text: articles, prepositions, pronouns, conjunctions and
/// the auxiliaries.
const UNSTRESSED: [&str; 65] = [
    "der", "die", "das", "den", "dem", "des", "ein", "und", "in", "im", "ins", "an", "am", "auf", "bei", "mit", "nach",
    "von", "vom", "zu", "zum", "zur", "für", "um", "ich", "du", "er", "sie", "es", "wir", "ihr", "mich", "mir", "dich",
    "dir", "sich", "ihn", "ihm", "uns", "ist", "war", "hat", "als", "wie", "dass", "daß", "so", "noch", "man", "dann",
    "doch", "da", "durch", "sein", "waren", "wollte", "denn", "bin", "beim", "habe", "dies", "sind", "mein", "aufs",
    "bis",
];

/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 50] = [
    "eine", "einen", "einem", "einer", "eines", "werden", "wurde", "aber", "auch", "aus", "oder", "ob", "hatte",
    "hätte", "schon", "wenn", "ganz", "seine", "jetzt", "diese", "ihre", "ihrer", "ihren", "ihnen", "über", "unter",
    "seinem", "seinen", "seiner", "seines", "während", "vor", "haben", "dieses", "meine", "meinen", "meinem", "meiner",
    "sollte", "dieser", "diesem", "wird", "wegen", "sollen", "hatten", "hätten", "kann", "geworden", "gegen", "mehr",
];

/// How each token (text and kind) of a German sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings),
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &German, &Reader)
}

/// How German says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && let Some(name) = German.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        pronounce(respelling(&lower).unwrap_or(&lower))
    }

    /// The parts of a number joined by hyphens, said as one word, each with its stress but und
    /// (und-zwanzig ʊnttsvˈantsɪç, ein-hundert ˈaɪnhˈʊndɜt).
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
                let stress = if part == "und" {
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

    /// A function word left without stress has a short e before r (der dɛɾ, er ɛɾ; but den deːn).
    fn run(&self, words: &mut [&mut Pronunciation]) {
        for said in words.iter_mut() {
            if said
                .syllables
                .iter()
                .all(|syllable| syllable.stress == Stress::Unstressed)
            {
                for syllable in &mut said.syllables {
                    let next_r = syllable.coda.first() == Some(&R_VOCALIC);
                    let Some(vowel) = syllable.nucleus.first_mut() else {
                        continue;
                    };
                    if *vowel == E_LONG && next_r {
                        *vowel = E;
                    }
                }
            }
        }
    }
}

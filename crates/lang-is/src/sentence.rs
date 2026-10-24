use jespersen_phonemes::{Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Icelandic,
    numbers::is_number_word,
    phonemes::{GH, II, II_LONG, X, legal_onset},
    spelling::pronounce,
};

/// Words said without stress in running text: conjunctions, prepositions, pronouns and short
/// forms of vera and hafa.
const UNSTRESSED: [&str; 38] = [
    "og", "að", "í", "á", "en", "það", "sem", "til", "af", "um", "þeir", "því", "frá", "með", "við", "hann", "hún",
    "var", "er", "voru", "ég", "þá", "hans", "inn", "ef", "þeim", "þær", "sá", "þess", "þau", "fram", "mín", "minn",
    "mig", "mér", "hjá", "þú", "hef",
];

/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 13] = [
    "hefur", "vera", "hafa", "eru", "þetta", "þessu", "fyrir", "verið", "ekki", "undir", "eftir", "geta", "eða",
];

/// How each token (text and kind) of an Icelandic sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols
/// as words. The words between punctuation marks are read
/// together: some function words lose their stress.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Icelandic, &Reader)
}

/// How Icelandic says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own by its name, said as one word
    /// (W tvöfalt vaff).
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && letter.is_uppercase()
            && let Some(name) = Icelandic.letter(letter)
        {
            return pronounce(&name.replace(' ', "-"));
        }
        pronounce(word)
    }

    /// The parts of a number joined by hyphens, said as one word as eSpeak NG says them: the tens
    /// with a lighter last syllable, a lighter tíu after its units (fjöru-tíu fjˈœːɾrytˌiy), og
    /// unstressed with ɣ before the vowel of the unit (tuttugu-og-einn tˈyhdyɣˌyɔːɣˈeɪdn).
    fn say_joined(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let parts: Vec<&str> = lower.split('-').filter(|part| !part.is_empty()).collect();
        if parts.len() < 2 || !parts.iter().all(|part| is_number_word(part)) {
            return None;
        }
        let mut joined: Vec<Segment> = Vec::new();
        for (index, part) in parts.iter().enumerate() {
            let said = self.say(part)?;
            let count = said.syllables.len();
            let tens = index > 0 && *part == "tíu";
            for (number, syllable) in said.syllables.iter().enumerate() {
                let stress = if *part == "og" {
                    Stress::Unstressed
                } else if (index + 1 < parts.len() && count > 2 && number + 1 == count) || (tens && number == 0) {
                    Stress::Secondary
                } else {
                    syllable.stress
                };
                for &phoneme in syllable.onset.iter().chain(&syllable.nucleus).chain(&syllable.coda) {
                    let phoneme = match phoneme {
                        X if *part == "og" => GH,
                        II_LONG if tens => II,
                        phoneme => phoneme,
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

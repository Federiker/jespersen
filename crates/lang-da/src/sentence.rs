use jespersen_phonemes::{Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Danish,
    nst,
    numbers::is_number_word,
    phonemes::{A_BACK, A_DARK, I, I_LONG, I_LONG_PLAIN, I_PLAIN, O_SHORT, O_SHORT_PLAIN, legal_onset},
    spelling::{pronounce, unstressed_form},
    words::respelling,
};

/// Words said without stress in running text: articles, prepositions, pronouns, conjunctions and
/// the auxiliaries.
const UNSTRESSED: [&str; 42] = [
    "og", "en", "et", "det", "den", "som", "i", "at", "han", "hun", "de", "du", "da", "dem", "er", "ham", "jeg", "på",
    "mig", "dig", "sig", "vi", "var", "har", "med", "ved", "til", "fra", "om", "der", "kan", "skal", "hende", "hendes",
    "sin", "sine", "sit", "hans", "min", "mit", "din", "dit",
];

/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 3] = ["havde", "deres", "disse"];

/// How each token (text and kind) of a Danish sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings),
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Danish, &Reader)
}

/// How Danish says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && letter.is_uppercase()
            && let Some(name) = Danish.letter(letter)
        {
            return pronounce(name);
        }
        // jespersen's list first, then NST's lexicon, then the spelling.
        match respelling(&lower) {
            Some(respelt) => pronounce(respelt),
            None => nst::pronounce(&lower).or_else(|| pronounce(&lower)),
        }
    }

    /// The parts of a number joined by hyphens, said as one word, each with its stress but og,
    /// said ʌ (en-og-tyve ˈʔenʌtˈʔyʋə, to-hundrede).
    fn say_joined(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let parts: Vec<&str> = lower.split('-').filter(|part| !part.is_empty()).collect();
        if !parts.iter().all(|part| is_number_word(part)) {
            return None;
        }
        let mut joined: Vec<Segment> = Vec::new();
        for part in parts {
            for syllable in &self.say(part)?.syllables {
                let stress = if part == "og" {
                    Stress::Unstressed
                } else {
                    syllable.stress
                };
                for &phoneme in syllable.onset.iter().chain(&syllable.nucleus).chain(&syllable.coda) {
                    let stress = if phoneme.is_vowel() { stress } else { Stress::Unstressed };
                    let phoneme = if part == "og" && phoneme == O_SHORT {
                        O_SHORT_PLAIN
                    } else {
                        phoneme
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

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    /// A function word left without stress has its i and a, and the vowel it ends in, without eSpeak
    /// NG's glottal stop (og ʌ, det de, sin; but hun hʔun, en ʔen).
    fn run(&self, words: &mut [&mut Pronunciation]) {
        for said in words.iter_mut() {
            if !said
                .syllables
                .iter()
                .all(|syllable| syllable.stress == Stress::Unstressed)
            {
                continue;
            }
            for vowel in said.syllables.iter_mut().flat_map(|syllable| &mut syllable.nucleus) {
                *vowel = match *vowel {
                    vowel if vowel == I => I_PLAIN,
                    vowel if vowel == I_LONG => I_LONG_PLAIN,
                    vowel if vowel == A_DARK => A_BACK,
                    vowel => vowel,
                };
            }
            if let Some(last) = said.syllables.last_mut()
                && last.coda.is_empty()
                && let Some(vowel) = last.nucleus.last_mut()
            {
                *vowel = if *vowel == O_SHORT {
                    O_SHORT_PLAIN
                } else {
                    unstressed_form(*vowel)
                };
            }
        }
    }
}

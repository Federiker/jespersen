use jespersen_phonemes::{Pronunciation, Stress};
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Luganda,
    spelling::{pronounce, sounds},
};

/// Words said without stress in running text: na, ne, nga, the locatives e, ku and mu, and
/// the connective -a (ya, wa, ba, kya, bya, za, ga, lya, ka, gwa, lwa, bwa, kwa).
const UNSTRESSED: [&str; 19] = [
    "na", "ne", "nga", "e", "ku", "mu", "ya", "wa", "ba", "kya", "bya", "za", "ga", "lya", "ka", "gwa", "lwa", "bwa",
    "kwa",
];

/// How each token (text and kind) of a Luganda sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and
/// symbols as words; the connectives lose their stress in running text, and an elided word
/// (n'abantu, y'omusajja) is said with the word after it.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Luganda, &Reader)
}

/// How Luganda says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        pronounce(word)
    }

    fn elide(&self, head: &str, word: Pronunciation) -> Option<Pronunciation> {
        elide(head, word)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }
}

/// An elided word said with the word after it: its consonants start the next word's first
/// syllable (n'abantu nabantu); one with a vowel left (nga') is said unstressed before it.
pub(crate) fn elide(head: &str, mut word: Pronunciation) -> Option<Pronunciation> {
    let letters: Vec<char> = head.to_lowercase().chars().collect();
    let head_sounds = sounds(&letters)?;
    if head_sounds.iter().any(|phoneme| phoneme.is_vowel()) {
        let mut said = pronounce(head)?;
        for syllable in &mut said.syllables {
            syllable.stress = Stress::Unstressed;
        }
        said.syllables.extend(word.syllables);
        return Some(said);
    }
    let first = word.syllables.first_mut()?;
    first.onset.splice(0..0, head_sounds);
    Some(word)
}

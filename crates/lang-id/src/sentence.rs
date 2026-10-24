use jespersen_phonemes::{Pronunciation, Stress};
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Indonesian,
    spelling::{E, E_STRESSED, pronounce},
};

/// Words said without stress in running text: prepositions, dan and yang.
const UNSTRESSED: [&str; 6] = ["di", "ke", "dan", "yang", "dari", "untuk"];
/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 4] = ["adalah", "ialah", "pada", "sebagai"];

/// How each token (text and kind) of an Indonesian sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols
/// as words, a reduplicated word as its two halves (baik-baik); function words lose their stress
/// in running text.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Indonesian, &Reader)
}

/// How Indonesian says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        pronounce(word)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    /// A word that loses its stress in running text has ə again (ke kə).
    fn run(&self, words: &mut [&mut Pronunciation]) {
        for syllable in words.iter_mut().flat_map(|said| said.syllables.iter_mut()) {
            if syllable.stress != Stress::Primary && syllable.nucleus == [E_STRESSED] {
                syllable.nucleus = vec![E];
            }
        }
    }
}

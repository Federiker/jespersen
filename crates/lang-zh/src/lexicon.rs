use jespersen_lexicon::{Lexicon, LexiconError};

use crate::pinyin::Pinyin;

/// Mandarin words and characters with their readings in numbered pinyin (compiled by build.rs).
static MANDARIN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/mandarin.lex"));

/// The longest word in the lexicon, in characters.
pub(crate) const LONGEST: usize = include!(concat!(env!("OUT_DIR"), "/longest.rs"));

fn mandarin() -> Result<Lexicon<'static>, LexiconError> {
    Lexicon::new(MANDARIN)
}

/// The syllables of `word` (in Simplified or Traditional characters), if the lexicon knows it.
///
/// # Errors
///
/// When the embedded lexicon is damaged, including a reading that is not pinyin.
pub(crate) fn reading(word: &str) -> Result<Option<Vec<Pinyin>>, LexiconError> {
    let Some(values) = mandarin()?.get(word)? else {
        return Ok(None);
    };
    let first = values.first().ok_or(LexiconError::Damaged)?;
    let text = std::str::from_utf8(first).map_err(|_| LexiconError::Damaged)?;
    text.split(' ')
        .map(|syllable| Pinyin::parse(syllable).ok_or(LexiconError::Damaged))
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

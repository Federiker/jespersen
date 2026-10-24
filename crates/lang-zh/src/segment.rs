use jespersen_lexicon::LexiconError;

use crate::{
    lexicon::{LONGEST, reading},
    pinyin::Pinyin,
};

/// A word found in a run of Chinese characters: how many characters it takes and its syllables,
/// `None` for a character the lexicon does not know.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Word {
    pub length: usize,
    pub syllables: Option<Vec<Pinyin>>,
}

/// Splits a run of Chinese characters into the lexicon's words: the split with the fewest words,
/// then the fewest words of one character, then the longest last words, so that known words are read whole (银行 háng, not
/// 银 + 行 xíng).
///
/// # Errors
///
/// When the embedded lexicon is damaged.
pub(crate) fn segment(characters: &[char]) -> Result<Vec<Word>, LexiconError> {
    // The cheapest split of the first `end` characters: its cost, as (words, single characters),
    // and its last word.
    let mut best: Vec<Option<Word>> = vec![None; characters.len() + 1];
    let mut costs = vec![(usize::MAX, usize::MAX); characters.len() + 1];
    costs[0] = (0, 0);
    for start in 0..characters.len() {
        if costs[start].0 == usize::MAX {
            continue;
        }
        for length in 1..=LONGEST.min(characters.len() - start) {
            let text: String = characters[start..start + length].iter().collect();
            let syllables = reading(&text)?;
            if syllables.is_none() && length > 1 {
                continue;
            }
            let (words, singles) = costs[start];
            let cost = (words + 1, singles + usize::from(length == 1));
            let end = start + length;
            // On a tie, the longer last word wins, as matching from the end of the run does (我的确信:
            // 我 + 的 + 确信, not 我 + 的确 + 信).
            let longer = best[end].as_ref().is_some_and(|word| length > word.length);
            if cost < costs[end] || (cost == costs[end] && longer) {
                costs[end] = cost;
                best[end] = Some(Word { length, syllables });
            }
        }
    }
    let mut words = Vec::new();
    let mut end = characters.len();
    while end > 0 {
        let Some(word) = best[end].take() else { break };
        end -= word.length;
        words.push(word);
    }
    words.reverse();
    Ok(words)
}

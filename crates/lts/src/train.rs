use std::collections::HashMap;

use crate::{CONTEXTS, align::align, key};

/// A word of the lexicon: its letters and the phonemes it says (codes, which may carry stress).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub letters: Vec<char>,
    pub phonemes: Vec<u8>,
}

/// A letter in a context ([`crate::key`]) and the phonemes it says (none, one or two codes).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rule {
    pub context: String,
    pub phonemes: Vec<u8>,
}

/// Learns rules from `entries`. `base` maps a phoneme code to the phoneme it is for aligning (the
/// same vowel with any stress). A rule is kept when its context was seen at least `min_count`
/// times and it says something other than the shorter context it refines; every letter's
/// context-free rule is kept. Words that cannot be aligned are left out. The result is sorted by
/// context and the same for the same input.
#[must_use]
pub fn train(entries: &[Entry], base: impl Fn(u8) -> u8, min_count: usize) -> Vec<Rule> {
    let mut alphabet: HashMap<char, usize> = HashMap::new();
    let mut phoneme_count = 0;
    let words: Vec<(Vec<usize>, Vec<u8>)> = entries
        .iter()
        .map(|entry| {
            let letters = entry
                .letters
                .iter()
                .map(|&letter| {
                    let next = alphabet.len();
                    *alphabet.entry(letter).or_insert(next)
                })
                .collect();
            let phonemes: Vec<u8> = entry.phonemes.iter().map(|&code| base(code)).collect();
            phoneme_count = phonemes
                .iter()
                .map(|&phoneme| usize::from(phoneme) + 1)
                .fold(phoneme_count, usize::max);
            (letters, phonemes)
        })
        .collect();
    let alignments = align(&words, alphabet.len(), phoneme_count);

    // Every aligned letter: its word, its index and the codes it says.
    let mut said: Vec<(&[char], usize, &[u8])> = Vec::new();
    for (entry, alignment) in entries.iter().zip(&alignments) {
        let Some(alignment) = alignment else { continue };
        let mut at = 0;
        for (index, &count) in alignment.iter().enumerate() {
            let end = at + usize::from(count);
            said.push((&entry.letters, index, &entry.phonemes[at..end]));
            at = end;
        }
    }

    let mut rules = Vec::new();
    let mut parent: HashMap<String, Vec<u8>> = HashMap::new();
    for (level, &context) in CONTEXTS.iter().enumerate() {
        let mut seen: Vec<(String, &[u8])> = said
            .iter()
            .map(|&(letters, index, codes)| (key(letters, index, context), codes))
            .collect();
        seen.sort_unstable();
        let mut majorities: HashMap<String, Vec<u8>> = HashMap::new();
        for group in seen.chunk_by(|a, b| a.0 == b.0) {
            let total = group.len();
            let majority = group
                .chunk_by(|a, b| a.1 == b.1)
                .max_by(|a, b| a.len().cmp(&b.len()).then_with(|| b[0].1.cmp(a[0].1)))
                .map_or_else(Vec::new, |run| run[0].1.to_vec());
            let context_key = group[0].0.clone();
            let refined = if level == 0 {
                None
            } else {
                parent.get(&shorter(&context_key, context, CONTEXTS[level - 1]))
            };
            if level == 0 || (total >= min_count && refined != Some(&majority)) {
                rules.push(Rule {
                    context: context_key.clone(),
                    phonemes: majority.clone(),
                });
            }
            majorities.insert(context_key, majority);
        }
        parent = majorities;
    }
    rules.sort();
    rules
}

/// The key of the shorter context `to` that a key in context `from` refines.
fn shorter(key: &str, from: (usize, usize), to: (usize, usize)) -> String {
    let letters: Vec<char> = key.chars().collect();
    letters[from.0 - to.0..letters.len() - (from.1 - to.1)].iter().collect()
}

//! Aligning letters with phonemes: each letter says no phoneme, one, or two (x: k s), and which
//! letter says which is learned over the whole lexicon by repeated best alignment (hard EM).
#![expect(clippy::cast_precision_loss, reason = "word lengths and counts are far below 2^52")]

use std::collections::HashMap;

/// How many rounds of aligning and re-estimating.
const ROUNDS: usize = 5;
/// Smoothing added to every count.
const SMOOTHING: f64 = 0.1;
/// The extra cost of a pair of phonemes no letter has said yet.
const UNSEEN_PAIR: f64 = 4.0;

/// The cost (a negative log probability) of each letter saying nothing, a phoneme or a pair.
struct Model {
    silent: Vec<f64>,
    single: Vec<Vec<f64>>,
    pair: HashMap<(usize, u8, u8), f64>,
    unseen_pair: Vec<f64>,
}

/// Aligns each word (letters as ids below `letters`, phonemes as ids below `phonemes`): how many
/// phonemes each letter says (0, 1 or 2); `None` for a word that cannot be aligned that way.
pub fn align(words: &[(Vec<usize>, Vec<u8>)], letters: usize, phonemes: usize) -> Vec<Option<Vec<u8>>> {
    let mut model = first_model(words, letters, phonemes);
    let mut alignments = Vec::new();
    for _ in 0..ROUNDS {
        alignments = words.iter().map(|(word, said)| best(word, said, &model)).collect();
        model = estimate(words, &alignments, letters, phonemes);
    }
    alignments
}

/// A first model from how often a letter and a phoneme sit at about the same place in a word.
fn first_model(words: &[(Vec<usize>, Vec<u8>)], letters: usize, phonemes: usize) -> Model {
    let mut counts = vec![vec![0.0; phonemes]; letters];
    for (word, said) in words {
        let (n, m) = (word.len(), said.len());
        if n == 0 || m == 0 {
            continue;
        }
        for (i, &letter) in word.iter().enumerate() {
            for (j, &phoneme) in said.iter().enumerate() {
                let distance = ((i as f64 + 0.5) / n as f64 - (j as f64 + 0.5) / m as f64).abs();
                if distance <= 1.0 / n.max(m) as f64 {
                    counts[letter][usize::from(phoneme)] += 1.0;
                }
            }
        }
    }
    let single = counts
        .iter()
        .map(|row| {
            let total: f64 = row.iter().sum::<f64>() + SMOOTHING * row.len() as f64;
            row.iter().map(|count| -((count + SMOOTHING) / total).ln()).collect()
        })
        .collect::<Vec<Vec<f64>>>();
    let silent = vec![-(0.1f64).ln(); letters];
    let unseen_pair = single
        .iter()
        .map(|row| row.iter().copied().fold(f64::INFINITY, f64::min) * 2.0 + UNSEEN_PAIR)
        .collect();
    Model {
        silent,
        single,
        pair: HashMap::new(),
        unseen_pair,
    }
}

/// The cheapest alignment of one word under `model`.
fn best(word: &[usize], said: &[u8], model: &Model) -> Option<Vec<u8>> {
    let (n, m) = (word.len(), said.len());
    if m > 2 * n {
        return None;
    }
    let width = m + 1;
    let mut cost = vec![f64::INFINITY; (n + 1) * width];
    let mut taken = vec![0u8; (n + 1) * width];
    cost[0] = 0.0;
    for i in 0..n {
        let letter = word[i];
        for j in 0..=m {
            let here = cost[i * width + j];
            if here.is_infinite() {
                continue;
            }
            let mut relax = |to: usize, step: f64, count: u8| {
                let next = (i + 1) * width + to;
                if here + step < cost[next] {
                    cost[next] = here + step;
                    taken[next] = count;
                }
            };
            relax(j, model.silent[letter], 0);
            if j < m {
                relax(j + 1, model.single[letter][usize::from(said[j])], 1);
            }
            if j + 1 < m {
                let pair = model
                    .pair
                    .get(&(letter, said[j], said[j + 1]))
                    .copied()
                    .unwrap_or(model.unseen_pair[letter]);
                relax(j + 2, pair, 2);
            }
        }
    }
    if cost[n * width + m].is_infinite() {
        return None;
    }
    let mut counts = vec![0u8; n];
    let mut j = m;
    for i in (1..=n).rev() {
        let count = taken[i * width + j];
        counts[i - 1] = count;
        j -= usize::from(count);
    }
    Some(counts)
}

/// A model from the counts of an alignment of every word.
fn estimate(words: &[(Vec<usize>, Vec<u8>)], alignments: &[Option<Vec<u8>>], letters: usize, phonemes: usize) -> Model {
    let mut silent = vec![0.0; letters];
    let mut single = vec![vec![0.0; phonemes]; letters];
    let mut pair: HashMap<(usize, u8, u8), f64> = HashMap::new();
    for ((word, said), alignment) in words.iter().zip(alignments) {
        let Some(alignment) = alignment else { continue };
        let mut j = 0;
        for (&letter, &count) in word.iter().zip(alignment) {
            match count {
                0 => silent[letter] += 1.0,
                1 => single[letter][usize::from(said[j])] += 1.0,
                _ => *pair.entry((letter, said[j], said[j + 1])).or_default() += 1.0,
            }
            j += usize::from(count);
        }
    }
    let mut pairs_by_letter = vec![0.0; letters];
    for (&(letter, _, _), &count) in &pair {
        pairs_by_letter[letter] += count;
    }
    let totals: Vec<f64> = (0..letters)
        .map(|letter| {
            silent[letter]
                + single[letter].iter().sum::<f64>()
                + pairs_by_letter[letter]
                + SMOOTHING * (phonemes + 2) as f64
        })
        .collect();
    let cost = |count: f64, total: f64| -((count + SMOOTHING) / total).ln();
    Model {
        silent: (0..letters)
            .map(|letter| cost(silent[letter], totals[letter]))
            .collect(),
        single: (0..letters)
            .map(|letter| {
                single[letter]
                    .iter()
                    .map(|&count| cost(count, totals[letter]))
                    .collect()
            })
            .collect(),
        pair: pair
            .iter()
            .map(|(&key, &count)| (key, cost(count, totals[key.0])))
            .collect(),
        unseen_pair: totals.iter().map(|&total| cost(0.0, total) + UNSEEN_PAIR).collect(),
    }
}

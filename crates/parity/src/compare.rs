use std::collections::HashMap;

use jespersen::{Language, Notation, WriteOptions};

/// How often jespersen agreed with the oracle.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// Texts compared.
    pub lines: usize,
    /// Texts jespersen wrote exactly as the oracle did.
    pub exact_lines: usize,
    /// Texts jespersen could not read yet (unknown words, numbers…), left out of the word counts.
    pub unreadable_lines: usize,
    /// Words in the oracle's answers to the texts jespersen read.
    pub words: usize,
    /// Of those, words jespersen wrote the same, aligned in order (longest common subsequence).
    pub matching_words: usize,
    /// Words that differed, as (jespersen, oracle), with how often, most frequent first.
    pub differences: Vec<((String, String), usize)>,
}

impl Report {
    /// The share of exact texts among those jespersen read, in per cent.
    #[must_use]
    pub fn exact_percent(&self) -> f64 {
        percent(self.exact_lines, self.lines - self.unreadable_lines)
    }

    /// The share of matching words, in per cent.
    #[must_use]
    pub fn word_percent(&self) -> f64 {
        percent(self.matching_words, self.words)
    }
}

#[expect(clippy::cast_precision_loss, reason = "counts far below 2^52")]
fn percent(part: usize, whole: usize) -> f64 {
    if whole == 0 {
        0.0
    } else {
        part as f64 * 100.0 / whole as f64
    }
}

/// Compares jespersen, in `language` and eSpeak NG's notation without punctuation, with the oracle's
/// `answers` (text, answer).
#[must_use]
pub fn compare(answers: &[(String, String)], language: Language) -> Report {
    let options = WriteOptions {
        notation: Notation::Espeak,
        drop_punctuation: true,
    };
    let mut report = Report {
        lines: answers.len(),
        ..Report::default()
    };
    let mut differences: HashMap<(String, String), usize> = HashMap::new();
    for (text, answer) in answers {
        let Ok(ours) = jespersen::phonemize(text, language).and_then(|sentences| jespersen::write(&sentences, options))
        else {
            report.unreadable_lines += 1;
            continue;
        };
        if ours == *answer {
            report.exact_lines += 1;
        }
        let ours: Vec<&str> = ours.split_whitespace().collect();
        let theirs: Vec<&str> = answer.split_whitespace().collect();
        report.words += theirs.len();
        let (matching, pairs) = align(&ours, &theirs);
        report.matching_words += matching;
        for pair in pairs {
            *differences.entry(pair).or_default() += 1;
        }
    }
    let mut differences: Vec<_> = differences.into_iter().collect();
    differences.sort_by(|(a, count_a), (b, count_b)| count_b.cmp(count_a).then_with(|| a.cmp(b)));
    report.differences = differences;
    report
}

/// The longest common subsequence of two word lists, and the differing words between matches,
/// paired one to one where both sides have a word (a gap with words on one side only is not
/// paired).
fn align(ours: &[&str], theirs: &[&str]) -> (usize, Vec<(String, String)>) {
    let (n, m) = (ours.len(), theirs.len());
    let mut lengths = vec![vec![0usize; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            lengths[i][j] = if ours[i] == theirs[j] {
                lengths[i + 1][j + 1] + 1
            } else {
                lengths[i + 1][j].max(lengths[i][j + 1])
            };
        }
    }
    let (mut i, mut j) = (0, 0);
    let mut pairs = Vec::new();
    let (mut gap_ours, mut gap_theirs) = (Vec::new(), Vec::new());
    let flush = |gap_ours: &mut Vec<&str>, gap_theirs: &mut Vec<&str>, pairs: &mut Vec<(String, String)>| {
        for (a, b) in gap_ours.iter().zip(gap_theirs.iter()) {
            pairs.push(((*a).to_owned(), (*b).to_owned()));
        }
        gap_ours.clear();
        gap_theirs.clear();
    };
    while i < n || j < m {
        if i < n && j < m && ours[i] == theirs[j] {
            flush(&mut gap_ours, &mut gap_theirs, &mut pairs);
            i += 1;
            j += 1;
        } else if j < m && (i == n || lengths[i][j + 1] >= lengths[i + 1][j]) {
            gap_theirs.push(theirs[j]);
            j += 1;
        } else {
            gap_ours.push(ours[i]);
            i += 1;
        }
    }
    flush(&mut gap_ours, &mut gap_theirs, &mut pairs);
    (lengths[0][0], pairs)
}

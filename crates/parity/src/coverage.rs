use std::collections::HashMap;

use jespersen::{Language, Notation, Script, WriteOptions};

/// How much of a text list jespersen reads without an oracle: lines it reads whole, and, for
/// languages written without spaces, how many Chinese characters it reads inside words of two
/// characters or more rather than one by one.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Coverage {
    pub lines: usize,
    /// Lines jespersen cannot read, each with why.
    pub unreadable: Vec<(String, String)>,
    pub han_characters: usize,
    pub han_in_words: usize,
    /// Two Chinese characters read one by one side by side, and how often, most frequent first: where
    /// a word may be missing from the lexicon.
    pub single_pairs: Vec<(String, usize)>,
}

impl Coverage {
    #[must_use]
    pub fn readable_percent(&self) -> f64 {
        percent(self.lines - self.unreadable.len(), self.lines)
    }

    #[must_use]
    pub fn in_words_percent(&self) -> f64 {
        percent(self.han_in_words, self.han_characters)
    }
}

#[expect(
    clippy::cast_precision_loss,
    reason = "counts of lines and characters are far below 2^52"
)]
fn percent(part: usize, whole: usize) -> f64 {
    if whole == 0 {
        0.0
    } else {
        100.0 * part as f64 / whole as f64
    }
}

/// Reads each of `texts` in `language`, in plain IPA, and counts what it read.
#[must_use]
pub fn coverage(texts: &[String], language: Language) -> Coverage {
    let mut coverage = Coverage::default();
    let mut pairs: HashMap<String, usize> = HashMap::new();
    for text in texts {
        coverage.lines += 1;
        let read = jespersen::phonemize(text, language).and_then(|sentences| {
            jespersen::write(
                &sentences,
                WriteOptions {
                    notation: Notation::Ipa,
                    drop_punctuation: false,
                },
            )
            .map(|_| sentences)
        });
        match read {
            Ok(sentences) => {
                for sentence in &sentences {
                    for pair in sentence.tokens.windows(2) {
                        let single = |token: &jespersen::Token<'_>| {
                            token.script == Script::Han && token.text.chars().count() == 1
                        };
                        if single(&pair[0]) && single(&pair[1]) && pair[0].span.end == pair[1].span.start {
                            *pairs.entry(format!("{}{}", pair[0].text, pair[1].text)).or_insert(0) += 1;
                        }
                    }
                }
                for token in sentences.iter().flat_map(|sentence| &sentence.tokens) {
                    if token.script == Script::Han {
                        let characters = token.text.chars().count();
                        coverage.han_characters += characters;
                        if characters >= 2 {
                            coverage.han_in_words += characters;
                        }
                    }
                }
            }
            Err(error) => coverage.unreadable.push((text.clone(), error.to_string())),
        }
    }
    coverage.single_pairs = pairs.into_iter().collect();
    coverage
        .single_pairs
        .sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    coverage
}

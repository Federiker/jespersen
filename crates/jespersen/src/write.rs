use jespersen_phonemes::Notation;
use jespersen_text::TokenKind;

use crate::{Error, Sentence};

/// How [`write`] writes sentences out.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WriteOptions {
    pub notation: Notation,
    /// Drop punctuation, as eSpeak NG does; by default it is kept, which voices such as Kokoro use
    /// for their rhythm (docs/decisions.md, decision 24).
    pub drop_punctuation: bool,
}

/// Writes sentences as one line: each token's words, and punctuation unless dropped. Words are
/// separated by one space; punctuation keeps the text's spacing (a comma stays on its word).
/// Chinese punctuation is written as its Latin counterpart, spaced as Latin is (`，` as `, `).
/// eSpeak NG's notation writes some words joined to the next (in the). Sentences are separated by
/// one space.
///
/// # Errors
///
/// [`Error::UnsupportedNotation`] when a sentence's language is not written in the notation;
/// [`Error::Unpronounced`] when tokens could not be read, listing each once.
pub fn write(sentences: &[Sentence<'_>], options: WriteOptions) -> Result<String, Error> {
    if let Some(sentence) = sentences
        .iter()
        .find(|sentence| !sentence.language.notations().contains(&options.notation))
    {
        return Err(Error::UnsupportedNotation {
            language: sentence.language.tag(),
            notation: options.notation,
        });
    }
    let mut unpronounced: Vec<String> = Vec::new();
    for token in sentences.iter().flat_map(|sentence| &sentence.tokens) {
        if token.words.is_none() && !unpronounced.iter().any(|word| word == token.text) {
            unpronounced.push(token.text.to_owned());
        }
    }
    if !unpronounced.is_empty() {
        return Err(Error::Unpronounced(unpronounced));
    }

    let mut out = String::new();
    for sentence in sentences {
        let mut previous: Option<Previous> = None;
        for token in &sentence.tokens {
            let is_punctuation = token.kind == TokenKind::Punctuation;
            let words = token.words.as_deref().unwrap_or_default();
            if (is_punctuation && options.drop_punctuation) || (!is_punctuation && words.is_empty()) {
                // A silent token still moves the place spacing is measured from (20 °C, then).
                if let Some(previous) = previous.as_mut() {
                    previous.end = token.span.end;
                }
                continue;
            }
            let latin = is_punctuation.then(|| latin_punctuation(token.text)).flatten();
            let spaced = match previous {
                // The first token of a sentence: separate it from the sentence before.
                None => !out.is_empty(),
                // A word eSpeak NG's notation joins to this one (in the: ɪnðə).
                Some(previous) if previous.joined && !is_punctuation && options.notation == Notation::Espeak => false,
                // Chinese punctuation, spaced as Latin is: `，` closes up to the word before and is
                // followed by a space, `（` the other way round.
                Some(_) if latin.is_some_and(|(_, opening)| !opening) || closing(token.text) => false,
                Some(previous) if previous.space_after || latin.is_some_and(|(_, opening)| opening) => true,
                // Spoken tokens are always apart (10%: ten percent); punctuation follows the text.
                Some(previous) => {
                    options.drop_punctuation
                        || token.span.start > previous.end
                        || (!is_punctuation && !previous.punctuation)
                }
            };
            if spaced {
                out.push(' ');
            }
            if is_punctuation {
                out.push_str(latin.map_or(token.text, |(latin, _)| latin));
            }
            for (index, word) in words.iter().enumerate() {
                if index > 0 {
                    match (token.hyphenated, options.notation) {
                        // A hyphenated word's parts: joined in eSpeak NG's notation (wˈɛlnˈoʊn),
                        // with hyphens in plain IPA (ˈwɛl-ˈnoʊn).
                        (true, Notation::Espeak | Notation::Misaki) => {}
                        (true, Notation::Ipa | Notation::Pinyin | Notation::Zhuyin) => out.push('-'),
                        (false, _) => out.push(' '),
                    }
                }
                word.write(options.notation, &mut out);
            }
            previous = Some(Previous {
                end: token.span.end,
                joined: token.joined_to_next,
                punctuation: is_punctuation,
                space_after: latin.is_some_and(|(_, opening)| !opening),
            });
        }
    }
    Ok(out)
}

/// The token written before, as far as spacing needs it.
#[derive(Clone, Copy)]
struct Previous {
    end: usize,
    joined: bool,
    punctuation: bool,
    /// Chinese punctuation written as Latin, which a space follows.
    space_after: bool,
}

/// Whether `text` is a closing quote or bracket, which stays on what it closes.
fn closing(text: &str) -> bool {
    matches!(text, "”" | "’" | "»" | ")" | "]" | "}")
}

/// The Latin punctuation written for Chinese (fullwidth) punctuation, and whether it opens (a
/// bracket or quote that a space goes before rather than after); `None` for other punctuation.
fn latin_punctuation(text: &str) -> Option<(&'static str, bool)> {
    Some(match text {
        "，" | "、" => (",", false),
        "。" | "．" => (".", false),
        "！" => ("!", false),
        "？" => ("?", false),
        "：" => (":", false),
        "；" => (";", false),
        "）" => (")", false),
        "》" | "」" | "』" | "】" | "〉" | "〗" => ("”", false),
        "（" => ("(", true),
        "《" | "「" | "『" | "【" | "〈" | "〖" => ("“", true),
        _ => return None,
    })
}

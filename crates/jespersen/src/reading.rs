use std::ops::Range;

use jespersen_phonemes::Pronunciation;
use jespersen_text::{Script, TokenKind};

use crate::{Error, Language};

/// A sentence as jespersen read it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sentence<'a> {
    pub text: &'a str,
    /// Byte offsets in the whole text.
    pub span: Range<usize>,
    /// The language it was read in.
    pub language: Language,
    pub tokens: Vec<Token<'a>>,
}

/// A token as jespersen read it: the text's token and the words it is said as. Chinese, written
/// without spaces, has a token per word found (银行), not per character.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token<'a> {
    pub text: &'a str,
    /// Byte offsets in the whole text.
    pub span: Range<usize>,
    pub kind: TokenKind,
    pub script: Script,
    /// The words the token is said as: one for a word, several for a number (1999: nineteen
    /// ninety-nine), none for punctuation or a currency sign read after its amount. `None` when
    /// jespersen cannot read the token yet.
    pub words: Option<Vec<Pronunciation>>,
    /// Whether eSpeak NG's notation writes the token's last word joined to the next word (in the:
    /// `ɪnðə`).
    pub joined_to_next: bool,
    /// Whether the words are a hyphenated word's parts (well-known): joined in eSpeak NG's
    /// notation, with hyphens in plain IPA.
    pub hyphenated: bool,
}

/// Reads `text` in `language`: its sentences, their tokens and the words each is said as.
///
/// Words come from the lexicon (unknown words cannot be read yet); numbers, currency amounts and
/// some symbols are read as words.
///
/// # Errors
///
/// When an embedded lexicon is damaged.
pub fn phonemize(text: &str, language: Language) -> Result<Vec<Sentence<'_>>, Error> {
    let mut sentences = Vec::new();
    for sentence in jespersen_text::sentences(text, |word| language.is_abbreviation(word)) {
        let mut tokens = Vec::with_capacity(sentence.tokens.len());
        // What each token is read from: its text, without an abbreviation's stop.
        let mut read_as: Vec<(&str, TokenKind)> = Vec::with_capacity(sentence.tokens.len());
        let mut found = sentence.tokens.iter().peekable();
        while let Some(token) = found.next() {
            let mut span = token.span.clone();
            // An abbreviation's full stop is part of the word, not a pause ("Mr. Smith"), unless it
            // also ends the sentence.
            if token.kind == TokenKind::Word
                && language.is_abbreviation(token.text)
                && let Some(stop) = found.next_if(|next| {
                    next.text == "." && next.span.start == span.end && next.span.end < sentence.span.end
                })
            {
                span.end = stop.span.end;
            }
            tokens.push(Token {
                text: &text[span.clone()],
                span,
                kind: token.kind,
                script: token.script,
                words: None,
                joined_to_next: false,
                hyphenated: false,
            });
            read_as.push((token.text, token.kind));
        }
        let mut read: Vec<Token<'_>> = Vec::with_capacity(tokens.len());
        let mut readings = language.read(&read_as)?.into_iter();
        let mut tokens = tokens.into_iter();
        while let (Some(mut token), Some(reading)) = (tokens.next(), readings.next()) {
            if let Some(reading) = reading {
                // A word written over several tokens (a Chinese word of two characters) is one token.
                for _ in 1..reading.tokens {
                    if let (Some(next), Some(_)) = (tokens.next(), readings.next()) {
                        token.span.end = next.span.end;
                    }
                }
                token.text = &text[token.span.clone()];
                token.words = Some(reading.words);
                token.joined_to_next = reading.joined_to_next;
                token.hyphenated = reading.hyphenated;
            }
            read.push(token);
        }
        sentences.push(Sentence {
            text: sentence.text,
            span: sentence.span,
            language,
            tokens: read,
        });
    }
    Ok(sentences)
}

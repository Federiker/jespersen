use std::ops::Range;

use unicode_segmentation::UnicodeSegmentation;

use crate::{Script, Token, TokenKind, addresses::addresses, classify::kind_of};

/// One sentence: its span in the text and its tokens.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sentence<'a> {
    pub text: &'a str,
    /// Byte offsets in the whole text, without the spaces around the sentence.
    pub span: Range<usize>,
    pub tokens: Vec<Token<'a>>,
}

/// Splits `text` into sentences of tokens.
///
/// Sentence boundaries follow Unicode's rules (UAX #29: `。` ends a sentence without a space, a
/// full stop followed by a lower-case word does not, a closing quote stays with its sentence),
/// except after an abbreviation: when a sentence ends in a word followed by a full stop and
/// `is_abbreviation` accepts that word (without the stop, e.g. `Mr`), the next sentence is joined
/// to it, and when it ends at a single line break without sentence-ending punctuation (prose
/// wrapped at a fixed width: a blank line still ends a sentence). Sentences with no tokens (only
/// spaces) are left out.
pub fn sentences<'a>(text: &'a str, is_abbreviation: impl Fn(&str) -> bool) -> Vec<Sentence<'a>> {
    let mut sentences: Vec<Sentence<'a>> = Vec::new();
    let mut join_next = false;
    let addresses = addresses(text);
    for (start, piece) in text.split_sentence_bound_indices() {
        let end = start + piece.len();
        let tokens = tokens(text, start..end, &addresses);
        let (Some(first), Some(last)) = (tokens.first(), tokens.last()) else {
            continue;
        };
        let span = first.span.start..last.span.end;
        let ends_in_abbreviation = ends_in_abbreviation(&tokens, &is_abbreviation);
        let cut_by_line_break = wrapped(piece, &tokens, &text[end..]);
        // Sentence-ending punctuation inside an address (page?q=1) ends no sentence.
        let inside_address = addresses.iter().any(|address| address.start < end && end < address.end);
        match sentences.last_mut() {
            Some(previous) if join_next => {
                previous.span.end = span.end;
                previous.text = &text[previous.span.clone()];
                previous.tokens.extend(tokens);
            }
            _ => sentences.push(Sentence {
                text: &text[span.clone()],
                span,
                tokens,
            }),
        }
        join_next = ends_in_abbreviation || cut_by_line_break || inside_address;
    }
    sentences
}

/// The tokens of `range` of `text`, its web and e-mail `addresses` each one word token (one that
/// starts here may run on past the range). Words joined by a
/// hyphen without spaces stay one word token (well-known, state-of-the-art), and numbers joined
/// by `:`, `/` or `-` without spaces one number token (10:30, 12/05/2024, 1990-1995), for
/// languages to read as compounds, times, dates or ranges.
fn tokens<'a>(text: &'a str, range: Range<usize>, addresses: &[Range<usize>]) -> Vec<Token<'a>> {
    let offset = range.start;
    let piece = &text[range];
    let mut tokens: Vec<Token<'a>> = Vec::new();
    for (start, segment) in piece.split_word_bound_indices() {
        // A web or e-mail address is one word token, whatever its pieces.
        if let Some(address) = addresses.iter().find(|address| address.contains(&(offset + start))) {
            if offset + start == address.start {
                tokens.push(Token {
                    text: &text[address.clone()],
                    span: address.clone(),
                    kind: TokenKind::Word,
                    script: Script::Latin,
                });
            }
            continue;
        }
        if segment.chars().all(char::is_whitespace) {
            continue;
        }
        let span = offset + start..offset + start + segment.len();
        let kind = kind_of(segment);
        if matches!(kind, TokenKind::Number | TokenKind::Word)
            && let [.., before, separator] = &tokens[..]
            && before.kind == kind
            && (if kind == TokenKind::Number {
                matches!(separator.text, ":" | "/" | "-")
            } else {
                matches!(separator.text, "-" | "\u{2010}")
            })
            && before.span.end == separator.span.start
            && separator.span.end == span.start
        {
            let merged = before.span.start..span.end;
            tokens.pop();
            if let Some(token) = tokens.last_mut() {
                token.text = &piece[merged.start - offset..merged.end - offset];
                token.span = merged;
            }
            continue;
        }
        // A sign right before a number, with nothing attached before it, is the number's (-5, +3).
        if kind == TokenKind::Number
            && let Some(sign) = tokens.last()
            && matches!(sign.text, "-" | "\u{2212}" | "+")
            && sign.span.end == span.start
            && tokens
                .len()
                .checked_sub(2)
                .is_none_or(|before| tokens[before].span.end < sign.span.start)
        {
            let merged = sign.span.start..span.end;
            if let Some(token) = tokens.last_mut() {
                token.text = &piece[merged.start - offset..merged.end - offset];
                token.span = merged;
                token.kind = TokenKind::Number;
                token.script = Script::Common;
            }
            continue;
        }
        tokens.push(Token {
            text: segment,
            span,
            kind,
            script: Script::of_text(segment),
        });
    }
    tokens
}

fn ends_in_abbreviation(tokens: &[Token<'_>], is_abbreviation: impl Fn(&str) -> bool) -> bool {
    match tokens {
        [.., word, stop] => {
            stop.text == "."
                && word.kind == TokenKind::Word
                && word.span.end == stop.span.start
                && is_abbreviation(word.text)
        }
        _ => false,
    }
}

/// Whether `piece` was cut only by a line break inside a sentence: it ends with exactly one line
/// break, its last token is not sentence-ending punctuation (or a closing quote or bracket after
/// it), and `rest` (the text after it) does not start with another line break.
fn wrapped(piece: &str, tokens: &[Token<'_>], rest: &str) -> bool {
    let trailing: &str = &piece[piece.trim_end().len()..];
    let breaks = trailing.matches('\n').count();
    if breaks != 1 || rest.trim_start_matches([' ', '\t', '\r']).starts_with('\n') {
        return false;
    }
    let closing = |text: &str| matches!(text, "\"" | "'" | "”" | "’" | ")" | "]" | "»" | "」" | "』");
    let ending = tokens.iter().rev().find(|token| !closing(token.text));
    !ending.is_some_and(|token| matches!(token.text, "." | "!" | "?" | "…" | "。" | "！" | "？" | "?!" | "!?"))
}

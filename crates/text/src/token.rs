use std::ops::Range;

use crate::Script;

/// What a token is, before any language reads it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    /// Letters, possibly with apostrophes, hyphens or inner full stops (`don't`, `U.S`).
    Word,
    /// Starts with a digit: `42`, `3.14`, `1,000`, `3rd`, `1990s`.
    Number,
    /// Punctuation, which a voice may use for its rhythm: `.`, `,`, `?`, `—`, `“`, `。`, `，`.
    Punctuation,
    /// Any other symbol: `%`, `$`, `&`, `+`, `@`, `©`.
    Symbol,
}

/// One token of a sentence, borrowed from the text it was found in. Spaces are not tokens; the
/// gaps between spans show where they were.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token<'a> {
    pub text: &'a str,
    /// Byte offsets in the whole text.
    pub span: Range<usize>,
    pub kind: TokenKind,
    pub script: Script,
}

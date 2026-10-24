use crate::TokenKind;

/// The kind of a non-space word-boundary segment (UAX #29), from its first character.
pub(crate) fn kind_of(segment: &str) -> TokenKind {
    match segment.chars().next() {
        Some(c) if c.is_numeric() => TokenKind::Number,
        Some(c) if c.is_alphabetic() => TokenKind::Word,
        Some(c) if is_punctuation(c) => TokenKind::Punctuation,
        _ => TokenKind::Symbol,
    }
}

/// Punctuation in the sense a reader pauses on, not every Unicode punctuation character: `#`,
/// `@`, `&`, `*`, `/` and `\` are read as symbols.
pub(crate) fn is_punctuation(c: char) -> bool {
    matches!(
        c,
        '!' | '"'
            | '\''
            | '('
            | ')'
            | ','
            | '-'
            | '.'
            | ':'
            | ';'
            | '?'
            | '['
            | ']'
            | '{'
            | '}'
            | '¡'
            | '«'
            | '·'
            | '»'
            | '¿'
            // General punctuation: dashes, quotes, bullets and ellipses; ‹ › ‼ ‽ ⁇ ⁈ ⁉; long dashes.
            // Per mille and primes (‰ ′ ″) are symbols: they are read as words.
            | '\u{2010}'..='\u{2027}'
            | '‹'
            | '›'
            | '‼'
            | '‽'
            | '\u{2047}'..='\u{2049}'
            | '\u{2E3A}'
            | '\u{2E3B}'
            // CJK punctuation: 、。〈〉《》「」『』【】〔〕〖〗〘〙〚〛〜〝〞〟 and friends.
            | '\u{3001}'..='\u{3003}'
            | '\u{3008}'..='\u{3011}'
            | '\u{3014}'..='\u{301F}'
            | '\u{30FB}'
            // Fullwidth forms of ASCII punctuation, and halfwidth CJK punctuation.
            | '！'
            | '＂'
            | '＇'
            | '（'
            | '）'
            | '，'
            | '－'
            | '．'
            | '：'
            | '；'
            | '？'
            | '［'
            | '］'
            | '｛'
            | '｝'
            | '\u{FF5F}'..='\u{FF65}'
            // Vertical and small forms used in CJK typesetting (not the small maths and currency
            // signs).
            | '\u{FE10}'..='\u{FE19}'
            | '\u{FE30}'..='\u{FE4F}'
            | '\u{FE50}'..='\u{FE61}'
            | '\u{FE63}'
    )
}

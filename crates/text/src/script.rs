/// The writing system of a token, as far as jespersen needs to tell them apart: mixed text switches
/// language when the script changes (docs/decisions.md, decision 25).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Script {
    /// Latin letters, with their accented forms.
    Latin,
    /// Chinese characters (hanzi), shared with Japanese kanji.
    Han,
    /// Digits, punctuation and symbols, which belong to whatever surrounds them.
    Common,
    /// Any other writing system.
    Other,
}

impl Script {
    /// The script of one character.
    #[must_use]
    pub fn of(c: char) -> Self {
        match c {
            'A'..='Z'
            | 'a'..='z'
            | 'ª'
            | 'º'
            | '\u{00C0}'..='\u{00D6}'
            | '\u{00D8}'..='\u{00F6}'
            | '\u{00F8}'..='\u{024F}'
            | '\u{1E00}'..='\u{1EFF}'
            | '\u{2C60}'..='\u{2C7F}'
            | '\u{A720}'..='\u{A7FF}'
            | '\u{FF21}'..='\u{FF3A}'
            | '\u{FF41}'..='\u{FF5A}' => Self::Latin,
            '\u{2E80}'..='\u{2FDF}'
            | '〆'
            | '〇'
            | '\u{3021}'..='\u{3029}'
            | '\u{3038}'..='\u{303B}'
            | '\u{3400}'..='\u{4DBF}'
            | '\u{4E00}'..='\u{9FFF}'
            | '\u{F900}'..='\u{FAFF}'
            | '\u{20000}'..='\u{3134F}' => Self::Han,
            // Combining marks take the script of the letter they follow; on their own they are common.
            c if c.is_alphabetic() => Self::Other,
            _ => Self::Common,
        }
    }

    /// The script of a run of text: the first that is not [`Script::Common`], or `Common`.
    #[must_use]
    pub fn of_text(text: &str) -> Self {
        text.chars()
            .map(Self::of)
            .find(|script| *script != Self::Common)
            .unwrap_or(Self::Common)
    }
}

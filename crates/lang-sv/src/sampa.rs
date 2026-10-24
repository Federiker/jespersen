//! NST's Swedish SAMPA, split into its symbols (build.rs checks every transcription with it).

/// The sounds NST writes, longest first where one starts another: x\ is ɧ, s' ɕ and u0 ɵ, and a
/// retroflex consonant ends in a backtick.
const SOUNDS: [&str; 39] = [
    "x\\", "s'", "s`", "t`", "d`", "n`", "l`", "u0", "}", "i", "I", "e", "E", "a", "A", "y", "Y", "2", "9", "u", "U",
    "o", "O", "p", "b", "t", "d", "k", "g", "f", "v", "s", "h", "m", "n", "N", "l", "r", "j",
];

/// NST's vowels.
pub(crate) const VOWELS: [&str; 16] = [
    "u0", "}", "i", "I", "e", "E", "a", "A", "y", "Y", "2", "9", "u", "U", "o", "O",
];

/// One symbol of a transcription.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Symbol {
    /// `"` or `""`: the main stress, accent 1 or 2, on the syllable it starts.
    Primary,
    /// `%`: a secondary stress, on the syllable it starts.
    Secondary,
    /// `$`: a syllable ends.
    Boundary,
    /// `:`: the vowel before is long.
    Long,
    Sound(&'static str),
}

/// `sampa` split into its symbols, the `*` that joins a diphthong's vowels left out; `None` for a
/// symbol NST does not use.
pub(crate) fn symbols(sampa: &str) -> Option<Vec<Symbol>> {
    let mut out = Vec::with_capacity(sampa.len());
    let mut rest = sampa;
    while let Some(c) = rest.chars().next() {
        let (symbol, length) = match c {
            '"' if rest.starts_with("\"\"") => (Some(Symbol::Primary), 2),
            '"' => (Some(Symbol::Primary), 1),
            '%' => (Some(Symbol::Secondary), 1),
            '$' => (Some(Symbol::Boundary), 1),
            ':' => (Some(Symbol::Long), 1),
            '*' => (None, 1),
            _ => {
                let sound = SOUNDS.iter().find(|sound| rest.starts_with(**sound))?;
                (Some(Symbol::Sound(sound)), sound.len())
            }
        };
        out.extend(symbol);
        rest = &rest[length..];
    }
    Some(out)
}

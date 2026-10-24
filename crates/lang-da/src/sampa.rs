//! NST's Danish SAMPA, split into its symbols (build.rs checks every transcription with it).

/// The sounds NST writes, longest first where one starts another (s' is ɕ).
const SOUNDS: [&str; 34] = [
    "s'", "i", "e", "E", "a", "A", "y", "2", "9", "u", "o", "O", "Q", "6", "@", "p", "t", "k", "b", "d", "g", "f", "v",
    "s", "h", "m", "n", "N", "l", "R", "r", "j", "w", "D",
];

/// NST's vowels; 6 is ɐ, or ʌ when stressed, and ɐ̯ after another vowel.
pub(crate) const VOWELS: [&str; 14] = ["i", "e", "E", "a", "A", "y", "2", "9", "u", "o", "O", "Q", "6", "@"];

/// One symbol of a transcription.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Symbol {
    /// `"`: the main stress, on the syllable it starts.
    Primary,
    /// `%`: a secondary stress, on the syllable it starts.
    Secondary,
    /// `$`: a syllable ends.
    Boundary,
    /// `:`: the vowel before is long.
    Long,
    /// `?`: stød, on the vowel or consonant before.
    Stod,
    Sound(&'static str),
}

/// `sampa` split into its symbols; `None` for one NST does not use.
pub(crate) fn symbols(sampa: &str) -> Option<Vec<Symbol>> {
    let mut out = Vec::with_capacity(sampa.len());
    let mut rest = sampa;
    while let Some(c) = rest.chars().next() {
        let (symbol, length) = match c {
            '"' => (Symbol::Primary, 1),
            '%' => (Symbol::Secondary, 1),
            '$' => (Symbol::Boundary, 1),
            ':' => (Symbol::Long, 1),
            '?' => (Symbol::Stod, 1),
            _ => {
                let sound = SOUNDS.iter().find(|sound| rest.starts_with(**sound))?;
                (Symbol::Sound(sound), sound.len())
            }
        };
        out.push(symbol);
        rest = &rest[length..];
    }
    Some(out)
}

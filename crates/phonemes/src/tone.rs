/// The pitch of a syllable in a tone language, as Chao's pitch levels: 1 the lowest, 5 the highest
/// (Mandarin's first tone is 55, its third 214).
///
/// A neutral tone has no pitch of its own: its levels, when known, come from the syllable before
/// it, and notations that mark only lexical tones leave it unmarked.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tone {
    levels: &'static [u8],
    neutral: bool,
}

impl Tone {
    /// A lexical tone with these pitch levels (each 1 to 5).
    #[must_use]
    pub const fn new(levels: &'static [u8]) -> Self {
        Self { levels, neutral: false }
    }

    /// A neutral tone, with the pitch levels it takes in context (none when not worked out).
    #[must_use]
    pub const fn neutral(levels: &'static [u8]) -> Self {
        Self { levels, neutral: true }
    }

    #[must_use]
    pub const fn levels(self) -> &'static [u8] {
        self.levels
    }

    #[must_use]
    pub const fn is_neutral(self) -> bool {
        self.neutral
    }

    /// Appends the tone in IPA tone letters (˥ for 5 down to ˩ for 1) to `out`.
    pub fn write_letters(self, out: &mut String) {
        for level in self.levels {
            out.push(match level {
                5 => '˥',
                4 => '˦',
                3 => '˧',
                2 => '˨',
                _ => '˩',
            });
        }
    }

    /// The arrow misaki's notation writes for the tone's shape: → level, ↗ rising, ↘ falling from
    /// high, ↓ low (dipping, or falling from low); none for a neutral tone.
    #[must_use]
    pub fn arrow(self) -> Option<char> {
        let (&first, &last) = (self.levels.first()?, self.levels.last()?);
        if self.neutral {
            return None;
        }
        let lowest = self.levels.iter().copied().min().unwrap_or(first);
        Some(if lowest < first && lowest < last {
            '↓'
        } else if last > first {
            '↗'
        } else if last < first {
            if first >= 4 { '↘' } else { '↓' }
        } else {
            '→'
        })
    }
}

use crate::Notation;

/// Whether a phoneme can be the nucleus of a syllable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PhonemeKind {
    Vowel,
    Consonant,
}

/// The stress a syllable carries.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stress {
    #[default]
    Unstressed,
    /// A secondary stress of rhythm (Spanish ˌimpoɾˈtante), which eSpeak NG's notation marks and plain
    /// IPA, as dictionaries do, leaves out.
    Rhythmic,
    Secondary,
    Primary,
}

impl Stress {
    /// The IPA mark written for this stress in `notation`, if any.
    #[must_use]
    pub fn mark(self, notation: Notation) -> Option<char> {
        match self {
            Self::Unstressed => None,
            Self::Rhythmic if notation == Notation::Ipa => None,
            Self::Rhythmic | Self::Secondary => Some('ˌ'),
            Self::Primary => Some('ˈ'),
        }
    }
}

/// One phoneme of a language, with the symbol each notation writes for it.
///
/// Languages define their phonemes as constants; two phonemes are the same when all their fields
/// are, so a language's inventory decides what counts as distinct.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Phoneme {
    ipa: &'static str,
    espeak: &'static str,
    misaki: &'static str,
    kind: PhonemeKind,
}

impl Phoneme {
    /// A vowel (or another syllable nucleus), written `ipa` in plain IPA and `espeak` in eSpeak NG's
    /// notation, and in misaki's as in eSpeak NG's unless [`Phoneme::misaki`] says otherwise.
    #[must_use]
    pub const fn vowel(ipa: &'static str, espeak: &'static str) -> Self {
        Self {
            ipa,
            espeak,
            misaki: espeak,
            kind: PhonemeKind::Vowel,
        }
    }

    /// A consonant, written `ipa` in plain IPA and `espeak` in eSpeak NG's notation, and in
    /// misaki's as in eSpeak NG's unless [`Phoneme::misaki`] says otherwise.
    #[must_use]
    pub const fn consonant(ipa: &'static str, espeak: &'static str) -> Self {
        Self {
            ipa,
            espeak,
            misaki: espeak,
            kind: PhonemeKind::Consonant,
        }
    }

    /// The same phoneme, written `symbol` in misaki's notation.
    #[must_use]
    pub const fn misaki(self, symbol: &'static str) -> Self {
        Self { misaki: symbol, ..self }
    }

    #[must_use]
    pub const fn kind(self) -> PhonemeKind {
        self.kind
    }

    #[must_use]
    pub const fn is_vowel(self) -> bool {
        matches!(self.kind, PhonemeKind::Vowel)
    }

    /// The symbol written for this phoneme in `notation`.
    #[must_use]
    pub const fn symbol(self, notation: Notation) -> &'static str {
        match notation {
            Notation::Ipa | Notation::Pinyin | Notation::Zhuyin => self.ipa,
            Notation::Espeak => self.espeak,
            Notation::Misaki => self.misaki,
        }
    }
}

/// Which English is read.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Variety {
    #[default]
    American,
    British(BathVowel),
}

/// The vowel of bath, dance and path in British English (docs/decisions.md, decision 28).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum BathVowel {
    /// Broad ɑː in plain IPA, TRAP's a in eSpeak NG's notation, as eSpeak NG's British voice.
    #[default]
    ByNotation,
    /// Broad ɑː in both notations (RP).
    Broad,
    /// TRAP's vowel in both notations (northern English).
    Flat,
}

/// Which Portuguese (docs/decisions.md, decision 40).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Variety {
    /// Portugal's: unstressed vowels reduced (ɐ, ɨ, u), ʃ for a final s (eSpeak NG's `pt` voice).
    #[default]
    European,
    /// Brazil's: unstressed vowels kept, tʃ and dʒ before i, w for a final l (eSpeak NG's `pt-br`),
    /// as Kokoro's Portuguese voices speak.
    Brazilian,
}

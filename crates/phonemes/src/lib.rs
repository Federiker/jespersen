//! The language-independent phoneme model: phonemes, stress, tone, syllables
//! and words, and the notations they are written in. Each language defines its
//! own phonemes and phonotactics on top of it.

mod notation;
mod phoneme;
mod pronunciation;
mod syllabify;
mod tone;

pub use notation::Notation;
pub use phoneme::{Phoneme, PhonemeKind, Stress};
pub use pronunciation::{Pronunciation, Spelling, Syllable};
pub use syllabify::{Segment, syllabify};
pub use tone::Tone;

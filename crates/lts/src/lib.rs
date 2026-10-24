//! Letter-to-sound rules learned from a lexicon (docs/decisions.md, decision
//! 29). [`train`] aligns each word's letters with its phonemes and keeps, for
//! each letter in each context of neighbouring letters, the phonemes it most
//! often says, where that differs from what a shorter context would say.
//! [`predict`] reads a word with the most specific rule that matches each
//! letter. Phonemes are a language's own one-byte codes; this crate only
//! needs to know which codes are the same phoneme for aligning (`base`).

mod align;
mod context;
mod predict;
mod table;
mod train;

pub use context::{BOUNDARY, CONTEXTS, key};
pub use predict::predict;
pub use table::{read_table, write_table};
pub use train::{Entry, Rule, train};

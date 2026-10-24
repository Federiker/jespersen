//! Compact lexicons: a language's build script compiles its entries (a word and
//! one or more encoded pronunciations) with [`compile`]; the language embeds the
//! bytes and searches them in place with [`Lexicon`], without parsing anything
//! when the program starts.
//!
//! The value bytes are the language's own encoding of a pronunciation; this
//! crate only stores and finds them.

mod compile;
mod error;
mod format;
mod lexicon;

pub use compile::compile;
pub use error::LexiconError;
pub use lexicon::Lexicon;

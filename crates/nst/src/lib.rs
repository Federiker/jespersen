//! A development tool, never shipped: the words, parts of speech and pronunciations jespersen reads
//! from NST's lexicons (Nordisk Språkteknologi; Språkbanken at the National Library of Norway, CC0
//! 1.0; docs/decisions.md, decisions 41 to 43). It depends on no language crate, so a broken
//! derived file never stops it from writing a new one.

mod lexicon;

pub use lexicon::{Entry, Language, entries, extract, latin1};

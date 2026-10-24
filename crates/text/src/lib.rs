//! The language-independent text front end: sentences, tokens, their kinds
//! and scripts. Languages read the tokens; they supply what this crate cannot
//! know, such as their abbreviations.

mod addresses;
mod classify;
mod script;
mod sentences;
mod token;

pub use script::Script;
pub use sentences::{Sentence, sentences};
pub use token::{Token, TokenKind};

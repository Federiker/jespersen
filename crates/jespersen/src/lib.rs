//! jespersen: the one crate programs depend on. [`phonemize`] reads text in a
//! [`Language`] into sentences of tokens with their pronunciations; [`write`]
//! turns them into a string in a [`Notation`].

mod error;
mod language;
mod reading;
mod version;
mod write;

pub use error::Error;
#[cfg(feature = "en")]
pub use jespersen_lang_en::BathVowel;
#[cfg(feature = "es")]
pub use jespersen_lang_es::Variety as SpanishVariety;
#[cfg(feature = "pt")]
pub use jespersen_lang_pt::Variety as PortugueseVariety;
#[cfg(feature = "zh")]
pub use jespersen_lang_zh::ToneSandhi;
pub use jespersen_lexicon::LexiconError;
pub use jespersen_phonemes::{Notation, Pronunciation};
pub use jespersen_text::{Script, TokenKind};
pub use language::Language;
pub use reading::{Sentence, Token, phonemize};
pub use version::VERSION;
pub use write::{WriteOptions, write};

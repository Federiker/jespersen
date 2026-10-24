//! Mandarin Chinese, in Simplified or Traditional characters: words found with
//! the lexicon (ipa-dict's lists from 開放漢語詞典, each character's reading
//! from Unihan, jespersen's own entries), numbers written out in Chinese, tone
//! sandhi as spoken or tones as written, in plain IPA or misaki's notation.

mod context;
mod lexicon;
mod numbers;
mod phonemes;
mod pinyin;
mod read;
mod sandhi;
mod segment;
mod spelling;

pub use read::{TokenReading, read_tokens};
pub use sandhi::ToneSandhi;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;

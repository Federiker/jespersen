//! A development tool, never shipped: how often jespersen agrees with eSpeak NG
//! on jespersen's own texts (docs/clean-room.md, "How agreement is measured").
//! `capture` runs eSpeak NG as a black box and keeps its answers in the
//! gitignored `tmp/`; `report` compares jespersen with them. `coverage` needs no oracle: how much
//! of a text list jespersen reads, and how much of Chinese as whole words.

mod capture;
mod compare;
mod coverage;
mod prose;
mod texts;

pub use capture::{Capture, capture, read_capture, write_capture};
pub use compare::{Report, compare};
pub use coverage::{Coverage, coverage};
pub use prose::sentences;
pub use texts::read_texts;

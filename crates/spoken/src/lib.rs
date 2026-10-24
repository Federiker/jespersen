//! What a sentence's tokens are said as, for languages read from their spelling: a front end that
//! finds numbers, dates, money, signs, abbreviations, initialisms and addresses and asks the
//! language's [`Locale`] for their words, and a reader that says each run of words between pauses
//! with the language's [`Speaker`].

mod locale;
pub mod marks;
mod normalise;
mod reading;
mod roman;

pub use locale::{Gender, Locale};
pub use normalise::{Spoken, spoken};
pub use reading::{Speaker, TokenReading, read_tokens};

use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::{Macedonian, expansion},
    spelling::{as_clitic, pronounce, restress},
};

/// Clitics, which join the stressed word after them into one accentual word: the negation не, the
/// future ќе, and the short forms of the pronouns.
const CLITICS: [&str; 17] = [
    "не", "ќе", "ме", "те", "го", "ја", "ве", "ги", "ми", "ти", "му", "ѝ", "ни", "ви", "им", "се", "си",
];
/// Words said without stress in running text: the clitics, prepositions, conjunctions and the
/// short forms of биде (е, сум).
const UNSTRESSED: [&str; 34] = [
    "не", "ќе", "ме", "те", "го", "ја", "ве", "ги", "ми", "ти", "му", "ѝ", "ни", "ви", "им", "се", "си", "во", "на",
    "од", "со", "за", "до", "по", "кон", "и", "а", "но", "да", "што", "е", "сум", "сме", "сте",
];

/// How each token (text and kind) of a Macedonian sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols
/// as words. The words between punctuation marks are read together: function words lose their
/// stress, and clitics join the word after them, which takes the stress on the third syllable
/// from the end of the whole (не знам ˈnɛ znam).
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Macedonian, &Reader)
}

/// How Macedonian says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling, a clitic marked as one; a letter on its own that is not a word by
    /// its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = lower.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && !"аиоувсќ".contains(letter)
            && let Some(name) = Macedonian.letter(letter)
        {
            // A name of two words (дабл ју) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        let said = pronounce(&lower)?;
        Some(if CLITICS.contains(&lower.as_str()) {
            as_clitic(said)
        } else {
            said
        })
    }

    /// An abbreviation written with a hyphen (д-р доктор) is said as its word.
    fn say_joined(&self, word: &str) -> Option<Pronunciation> {
        pronounce(expansion(word)?)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn run(&self, words: &mut [&mut Pronunciation]) {
        restress(words);
    }
}

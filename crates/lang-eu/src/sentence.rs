use jespersen_phonemes::{Phoneme, Pronunciation};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Basque,
    phonemes::{B, B_SOFT, D, D_SOFT, G, G_SOFT, M, N, NY},
    spelling::pronounce,
};

/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 3] = ["eta", "nire", "bere"];

/// How each token (text and kind) of a Basque sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols
/// as words. The words between punctuation marks are read together: some lose their stress, and
/// eSpeak NG's notation writes b, d and g soft except at the start and after a nasal.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Basque, &Reader)
}

/// How Basque says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a letter on its own that is not a word by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = lower.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && !"aeiou".contains(letter)
            && let Some(name) = Basque.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        pronounce(&lower)
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    /// b, d and g are β, ð and ɣ in eSpeak NG's notation except at the start of the run and after
    /// a nasal (dago ðˈaɣo after a vowel, ondo).
    fn run(&self, words: &mut [&mut Pronunciation]) {
        let mut before: Option<Phoneme> = None;
        for said in words.iter_mut() {
            for syllable in &mut said.syllables {
                for part in [&mut syllable.onset, &mut syllable.nucleus, &mut syllable.coda] {
                    for phoneme in part.iter_mut() {
                        let after_nasal = before.is_none_or(|before| [M, N, NY].contains(&before));
                        if !after_nasal {
                            *phoneme = match *phoneme {
                                p if p == B => B_SOFT,
                                p if p == D => D_SOFT,
                                p if p == G => G_SOFT,
                                p => p,
                            };
                        }
                        before = Some(*phoneme);
                    }
                }
            }
        }
    }
}

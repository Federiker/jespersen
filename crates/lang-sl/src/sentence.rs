use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Slovenian,
    phonemes::{is_voiceless, voices},
    spelling::{pronounce, shorten, voice_end},
    words::respelling,
};

/// Words said without stress in running text: the clitic forms of biti and the pronouns, and the
/// conjunctions and prepositions eSpeak NG leaves unstressed.
const UNSTRESSED: [&str; 13] = [
    "je", "so", "sem", "si", "smo", "ste", "sta", "sva", "za", "o", "bil", "ter", "bom",
];
/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 2] = ["bila", "bili"];
/// Prepositions of one consonant, said as the start of the next word without taking its stress (v
/// redu uˈreːdu, z žogo).
const PROCLITICS: [&str; 4] = ["v", "z", "s", "k"];

/// How each token (text and kind) of a Slovenian sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings),
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress, the prepositions of one consonant join the next
/// word, and a word's last obstruents take the voicing of the next word (brat dela brad).
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Slovenian, &Reader)
}

/// How Slovenian says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a letter on its own that is not a word by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = lower.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && !"aiouvzsk".contains(letter)
            && let Some(name) = Slovenian.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        pronounce(respelling(&lower).unwrap_or(&lower))
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    fn proclitics(&self) -> &[&str] {
        &PROCLITICS
    }

    fn proclitic_takes_stress(&self) -> bool {
        false
    }

    /// The short vowels of words that lost their stress, and voicing across words: a word's last
    /// obstruents are voiced before a voiced obstruent (brat dela brad), voiceless before a
    /// voiceless one.
    fn run(&self, words: &mut [&mut Pronunciation]) {
        for said in words.iter_mut() {
            shorten(said);
        }
        for index in 0..words.len().saturating_sub(1) {
            let Some(next) = words[index + 1].phonemes().next() else {
                continue;
            };
            if voices(next) {
                voice_end(words[index], true);
            } else if is_voiceless(next) {
                voice_end(words[index], false);
            }
        }
    }
}

/// How a word is said on its own, for the tests.
#[cfg(test)]
pub(crate) fn say_word(word: &str) -> Option<Pronunciation> {
    Reader.say(word)
}

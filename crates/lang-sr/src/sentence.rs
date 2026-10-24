use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    cyrillic::to_latin,
    locale::Serbian,
    phonemes::{is_voiceless, voices},
    spelling::{pronounce, unstress, voice_end},
};

/// Words said without stress in running text, in both scripts: the clitic forms of biti and the
/// pronouns, the conjunctions i and a, and the prepositions u, o, s and k.
const UNSTRESSED: [&str; 36] = [
    "je", "su", "sam", "si", "smo", "ste", "ću", "ćeš", "li", "ga", "mi", "ti", "i", "a", "u", "o", "s", "k", "је",
    "су", "сам", "си", "смо", "сте", "ћу", "ћеш", "ли", "га", "ми", "ти", "и", "а", "у", "о", "с", "к",
];
/// Words said with a lighter stress (eSpeak NG writes a secondary one in Croatian).
const LIGHTER: [&str; 8] = ["bio", "bila", "bili", "imam", "био", "била", "били", "имам"];

/// How each token (text and kind) of a Serbian sentence, in Cyrillic or Latin letters, is read,
/// in order; `None` for a token jespersen cannot read yet. Words are read from their spelling,
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress, and a word's last obstruents take the voicing of
/// the next word (put do pud do).
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Serbian, &Reader)
}

/// How Serbian says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a letter on its own that is not a word by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = lower.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && !"aiouks".contains(to_latin(&letter.to_string()).as_str())
            && let Some(name) = Serbian.letter(letter)
        {
            // A name of two words (dvostruko ve) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        pronounce(&lower)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    /// Voicing across words (od povrća ot, put do pud), and the unstressed vowels of the words
    /// that lost their stress (u ʊ, i ɪ).
    fn run(&self, words: &mut [&mut Pronunciation]) {
        for index in 0..words.len() {
            let next = words.get(index + 1).and_then(|said| said.phonemes().next());
            let said = &mut *words[index];
            if let Some(next) = next {
                if voices(next) {
                    voice_end(said, true);
                } else if is_voiceless(next) {
                    voice_end(said, false);
                }
            }
            unstress(said);
        }
    }
}

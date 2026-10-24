use jespersen_phonemes::{Phoneme, Pronunciation};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Polish,
    phonemes::{is_obstruent, is_v, is_voiced, with_voicing},
    spelling::pronounce,
};

/// Words said without stress in running text: most prepositions, conjunctions and clitics, and
/// jest and nie, as eSpeak NG reads them.
const UNSTRESSED: [&str; 35] = [
    "w", "we", "z", "na", "do", "od", "o", "po", "pod", "nad", "przed", "przy", "za", "i", "a", "dla", "się", "nie",
    "mi", "ci", "mu", "ja", "ten", "ta", "te", "to", "tym", "on", "jej", "czy", "jest", "są", "ich", "tych", "my",
];
/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 24] = [
    "ale",
    "ona",
    "ono",
    "oni",
    "one",
    "ani",
    "oraz",
    "który",
    "która",
    "które",
    "była",
    "było",
    "byli",
    "jestem",
    "tylko",
    "tego",
    "których",
    "którego",
    "którą",
    "będzie",
    "będziemy",
    "były",
    "mają",
    "którzy",
];

/// How each token (text and kind) of a Polish sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols
/// as words; function words lose their stress in running text, and a word's last obstruents take
/// the voicing of the word after it (jest dużym jɛzd).
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Polish, &Reader)
}

/// How Polish says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a letter on its own that is not a word (i t. d.) by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && !"aiouwzAIOUWZ".contains(letter)
            && let Some(name) = Polish.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        pronounce(word)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    /// Voicing across words, from the last word back: a word's last obstruents are voiced before
    /// a voiced obstruent other than v (jest dużym, nad rzeką), and a word without a vowel (w, z)
    /// loses its voice before a voiceless one (w sobotę f).
    fn run(&self, words: &mut [&mut Pronunciation]) {
        for index in (0..words.len()).rev() {
            let next = words.get(index + 1).and_then(|said| said.phonemes().next());
            let said = &mut *words[index];
            let has_vowel = said.phonemes().any(Phoneme::is_vowel);
            let voiced = match next {
                Some(first) if has_vowel => is_voiced(first) && !is_v(first),
                Some(first) if is_obstruent(first) => is_voiced(first),
                _ => continue,
            };
            if !has_vowel || voiced {
                voice_end(said, voiced);
            }
        }
    }
}

/// Gives the obstruents at the end of `said` the voicing `voiced`.
fn voice_end(said: &mut Pronunciation, voiced: bool) {
    let Some(last) = said.syllables.last_mut() else {
        return;
    };
    let end = if last.nucleus.is_empty() {
        &mut last.onset
    } else {
        &mut last.coda
    };
    for phoneme in end.iter_mut().rev() {
        if !is_obstruent(*phoneme) {
            break;
        }
        *phoneme = with_voicing(*phoneme, voiced);
    }
}

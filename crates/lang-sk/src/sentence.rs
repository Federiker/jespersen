use jespersen_phonemes::{Phoneme, Pronunciation};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Slovak,
    phonemes::{
        K, K_VOICED, S, S_VOICED, V, V_DEVOICED, Z, Z_DEVOICED, before_next, is_final_form, is_obstruent, is_voiced,
    },
    spelling::{pronounce, rhythm},
    words::respelling,
};

/// Words said without stress in running text: the clitic pronouns and forms of byť, the
/// conjunctions a, i and že, and the prepositions na, o, pri and bez (which take the stress of the
/// word they join) and those of one consonant.
const UNSTRESSED: [&str; 23] = [
    "je", "sa", "si", "som", "sme", "ste", "sú", "by", "mi", "ti", "a", "i", "že", "v", "z", "s", "k", "na", "o",
    "pri", "bez", "vo", "so",
];
/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 2] = ["ako", "aby"];
/// Prepositions said as the start of the word after them, which gives them its stress (na stole
/// ˈnastole, pri ohni); do, za, od and pred keep a stress of their own.
const PROCLITICS: [&str; 4] = ["na", "o", "pri", "bez"];

/// How each token (text and kind) of a Slovak sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings),
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress, prepositions join the next word, and a word's
/// last obstruents take the voicing of the next word (otec ide otedz ide).
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Slovak, &Reader)
}

/// How Slovak says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a letter on its own that is not a word by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = lower.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && !"aiouvzsk".contains(letter)
            && let Some(name) = Slovak.letter(letter)
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

    /// eSpeak NG joins a preposition to a word of up to three syllables (nˈavetʃˌeru, but na
    /// kˈonferˌentsii).
    fn joins(&self, next: &Pronunciation) -> bool {
        next.syllables.len() <= 3
    }

    /// Voicing across words, from the last word back: a word's last obstruents are voiced before
    /// a vowel, a sonorant or a voiced obstruent other than v; a preposition of one consonant (v,
    /// z, s, k) takes the voicing of the word after it in plain IPA, which eSpeak NG writes as
    /// spelt. Then the rhythm of the words prepositions joined.
    fn run(&self, words: &mut [&mut Pronunciation]) {
        for index in (0..words.len()).rev() {
            let next = words.get(index + 1).and_then(|said| said.phonemes().next());
            let said = &mut *words[index];
            let has_vowel = said.phonemes().any(Phoneme::is_vowel);
            if has_vowel {
                if let Some(first) = next {
                    let voiced_obstruent = is_obstruent(first) && is_voiced(first) && first != V;
                    if voiced_obstruent || !is_obstruent(first) {
                        voice_end(said, voiced_obstruent);
                    }
                }
            } else if let Some(next) = next {
                preposition(said, next);
            }
            rhythm(said);
        }
    }
}

/// The obstruents at the end of `said` before the next word, voiced by it.
fn voice_end(said: &mut Pronunciation, voiced_obstruent: bool) {
    let Some(last) = said.syllables.last_mut() else {
        return;
    };
    for phoneme in last.coda.iter_mut().rev() {
        if !is_obstruent(*phoneme) && !is_final_form(*phoneme) {
            break;
        }
        *phoneme = before_next(*phoneme, voiced_obstruent);
    }
}

/// A preposition of one consonant before `next`, the first sound of the word after it.
fn preposition(said: &mut Pronunciation, next: Phoneme) {
    let Some([consonant]) = said.syllables.first_mut().map(|syllable| &mut syllable.onset[..]) else {
        return;
    };
    // v gives no voice and takes none (s vami, z vody).
    let voiced = if is_obstruent(next) && next != V {
        is_voiced(next)
    } else {
        [V, Z].contains(consonant)
    };
    *consonant = match (*consonant, voiced) {
        (phoneme, false) if phoneme == V => V_DEVOICED,
        (phoneme, false) if phoneme == Z => Z_DEVOICED,
        (phoneme, true) if phoneme == S => S_VOICED,
        (phoneme, true) if phoneme == K => K_VOICED,
        (phoneme, _) => phoneme,
    };
}

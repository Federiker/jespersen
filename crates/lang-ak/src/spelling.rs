//! How an Akan word is said from its Twi spelling: seven vowels (ɛ, ɔ), the digraphs ky (tɕ), gy
//! (dʑ), hy (ɕ), tw, dw and hw (the same, rounded), kw, nw and ny, r as ɾ, n as ŋ before k and g
//! (nkɔ), and m and n syllabic at the start of a word or after a consonant when no vowel follows
//! (mpɔ, nnipa), but closing the syllable after a vowel (ɔman).
//!
//! Akan is a tone language whose everyday spelling marks no tones; a text that marks them is read
//! without the marks. No syllable is stressed in Akan, but Kokoro learnt eSpeak NG's notation, in
//! which every word that is not a function word carries a stress mark, so each word is marked on
//! its last syllable but one (a syllabic nasal counting as one), as the other West African
//! languages are. eSpeak NG's notation keeps to Kokoro's symbols (decision 48): tʃ, dʒ and ʃ for
//! the alveolo-palatals, w and ɲw for the rounding, and a syllabic nasal as the nasal alone.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::marks::{Letter, letters};

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

const A: Phoneme = vowel("a", "a");
const E: Phoneme = vowel("e", "e");
const E_OPEN: Phoneme = vowel("ɛ", "ɛ");
const I: Phoneme = vowel("i", "i");
const O: Phoneme = vowel("o", "o");
const O_OPEN: Phoneme = vowel("ɔ", "ɔ");
const U: Phoneme = vowel("u", "u");
/// m, n and ŋ as a syllable of their own (mpɔ ˈm̩pɔ, nkɔ ˈŋ̍kɔ).
const M_SYLLABIC: Phoneme = vowel("m\u{329}", "m");
const N_SYLLABIC: Phoneme = vowel("n\u{329}", "n");
const NG_SYLLABIC: Phoneme = vowel("ŋ\u{329}", "ŋ");

const B: Phoneme = consonant("b", "b");
const D: Phoneme = consonant("d", "d");
const DW: Phoneme = consonant("dʑʷ", "dʒw");
const F: Phoneme = consonant("f", "f");
const G: Phoneme = consonant("ɡ", "ɡ");
const GY: Phoneme = consonant("dʑ", "dʒ");
const H: Phoneme = consonant("h", "h");
const HW: Phoneme = consonant("ɕʷ", "ʃw");
const HY: Phoneme = consonant("ɕ", "ʃ");
const K: Phoneme = consonant("k", "k");
const KW: Phoneme = consonant("kʷ", "kw");
const KY: Phoneme = consonant("tɕ", "tʃ");
const L: Phoneme = consonant("l", "l");
const M: Phoneme = consonant("m", "m");
const N: Phoneme = consonant("n", "n");
const NG: Phoneme = consonant("ŋ", "ŋ");
const NW: Phoneme = consonant("ɲʷ", "ɲw");
const NY: Phoneme = consonant("ɲ", "ɲ");
const P: Phoneme = consonant("p", "p");
const R: Phoneme = consonant("ɾ", "ɾ");
const S: Phoneme = consonant("s", "s");
const T: Phoneme = consonant("t", "t");
const TW: Phoneme = consonant("tɕʷ", "tʃw");
const V: Phoneme = consonant("v", "v");
const W: Phoneme = consonant("w", "w");
const Y: Phoneme = consonant("j", "j");
const Z: Phoneme = consonant("z", "z");

fn is_vowel(letter: Option<&Letter>) -> bool {
    letter.is_some_and(|letter| matches!(letter.base, 'a' | 'e' | 'ɛ' | 'i' | 'o' | 'ɔ' | 'u'))
}

/// The sounds of `letters`, in order; `None` for a letter Akan does not read.
fn sounds(letters: &[Letter]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let letter = letters[at].base;
        let next = letters.get(at + 1);
        let pair = match (letter, next.map(|next| next.base)) {
            ('k', Some('y')) => Some(KY),
            ('g', Some('y')) => Some(GY),
            ('h', Some('y')) => Some(HY),
            ('t', Some('w')) => Some(TW),
            ('d', Some('w')) => Some(DW),
            ('h', Some('w')) => Some(HW),
            ('k', Some('w')) => Some(KW),
            ('n', Some('w')) => Some(NW),
            ('n', Some('y')) => Some(NY),
            _ => None,
        };
        if let Some(pair) = pair {
            sounds.push(pair);
            at += 2;
            continue;
        }
        let after_vowel = sounds.last().is_some_and(|sound| sound.is_vowel() && !is_nasal(*sound));
        let velar_next = next.is_some_and(|next| matches!(next.base, 'k' | 'g'));
        let phonemes: &[Phoneme] = match letter {
            'a' => &[A],
            'e' => &[E],
            'ɛ' => &[E_OPEN],
            'i' => &[I],
            'o' => &[O],
            'ɔ' => &[O_OPEN],
            'u' => &[U],
            // m and n with no vowel after them: a syllable of their own, unless they close one.
            'n' | 'm' if !is_vowel(next) && !after_vowel => match letter {
                'm' => &[M_SYLLABIC],
                _ if velar_next => &[NG_SYLLABIC],
                _ => &[N_SYLLABIC],
            },
            'n' if velar_next => &[NG],
            'n' => &[N],
            'm' => &[M],
            'b' => &[B],
            'd' => &[D],
            'j' => &[GY],
            'f' => &[F],
            'g' => &[G],
            'h' => &[H],
            'k' | 'c' | 'q' => &[K],
            'l' => &[L],
            'p' => &[P],
            'r' => &[R],
            's' => &[S],
            't' => &[T],
            'v' => &[V],
            'w' => &[W],
            'y' => &[Y],
            'z' => &[Z],
            'x' => &[K, S],
            '\'' | '’' | '-' => &[],
            _ => return None,
        };
        sounds.extend_from_slice(phonemes);
        at += 1;
    }
    Some(sounds)
}

fn is_nasal(sound: Phoneme) -> bool {
    [M_SYLLABIC, N_SYLLABIC, NG_SYLLABIC].contains(&sound)
}

/// Whether `cluster` can start a syllable: Akan's syllables have one consonant at most.
fn legal_onset(cluster: &[Phoneme]) -> bool {
    cluster.len() == 1
}

/// How `word` is said, marked on its last syllable but one; `None` for a letter Akan does not
/// read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let sounds = sounds(&letters(word))?;
    let vowels: Vec<usize> = (0..sounds.len()).filter(|&index| sounds[index].is_vowel()).collect();
    let &stressed = vowels.get(vowels.len().saturating_sub(2))?;
    let segments: Vec<Segment> = sounds
        .iter()
        .enumerate()
        .map(|(index, phoneme)| {
            Segment::new(
                *phoneme,
                if index == stressed {
                    Stress::Primary
                } else {
                    Stress::Unstressed
                },
            )
        })
        .collect();
    Some(syllabify(&segments, legal_onset))
}

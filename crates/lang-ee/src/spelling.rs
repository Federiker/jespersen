//! How an Ewe word is said from its spelling: seven vowels (ɛ, ɔ), nasal with a tilde (ɛ̃, ɔ̃) or
//! before an n that no vowel follows (dɔn), ɖ, ƒ as ɸ, ɣ, h as ɦ, ŋ, ʋ, x, the digraphs dz, gb, kp,
//! ny and ts, and m, n and ŋ syllabic beside no vowel (ŋkeke, m̀).
//!
//! Ewe is a tone language: its tone marks (á, à, ǎ) are read, whether precomposed or combining,
//! and not said, since Kokoro, which reads eSpeak NG's notation, has no tones; plain IPA leaves
//! them out too. No syllable is stressed in Ewe, but Kokoro learnt eSpeak NG's notation, in which
//! every word that is not a function word carries a stress mark, so each word is marked on its
//! last syllable but one (a syllabic nasal counting as one), as the other West African languages
//! are. eSpeak NG's notation keeps to Kokoro's symbols (decision 48): ɡb and kp without the tie,
//! h for ɦ, and a syllabic nasal as the nasal alone.

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
const A_NASAL: Phoneme = vowel("a\u{303}", "a\u{303}");
const E_NASAL: Phoneme = vowel("e\u{303}", "e\u{303}");
const E_OPEN_NASAL: Phoneme = vowel("ɛ\u{303}", "ɛ\u{303}");
const I_NASAL: Phoneme = vowel("i\u{303}", "i\u{303}");
const O_NASAL: Phoneme = vowel("o\u{303}", "o\u{303}");
const O_OPEN_NASAL: Phoneme = vowel("ɔ\u{303}", "ɔ\u{303}");
const U_NASAL: Phoneme = vowel("u\u{303}", "u\u{303}");
/// m, n and ŋ as a syllable of their own (ŋkeke ŋ̍ˈkeke).
const M_SYLLABIC: Phoneme = vowel("m\u{329}", "m");
const N_SYLLABIC: Phoneme = vowel("n\u{329}", "n");
const NG_SYLLABIC: Phoneme = vowel("ŋ\u{329}", "ŋ");

const B: Phoneme = consonant("b", "b");
const D: Phoneme = consonant("d", "d");
const D_RETROFLEX: Phoneme = consonant("ɖ", "ɖ");
const DZ: Phoneme = consonant("dz", "dz");
const F: Phoneme = consonant("f", "f");
const F_BILABIAL: Phoneme = consonant("ɸ", "ɸ");
const G: Phoneme = consonant("ɡ", "ɡ");
const GB: Phoneme = consonant("ɡ\u{361}b", "ɡb");
const GH: Phoneme = consonant("ɣ", "ɣ");
const H: Phoneme = consonant("ɦ", "h");
const K: Phoneme = consonant("k", "k");
const KP: Phoneme = consonant("k\u{361}p", "kp");
const L: Phoneme = consonant("l", "l");
const M: Phoneme = consonant("m", "m");
const N: Phoneme = consonant("n", "n");
const NG: Phoneme = consonant("ŋ", "ŋ");
const NY: Phoneme = consonant("ɲ", "ɲ");
const P: Phoneme = consonant("p", "p");
const R: Phoneme = consonant("ɾ", "ɾ");
const S: Phoneme = consonant("s", "s");
const T: Phoneme = consonant("t", "t");
const TS: Phoneme = consonant("ts", "ts");
const V: Phoneme = consonant("v", "v");
const V_BILABIAL: Phoneme = consonant("β", "β");
const W: Phoneme = consonant("w", "w");
const X: Phoneme = consonant("x", "x");
const Y: Phoneme = consonant("j", "j");
const Z: Phoneme = consonant("z", "z");

fn is_vowel(letter: Option<&Letter>) -> bool {
    letter.is_some_and(|letter| matches!(letter.base, 'a' | 'e' | 'ɛ' | 'i' | 'o' | 'ɔ' | 'u'))
}

/// The oral or nasal vowel a vowel letter stands for.
fn vowel_sound(letter: char, nasal: bool) -> Phoneme {
    match (letter, nasal) {
        ('a', false) => A,
        ('a', true) => A_NASAL,
        ('e', false) => E,
        ('e', true) => E_NASAL,
        ('ɛ', false) => E_OPEN,
        ('ɛ', true) => E_OPEN_NASAL,
        ('i', false) => I,
        ('i', true) => I_NASAL,
        ('o', false) => O,
        ('o', true) => O_NASAL,
        ('ɔ', false) => O_OPEN,
        ('ɔ', true) => O_OPEN_NASAL,
        (_, false) => U,
        (_, true) => U_NASAL,
    }
}

/// The sounds of `letters`, in order; `None` for a letter Ewe does not read.
fn sounds(letters: &[Letter]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let letter = letters[at];
        let next = letters.get(at + 1);
        let pair = match (letter.base, next.map(|next| next.base)) {
            ('d', Some('z')) => Some(DZ),
            ('g', Some('b')) => Some(GB),
            ('k', Some('p')) => Some(KP),
            ('n', Some('y')) => Some(NY),
            ('t', Some('s')) => Some(TS),
            _ => None,
        };
        if let Some(pair) = pair {
            sounds.push(pair);
            at += 2;
            continue;
        }
        let mut step = 1;
        let after_vowel = sounds.last().is_some_and(|sound| sound.is_vowel());
        let phonemes: &[Phoneme] = match letter.base {
            vowel @ ('a' | 'e' | 'ɛ' | 'i' | 'o' | 'ɔ' | 'u') => {
                // A vowel before an n that no vowel follows is nasal, and the n is not said.
                let nasal_n = next.is_some_and(|next| next.base == 'n') && !is_vowel(letters.get(at + 2));
                if nasal_n {
                    step = 2;
                }
                sounds.push(vowel_sound(vowel, letter.tilde || nasal_n));
                &[]
            }
            // m, n and ŋ with no vowel before or after them are a syllable of their own.
            'm' if !is_vowel(next) && !after_vowel => &[M_SYLLABIC],
            'n' if !is_vowel(next) && !after_vowel => &[N_SYLLABIC],
            'ŋ' if !is_vowel(next) && !after_vowel => &[NG_SYLLABIC],
            'm' => &[M],
            'n' => &[N],
            'ŋ' => &[NG],
            'b' => &[B],
            'd' => &[D],
            'ɖ' => &[D_RETROFLEX],
            'f' => &[F],
            'ƒ' => &[F_BILABIAL],
            'g' => &[G],
            'ɣ' => &[GH],
            'h' => &[H],
            'k' | 'c' | 'q' => &[K],
            'l' => &[L],
            'p' => &[P],
            'r' => &[R],
            's' => &[S],
            't' => &[T],
            'v' => &[V],
            'ʋ' => &[V_BILABIAL],
            'w' => &[W],
            'x' => &[X],
            'y' => &[Y],
            'z' => &[Z],
            '\'' | '’' | '-' => &[],
            _ => return None,
        };
        sounds.extend_from_slice(phonemes);
        at += step;
    }
    Some(sounds)
}

/// Whether `cluster` can start a syllable: one consonant, or one before l or r (blema, kple).
fn legal_onset(cluster: &[Phoneme]) -> bool {
    match cluster {
        [_] => true,
        [first, second] => [L, R].contains(second) && ![L, R, M, N, NG, NY, W, Y].contains(first),
        _ => false,
    }
}

/// How `word` is said, marked on its last syllable but one; `None` for a letter Ewe does not read.
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

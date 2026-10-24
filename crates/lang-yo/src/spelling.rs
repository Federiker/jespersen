//! How a Yoruba word is said from its spelling: seven oral vowels (ẹ ɛ, ọ ɔ), a vowel before an n
//! that no vowel follows nasal (ìlú wọn, ẹran), n and m syllabic when no vowel is next to them
//! (ń lọ, mbá), gb as ɡ͡b, p as k͡p, ṣ as ʃ and j as ɟ.
//!
//! Yoruba is a tone language: its accents (á high, à low, none for mid) are read, whether written
//! precomposed or combining, and not said, since Kokoro, which reads eSpeak NG's notation, has no
//! tones; plain IPA leaves them out too, for one reading in both notations. No syllable is
//! stressed in Yoruba, but Kokoro learnt eSpeak NG's notation, in which every word that is not a
//! function word carries a stress mark and a word without one is said reduced, so each word is
//! marked on its last syllable but one, as Swahili is (a syllabic nasal counting as a syllable).
//! eSpeak NG's notation keeps to Kokoro's symbols (decision 48): ɡb and kp without the tie, and a
//! syllabic nasal as the nasal alone.

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
/// n and m as a syllable of their own (ń lọ ˈn̩lɔ).
const N_SYLLABIC: Phoneme = vowel("n\u{329}", "n");
const M_SYLLABIC: Phoneme = vowel("m\u{329}", "m");

const B: Phoneme = consonant("b", "b");
const D: Phoneme = consonant("d", "d");
const F: Phoneme = consonant("f", "f");
const G: Phoneme = consonant("ɡ", "ɡ");
const GB: Phoneme = consonant("ɡ\u{361}b", "ɡb");
const H: Phoneme = consonant("h", "h");
const J: Phoneme = consonant("ɟ", "ɟ");
const K: Phoneme = consonant("k", "k");
const KP: Phoneme = consonant("k\u{361}p", "kp");
const L: Phoneme = consonant("l", "l");
const M: Phoneme = consonant("m", "m");
const N: Phoneme = consonant("n", "n");
const R: Phoneme = consonant("ɾ", "ɾ");
const S: Phoneme = consonant("s", "s");
const SH: Phoneme = consonant("ʃ", "ʃ");
const T: Phoneme = consonant("t", "t");
const V: Phoneme = consonant("v", "v");
const W: Phoneme = consonant("w", "w");
const Y: Phoneme = consonant("j", "j");
const Z: Phoneme = consonant("z", "z");

fn is_vowel(letter: Option<&Letter>) -> bool {
    letter.is_some_and(|letter| matches!(letter.base, 'a' | 'e' | 'i' | 'o' | 'u'))
}

/// The oral or nasal vowel a vowel letter stands for.
fn vowel_sound(letter: Letter, nasal: bool) -> Phoneme {
    match (letter.base, letter.below, nasal) {
        ('a', _, false) => A,
        ('a', _, true) => A_NASAL,
        ('e', false, false) => E,
        ('e', false, true) => E_NASAL,
        ('e', true, false) => E_OPEN,
        ('e', true, true) => E_OPEN_NASAL,
        ('i', _, false) => I,
        ('i', _, true) => I_NASAL,
        ('o', false, false) => O,
        ('o', false, true) => O_NASAL,
        ('o', true, false) => O_OPEN,
        ('o', true, true) => O_OPEN_NASAL,
        (_, _, false) => U,
        (_, _, true) => U_NASAL,
    }
}

/// The sounds of `letters`, in order; `None` for a letter Yoruba does not read.
fn sounds(letters: &[Letter]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let letter = letters[at];
        let next = letters.get(at + 1);
        let mut step = 1;
        let phonemes: &[Phoneme] = match letter.base {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                // A vowel before an n that no vowel follows is nasal, and the n is not said.
                let nasal_n =
                    next.is_some_and(|next| next.base == 'n' && !next.above) && !is_vowel(letters.get(at + 2));
                if nasal_n {
                    step = 2;
                }
                sounds.push(vowel_sound(letter, letter.tilde || nasal_n));
                &[]
            }
            // n and m with no vowel before or after them are a syllable of their own.
            'n' | 'm' if !is_vowel(next) && !sounds.last().is_some_and(|sound| sound.is_vowel()) => {
                if letter.base == 'n' {
                    &[N_SYLLABIC]
                } else {
                    &[M_SYLLABIC]
                }
            }
            'n' => &[N],
            'm' => &[M],
            'g' if next.is_some_and(|next| next.base == 'b') => {
                step = 2;
                &[GB]
            }
            'g' => &[G],
            'p' => &[KP],
            's' if letter.below => &[SH],
            's' => &[S],
            'b' => &[B],
            'd' => &[D],
            'f' => &[F],
            'h' => &[H],
            'j' => &[J],
            'k' | 'c' | 'q' => &[K],
            'l' => &[L],
            'r' => &[R],
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
        at += step;
    }
    Some(sounds)
}

/// Whether `cluster` can start a syllable: Yoruba's syllables have one consonant at most.
fn legal_onset(cluster: &[Phoneme]) -> bool {
    cluster.len() == 1
}

/// How `word` is said, marked on its last syllable but one; `None` for a letter Yoruba does not
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

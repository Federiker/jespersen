//! How a Hausa word is said from its spelling (boko): five vowels and the diphthongs ai and au,
//! the implosives ɓ and ɗ, the ejectives ƙ and ts, ʼy (or ƴ) as ʔʲ, c as tʃ, f as ɸ, sh as ʃ, the
//! labialised and palatalised gw, kw, ƙw, gy, ky, ƙy and fy, an apostrophe between vowels as a
//! glottal stop (saʼa), and a doubled consonant long (dalla).
//!
//! Hausa's tones and vowel lengths are not written in everyday spelling, so they are not read: a
//! text that marks them (à, â, ā) is read without the marks. No syllable is stressed in Hausa, but
//! Kokoro learnt eSpeak NG's notation, in which every word that is not a function word carries a
//! stress mark, so each word is marked on its last syllable but one, as the other West African
//! languages are. eSpeak NG's notation keeps to Kokoro's symbols (decision 48): b and d for the
//! implosives, k and s for the ejectives, j for ʔʲ, f for ɸ, and w and j for the labialisation and
//! palatalisation.

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
const I: Phoneme = vowel("i", "i");
const O: Phoneme = vowel("o", "o");
const U: Phoneme = vowel("u", "u");
const AI: Phoneme = vowel("ai", "aɪ");
const AU: Phoneme = vowel("au", "aʊ");

const B: Phoneme = consonant("b", "b");
const B_IMPLOSIVE: Phoneme = consonant("ɓ", "b");
const C: Phoneme = consonant("tʃ", "tʃ");
const D: Phoneme = consonant("d", "d");
const D_IMPLOSIVE: Phoneme = consonant("ɗ", "d");
const F: Phoneme = consonant("ɸ", "f");
const FY: Phoneme = consonant("ɸʲ", "fj");
const G: Phoneme = consonant("ɡ", "ɡ");
const GW: Phoneme = consonant("ɡʷ", "ɡw");
const GY: Phoneme = consonant("ɡʲ", "ɡj");
const GLOTTAL: Phoneme = consonant("ʔ", "ʔ");
const H: Phoneme = consonant("h", "h");
const J: Phoneme = consonant("dʒ", "dʒ");
const K: Phoneme = consonant("k", "k");
const KW: Phoneme = consonant("kʷ", "kw");
const KY: Phoneme = consonant("kʲ", "kj");
const K_EJECTIVE: Phoneme = consonant("kʼ", "k");
const KW_EJECTIVE: Phoneme = consonant("kʼʷ", "kw");
const KY_EJECTIVE: Phoneme = consonant("kʼʲ", "kj");
const L: Phoneme = consonant("l", "l");
const M: Phoneme = consonant("m", "m");
const N: Phoneme = consonant("n", "n");
const P: Phoneme = consonant("p", "p");
const R: Phoneme = consonant("ɾ", "ɾ");
const S: Phoneme = consonant("s", "s");
const SH: Phoneme = consonant("ʃ", "ʃ");
const T: Phoneme = consonant("t", "t");
const TS: Phoneme = consonant("sʼ", "s");
const V: Phoneme = consonant("v", "v");
const W: Phoneme = consonant("w", "w");
const Y: Phoneme = consonant("j", "j");
const Y_GLOTTAL: Phoneme = consonant("ʔʲ", "j");
const Z: Phoneme = consonant("z", "z");
/// The length of a doubled consonant (dalla ˈdalːa).
const LONG: Phoneme = consonant("ː", "ː");

fn is_vowel(letter: Option<&Letter>) -> bool {
    letter.is_some_and(|letter| matches!(letter.base, 'a' | 'e' | 'i' | 'o' | 'u'))
}

fn base(letter: Option<&Letter>) -> Option<char> {
    letter.map(|letter| letter.base)
}

fn is_apostrophe(c: Option<char>) -> bool {
    matches!(c, Some('\'' | '’' | 'ʼ'))
}

/// The sound of a consonant letter, or of it and the letter after it, and how many letters it
/// takes; `None` for a letter Hausa does not read.
fn consonant_sound(letter: char, next: Option<char>) -> Option<(Phoneme, usize)> {
    Some(match (letter, next) {
        ('s', Some('h')) => (SH, 2),
        ('t', Some('s')) => (TS, 2),
        ('g', Some('w')) => (GW, 2),
        ('g', Some('y')) => (GY, 2),
        ('k', Some('w')) => (KW, 2),
        ('k', Some('y')) => (KY, 2),
        ('ƙ', Some('w')) => (KW_EJECTIVE, 2),
        ('ƙ', Some('y')) => (KY_EJECTIVE, 2),
        ('f', Some('y')) => (FY, 2),
        ('b', _) => (B, 1),
        ('ɓ', _) => (B_IMPLOSIVE, 1),
        ('c', _) => (C, 1),
        ('d', _) => (D, 1),
        ('ɗ', _) => (D_IMPLOSIVE, 1),
        ('f', _) => (F, 1),
        ('g', _) => (G, 1),
        ('h', _) => (H, 1),
        ('j', _) => (J, 1),
        ('k' | 'q', _) => (K, 1),
        ('ƙ', _) => (K_EJECTIVE, 1),
        ('l', _) => (L, 1),
        ('m', _) => (M, 1),
        ('n', _) => (N, 1),
        ('p', _) => (P, 1),
        ('r', _) => (R, 1),
        ('s', _) => (S, 1),
        ('t', _) => (T, 1),
        ('v', _) => (V, 1),
        ('w', _) => (W, 1),
        ('y', _) => (Y, 1),
        ('ƴ', _) => (Y_GLOTTAL, 1),
        ('z', _) => (Z, 1),
        _ => return None,
    })
}

/// The sounds of `letters`, in order; `None` for a letter Hausa does not read.
fn sounds(letters: &[Letter]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let letter = letters[at].base;
        let next = base(letters.get(at + 1));
        match letter {
            'a' if matches!(next, Some('i' | 'u')) && !is_vowel(letters.get(at + 2)) => {
                sounds.push(if next == Some('i') { AI } else { AU });
                at += 2;
            }
            'a' | 'e' | 'i' | 'o' | 'u' => {
                sounds.push(match letter {
                    'a' => A,
                    'e' => E,
                    'i' => I,
                    'o' => O,
                    _ => U,
                });
                at += 1;
            }
            // ʼy is a glottalised y; an apostrophe between vowels a glottal stop.
            c if is_apostrophe(Some(c)) && next == Some('y') => {
                sounds.push(Y_GLOTTAL);
                at += 2;
            }
            c if is_apostrophe(Some(c)) => {
                if !sounds.is_empty() && is_vowel(letters.get(at + 1)) {
                    sounds.push(GLOTTAL);
                }
                at += 1;
            }
            '-' => at += 1,
            'x' => {
                sounds.extend([K, S]);
                at += 1;
            }
            letter => {
                let (sound, step) = consonant_sound(letter, next)?;
                // A doubled consonant (dd, kk, shsh) is long.
                let doubled = letters.get(at + step..at + 2 * step).is_some_and(|following| {
                    following
                        .iter()
                        .zip(&letters[at..at + step])
                        .all(|(one, other)| one.base == other.base)
                });
                sounds.push(sound);
                if doubled {
                    sounds.push(LONG);
                    at += step;
                }
                at += step;
            }
        }
    }
    Some(sounds)
}

/// Whether `cluster` can start a syllable: one consonant, or the length of a doubled one, whose
/// first half closes the syllable before (dal.ːa).
fn legal_onset(cluster: &[Phoneme]) -> bool {
    cluster.len() == 1
}

/// How `word` is said, marked on its last syllable but one; `None` for a letter Hausa does not
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

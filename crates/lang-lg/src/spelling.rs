//! How a Luganda word is said from its spelling: one sound per letter or group (c and ky c, j and
//! gy ɟ, ny ɲ, ŋ and ng' ŋ), r as ɾ, a doubled vowel long, a doubled consonant long and split
//! between two syllables (ekkubo ek.ku.bo, nnyini ɲ.ɲi.ni), n as ŋ before k and g, and a vowel
//! long before a nasal and a consonant (Luganda luɡaːnda).
//!
//! Luganda is a tone language without stress, and its tones are not written. For voices that need
//! a stress, the last syllable but one takes it, as in Swahili. eSpeak NG has no Luganda voice, so
//! its notation keeps to the symbols Kokoro reads (decision 48).

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

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
const A_LONG: Phoneme = vowel("aː", "aː");
const E_LONG: Phoneme = vowel("eː", "eː");
const I_LONG: Phoneme = vowel("iː", "iː");
const O_LONG: Phoneme = vowel("oː", "oː");
const U_LONG: Phoneme = vowel("uː", "uː");

const B: Phoneme = consonant("b", "b");
/// c and ky, a palatal stop.
const C: Phoneme = consonant("c", "c");
const D: Phoneme = consonant("d", "d");
const F: Phoneme = consonant("f", "f");
const G: Phoneme = consonant("ɡ", "ɡ");
const H: Phoneme = consonant("h", "h");
/// j and gy, a voiced palatal stop.
const J: Phoneme = consonant("ɟ", "ɟ");
const K: Phoneme = consonant("k", "k");
const L: Phoneme = consonant("l", "l");
const M: Phoneme = consonant("m", "m");
const N: Phoneme = consonant("n", "n");
const NG: Phoneme = consonant("ŋ", "ŋ");
const NY: Phoneme = consonant("ɲ", "ɲ");
const P: Phoneme = consonant("p", "p");
const R: Phoneme = consonant("ɾ", "ɾ");
const S: Phoneme = consonant("s", "s");
const T: Phoneme = consonant("t", "t");
const V: Phoneme = consonant("v", "v");
const W: Phoneme = consonant("w", "w");
const Y: Phoneme = consonant("j", "j");
const Z: Phoneme = consonant("z", "z");

/// Whether `cluster` can start a syllable: any cluster but a long consonant, which is split
/// between two syllables (ekkubo ek.ku.bo); Luganda's syllables are otherwise open.
fn legal_onset(cluster: &[Phoneme]) -> bool {
    !matches!(cluster, [first, second, ..] if first == second)
}

/// A vowel letter without a tone mark, should a text write one.
fn plain_vowel(letter: char) -> Option<char> {
    Some(match letter {
        'a' | 'á' | 'à' | 'â' => 'a',
        'e' | 'é' | 'è' | 'ê' => 'e',
        'i' | 'í' | 'ì' | 'î' => 'i',
        'o' | 'ó' | 'ò' | 'ô' => 'o',
        'u' | 'ú' | 'ù' | 'û' => 'u',
        _ => return None,
    })
}

fn vowel_of(letter: char, long: bool) -> Phoneme {
    match (letter, long) {
        ('a', false) => A,
        ('a', true) => A_LONG,
        ('e', false) => E,
        ('e', true) => E_LONG,
        ('i', false) => I,
        ('i', true) => I_LONG,
        ('o', false) => O,
        ('o', true) => O_LONG,
        (_, false) => U,
        (_, true) => U_LONG,
    }
}

fn long_vowel(phoneme: Phoneme) -> Phoneme {
    [(A, A_LONG), (E, E_LONG), (I, I_LONG), (O, O_LONG), (U, U_LONG)]
        .iter()
        .find(|(short, _)| *short == phoneme)
        .map_or(phoneme, |(_, long)| *long)
}

/// The sound of one consonant letter or group at `at` and how many letters it takes; `None` for
/// a letter Luganda does not read.
fn consonant_at(letters: &[char], at: usize) -> Option<(&'static [Phoneme], usize)> {
    let next = letters.get(at + 1).copied();
    let then_y = letters.get(at + 2) == Some(&'y');
    Some(match (letters[at], next) {
        // nny, kky and ggy: the long ɲ, c and ɟ (nnyini, ekkya).
        ('n', Some('n')) if then_y => (&[NY, NY], 3),
        ('k', Some('k')) if then_y => (&[C, C], 3),
        ('g', Some('g')) if then_y => (&[J, J], 3),
        ('n', Some('y')) => (&[NY], 2),
        // ng' is ŋ (ng'ombe); n before k and g is ŋ (enkoko).
        ('n', Some('g')) if matches!(letters.get(at + 2), Some('\'' | '’')) => (&[NG], 3),
        ('n', Some('k' | 'g')) => (&[NG], 1),
        ('k', Some('y')) if letters.get(at + 2).copied().and_then(plain_vowel).is_some() => (&[C], 2),
        ('g', Some('y')) if letters.get(at + 2).copied().and_then(plain_vowel).is_some() => (&[J], 2),
        (letter, _) => (
            match letter {
                'b' => &[B],
                'c' => &[C],
                'd' => &[D],
                'f' => &[F],
                'g' => &[G],
                'h' => &[H],
                'j' => &[J],
                'k' | 'q' => &[K],
                'l' => &[L],
                'm' => &[M],
                'n' => &[N],
                'ŋ' => &[NG],
                'p' => &[P],
                'r' => &[R],
                's' => &[S],
                't' => &[T],
                'v' => &[V],
                'w' => &[W],
                'x' => &[K, S],
                'y' => &[Y],
                'z' => &[Z],
                '\'' | '’' | '-' => &[],
                _ => return None,
            },
            1,
        ),
    })
}

/// The sounds of `letters`, in order; `None` for a letter Luganda does not read. A doubled letter
/// is said twice, a long consonant (kk, nny, ŋŋ), except a doubled vowel, which is one long vowel.
pub(crate) fn sounds(letters: &[char]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        if let Some(plain) = plain_vowel(letters[at]) {
            let long = letters.get(at + 1).copied().and_then(plain_vowel) == Some(plain);
            sounds.push(vowel_of(plain, long));
            at += if long { 2 } else { 1 };
            continue;
        }
        let (phonemes, step) = consonant_at(letters, at)?;
        sounds.extend_from_slice(phonemes);
        at += step;
    }
    Some(sounds)
}

/// A vowel before a nasal and another consonant is long (Luganda luɡaːnda, enkoko eːŋkoko).
fn lengthen_before_nasals(sounds: &mut [Phoneme]) {
    for index in 0..sounds.len() {
        let Some(&nasal) = sounds.get(index + 1) else { break };
        let consonant = sounds
            .get(index + 2)
            .is_some_and(|after| !after.is_vowel() && ![W, Y, nasal].contains(after));
        if sounds[index].is_vowel() && [M, N, NG, NY].contains(&nasal) && consonant {
            sounds[index] = long_vowel(sounds[index]);
        }
    }
}

/// The sounds of `word`, in lower case, with the long vowels before a nasal; `None` for a letter
/// Luganda does not read.
pub(crate) fn word_sounds(word: &str) -> Option<Vec<Phoneme>> {
    let letters: Vec<char> = word.to_lowercase().chars().collect();
    let mut sounds = sounds(&letters)?;
    lengthen_before_nasals(&mut sounds);
    Some(sounds)
}

/// `sounds` as a word stressed on its last syllable but one; `None` without a vowel.
pub(crate) fn stressed(sounds: &[Phoneme]) -> Option<Pronunciation> {
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

/// How `word` is said, stressed on its last syllable but one; `None` for a letter Luganda does
/// not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    stressed(&word_sounds(word)?)
}

//! How a Lingala word is said from its spelling: seven vowels (a, e, ɛ, i, o, ɔ, u; è and ò taken
//! for ɛ and ɔ), its tone marks set aside (mbóka, ndeko), one sound per letter or group (ny ɲ, sh
//! ʃ, gb, kp), n as ŋ before k and g, prenasalised consonants starting their syllable (mbóka
//! ˈmboka), and a doubled vowel long.
//!
//! Lingala is a tone language without stress. For voices that need a stress, the last syllable
//! but one takes it, as in Swahili. eSpeak NG has no Lingala voice, so its notation keeps to the
//! symbols Kokoro reads (decision 48).

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

const A: Phoneme = vowel("a", "a");
const E: Phoneme = vowel("e", "e");
const EH: Phoneme = vowel("ɛ", "ɛ");
const I: Phoneme = vowel("i", "i");
const O: Phoneme = vowel("o", "o");
const OH: Phoneme = vowel("ɔ", "ɔ");
const U: Phoneme = vowel("u", "u");
const A_LONG: Phoneme = vowel("aː", "aː");
const E_LONG: Phoneme = vowel("eː", "eː");
const EH_LONG: Phoneme = vowel("ɛː", "ɛː");
const I_LONG: Phoneme = vowel("iː", "iː");
const O_LONG: Phoneme = vowel("oː", "oː");
const OH_LONG: Phoneme = vowel("ɔː", "ɔː");
const U_LONG: Phoneme = vowel("uː", "uː");

const B: Phoneme = consonant("b", "b");
const D: Phoneme = consonant("d", "d");
const F: Phoneme = consonant("f", "f");
const G: Phoneme = consonant("ɡ", "ɡ");
const H: Phoneme = consonant("h", "h");
const J: Phoneme = consonant("dʒ", "dʒ");
const K: Phoneme = consonant("k", "k");
const L: Phoneme = consonant("l", "l");
const M: Phoneme = consonant("m", "m");
const N: Phoneme = consonant("n", "n");
const NG: Phoneme = consonant("ŋ", "ŋ");
const NY: Phoneme = consonant("ɲ", "ɲ");
const P: Phoneme = consonant("p", "p");
const R: Phoneme = consonant("ɾ", "ɾ");
const S: Phoneme = consonant("s", "s");
const SH: Phoneme = consonant("ʃ", "ʃ");
const T: Phoneme = consonant("t", "t");
const V: Phoneme = consonant("v", "v");
const W: Phoneme = consonant("w", "w");
const Y: Phoneme = consonant("j", "j");
const Z: Phoneme = consonant("z", "z");

/// Every cluster can start a syllable: Lingala's syllables are open, and the consonants before a
/// vowel all begin its syllable (mbóka, nzambe).
fn legal_onset(_cluster: &[Phoneme]) -> bool {
    true
}

/// A vowel letter without its tone mark (á, â, ǎ, ɛ́…), as one of Lingala's seven vowels; `None`
/// for another letter.
fn plain_vowel(letter: char) -> Option<char> {
    Some(match letter {
        'a' | 'á' | 'à' | 'â' | 'ǎ' => 'a',
        'e' | 'é' | 'ê' | 'ě' => 'e',
        'ɛ' | 'è' => 'ɛ',
        'i' | 'í' | 'ì' | 'î' | 'ǐ' => 'i',
        'o' | 'ó' | 'ô' | 'ǒ' => 'o',
        'ɔ' | 'ò' => 'ɔ',
        'u' | 'ú' | 'ù' | 'û' | 'ǔ' => 'u',
        _ => return None,
    })
}

fn vowel_of(letter: char, long: bool) -> Phoneme {
    match (letter, long) {
        ('a', false) => A,
        ('a', true) => A_LONG,
        ('e', false) => E,
        ('e', true) => E_LONG,
        ('ɛ', false) => EH,
        ('ɛ', true) => EH_LONG,
        ('i', false) => I,
        ('i', true) => I_LONG,
        ('o', false) => O,
        ('o', true) => O_LONG,
        ('ɔ', false) => OH,
        ('ɔ', true) => OH_LONG,
        (_, false) => U,
        (_, true) => U_LONG,
    }
}

/// The sounds of `letters`, in order, tone marks written as combining accents left out; `None`
/// for a letter Lingala does not read.
fn sounds(letters: &[char]) -> Option<Vec<Phoneme>> {
    let letters: Vec<char> = letters
        .iter()
        .copied()
        .filter(|letter| !('\u{300}'..='\u{36f}').contains(letter))
        .collect();
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        if let Some(plain) = plain_vowel(letters[at]) {
            let long = letters.get(at + 1).copied().and_then(plain_vowel) == Some(plain);
            sounds.push(vowel_of(plain, long));
            at += if long { 2 } else { 1 };
            continue;
        }
        let next = letters.get(at + 1).copied();
        let (phonemes, step): (&[Phoneme], usize) = match (letters[at], next) {
            ('n', Some('y')) => (&[NY], 2),
            ('s', Some('h')) => (&[SH], 2),
            // n is ŋ before k and g (nkoko, ngai).
            ('n', Some('k' | 'g')) => (&[NG], 1),
            (letter, _) => (
                match letter {
                    'b' => &[B],
                    'd' => &[D],
                    'f' => &[F],
                    'g' => &[G],
                    'h' => &[H],
                    'j' => &[J],
                    'k' | 'c' | 'q' => &[K],
                    'l' => &[L],
                    'm' => &[M],
                    'n' => &[N],
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
        };
        sounds.extend_from_slice(phonemes);
        at += step;
    }
    Some(sounds)
}

/// How `word` is said, stressed on its last syllable but one; `None` for a letter Lingala does not
/// read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let letters: Vec<char> = word.to_lowercase().chars().collect();
    let sounds = sounds(&letters)?;
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

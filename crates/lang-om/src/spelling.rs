//! How an Oromo word is said from its Qubee spelling: one letter or digraph, one sound, with the
//! ejectives c tʃʼ, ph pʼ, q kʼ, x tʼ and ts tsʼ, the implosive dh ɗ, ch tʃ, ny ɲ, sh ʃ, zh ʒ and
//! the apostrophe ʔ; a doubled vowel long, a doubled consonant geminate, and a digraph made
//! geminate by doubling its first letter (qopphaa'uu, the ph said twice).
//!
//! Oromo's pitch accent is not written; jespersen puts the stress on the last syllable but one, the
//! commonest place for it in nouns and verbs alike (barataa baˈrataː).
//!
//! eSpeak NG's Oromo voice is not in manuzio's helper, and its notation keeps to the symbols
//! Kokoro reads (decision 48): an ejective is written as the plain stop or affricate and ɗ as d.

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
const CH: Phoneme = consonant("tʃ", "tʃ");
const D: Phoneme = consonant("d", "d");
const F: Phoneme = consonant("f", "f");
const G: Phoneme = consonant("ɡ", "ɡ");
const H: Phoneme = consonant("h", "h");
const J: Phoneme = consonant("dʒ", "dʒ");
const K: Phoneme = consonant("k", "k");
const L: Phoneme = consonant("l", "l");
const M: Phoneme = consonant("m", "m");
const N: Phoneme = consonant("n", "n");
const NY: Phoneme = consonant("ɲ", "ɲ");
const P: Phoneme = consonant("p", "p");
const R: Phoneme = consonant("r", "r");
const S: Phoneme = consonant("s", "s");
const SH: Phoneme = consonant("ʃ", "ʃ");
const T: Phoneme = consonant("t", "t");
const V: Phoneme = consonant("v", "v");
const W: Phoneme = consonant("w", "w");
const Y: Phoneme = consonant("j", "j");
const Z: Phoneme = consonant("z", "z");
const ZH: Phoneme = consonant("ʒ", "ʒ");
/// The apostrophe (hudhaa) and the glottal stop it writes (ja'a ˈdʒaʔa).
const GLOTTAL: Phoneme = consonant("ʔ", "ʔ");
/// c, ph, q, x and ts: ejectives, written as plain sounds in Kokoro's symbols.
const C: Phoneme = consonant("tʃʼ", "tʃ");
const PH: Phoneme = consonant("pʼ", "p");
const Q: Phoneme = consonant("kʼ", "k");
const X: Phoneme = consonant("tʼ", "t");
const TS: Phoneme = consonant("tsʼ", "ts");
/// dh, an implosive (dhugaa ˈɗuɡaː), d in Kokoro's symbols.
const DH: Phoneme = consonant("ɗ", "d");

/// Whether `cluster` can start a syllable inside a word: one consonant only, since Oromo
/// syllables begin with at most one.
fn legal_onset(cluster: &[Phoneme]) -> bool {
    cluster.len() == 1
}

/// The digraph that starts at `at`, and its sound.
fn digraph(letters: &[char], at: usize) -> Option<Phoneme> {
    Some(match (letters.get(at)?, letters.get(at + 1)?) {
        ('c', 'h') => CH,
        ('d', 'h') => DH,
        ('n', 'y') => NY,
        ('p', 'h') => PH,
        ('s', 'h') => SH,
        ('t', 's') => TS,
        ('z', 'h') => ZH,
        _ => return None,
    })
}

/// A single letter's sound; `None` for a letter Oromo does not read.
fn letter(c: char) -> Option<Phoneme> {
    Some(match c {
        'a' => A,
        'e' => E,
        'i' => I,
        'o' => O,
        'u' => U,
        'b' => B,
        'c' => C,
        'd' => D,
        'f' => F,
        'g' => G,
        'h' => H,
        'j' => J,
        'k' => K,
        'l' => L,
        'm' => M,
        'n' => N,
        'p' => P,
        'q' => Q,
        'r' => R,
        's' => S,
        't' => T,
        'v' => V,
        'w' => W,
        'x' => X,
        'y' => Y,
        'z' => Z,
        '\'' | '’' => GLOTTAL,
        _ => return None,
    })
}

/// A short vowel made long by its doubled letter.
fn lengthened(phoneme: Phoneme) -> Phoneme {
    match phoneme {
        phoneme if phoneme == A => A_LONG,
        phoneme if phoneme == E => E_LONG,
        phoneme if phoneme == I => I_LONG,
        phoneme if phoneme == O => O_LONG,
        _ => U_LONG,
    }
}

/// The sounds of `letters`, in order, a doubled vowel as one long vowel and a geminate as its
/// consonant said twice; `None` for a letter Oromo does not read.
fn sounds(letters: &[char]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let doubled = letters.get(at + 1) == Some(&letters[at]);
        // A digraph's first letter doubled makes it geminate (pph, ddh, cch).
        if doubled && let Some(phoneme) = digraph(letters, at + 1) {
            sounds.extend([phoneme, phoneme]);
            at += 3;
        } else if let Some(phoneme) = digraph(letters, at) {
            sounds.push(phoneme);
            at += 2;
        } else {
            let phoneme = letter(letters[at])?;
            if doubled && phoneme.is_vowel() {
                sounds.push(lengthened(phoneme));
                at += 2;
            } else {
                sounds.push(phoneme);
                at += 1;
            }
        }
    }
    Some(sounds)
}

/// How `word` is said, stressed on its last syllable but one; `None` for a letter Oromo does not
/// read, or a word without a vowel.
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

//! Basque phonemes in plain IPA and eSpeak NG's notation, which also writes the soft b, d and g
//! of running speech (β, ð, ɣ).

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const E: Phoneme = vowel("e", "e");
pub(crate) const I: Phoneme = vowel("i", "i");
pub(crate) const O: Phoneme = vowel("o", "o");
pub(crate) const U: Phoneme = vowel("u", "u");
/// The diphthongs ai, ei, oi, au and eu, one syllable each.
pub(crate) const AI: Phoneme = vowel("ai̯", "aɪ");
pub(crate) const EI: Phoneme = vowel("ei̯", "eɪ");
pub(crate) const OI: Phoneme = vowel("oi̯", "oɪ");
pub(crate) const AU: Phoneme = vowel("au̯", "aʊ");
pub(crate) const EU: Phoneme = vowel("eu̯", "eʊ");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
/// b, d and g between sounds, written β, ð and ɣ in eSpeak NG's notation.
pub(crate) const B_SOFT: Phoneme = consonant("b", "β");
pub(crate) const D_SOFT: Phoneme = consonant("d", "ð");
pub(crate) const G_SOFT: Phoneme = consonant("ɡ", "ɣ");
/// tt and dd, palatal stops (ttipi).
pub(crate) const TT: Phoneme = consonant("c", "c");
pub(crate) const DD: Phoneme = consonant("ɟ", "ɟ");
pub(crate) const F: Phoneme = consonant("f", "f");
/// z, laminal, and s, apical.
pub(crate) const Z: Phoneme = consonant("s̻", "s̻");
pub(crate) const S: Phoneme = consonant("s̺", "s̺");
/// x.
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
/// tz, ts and tx.
pub(crate) const TZ: Phoneme = consonant("ts̻", "ts̻");
pub(crate) const TS: Phoneme = consonant("ts̺", "ts̺");
pub(crate) const TX: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NY: Phoneme = consonant("ɲ", "ɲ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const LL: Phoneme = consonant("ʎ", "ʎ");
pub(crate) const R: Phoneme = consonant("ɾ", "ɾ");
pub(crate) const RR: Phoneme = consonant("r", "r");
pub(crate) const J: Phoneme = consonant("j", "j");
/// j as eSpeak NG writes it in most words, ɟ (jende ɟˈende).
pub(crate) const J_STOP: Phoneme = consonant("j", "ɟ");

/// Whether `cluster` can start a syllable inside a word: one consonant, or a stop or f before l
/// or r (prestatu, fruta).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    match cluster {
        [_] => true,
        [first, second] => [P, B, T, D, K, G, F].contains(first) && [L, R].contains(second),
        _ => false,
    }
}

//! Slovenian phonemes in plain IPA and eSpeak NG's notation, and how they pair by voicing.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

// Stressed vowels are long; e and o closed unless jespersen's list opens them.
pub(crate) const AA: Phoneme = vowel("aː", "aː");
pub(crate) const EE: Phoneme = vowel("eː", "eː");
pub(crate) const EE_OPEN: Phoneme = vowel("ɛː", "ɛː");
pub(crate) const II: Phoneme = vowel("iː", "iː");
pub(crate) const OO: Phoneme = vowel("oː", "oː");
pub(crate) const OO_OPEN: Phoneme = vowel("ɔː", "ɔː");
pub(crate) const UU: Phoneme = vowel("uː", "uː");
/// Short stressed e and o, from jespersen's respellings (še šè, ker kèr).
pub(crate) const E_SHORT_STRESSED: Phoneme = vowel("ɛ", "ɛ").misaki("è");
pub(crate) const O_SHORT_STRESSED: Phoneme = vowel("ɔ", "ɔ").misaki("ò");
// Unstressed vowels.
pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const E: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const I: Phoneme = vowel("i", "i");
pub(crate) const O: Phoneme = vowel("ɔ", "ɔ");
pub(crate) const U: Phoneme = vowel("u", "u");
/// The schwa, written e (dober, preden) or in jespersen's respellings ə (sem səm).
pub(crate) const SCHWA: Phoneme = vowel("ə", "ə");
/// r as the nucleus of a syllable, said with a schwa before it (vrt ʋərt, trg).
pub(crate) const R_SYLLABIC: Phoneme = vowel("ər", "ər");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("ʋ", "ʋ");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const Z: Phoneme = consonant("z", "z");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const ZH: Phoneme = consonant("ʒ", "ʒ");
pub(crate) const X: Phoneme = consonant("x", "x");
/// h before a consonant, which eSpeak NG writes h (lehko).
pub(crate) const H_BEFORE_CONSONANT: Phoneme = consonant("x", "h");
pub(crate) const GH: Phoneme = consonant("ɣ", "ɣ");
pub(crate) const TS: Phoneme = consonant("ts", "ts");
pub(crate) const DZ: Phoneme = consonant("dz", "dz");
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const DZH: Phoneme = consonant("dʒ", "dʒ");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NY: Phoneme = consonant("ɲ", "ɲ");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const LJ: Phoneme = consonant("ʎ", "ʎ");
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const J: Phoneme = consonant("j", "j");
/// l and v ending a syllable after a vowel, said u̯ (bil biu̯, volk, lev), which eSpeak NG writes w.
pub(crate) const W: Phoneme = consonant("u̯", "w");
/// v starting a word before a consonant, and the preposition v, said u (vse use, v redu).
pub(crate) const V_VOWEL: Phoneme = vowel("u", "u").misaki("v");

/// Voiceless obstruents and their voiced partners; v takes no part.
const PAIRS: [(Phoneme, Phoneme); 9] = [
    (P, B),
    (T, D),
    (K, G),
    (S, Z),
    (SH, ZH),
    (TS, DZ),
    (CH, DZH),
    (X, GH),
    (F, consonant("v", "v")),
];

/// Whether `phoneme` is an obstruent, which voicing changes.
pub(crate) fn is_obstruent(phoneme: Phoneme) -> bool {
    PAIRS
        .iter()
        .any(|&(voiceless, voiced)| phoneme == voiceless || phoneme == voiced)
}

/// Whether `phoneme` is a voiced obstruent, which voices the one before it.
pub(crate) fn voices(phoneme: Phoneme) -> bool {
    PAIRS.iter().any(|&(_, voiced)| phoneme == voiced)
}

/// Whether `phoneme` is a voiceless obstruent.
pub(crate) fn is_voiceless(phoneme: Phoneme) -> bool {
    PAIRS.iter().any(|&(voiceless, _)| phoneme == voiceless)
}

/// `phoneme` with the voicing `voiced`, if it is an obstruent that has a partner.
pub(crate) fn with_voicing(phoneme: Phoneme, voiced: bool) -> Phoneme {
    PAIRS
        .iter()
        .find_map(|&(voiceless, partner)| {
            if voiced {
                (phoneme == voiceless).then_some(partner)
            } else {
                (phoneme == partner).then_some(voiceless)
            }
        })
        .unwrap_or(phoneme)
}

/// Whether `cluster` can start a syllable inside a word: one consonant, or obstruents (two only
/// if one is a fricative) with at most one sonorant or j after them (pr, sk, str, sv).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    if cluster.len() == 1 {
        return true;
    }
    let sonorant = |phoneme: Phoneme| [M, N, NY, L, LJ, R, J, V].contains(&phoneme);
    let fricative = |phoneme: Phoneme| [F, V, S, Z, SH, ZH, X].contains(&phoneme);
    let obstruents = cluster
        .iter()
        .take_while(|&&phoneme| is_obstruent(phoneme) && !sonorant(phoneme))
        .count();
    let sonorants_fit = match &cluster[obstruents..] {
        [] => true,
        [one] | [one, J] => sonorant(*one),
        _ => false,
    };
    sonorants_fit
        && match &cluster[..obstruents] {
            [] => cluster.len() == 2 && cluster[1] == J,
            [_] => true,
            [first, second] => fricative(*first) || fricative(*second),
            _ => false,
        }
}

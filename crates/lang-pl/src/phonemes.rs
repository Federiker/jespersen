//! Polish phonemes, in plain IPA and eSpeak NG's notation, and how they pair by voicing.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const E: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const I: Phoneme = vowel("i", "i");
pub(crate) const O: Phoneme = vowel("ɔ", "ɔ");
pub(crate) const U: Phoneme = vowel("u", "u");
pub(crate) const Y: Phoneme = vowel("ɨ", "ɨ");
/// ą and ę before a fricative, and ą at the end of a word.
pub(crate) const O_NASAL: Phoneme = vowel("ɔ̃", "ɔ̃");
pub(crate) const E_NASAL: Phoneme = vowel("ɛ̃", "ɛ̃");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const Z: Phoneme = consonant("z", "z");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const ZH: Phoneme = consonant("ʒ", "ʒ");
pub(crate) const SJ: Phoneme = consonant("ɕ", "ɕ");
pub(crate) const ZJ: Phoneme = consonant("ʑ", "ʑ");
pub(crate) const X: Phoneme = consonant("x", "x");
/// ch voiced before a voiced obstruent (ich dom).
pub(crate) const GH: Phoneme = consonant("ɣ", "ɣ");
pub(crate) const TS: Phoneme = consonant("ts", "ts");
pub(crate) const DZ: Phoneme = consonant("dz", "dz");
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const DZH: Phoneme = consonant("dʒ", "dʒ");
pub(crate) const TJ: Phoneme = consonant("tɕ", "tɕ");
pub(crate) const DJ: Phoneme = consonant("dʑ", "dʑ");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NY: Phoneme = consonant("ɲ", "ɲ");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const J: Phoneme = consonant("j", "j");
/// ł.
pub(crate) const W: Phoneme = consonant("w", "w");

// Consonants before an i that only softens them (pies, biuro, kiedy, nie): plain IPA writes the j,
// eSpeak NG a palatalised consonant, but j after b.
pub(crate) const P_SOFT: Phoneme = consonant("pj", "pʲ");
pub(crate) const B_SOFT: Phoneme = consonant("bj", "bj");
pub(crate) const F_SOFT: Phoneme = consonant("fj", "fʲ");
pub(crate) const V_SOFT: Phoneme = consonant("vj", "vʲ");
pub(crate) const M_SOFT: Phoneme = consonant("mj", "mʲ");
pub(crate) const M_SOFT_INSIDE: Phoneme = consonant("mj", "mj");
pub(crate) const K_SOFT: Phoneme = consonant("kʲ", "kʲ");
pub(crate) const G_SOFT: Phoneme = consonant("ɡʲ", "ɡʲ");
pub(crate) const N_SOFT: Phoneme = consonant("ɲ", "ɲʲ");
// eSpeak NG writes b and g before the vowel i palatalised (lubi lˈubʲi, drogi drˈɔɡʲi), and ch as ç.
pub(crate) const B_BEFORE_I: Phoneme = consonant("b", "bʲ");
pub(crate) const G_BEFORE_I: Phoneme = consonant("ɡ", "ɡʲ");
pub(crate) const X_BEFORE_I: Phoneme = consonant("x", "ç");
// ę before ś and ź: a nasal vowel in plain IPA, a vowel and ɲ in eSpeak NG's notation (część
// tʃˈɛɲɕtɕ).
pub(crate) const E_NASAL_SPLIT: Phoneme = vowel("ɛ̃", "ɛ");
pub(crate) const NY_SPLIT: Phoneme = consonant("", "ɲ");
/// The preposition w where it stays voiced, which eSpeak NG writes w (w domu w dˈɔmu).
pub(crate) const V_ALONE: Phoneme = consonant("v", "w");

/// Voiceless obstruents and their voiced partners.
const PAIRS: [(Phoneme, Phoneme); 17] = [
    (P, B),
    (T, D),
    (K, G),
    (F, V),
    (S, Z),
    (SH, ZH),
    (SJ, ZJ),
    (TS, DZ),
    (CH, DZH),
    (TJ, DJ),
    (P_SOFT, B_SOFT),
    (F_SOFT, V_SOFT),
    (K_SOFT, G_SOFT),
    (P, B_BEFORE_I),
    (F, V_ALONE),
    (K, G_BEFORE_I),
    (X, GH),
];

/// Whether `phoneme` is an obstruent (a stop, fricative or affricate), which voicing changes.
pub(crate) fn is_obstruent(phoneme: Phoneme) -> bool {
    phoneme == X_BEFORE_I
        || PAIRS
            .iter()
            .any(|&(voiceless, voiced)| phoneme == voiceless || phoneme == voiced)
}

/// Whether `phoneme` is a voiced obstruent.
pub(crate) fn is_voiced(phoneme: Phoneme) -> bool {
    PAIRS.iter().any(|&(_, voiced)| phoneme == voiced)
}

/// Whether `phoneme` is a v, which takes the voicing of what is before it but gives none (twój tfˈuj, w
/// sobotę).
pub(crate) fn is_v(phoneme: Phoneme) -> bool {
    [V, V_SOFT, V_ALONE].contains(&phoneme)
}

/// `phoneme` with the voicing `voiced`, if it is an obstruent that has a partner (x has none).
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

/// Whether `phoneme` is a sonorant consonant (m n ɲ ŋ l r j w).
pub(crate) fn is_sonorant(phoneme: Phoneme) -> bool {
    [M, M_SOFT, M_SOFT_INSIDE, N, N_SOFT, NY, NG, L, R, J, W].contains(&phoneme)
}

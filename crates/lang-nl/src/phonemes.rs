//! Dutch phonemes in plain IPA and eSpeak NG's notation.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("ɑ", "ɑ");
pub(crate) const A_LONG: Phoneme = vowel("aː", "aː");
pub(crate) const E: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const E_LONG: Phoneme = vowel("eː", "eː");
/// Long e before r, ɪː in eSpeak NG's notation (meer mˈɪːr).
pub(crate) const E_BEFORE_R: Phoneme = vowel("eː", "ɪː");
pub(crate) const SCHWA: Phoneme = vowel("ə", "ə");
pub(crate) const I: Phoneme = vowel("ɪ", "ɪ");
/// ie, and i in an open syllable.
pub(crate) const IE: Phoneme = vowel("i", "i");
pub(crate) const O: Phoneme = vowel("ɔ", "ɔ");
pub(crate) const O_LONG: Phoneme = vowel("oː", "oː");
/// Short u, ɵ in eSpeak NG's notation (bus bˈɵs), and uu or u in an open syllable (muur,
/// studeren).
pub(crate) const U: Phoneme = vowel("ʏ", "ɵ");
pub(crate) const UU: Phoneme = vowel("y", "y");
/// oe, eu, ui, ei and ij, ou and au.
pub(crate) const OE: Phoneme = vowel("u", "u");
pub(crate) const EU: Phoneme = vowel("øː", "øː");
pub(crate) const UI: Phoneme = vowel("œy", "œy");
pub(crate) const EI: Phoneme = vowel("ɛi", "ɛɪ");
pub(crate) const AU: Phoneme = vowel("ʌu", "ʌʊ");

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
/// g and ch.
pub(crate) const GH: Phoneme = consonant("ɣ", "ɣ");
pub(crate) const X: Phoneme = consonant("x", "x");
pub(crate) const H: Phoneme = consonant("h", "h");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const J: Phoneme = consonant("j", "j");
/// w.
pub(crate) const W: Phoneme = consonant("ʋ", "ʋ");
/// The w of eeuw, ieuw and uw (nieuw nˈiw).
pub(crate) const W_GLIDE: Phoneme = consonant("u̯", "w");

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop or f, v or ɣ
/// before l or r, s before a stop, m, n, l or x (straat, school), str, spr and sxr, and a
/// consonant before ʋ or j (twee, natie).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let obstruent = |phoneme: &Phoneme| [P, B, T, D, K, G, F, V, GH, X].contains(phoneme);
    let liquid = |phoneme: &Phoneme| [L, R].contains(phoneme);
    match cluster {
        [_] => true,
        [_, second] if [W, J].contains(second) => true,
        [first, second] if liquid(second) => obstruent(first),
        [first, second] => *first == S && [P, T, K, M, N, L, X].contains(second),
        [first, second, third] => *first == S && [P, T, X].contains(second) && *third == R,
        _ => false,
    }
}

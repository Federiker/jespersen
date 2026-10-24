//! German phonemes in plain IPA and eSpeak NG's notation.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
/// Long a, which eSpeak NG writes ɑː (Vater fˈɑːtɜ).
pub(crate) const A_LONG: Phoneme = vowel("aː", "ɑː");
pub(crate) const E: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const E_LONG: Phoneme = vowel("eː", "eː");
/// Long ä.
pub(crate) const AE_LONG: Phoneme = vowel("ɛː", "ɛː");
pub(crate) const SCHWA: Phoneme = vowel("ə", "ə");
/// The unstressed -er (Vater), ɜ in eSpeak NG's notation.
pub(crate) const ER: Phoneme = vowel("ɐ", "ɜ");
pub(crate) const I: Phoneme = vowel("ɪ", "ɪ");
pub(crate) const I_LONG: Phoneme = vowel("iː", "iː");
pub(crate) const O: Phoneme = vowel("ɔ", "ɔ");
pub(crate) const O_LONG: Phoneme = vowel("oː", "oː");
pub(crate) const U: Phoneme = vowel("ʊ", "ʊ");
pub(crate) const U_LONG: Phoneme = vowel("uː", "uː");
pub(crate) const OE: Phoneme = vowel("œ", "œ");
pub(crate) const OE_LONG: Phoneme = vowel("øː", "øː");
pub(crate) const UE: Phoneme = vowel("ʏ", "ʏ");
pub(crate) const UE_LONG: Phoneme = vowel("yː", "yː");
/// ei and ai, au, eu and äu (ɔø in eSpeak NG's notation).
pub(crate) const AI: Phoneme = vowel("aɪ̯", "aɪ");
pub(crate) const AU: Phoneme = vowel("aʊ̯", "aʊ");
pub(crate) const EU: Phoneme = vowel("ɔʏ̯", "ɔø");

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
/// ch after a front vowel or a consonant (ich), and after a back vowel (ach).
pub(crate) const ICH: Phoneme = consonant("ç", "ç");
pub(crate) const ACH: Phoneme = consonant("x", "x");
pub(crate) const H: Phoneme = consonant("h", "h");
pub(crate) const TS: Phoneme = consonant("ts", "ts");
pub(crate) const PF: Phoneme = consonant("pf", "pf");
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const DZH: Phoneme = consonant("dʒ", "dʒ");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const J: Phoneme = consonant("j", "j");
/// r before a vowel: r in eSpeak NG's notation at the start, between vowels and after a fricative
/// (Rat, Ehre, Frau), ɾ after a stop (drei).
pub(crate) const R: Phoneme = consonant("ʁ", "r");
pub(crate) const R_TAP: Phoneme = consonant("ʁ", "ɾ");
/// r after a vowel (vier, Garten), ɐ in eSpeak NG's notation after a short u (durch).
pub(crate) const R_VOCALIC: Phoneme = consonant("ɐ̯", "ɾ");
pub(crate) const R_AFTER_U: Phoneme = consonant("ɐ̯", "ɐ");

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop or f before l or
/// r, ʃ before a stop, l or r (Schrank), ʃt and ʃp before r, ts before v (zwei), k before v
/// (Quelle), and a consonant before j (Nation).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let stop = |phoneme: &Phoneme| [P, B, T, D, K, G, F, PF].contains(phoneme);
    let liquid = |phoneme: &Phoneme| [L, R, R_TAP].contains(phoneme);
    match cluster {
        [_] => true,
        [_, second] if *second == J => true,
        [first, second] if liquid(second) => stop(first) || *first == SH,
        [first, second] => {
            (*first == SH && [P, T, M, N, V].contains(second)) || ([TS, K].contains(first) && *second == V)
        }
        [first, second, third] => *first == SH && [P, T].contains(second) && liquid(third),
        _ => false,
    }
}

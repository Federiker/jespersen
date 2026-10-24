//! Latvian phonemes in plain IPA and eSpeak NG's notation.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const AA: Phoneme = vowel("aː", "aː");
/// Closed e and ē, and open ones (zēns zˈæːns).
pub(crate) const E: Phoneme = vowel("e", "e");
pub(crate) const EE: Phoneme = vowel("eː", "eː");
pub(crate) const AE: Phoneme = vowel("æ", "æ");
pub(crate) const AEAE: Phoneme = vowel("æː", "æː");
pub(crate) const I: Phoneme = vowel("i", "i");
pub(crate) const II: Phoneme = vowel("iː", "iː");
pub(crate) const U: Phoneme = vowel("u", "u");
pub(crate) const UU: Phoneme = vowel("uː", "uː");
/// The o of loanwords, short and long (hokejs, telefons).
pub(crate) const O: Phoneme = vowel("ɔ", "o");
pub(crate) const OO: Phoneme = vowel("ɔː", "oː");
/// The diphthongs: o of native words (skola skˈuola), ie, ai, ei, au, ui.
pub(crate) const UO: Phoneme = vowel("uɔ", "uo");
pub(crate) const IE: Phoneme = vowel("iɛ", "ie");
pub(crate) const AI: Phoneme = vowel("ai̯", "ai");
pub(crate) const EI: Phoneme = vowel("ei̯", "ei");
pub(crate) const AU: Phoneme = vowel("au̯", "au");
pub(crate) const UI: Phoneme = vowel("ui̯", "ui");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
/// ķ and ģ.
pub(crate) const KJ: Phoneme = consonant("c", "c");
pub(crate) const GJ: Phoneme = consonant("ɟ", "ɟ");
pub(crate) const F: Phoneme = consonant("f", "f");
/// v, ʋ in eSpeak NG's notation.
pub(crate) const V: Phoneme = consonant("v", "ʋ");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const Z: Phoneme = consonant("z", "z");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const ZH: Phoneme = consonant("ʒ", "ʒ");
/// c and č, which eSpeak NG writes with a tie (t͡s, t͡ʃ); dz and dž, dʐ in its notation.
pub(crate) const TS: Phoneme = consonant("ts", "t͡s");
pub(crate) const CH: Phoneme = consonant("tʃ", "t͡ʃ");
pub(crate) const DZ: Phoneme = consonant("dz", "dz");
pub(crate) const DZH: Phoneme = consonant("dʒ", "dʐ");
pub(crate) const H: Phoneme = consonant("x", "h");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
/// ņ, and n before k and g.
pub(crate) const NJ: Phoneme = consonant("ɲ", "ɲ");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
/// l before a voiced consonant, which eSpeak NG writes ll (kalna kˈallna).
pub(crate) const L_LONG: Phoneme = consonant("l", "ll");
/// ļ.
pub(crate) const LJ: Phoneme = consonant("ʎ", "ʎ");
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const J: Phoneme = consonant("j", "j");

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop or f before l, r
/// or j, s or z before a stop, a nasal, l, r or v (skola, strādāt), and str, spr, skr.
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let stop = |phoneme: &Phoneme| [P, B, T, D, K, G, F].contains(phoneme);
    let liquid = |phoneme: &Phoneme| [L, R, J].contains(phoneme);
    match cluster {
        [_] => true,
        [first, second] if liquid(second) => stop(first),
        [first, second] => [S, Z, SH].contains(first) && (stop(second) || [M, N, V, L, R].contains(second)),
        [first, second, third] => [S, Z].contains(first) && stop(second) && [L, R].contains(third),
        _ => false,
    }
}

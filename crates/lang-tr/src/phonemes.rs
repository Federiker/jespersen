//! Turkish phonemes. Plain IPA is broad; eSpeak NG's notation writes many allophones, which are
//! phonemes here with the plain IPA symbol: lax vowels in closed syllables (ɪ ʊ ɔ ɛ, ü as ø, e as
//! æ before l, m and n), ɟ for g by front vowels, ɫ for l by back ones, ɾ for r between vowels.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const E: Phoneme = vowel("e", "e");
pub(crate) const E_LAX: Phoneme = vowel("e", "ɛ");
pub(crate) const E_WIDE: Phoneme = vowel("e", "æ");
pub(crate) const BACK_I: Phoneme = vowel("ɯ", "ɯ");
pub(crate) const I: Phoneme = vowel("i", "i");
pub(crate) const I_LAX: Phoneme = vowel("i", "ɪ");
pub(crate) const O: Phoneme = vowel("o", "o");
pub(crate) const O_LAX: Phoneme = vowel("o", "ɔ");
pub(crate) const OE: Phoneme = vowel("ø", "ø");
pub(crate) const OE_OPEN: Phoneme = vowel("ø", "œ");
pub(crate) const U: Phoneme = vowel("u", "u");
pub(crate) const U_LAX: Phoneme = vowel("u", "ʊ");
pub(crate) const UE: Phoneme = vowel("y", "y");
pub(crate) const UE_LAX: Phoneme = vowel("y", "ø");

/// Vowels made long by a following ğ (dağ dˈaː).
pub(crate) const A_LONG: Phoneme = vowel("aː", "aː");
pub(crate) const E_LONG: Phoneme = vowel("eː", "eː");
pub(crate) const BACK_I_LONG: Phoneme = vowel("ɯː", "ɯː");
pub(crate) const I_LONG: Phoneme = vowel("iː", "iː");
pub(crate) const O_LONG: Phoneme = vowel("oː", "oː");
pub(crate) const OE_LONG: Phoneme = vowel("øː", "œː");
pub(crate) const U_LONG: Phoneme = vowel("uː", "uː");
pub(crate) const UE_LONG: Phoneme = vowel("yː", "yː");

/// The long form of a vowel.
pub(crate) fn long(vowel: Phoneme) -> Phoneme {
    match vowel {
        A => A_LONG,
        E => E_LONG,
        BACK_I => BACK_I_LONG,
        I => I_LONG,
        O => O_LONG,
        OE => OE_LONG,
        U => U_LONG,
        UE => UE_LONG,
        other => other,
    }
}

/// Whether a vowel is front (e, i, ö, ü), which makes g ɟ and l clear.
pub(crate) fn is_front(vowel: Phoneme) -> bool {
    [E, E_LONG, I, I_LONG, OE, OE_LONG, UE, UE_LONG].contains(&vowel)
}

pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const C: Phoneme = consonant("dʒ", "dʒ");
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const G_FRONT: Phoneme = consonant("ɡ", "ɟ");
pub(crate) const H: Phoneme = consonant("h", "h");
pub(crate) const ZH: Phoneme = consonant("ʒ", "ʒ");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const L_DARK: Phoneme = consonant("l", "ɫ");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const R_TAP: Phoneme = consonant("r", "ɾ");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const V: Phoneme = consonant("v", "v");
pub(crate) const J: Phoneme = consonant("j", "j");
pub(crate) const Z: Phoneme = consonant("z", "z");

pub(crate) const P_FIRST: Phoneme = consonant("p", "");
pub(crate) const T_FIRST: Phoneme = consonant("t", "");
pub(crate) const K_FIRST: Phoneme = consonant("k", "");
pub(crate) const P_LONG: Phoneme = consonant("p", "pː");
pub(crate) const T_LONG: Phoneme = consonant("t", "tː");
pub(crate) const K_LONG: Phoneme = consonant("k", "kː");

/// Whether `cluster` can start a syllable inside a word: one consonant, not the first half of a
/// long stop.
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    matches!(cluster, [single] if ![P_FIRST, T_FIRST, K_FIRST].contains(single))
}

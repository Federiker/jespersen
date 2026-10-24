//! French phonemes in plain IPA and eSpeak NG's notation, and the silent markers the reader uses
//! across words: the consonant a liaison would say, an aspirated h, and eSpeak NG's dash after
//! some function words.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
/// â, which eSpeak NG writes long (âne ˈaːn).
pub(crate) const A_BACK: Phoneme = vowel("ɑ", "aː");
pub(crate) const E: Phoneme = vowel("e", "e");
pub(crate) const E_OPEN: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const SCHWA: Phoneme = vowel("ə", "ə");
pub(crate) const I: Phoneme = vowel("i", "i");
pub(crate) const O: Phoneme = vowel("o", "o");
pub(crate) const O_OPEN: Phoneme = vowel("ɔ", "ɔ");
pub(crate) const U: Phoneme = vowel("u", "u");
pub(crate) const Y: Phoneme = vowel("y", "y");
pub(crate) const EU: Phoneme = vowel("ø", "ø");
pub(crate) const EU_OPEN: Phoneme = vowel("œ", "œ");
pub(crate) const A_NASAL: Phoneme = vowel("ɑ̃", "ɑ̃");
pub(crate) const E_NASAL: Phoneme = vowel("ɛ̃", "ɛ̃");
pub(crate) const O_NASAL: Phoneme = vowel("ɔ̃", "ɔ̃");
pub(crate) const U_NASAL: Phoneme = vowel("œ̃", "œ̃");

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
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NY: Phoneme = consonant("ɲ", "ɲ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const R: Phoneme = consonant("ʁ", "ʁ");
pub(crate) const J: Phoneme = consonant("j", "j");
pub(crate) const W: Phoneme = consonant("w", "w");
/// The glide of nuit and huit, which eSpeak NG writes y (nyˈi).
pub(crate) const UI: Phoneme = consonant("ɥ", "y");

/// A consonant a word ends with only before a vowel (les amis, petit ami, un homme): silent until
/// the reader says it, and then the consonant itself.
pub(crate) const LIAISON_Z: Phoneme = consonant("", "").misaki("‿z");
pub(crate) const LIAISON_T: Phoneme = consonant("", "").misaki("‿t");
pub(crate) const LIAISON_N: Phoneme = consonant("", "").misaki("‿n");
pub(crate) const LIAISON_P: Phoneme = consonant("", "").misaki("‿p");
pub(crate) const LIAISON_R: Phoneme = consonant("", "").misaki("‿r");
/// The end of six and dix: z before a vowel, nothing before a consonant, s at the end of the run.
pub(crate) const LIAISON_ZS: Phoneme = consonant("", "").misaki("‿zs");
/// An aspirated h, silent, before which there is neither liaison nor elision (les héros).
pub(crate) const H_ASPIRATED: Phoneme = consonant("", "").misaki("h");
/// The dash eSpeak NG writes after le, la, les, de and the like (lə- ʃˈa, le-z amˈi).
pub(crate) const DASH: Phoneme = consonant("", "-");

/// The consonant a liaison marker says.
pub(crate) fn liaison(marker: Phoneme) -> Option<Phoneme> {
    match marker {
        marker if marker == LIAISON_Z || marker == LIAISON_ZS => Some(Z),
        marker if marker == LIAISON_T => Some(T),
        marker if marker == LIAISON_N => Some(N),
        marker if marker == LIAISON_P => Some(P),
        marker if marker == LIAISON_R => Some(R),
        _ => None,
    }
}

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop or f or v before l
/// or r (près, table), or a consonant before a glide (bien, nuit, moi).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let obstruent = |phoneme: Phoneme| [P, B, T, D, K, G, F, V].contains(&phoneme);
    let glide = |phoneme: Phoneme| [J, W, UI].contains(&phoneme);
    match cluster {
        [_] => true,
        [first, second] if glide(*second) => !glide(*first),
        [first, second] => obstruent(*first) && [L, R].contains(second),
        [first, second, third] => obstruent(*first) && [L, R].contains(second) && glide(*third),
        _ => false,
    }
}

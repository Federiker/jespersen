//! Romanian phonemes in plain IPA and eSpeak NG's notation.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
/// ă.
pub(crate) const SCHWA: Phoneme = vowel("ə", "ə");
/// â and î.
pub(crate) const Y: Phoneme = vowel("ɨ", "ɨ");
/// â before i, which eSpeak NG writes y (câine kˈyɪne).
pub(crate) const Y_BEFORE_I: Phoneme = vowel("ɨ", "y");
pub(crate) const E: Phoneme = vowel("e", "e");
pub(crate) const I: Phoneme = vowel("i", "i");
pub(crate) const O: Phoneme = vowel("o", "o");
/// The diphthongs ea and oa, one syllable each (seară sˈeaɾə, toată tˈɔatə).
pub(crate) const EA: Phoneme = vowel("e̯a", "ea");
pub(crate) const OA: Phoneme = vowel("o̯a", "ɔa");
pub(crate) const U: Phoneme = vowel("u", "u");

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
pub(crate) const H: Phoneme = consonant("h", "h");
pub(crate) const TS: Phoneme = consonant("ts", "ts");
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const DZH: Phoneme = consonant("dʒ", "dʒ");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
/// r, a tap between vowels.
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const R_TAP: Phoneme = consonant("ɾ", "ɾ");
pub(crate) const J: Phoneme = consonant("j", "j");
pub(crate) const W: Phoneme = consonant("w", "w");
/// The second half of ai, ei, oi, ui and of au, eu, ou.
pub(crate) const OFFGLIDE_I: Phoneme = consonant("j", "ɪ");
pub(crate) const OFFGLIDE_U: Phoneme = consonant("w", "ʊ");
/// The u of ouă, a glide eSpeak NG writes u (două dˈouə).
pub(crate) const U_IN_OUA: Phoneme = consonant("w", "u");
/// A final i after a consonant, which only softens it (lupi lupʲ); eSpeak NG writes ʲʲ, ʲ after
/// -ur and k and g (dealuri, ochi).
pub(crate) const SOFT: Phoneme = consonant("ʲ", "ʲʲ");
pub(crate) const SOFT_SHORT: Phoneme = consonant("ʲ", "ʲ");

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop or f or v before l
/// or r (pregătește, plouat), s before consonants (strada, școală), or a consonant before a glide.
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let obstruent = |phoneme: &Phoneme| [P, B, T, D, K, G, F, V].contains(phoneme);
    let glide = |phoneme: &Phoneme| [J, W].contains(phoneme);
    match cluster {
        [_] => true,
        [first, second] if glide(second) => !glide(first),
        [first, second] => {
            (obstruent(first) && [L, R, R_TAP].contains(second)) || ([S, SH, Z].contains(first) && !glide(second))
        }
        [first, second, third] => [S, SH, Z].contains(first) && obstruent(second) && [L, R, R_TAP].contains(third),
        _ => false,
    }
}

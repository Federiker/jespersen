//! Welsh phonemes in plain IPA and eSpeak NG's notation, which reads the northern ɨ.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const A_LONG: Phoneme = vowel("ɑː", "ɑː");
pub(crate) const E: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const E_LONG: Phoneme = vowel("eː", "eː");
/// Short i, ɨ in eSpeak NG's notation (brenin brˈɛnɨn).
pub(crate) const I: Phoneme = vowel("ɪ", "ɨ");
pub(crate) const I_LONG: Phoneme = vowel("iː", "iː");
pub(crate) const O: Phoneme = vowel("ɔ", "ɔ");
pub(crate) const O_LONG: Phoneme = vowel("oː", "oː");
/// u, and y in a last syllable: long ɨː (un ˈɨːn, dydd dˈɨːð), short ɨ, which eSpeak NG writes ø
/// (pump pˈømp, Cymru kˈʌmrø).
pub(crate) const U: Phoneme = vowel("ɨ", "ø");
pub(crate) const U_LONG: Phoneme = vowel("ɨː", "ɨː");
/// y before the last syllable, ʌ in eSpeak NG's notation (ysgol ˈʌsɡɔl), and in y, yr, yn.
pub(crate) const Y: Phoneme = vowel("ə", "ʌ");
pub(crate) const SCHWA: Phoneme = vowel("ə", "ə");
/// w as a vowel (dŵr dˈuːr, hwn hˈʊn).
pub(crate) const W_VOWEL: Phoneme = vowel("ʊ", "ʊ");
pub(crate) const W_LONG: Phoneme = vowel("uː", "uː");
/// The diphthongs: ai, ae and au, ei and eu, oe, wy, aw, ew, iw and yw.
pub(crate) const AI: Phoneme = vowel("aɪ", "aɪ");
pub(crate) const AE: Phoneme = vowel("ɑɨ", "ɑɨ");
pub(crate) const AU: Phoneme = vowel("aɨ", "aɨ");
pub(crate) const EI: Phoneme = vowel("əɪ", "əɪ");
pub(crate) const EU: Phoneme = vowel("əɨ", "əɨ");
pub(crate) const OE: Phoneme = vowel("ɔɨ", "ɔɨ");
pub(crate) const WY: Phoneme = vowel("uɨ", "uɨ");
pub(crate) const AW: Phoneme = vowel("aʊ", "aʊ");
pub(crate) const EW: Phoneme = vowel("ɛʊ", "eʊ");
pub(crate) const IW: Phoneme = vowel("ɪʊ", "ɪu");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
/// th and dd.
pub(crate) const TH: Phoneme = consonant("θ", "θ");
pub(crate) const DH: Phoneme = consonant("ð", "ð");
pub(crate) const S: Phoneme = consonant("s", "s");
/// si before a vowel (siarad ʃˈarad).
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
/// ch.
pub(crate) const X: Phoneme = consonant("χ", "x");
/// ch right after e or i, ç in eSpeak NG's notation (dechrau dˈeːçraɨ).
pub(crate) const X_FRONT: Phoneme = consonant("χ", "ç");
pub(crate) const H: Phoneme = consonant("h", "h");
/// ll.
pub(crate) const LL: Phoneme = consonant("ɬ", "ɬ");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const R: Phoneme = consonant("r", "r");
/// rh, hr in eSpeak NG's notation (rhaid hrˈaɪd).
pub(crate) const RH: Phoneme = consonant("r̥", "hr");
pub(crate) const J: Phoneme = consonant("j", "j");
pub(crate) const W: Phoneme = consonant("w", "w");
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const DZH: Phoneme = consonant("dʒ", "dʒ");

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop or f before l, r
/// or w, s before a stop, and a consonant before j or w.
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let obstruent = |phoneme: &Phoneme| [P, B, T, D, K, G, F, V, TH, DH, X].contains(phoneme);
    match cluster {
        [_] => true,
        [_, second] if [J, W].contains(second) => true,
        [first, second] if [L, R].contains(second) => obstruent(first),
        [first, second] => *first == S && [P, T, K].contains(second),
        [first, second, third] => *first == S && [P, T, K].contains(second) && [L, R].contains(third),
        _ => false,
    }
}

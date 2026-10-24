//! Albanian phonemes in plain IPA and eSpeak NG's notation.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

/// a, and a at the end of a word, ɑ in eSpeak NG's notation (nata nˈatɑ).
pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const A_FINAL: Phoneme = vowel("a", "ɑ");
/// e, ɛ in eSpeak NG's notation, e after j, q and l (dhjetë ðjˈetə, fle fɫˈe).
pub(crate) const E: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const E_CLOSE: Phoneme = vowel("ɛ", "e");
/// ë: ə, and ʌ in eSpeak NG's notation under a stress (një ɲˈʌ).
pub(crate) const SCHWA: Phoneme = vowel("ə", "ə");
pub(crate) const SCHWA_STRESSED: Phoneme = vowel("ə", "ʌ");
pub(crate) const I: Phoneme = vowel("i", "i");
/// An unstressed i in a closed last syllable, ɪ in eSpeak NG's notation (faleminderit).
pub(crate) const I_LAX: Phoneme = vowel("i", "ɪ");
pub(crate) const O: Phoneme = vowel("ɔ", "ɔ");
pub(crate) const U: Phoneme = vowel("u", "u");
pub(crate) const Y: Phoneme = vowel("y", "y");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
/// q and gj, tɕ and dʑ in eSpeak NG's notation.
pub(crate) const Q: Phoneme = consonant("c", "tɕ");
pub(crate) const GJ: Phoneme = consonant("ɟ", "dʑ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
/// th and dh.
pub(crate) const TH: Phoneme = consonant("θ", "θ");
pub(crate) const DH: Phoneme = consonant("ð", "ð");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const Z: Phoneme = consonant("z", "z");
/// sh and zh.
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const ZH: Phoneme = consonant("ʒ", "ʒ");
/// c, x, ç and xh.
pub(crate) const TS: Phoneme = consonant("ts", "ts");
pub(crate) const DZ: Phoneme = consonant("dz", "dz");
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const DZH: Phoneme = consonant("dʒ", "dʒ");
pub(crate) const H: Phoneme = consonant("h", "h");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
/// nj.
pub(crate) const NJ: Phoneme = consonant("ɲ", "ɲ");
/// l, ɫ in eSpeak NG's notation but at the end of a word; ll, which eSpeak NG writes ll.
pub(crate) const L: Phoneme = consonant("l", "ɫ");
pub(crate) const L_FINAL: Phoneme = consonant("l", "l");
pub(crate) const LL: Phoneme = consonant("ɫ", "ll");
/// r, a tap, and rr, trilled; eSpeak NG writes both r.
pub(crate) const R: Phoneme = consonant("ɾ", "r");
pub(crate) const RR: Phoneme = consonant("r", "r");
pub(crate) const J: Phoneme = consonant("j", "j");
/// j after a, e or o, and after any vowel before a consonant or the end (majë mˈaɪə, muaj).
pub(crate) const J_GLIDE: Phoneme = consonant("j", "ɪ");
/// u before a vowel (grua ɡrwˈɑ).
pub(crate) const W: Phoneme = consonant("w", "w");

/// Whether `cluster` can start a syllable inside a word: one consonant, a consonant before r, l,
/// j or w, and s, sh, z, zh or f before one or two consonants (shkollë, fshat). A word starts
/// with more (mbret, ndihmë), but inside one mb, nd and ng are split.
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let fricative = |phoneme: &Phoneme| [S, SH, Z, ZH, F].contains(phoneme);
    match cluster {
        [_] => true,
        [_, second] if [R, RR, L, LL, J, W].contains(second) => true,
        [first, _] if fricative(first) => true,
        [first, _, _] => fricative(first),
        _ => false,
    }
}

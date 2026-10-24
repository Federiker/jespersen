//! Norwegian Bokmål phonemes in plain IPA and eSpeak NG's notation.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

/// Short a, ɑ, and long a, which eSpeak NG writes a (dag dˈaɡ).
pub(crate) const A: Phoneme = vowel("ɑ", "ɑ");
pub(crate) const A_LONG: Phoneme = vowel("ɑː", "a");
/// A long a ending a word of one syllable, ɑ in eSpeak NG's notation (da dˈɑ).
pub(crate) const A_FINAL: Phoneme = vowel("ɑː", "ɑ");
/// An unstressed final a, written a in eSpeak NG's notation (barna bˈɑrna).
pub(crate) const A_UNSTRESSED: Phoneme = vowel("ɑ", "a");
pub(crate) const E: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const E_LONG: Phoneme = vowel("eː", "eː");
/// A short e before r, æ, written a in eSpeak NG's notation (hjerte jˈarta), and æ (kjære
/// xˈaːra).
pub(crate) const AE: Phoneme = vowel("æ", "a");
pub(crate) const AE_LONG: Phoneme = vowel("æː", "aː");
/// A long e before r in a word of one syllable (der dˈɛːr).
pub(crate) const E_BEFORE_R: Phoneme = vowel("æː", "ɛː");
/// An unstressed e, written a at the end of a word in eSpeak NG's notation (skole skˈuːla).
pub(crate) const SCHWA: Phoneme = vowel("ə", "ə");
pub(crate) const SCHWA_FINAL: Phoneme = vowel("ə", "a");
pub(crate) const I: Phoneme = vowel("ɪ", "ɪ");
pub(crate) const I_LONG: Phoneme = vowel("iː", "iː");
/// o: long uː, short ɔ (bok, sommer); å: long oː, short ɔ.
pub(crate) const O: Phoneme = vowel("ɔ", "ɔ");
pub(crate) const OO: Phoneme = vowel("uː", "uː");
pub(crate) const AA: Phoneme = vowel("oː", "oː");
/// u: long ʉː, short ʉ (gutt ɡˈʉtː).
pub(crate) const U: Phoneme = vowel("ʉ", "ʉ");
pub(crate) const U_LONG: Phoneme = vowel("ʉː", "ʉː");
/// y: long yː, short ʏ (y in eSpeak NG's notation).
pub(crate) const Y: Phoneme = vowel("ʏ", "y");
pub(crate) const Y_LONG: Phoneme = vowel("yː", "yː");
/// ø: long øː, short œ.
pub(crate) const OE: Phoneme = vowel("œ", "œ");
pub(crate) const OE_LONG: Phoneme = vowel("øː", "øː");
/// ei, øy and au.
pub(crate) const EI: Phoneme = vowel("æɪ", "aɪ");
pub(crate) const OEY: Phoneme = vowel("øʏ", "øy");
pub(crate) const AU: Phoneme = vowel("æʉ", "aʊ");
/// eg ending a word of one syllable, ɛːɪ in eSpeak NG's notation (jeg jˈɛːɪ).
pub(crate) const EG: Phoneme = vowel("æɪ", "ɛːɪ");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
pub(crate) const S: Phoneme = consonant("s", "s");
/// sj, skj and sk before a front vowel, and rs (sju ʃˈʉː, norsk nˈɔrʃk).
pub(crate) const SJ: Phoneme = consonant("ʃ", "ʃ");
/// kj and k before a front vowel, x in eSpeak NG's notation (kjøkken xˈœkːən); tj, ʂ in it (tjue
/// ʂˈʉːa).
pub(crate) const KJ: Phoneme = consonant("ç", "x");
pub(crate) const TJ: Phoneme = consonant("ç", "ʂ");
pub(crate) const H: Phoneme = consonant("h", "h");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const J: Phoneme = consonant("j", "j");
/// The length of a doubled stop (gutt ɡˈʉtː), which eSpeak NG writes ː.
pub(crate) const LONG: Phoneme = consonant("ː", "ː");

/// The phoneme said again for a doubled l, m, n or r between vowels: written ː in plain IPA and
/// as the consonant itself in eSpeak NG's notation (kvinne kvˈɪnna).
pub(crate) fn doubled(phoneme: Phoneme) -> Phoneme {
    match phoneme {
        phoneme if phoneme == M => consonant("ː", "m"),
        phoneme if phoneme == N => consonant("ː", "n"),
        phoneme if phoneme == L => consonant("ː", "l"),
        phoneme if phoneme == R => consonant("ː", "r"),
        _ => LONG,
    }
}

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop or f before l, r
/// or j, s before a stop, m, n, l or v, str, spr and skr, and k before v (kvinne).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let obstruent = |phoneme: &Phoneme| [P, B, T, D, K, G, F, V].contains(phoneme);
    match cluster {
        [_] => true,
        [first, second] if [L, R, J].contains(second) => obstruent(first),
        [first, second] if *second == V => [K, S, T, D].contains(first),
        [first, second] => *first == S && [P, T, K, M, N, L].contains(second),
        [first, second, third] => *first == S && [P, T, K].contains(second) && *third == R,
        _ => false,
    }
}

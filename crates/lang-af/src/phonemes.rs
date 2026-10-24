//! Afrikaans phonemes in plain IPA and eSpeak NG's notation.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

/// Short a, ɐ in eSpeak NG's notation (dag dˈɐx), and long a (vader).
pub(crate) const A: Phoneme = vowel("a", "ɐ");
pub(crate) const A_LONG: Phoneme = vowel("ɑː", "ɑː");
/// Short e, æ before k, l and r (ek æk), and ê (sê sˈeː).
pub(crate) const E: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const E_OPEN: Phoneme = vowel("æ", "æ");
pub(crate) const E_LONG: Phoneme = vowel("ɛː", "eː");
/// ê before r, æː in eSpeak NG's notation (wêreld vˈæːrəlt).
pub(crate) const E_LONG_BEFORE_R: Phoneme = vowel("ɛː", "æː");
/// ee and e in an open syllable (meer mˈiər).
pub(crate) const EE: Phoneme = vowel("iə", "iə");
/// An unstressed e, and a short i (begin bəxˈən).
pub(crate) const SCHWA: Phoneme = vowel("ə", "ə");
/// ie, and i in an open syllable.
pub(crate) const IE: Phoneme = vowel("i", "i");
/// ie before r (vier fˈiːr).
pub(crate) const IE_LONG: Phoneme = vowel("iː", "iː");
pub(crate) const O: Phoneme = vowel("ɔ", "ɔ");
/// ô, and oo and o in an open syllable (skool skˈʊəl).
pub(crate) const O_LONG: Phoneme = vowel("ɔː", "ɔː");
pub(crate) const OO: Phoneme = vowel("uə", "ʊə");
/// Short u, û, and uu and u in an open syllable.
pub(crate) const U: Phoneme = vowel("œ", "œ");
pub(crate) const U_LONG: Phoneme = vowel("œː", "œː");
pub(crate) const UU: Phoneme = vowel("y", "y");
/// oe, eu, ui, ei and y, ou.
pub(crate) const OE: Phoneme = vowel("u", "u");
pub(crate) const EU: Phoneme = vowel("ɪø", "øə");
pub(crate) const UI: Phoneme = vowel("œy", "øy");
pub(crate) const EI: Phoneme = vowel("əi", "ɛɪ");
pub(crate) const OU: Phoneme = vowel("œu", "əʊ");
/// ai (baie bˈaɪə).
pub(crate) const AI: Phoneme = vowel("ai", "aɪ");
/// aai, ɑɪ in eSpeak NG's notation (waai vˈɑɪ).
pub(crate) const AAI: Phoneme = vowel("aːi", "ɑɪ");
/// oei, uɪ in eSpeak NG's notation (goeie xˈuɪə).
pub(crate) const OEI: Phoneme = vowel("ui", "uɪ");
/// ij in the older spelling (hij hˈəj).
pub(crate) const IJ: Phoneme = vowel("əi", "əj");
/// ooi (mooi mˈʊɪ), and oe before a final r or in oë (snoer, voël).
pub(crate) const OOI: Phoneme = vowel("oːi", "ʊɪ");
pub(crate) const OE_LONG: Phoneme = vowel("u", "uː");
/// a, e and o before ns, nasal (kans kˈɑ̃ns, mens mˈẽns, ons õns).
pub(crate) const A_NASAL: Phoneme = vowel("ɑ\u{303}ː", "ɑ\u{303}");
pub(crate) const E_NASAL: Phoneme = vowel("ɛ\u{303}", "e\u{303}");
pub(crate) const O_NASAL: Phoneme = vowel("ɔ\u{303}", "o\u{303}");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
/// g after r or l before a vowel (berge).
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const Z: Phoneme = consonant("z", "z");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
/// g and ch.
pub(crate) const X: Phoneme = consonant("χ", "x");
pub(crate) const H: Phoneme = consonant("ɦ", "h");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const J: Phoneme = consonant("j", "j");
/// w after t, k, s and d (twintig twˈəntəx).
pub(crate) const W: Phoneme = consonant("w", "w");

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop or f, v or x
/// before l or r, s before a stop, m, n, l or w (straat, skool), str, spr and skr, and a stop or s
/// before w (twee).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let obstruent = |phoneme: &Phoneme| [P, B, T, D, K, F, V, X].contains(phoneme);
    let liquid = |phoneme: &Phoneme| [L, R].contains(phoneme);
    match cluster {
        [_] => true,
        [_, second] if [W, J].contains(second) => true,
        [first, second] if liquid(second) => obstruent(first),
        [first, second] => *first == S && [P, T, K, M, N, L].contains(second),
        [first, second, third] => *first == S && [P, T, K].contains(second) && *third == R,
        _ => false,
    }
}

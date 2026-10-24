//! Serbian phonemes in plain IPA and eSpeak NG's notation, which writes unstressed vowels and
//! some allophones of its own (æ, ɪ, ʊ, ɑ before r and j, ɾ), and how they pair by voicing.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
/// a unstressed, which eSpeak NG writes æ (mačka mˈatʃkæ).
pub(crate) const A_UNSTRESSED: Phoneme = vowel("a", "æ");
/// a before r and j, which eSpeak NG writes ɑ (dobar dˈobɑr, igraju).
pub(crate) const A_BACK: Phoneme = vowel("a", "ɑ");
pub(crate) const E: Phoneme = vowel("e", "ɛ");
/// e after j, which eSpeak NG writes e (lijepo lˈijepo, je).
pub(crate) const E_AFTER_J: Phoneme = vowel("e", "e");
pub(crate) const I: Phoneme = vowel("i", "i");
pub(crate) const I_UNSTRESSED: Phoneme = vowel("i", "ɪ");
pub(crate) const O: Phoneme = vowel("o", "o");
pub(crate) const U: Phoneme = vowel("u", "u");
pub(crate) const U_UNSTRESSED: Phoneme = vowel("u", "ʊ");
/// The ae of -naest, said as one syllable, which eSpeak NG writes aɪ (dvanaest dvˈanaɪst).
pub(crate) const AE: Phoneme = vowel("ae", "aɪ");
/// r as the nucleus of a syllable (smrt, prst).
pub(crate) const R_SYLLABIC: Phoneme = vowel("r̩", "r");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
/// v before a voiceless consonant: still v in plain IPA, f in eSpeak NG's notation (novčanik).
pub(crate) const V_BEFORE_VOICELESS: Phoneme = consonant("v", "f");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const Z: Phoneme = consonant("z", "z");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const ZH: Phoneme = consonant("ʒ", "ʒ");
pub(crate) const X: Phoneme = consonant("x", "x");
/// h inside or at the end of a word, which eSpeak NG writes h (kuha kˈuhæ, vrh); at the start x.
pub(crate) const H_INNER: Phoneme = consonant("x", "h");
pub(crate) const TS: Phoneme = consonant("ts", "ts");
pub(crate) const DZ: Phoneme = consonant("dz", "dz");
/// č and dž.
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const DZH: Phoneme = consonant("dʒ", "dʒ");
/// ć and đ.
pub(crate) const TJ: Phoneme = consonant("tɕ", "tɕ");
pub(crate) const DJ: Phoneme = consonant("dʑ", "dʑ");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NY: Phoneme = consonant("ɲ", "ɲ");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const LJ: Phoneme = consonant("ʎ", "ʎ");
/// r between vowels and at the end of a word.
pub(crate) const R: Phoneme = consonant("r", "r");
/// r at the start of a word and after a consonant, which eSpeak NG writes ɾ (redu, krovu).
pub(crate) const R_TAP: Phoneme = consonant("r", "ɾ");
pub(crate) const J: Phoneme = consonant("j", "j");

/// Voiceless obstruents and their voiced partners; v takes the voicing of what follows it but
/// gives none.
const PAIRS: [(Phoneme, Phoneme); 10] = [
    (P, B),
    (T, D),
    (K, G),
    (F, V),
    (S, Z),
    (SH, ZH),
    (TS, DZ),
    (CH, DZH),
    (TJ, DJ),
    (X, consonant("ɣ", "ɣ")),
];

/// Whether `phoneme` is an obstruent, which voicing changes.
pub(crate) fn is_obstruent(phoneme: Phoneme) -> bool {
    PAIRS
        .iter()
        .any(|&(voiceless, voiced)| phoneme == voiceless || phoneme == voiced)
}

/// Whether `phoneme` is a voiced obstruent that voices the one before it (not v).
pub(crate) fn voices(phoneme: Phoneme) -> bool {
    phoneme != V && PAIRS.iter().any(|&(_, voiced)| phoneme == voiced)
}

/// Whether `phoneme` is a voiceless obstruent.
pub(crate) fn is_voiceless(phoneme: Phoneme) -> bool {
    PAIRS.iter().any(|&(voiceless, _)| phoneme == voiceless)
}

/// `phoneme` with the voicing `voiced`, if it is an obstruent that has a partner.
pub(crate) fn with_voicing(phoneme: Phoneme, voiced: bool) -> Phoneme {
    PAIRS
        .iter()
        .find_map(|&(voiceless, partner)| {
            if voiced {
                (phoneme == voiceless).then_some(partner)
            } else {
                (phoneme == partner).then_some(voiceless)
            }
        })
        .unwrap_or(phoneme)
}

/// Whether `cluster` can start a syllable inside a word: one consonant, or obstruents (two only
/// if one is a fricative) with at most one sonorant or j after them (pr, sk, str, sv).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    if cluster.len() == 1 {
        return true;
    }
    let sonorant = |phoneme: Phoneme| [M, N, NY, NG, L, LJ, R, R_TAP, J, V].contains(&phoneme);
    let fricative = |phoneme: Phoneme| [F, V, S, Z, SH, ZH, X].contains(&phoneme);
    let obstruents = cluster
        .iter()
        .take_while(|&&phoneme| is_obstruent(phoneme) && !sonorant(phoneme))
        .count();
    let sonorants_fit = match &cluster[obstruents..] {
        [] => true,
        [one] | [one, J] => sonorant(*one),
        _ => false,
    };
    sonorants_fit
        && match &cluster[..obstruents] {
            [] => cluster.len() == 2 && cluster[1] == J,
            [_] => true,
            [first, second] => fricative(*first) || fricative(*second),
            _ => false,
        }
}

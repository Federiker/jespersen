//! Macedonian phonemes in plain IPA and eSpeak NG's notation, which keeps to Kokoro's vocabulary
//! (decision 48: dark l written l), and how they pair by voicing.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const E: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const I: Phoneme = vowel("i", "i");
pub(crate) const O: Phoneme = vowel("ɔ", "ɔ");
pub(crate) const U: Phoneme = vowel("u", "u");
/// r as the nucleus of a syllable (прст, срце, 'рѓа).
pub(crate) const R_SYLLABIC: Phoneme = vowel("r̩", "r");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
/// ќ and ѓ, palatal stops.
pub(crate) const KJ: Phoneme = consonant("c", "c");
pub(crate) const GJ: Phoneme = consonant("ɟ", "ɟ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const Z: Phoneme = consonant("z", "z");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const ZH: Phoneme = consonant("ʒ", "ʒ");
pub(crate) const X: Phoneme = consonant("x", "x");
/// ц and ѕ.
pub(crate) const TS: Phoneme = consonant("ts", "ts");
pub(crate) const DZ: Phoneme = consonant("dz", "dz");
/// ч and џ.
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const DZH: Phoneme = consonant("dʒ", "dʒ");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NY: Phoneme = consonant("ɲ", "ɲ");
/// н before к and г (банка).
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
/// л, dark except before е, и and ј; Kokoro has no ɫ, so eSpeak NG's notation writes l.
pub(crate) const L_DARK: Phoneme = consonant("ɫ", "l");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const LJ: Phoneme = consonant("ʎ", "ʎ");
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const J: Phoneme = consonant("j", "j");

/// Voiceless obstruents and their voiced partners; в takes the voicing of what follows it but
/// gives none.
const PAIRS: [(Phoneme, Phoneme); 9] = [
    (P, B),
    (T, D),
    (K, G),
    (KJ, GJ),
    (F, V),
    (S, Z),
    (SH, ZH),
    (TS, DZ),
    (CH, DZH),
];

/// Whether `phoneme` is an obstruent, which voicing changes.
pub(crate) fn is_obstruent(phoneme: Phoneme) -> bool {
    phoneme == X
        || PAIRS
            .iter()
            .any(|&(voiceless, voiced)| phoneme == voiceless || phoneme == voiced)
}

/// Whether `phoneme` is a voiced obstruent that voices the one before it (not в).
pub(crate) fn voices(phoneme: Phoneme) -> bool {
    phoneme != V && PAIRS.iter().any(|&(_, voiced)| phoneme == voiced)
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
/// if one is a fricative) with at most one sonorant or ј after them (пр, ск, стр, св).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    if cluster.len() == 1 {
        return true;
    }
    let sonorant = |phoneme: Phoneme| [M, N, NY, NG, L, L_DARK, LJ, R, J, V].contains(&phoneme);
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

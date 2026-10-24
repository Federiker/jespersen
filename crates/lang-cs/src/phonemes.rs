//! Czech phonemes in plain IPA and eSpeak NG's notation, and how they pair by voicing.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const AA: Phoneme = vowel("aː", "aː");
pub(crate) const E: Phoneme = vowel("ɛ", "e");
pub(crate) const EE: Phoneme = vowel("ɛː", "eː");
pub(crate) const I: Phoneme = vowel("ɪ", "i");
pub(crate) const II: Phoneme = vowel("iː", "iː");
pub(crate) const O: Phoneme = vowel("o", "o");
pub(crate) const OO: Phoneme = vowel("oː", "oː");
pub(crate) const U: Phoneme = vowel("u", "u");
pub(crate) const UU: Phoneme = vowel("uː", "uː");
pub(crate) const OU: Phoneme = vowel("ou̯", "oʊ");
pub(crate) const AU: Phoneme = vowel("au̯", "aʊ");
pub(crate) const EU: Phoneme = vowel("ɛu̯", "eʊ");
/// r and l as the nucleus of a syllable (krk, vlk).
pub(crate) const R_SYLLABIC: Phoneme = vowel("r̩", "r̩");
pub(crate) const L_SYLLABIC: Phoneme = vowel("l̩", "l̩");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const TJ: Phoneme = consonant("c", "c");
pub(crate) const DJ: Phoneme = consonant("ɟ", "ɟ");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const Z: Phoneme = consonant("z", "z");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const ZH: Phoneme = consonant("ʒ", "ʒ");
pub(crate) const X: Phoneme = consonant("x", "x");
/// h, voiced ɦ, which eSpeak NG writes h.
pub(crate) const H: Phoneme = consonant("ɦ", "h");
pub(crate) const TS: Phoneme = consonant("ts", "ts");
pub(crate) const DZ: Phoneme = consonant("dz", "dz");
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const DZH: Phoneme = consonant("dʒ", "dʒ");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NY: Phoneme = consonant("ɲ", "ɲ");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const R: Phoneme = consonant("r", "r");
/// ř, and its voiceless form after or before a voiceless consonant (tři, přes).
pub(crate) const RZ: Phoneme = consonant("r̝", "r̝");
pub(crate) const RZ_VOICELESS: Phoneme = consonant("r̝̊", "r̝̊");
/// v and ř ending a word: voiceless in plain IPA, kept voiced by eSpeak NG (mrkev, kancelář).
pub(crate) const V_FINAL: Phoneme = consonant("f", "v");
pub(crate) const RZ_FINAL: Phoneme = consonant("r̝̊", "r̝");
pub(crate) const J: Phoneme = consonant("j", "j");
/// The j of ej before a consonant, which eSpeak NG writes ɪ (odejdeš ˈodeɪdeʃ).
pub(crate) const J_OFFGLIDE: Phoneme = consonant("j", "ɪ");

// A voiceless consonant voiced by a voiced one after it inside a word (kdo ɡdo, shoda zɦoda):
// eSpeak NG's notation keeps the letter.
pub(crate) const K_VOICED: Phoneme = consonant("ɡ", "k");
pub(crate) const S_VOICED: Phoneme = consonant("z", "s");
pub(crate) const T_VOICED: Phoneme = consonant("d", "t");
pub(crate) const P_VOICED: Phoneme = consonant("b", "p");
pub(crate) const SH_VOICED: Phoneme = consonant("ʒ", "ʃ");
pub(crate) const CH_VOICED: Phoneme = consonant("dʒ", "tʃ");
pub(crate) const TS_VOICED: Phoneme = consonant("dz", "ts");
pub(crate) const F_VOICED: Phoneme = consonant("v", "f");
pub(crate) const X_VOICED: Phoneme = consonant("ɦ", "x");
pub(crate) const TJ_VOICED: Phoneme = consonant("ɟ", "c");
/// The prepositions v and z as plain IPA says them by the next word (v pokoji f, z domu z), which
/// eSpeak NG writes v and s.
pub(crate) const V_DEVOICED: Phoneme = consonant("f", "v");
pub(crate) const Z_WRITTEN_S: Phoneme = consonant("z", "s");

/// Voiceless obstruents and their voiced partners.
const PAIRS: [(Phoneme, Phoneme); 11] = [
    (P, B),
    (T, D),
    (TJ, DJ),
    (K, G),
    (F, V),
    (S, Z),
    (SH, ZH),
    (X, H),
    (TS, DZ),
    (CH, DZH),
    (RZ_VOICELESS, RZ),
];

/// The voiced partners a word gives its voiceless consonants inside it, written as the letter in
/// eSpeak NG's notation.
const INSIDE: [(Phoneme, Phoneme); 10] = [
    (P, P_VOICED),
    (T, T_VOICED),
    (TJ, TJ_VOICED),
    (K, K_VOICED),
    (F, F_VOICED),
    (S, S_VOICED),
    (SH, SH_VOICED),
    (X, X_VOICED),
    (TS, TS_VOICED),
    (CH, CH_VOICED),
];

/// Whether `phoneme` is an obstruent, which voicing changes.
pub(crate) fn is_obstruent(phoneme: Phoneme) -> bool {
    PAIRS
        .iter()
        .chain(&INSIDE)
        .any(|&(voiceless, voiced)| phoneme == voiceless || phoneme == voiced)
}

/// Whether `phoneme` is a voiced obstruent.
pub(crate) fn is_voiced(phoneme: Phoneme) -> bool {
    PAIRS.iter().chain(&INSIDE).any(|&(_, voiced)| phoneme == voiced)
}

/// `phoneme` devoiced, or voiced across words (zavřít dveře zavr̝iːd dvɛr̝ɛ).
pub(crate) fn with_voicing(phoneme: Phoneme, voiced: bool) -> Phoneme {
    PAIRS
        .iter()
        .chain(&INSIDE)
        .find_map(|&(voiceless, partner)| {
            if voiced {
                (phoneme == voiceless).then_some(partner)
            } else {
                (phoneme == partner).then_some(voiceless)
            }
        })
        .unwrap_or(phoneme)
}

/// `phoneme` voiced inside a word, in plain IPA only.
pub(crate) fn voiced_inside(phoneme: Phoneme) -> Phoneme {
    INSIDE
        .iter()
        .find_map(|&(voiceless, partner)| (phoneme == voiceless).then_some(partner))
        .unwrap_or(phoneme)
}

/// Whether `cluster` can start a syllable inside a word: one consonant, or obstruents (two only
/// if the first is a fricative or the second one a sibilant) with at most one sonorant or j after
/// them (pr, sk, str, kv), or a sonorant and j (mj).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    if cluster.len() == 1 {
        return true;
    }
    let sonorant = |phoneme: Phoneme| [M, N, NY, NG, L, R, RZ, RZ_VOICELESS, J, V].contains(&phoneme);
    let fricative = |phoneme: Phoneme| [F, V, S, Z, SH, ZH, X, H].contains(&phoneme);
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

/// A voiced obstruent made voiceless at the end of a word, remembering its letter: voiceless at
/// a pause and before a voiceless consonant, voiced before a voiced obstruent, and before a vowel
/// or a sonorant voiceless in plain IPA but voiced again in eSpeak NG's notation (než odejdeš
/// nˈeʒ, muž mluví).
const FINALS: [(Phoneme, Phoneme, Phoneme); 8] = [
    (B, consonant("p", "p").misaki("b"), consonant("p", "b")),
    (D, consonant("t", "t").misaki("d"), consonant("t", "d")),
    (DJ, consonant("c", "c").misaki("ɟ"), consonant("c", "ɟ")),
    (G, consonant("k", "k").misaki("ɡ"), consonant("k", "ɡ")),
    (H, consonant("x", "x").misaki("ɦ"), consonant("x", "h")),
    (Z, consonant("s", "s").misaki("z"), consonant("s", "z")),
    (ZH, consonant("ʃ", "ʃ").misaki("ʒ"), consonant("ʃ", "ʒ")),
    (DZH, consonant("tʃ", "tʃ").misaki("dʒ"), consonant("tʃ", "dʒ")),
];

/// `phoneme` at the end of a word: a voiced one becomes its final form.
pub(crate) fn final_form(phoneme: Phoneme) -> Phoneme {
    FINALS
        .iter()
        .find_map(|&(voiced, final_form, _)| (phoneme == voiced).then_some(final_form))
        .unwrap_or_else(|| with_voicing(phoneme, false))
}

/// A final form before a vowel or a sonorant (voiced again in eSpeak NG's notation), or any
/// final obstruent before a voiced obstruent (voiced; k only in plain IPA: tak dobře tˈak).
pub(crate) fn before_next(phoneme: Phoneme, voiced_obstruent: bool) -> Phoneme {
    if let Some(&(voiced, _, before_vowel)) = FINALS.iter().find(|&&(_, final_form, _)| phoneme == final_form) {
        return if voiced_obstruent { voiced } else { before_vowel };
    }
    match phoneme {
        phoneme if !voiced_obstruent => phoneme,
        phoneme if phoneme == K => K_VOICED,
        phoneme => with_voicing(phoneme, true),
    }
}

/// Whether `phoneme` is a final form (a voiced letter made voiceless at the end of a word).
pub(crate) fn is_final_form(phoneme: Phoneme) -> bool {
    FINALS.iter().any(|&(_, final_form, _)| phoneme == final_form)
}

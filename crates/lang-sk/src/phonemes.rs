//! Slovak phonemes in plain IPA and eSpeak NG's notation, and how they pair by voicing.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const AA: Phoneme = vowel("aː", "aː");
pub(crate) const E: Phoneme = vowel("e", "e");
pub(crate) const EE: Phoneme = vowel("eː", "eː");
/// ä, which eSpeak NG reads as e (mäso).
pub(crate) const AE: Phoneme = vowel("æ", "e");
pub(crate) const I: Phoneme = vowel("i", "i");
pub(crate) const II: Phoneme = vowel("iː", "iː");
pub(crate) const O: Phoneme = vowel("o", "o");
pub(crate) const OO: Phoneme = vowel("oː", "oː");
pub(crate) const U: Phoneme = vowel("u", "u");
pub(crate) const UU: Phoneme = vowel("uː", "uː");
/// ô.
pub(crate) const UO: Phoneme = vowel("u̯o", "uo");
pub(crate) const OU: Phoneme = vowel("ou̯", "oʊ");
pub(crate) const AU: Phoneme = vowel("au̯", "aʊ");
/// r and l as the nucleus of a syllable (krk, vlk).
pub(crate) const R_SYLLABIC: Phoneme = vowel("r̩", "r̩");
pub(crate) const L_SYLLABIC: Phoneme = vowel("l̩", "l̩");
/// ŕ and ĺ, long syllabic r and l (vŕba, vĺča).
pub(crate) const R_SYLLABIC_LONG: Phoneme = vowel("r̩ː", "r̩ː");
pub(crate) const L_SYLLABIC_LONG: Phoneme = vowel("l̩ː", "l̩ː");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
/// ť and ď, and t and d before e and i, which eSpeak NG writes tʲ and dʲ.
pub(crate) const TJ: Phoneme = consonant("c", "tʲ");
pub(crate) const DJ: Phoneme = consonant("ɟ", "dʲ");
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
/// h made voiceless before a voiceless consonant, which eSpeak NG still writes h (ľahko).
pub(crate) const H_DEVOICED: Phoneme = consonant("x", "h");
pub(crate) const TS: Phoneme = consonant("ts", "ts");
pub(crate) const DZ: Phoneme = consonant("dz", "dz");
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const DZH: Phoneme = consonant("dʒ", "dʒ");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NY: Phoneme = consonant("ɲ", "ɲ");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
/// ľ.
pub(crate) const LJ: Phoneme = consonant("ʎ", "ʎ");
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const J: Phoneme = consonant("j", "j");
/// The i of ia, ie and iu, a glide into the vowel (piesok, ľudia).
pub(crate) const I_GLIDE: Phoneme = consonant("i̯", "i");
/// v after a vowel before a consonant or at the end, said u̯ (domov, pravda), which eSpeak NG
/// writes w.
pub(crate) const V_OFFGLIDE: Phoneme = consonant("u̯", "w");

// A voiceless consonant voiced by a voiced one after it inside a word (kde ɡɟe, shoda zɦoda):
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
pub(crate) const TJ_VOICED: Phoneme = consonant("ɟ", "tʲ");
/// The prepositions v and z as plain IPA says them before a voiceless consonant (v meste f, z poľa
/// s), which eSpeak NG writes v and z.
pub(crate) const V_DEVOICED: Phoneme = consonant("f", "v");
pub(crate) const Z_DEVOICED: Phoneme = consonant("s", "z");

/// Voiceless obstruents and their voiced partners.
const PAIRS: [(Phoneme, Phoneme); 10] = [
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

/// `phoneme` devoiced, or voiced across words (zavrieť dvere zavri̯eɟ dvere).
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
    let sonorant = |phoneme: Phoneme| [M, N, NY, NG, L, LJ, R, J, V].contains(&phoneme);
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
/// a pause and before a voiceless consonant, voiced again before a vowel, a sonorant or a voiced
/// obstruent (keď odídeš keɟ, dub rastie).
const FINALS: [(Phoneme, Phoneme); 8] = [
    (B, consonant("p", "p").misaki("b")),
    (D, consonant("t", "t").misaki("d")),
    (DJ, consonant("c", "tʲ").misaki("ɟ")),
    (G, consonant("k", "k").misaki("ɡ")),
    (H, consonant("x", "x").misaki("ɦ")),
    (Z, consonant("s", "s").misaki("z")),
    (ZH, consonant("ʃ", "ʃ").misaki("ʒ")),
    (DZH, consonant("tʃ", "tʃ").misaki("dʒ")),
];

/// `phoneme` at the end of a word: a voiced one becomes its final form.
pub(crate) fn final_form(phoneme: Phoneme) -> Phoneme {
    FINALS
        .iter()
        .find_map(|&(voiced, final_form)| (phoneme == voiced).then_some(final_form))
        .unwrap_or_else(|| with_voicing(phoneme, false))
}

/// A final obstruent before the next word, which voices it before a vowel, a sonorant or a voiced
/// obstruent (otec ide otedz ide, dub rastie). eSpeak NG's notation voices a voiceless letter only
/// before a voiced obstruent, k not even then (tak dobre tˈak).
pub(crate) fn before_next(phoneme: Phoneme, voiced_obstruent: bool) -> Phoneme {
    if let Some(&(voiced, _)) = FINALS.iter().find(|&&(_, final_form)| phoneme == final_form) {
        return voiced;
    }
    match phoneme {
        phoneme if !voiced_obstruent => voiced_inside(phoneme),
        phoneme if phoneme == K => K_VOICED,
        phoneme => with_voicing(phoneme, true),
    }
}

/// Whether `phoneme` is a final form (a voiced letter made voiceless at the end of a word).
pub(crate) fn is_final_form(phoneme: Phoneme) -> bool {
    FINALS.iter().any(|&(_, final_form)| phoneme == final_form)
}

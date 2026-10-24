//! Lithuanian phonemes in plain IPA and eSpeak NG's notation.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("ɐ", "a");
/// ą.
pub(crate) const A_LONG: Phoneme = vowel("aː", "aː");
pub(crate) const E: Phoneme = vowel("ɛ", "e");
/// ę, eɑ in eSpeak NG's notation (kęsti kʲˈeɑʂtʲi), and ė.
pub(crate) const E_LONG: Phoneme = vowel("æː", "eɑ");
pub(crate) const EE: Phoneme = vowel("eː", "ee");
/// i, ɪ in eSpeak NG's notation under the stress (jis jˈɪs) and i elsewhere.
pub(crate) const I: Phoneme = vowel("ɪ", "i");
pub(crate) const I_STRESSED: Phoneme = vowel("ɪ", "ɪ");
/// y and į.
pub(crate) const II: Phoneme = vowel("iː", "iː");
/// o, long in native words.
pub(crate) const O: Phoneme = vowel("oː", "oː");
pub(crate) const U: Phoneme = vowel("ʊ", "u");
/// ū and ų.
pub(crate) const UU: Phoneme = vowel("uː", "uː");
/// -iai after a consonant, one syllable (svečiai).
pub(crate) const EI: Phoneme = vowel("ɛi", "ei");
/// eu, in loanwords (euras ˈɛuras).
pub(crate) const EU: Phoneme = vowel("ɛu", "ɛu");
/// The diphthongs ie, uo, ai, au and ui (ei is said as e and i, with the stress on the i).
pub(crate) const IE: Phoneme = vowel("iɛ", "ie");
pub(crate) const UO: Phoneme = vowel("uɔ", "uo");
pub(crate) const AI: Phoneme = vowel("ɐi", "ai");
pub(crate) const AU: Phoneme = vowel("ɐu", "au");
pub(crate) const UI: Phoneme = vowel("ʊi", "ui");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("ʋ", "v");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const Z: Phoneme = consonant("z", "z");
/// š and ž.
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const ZH: Phoneme = consonant("ʒ", "ʒ");
/// c, č, dz and dž; č and dž are always soft, tɕ and dʑ in eSpeak NG's notation.
pub(crate) const TS: Phoneme = consonant("ts", "ts");
pub(crate) const CH: Phoneme = consonant("tʃʲ", "tɕ");
pub(crate) const DZ: Phoneme = consonant("dz", "dz");
pub(crate) const DZH: Phoneme = consonant("dʒʲ", "dʑ");
/// č, dž and š before an i and a back vowel, which eSpeak NG marks soft again (ačiū ˈatɕʲuː).
pub(crate) const CH_BACK: Phoneme = consonant("tʃʲ", "tɕʲ");
pub(crate) const DZH_BACK: Phoneme = consonant("dʒʲ", "dʑʲ");
pub(crate) const SH_BACK: Phoneme = consonant("ɕ", "ɕʲ");
/// n, s and ž, which eSpeak NG marks twice before an i and a back vowel (žmonių ʒmˈoːnʲʲuː, vaisius
/// vˈaiʂʲus).
pub(crate) const N_BACK: Phoneme = consonant("nʲ", "nʲʲ");
pub(crate) const S_BACK: Phoneme = consonant("sʲ", "ʂʲ");
pub(crate) const ZH_BACK: Phoneme = consonant("ʑ", "ʒʲ");
/// c soft (konferencija).
pub(crate) const TS_SOFT: Phoneme = consonant("tsʲ", "tsʲ");
/// h, and ch.
pub(crate) const H: Phoneme = consonant("ɣ", "h");
pub(crate) const X: Phoneme = consonant("x", "x");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
/// A hard l, l̩ in eSpeak NG's notation (labas l̩ˈabas).
pub(crate) const L: Phoneme = consonant("ɫ", "l\u{329}");
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const J: Phoneme = consonant("j", "j");

/// The soft consonants, before a front vowel, an i and a back vowel, or a soft consonant: ʲ in
/// eSpeak NG's notation, but ʂ for s, ɕ for š and ɭ for l, and nothing marked on ɡ, z, ž and h.
pub(crate) const P_SOFT: Phoneme = consonant("pʲ", "pʲ");
pub(crate) const B_SOFT: Phoneme = consonant("bʲ", "bʲ");
pub(crate) const T_SOFT: Phoneme = consonant("tʲ", "tʲ");
pub(crate) const D_SOFT: Phoneme = consonant("dʲ", "dʲ");
pub(crate) const K_SOFT: Phoneme = consonant("kʲ", "kʲ");
pub(crate) const G_SOFT: Phoneme = consonant("ɡʲ", "ɡ");
pub(crate) const F_SOFT: Phoneme = consonant("fʲ", "fʲ");
pub(crate) const V_SOFT: Phoneme = consonant("ʋʲ", "vʲ");
pub(crate) const S_SOFT: Phoneme = consonant("sʲ", "ʂ");
pub(crate) const Z_SOFT: Phoneme = consonant("zʲ", "z");
pub(crate) const SH_SOFT: Phoneme = consonant("ɕ", "ɕ");
pub(crate) const ZH_SOFT: Phoneme = consonant("ʑ", "ʒ");
pub(crate) const H_SOFT: Phoneme = consonant("ɣʲ", "h");
pub(crate) const X_SOFT: Phoneme = consonant("xʲ", "xʲ");
pub(crate) const M_SOFT: Phoneme = consonant("mʲ", "mʲ");
pub(crate) const N_SOFT: Phoneme = consonant("nʲ", "nʲ");
pub(crate) const R_SOFT: Phoneme = consonant("rʲ", "rʲ");
/// A soft l before a front vowel (lietus ɭˈietus), and before a back one, which eSpeak NG marks
/// again (kelionė kʲeɭʲˈoːnʲee).
pub(crate) const L_SOFT: Phoneme = consonant("lʲ", "ɭ");
pub(crate) const L_SOFT_BACK: Phoneme = consonant("lʲ", "ɭʲ");

/// Each hard consonant and its soft form.
const PAIRS: [(Phoneme, Phoneme); 19] = [
    (TS, TS_SOFT),
    (P, P_SOFT),
    (B, B_SOFT),
    (T, T_SOFT),
    (D, D_SOFT),
    (K, K_SOFT),
    (G, G_SOFT),
    (F, F_SOFT),
    (V, V_SOFT),
    (S, S_SOFT),
    (Z, Z_SOFT),
    (SH, SH_SOFT),
    (ZH, ZH_SOFT),
    (H, H_SOFT),
    (X, X_SOFT),
    (M, M_SOFT),
    (N, N_SOFT),
    (R, R_SOFT),
    (L, L_SOFT),
];

/// The soft form of a consonant, itself for one that has none or is soft already (j, č, dž).
pub(crate) fn soft(phoneme: Phoneme) -> Phoneme {
    PAIRS
        .iter()
        .find(|(hard, _)| *hard == phoneme)
        .map_or(phoneme, |(_, soft)| *soft)
}

/// The hard form of a consonant, itself for a hard one.
pub(crate) fn hard(phoneme: Phoneme) -> Phoneme {
    match phoneme {
        phoneme if phoneme == L_SOFT_BACK => return L,
        phoneme if phoneme == SH_BACK => return SH,
        phoneme if phoneme == CH_BACK => return CH,
        phoneme if phoneme == DZH_BACK => return DZH,
        phoneme if phoneme == N_BACK => return N,
        phoneme if phoneme == S_BACK => return S,
        phoneme if phoneme == ZH_BACK => return ZH,
        _ => {}
    }
    PAIRS
        .iter()
        .find(|(_, soft)| *soft == phoneme)
        .map_or(phoneme, |(hard, _)| *hard)
}

/// Whether `phoneme` is a soft consonant: one of the soft forms, or č, dž and j.
pub(crate) fn is_soft(phoneme: Phoneme) -> bool {
    [
        CH,
        DZH,
        J,
        L_SOFT_BACK,
        CH_BACK,
        DZH_BACK,
        SH_BACK,
        N_BACK,
        S_BACK,
        ZH_BACK,
    ]
    .contains(&phoneme)
        || PAIRS.iter().any(|(_, soft)| *soft == phoneme)
}

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop, f or v before l, r
/// or v, s, š, z or ž before a stop, a nasal, l, r or v (sniegas, šviesa), and str, skr, spr.
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let classes: Vec<Phoneme> = cluster.iter().map(|&phoneme| hard(phoneme)).collect();
    let stop = |phoneme: Phoneme| [P, B, T, D, K, G, F, V].contains(&phoneme);
    let liquid = |phoneme: Phoneme| [L, R, V].contains(&phoneme);
    let sibilant = |phoneme: Phoneme| [S, Z, SH, ZH].contains(&phoneme);
    match classes[..] {
        [_] => true,
        [first, second] => {
            (stop(first) && liquid(second))
                || (sibilant(first) && (stop(second) || liquid(second) || [M, N].contains(&second)))
        }
        [first, second, third] => sibilant(first) && stop(second) && liquid(third),
        _ => false,
    }
}

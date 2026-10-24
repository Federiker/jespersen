//! Spanish phonemes. Plain IPA is broad (phonemic); some phonemes are allophones written only in
//! eSpeak NG's notation, as its output does (β ð ɣ between vowels, ŋ and m for n, ɛ before a
//! nasal), so they have the plain phoneme's IPA symbol.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const E: Phoneme = vowel("e", "e");
/// Stressed e before n and a consonant, which eSpeak NG writes ɛ (tengo tˈɛŋɡo).
pub(crate) const E_OPEN: Phoneme = vowel("e", "ɛ");
pub(crate) const I: Phoneme = vowel("i", "i");
pub(crate) const O: Phoneme = vowel("o", "o");
pub(crate) const U: Phoneme = vowel("u", "u");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
/// b, d and g between vowels and after most consonants: β ð ɣ in eSpeak NG's notation.
pub(crate) const B_SOFT: Phoneme = consonant("b", "β");
pub(crate) const D_SOFT: Phoneme = consonant("d", "ð");
pub(crate) const G_SOFT: Phoneme = consonant("ɡ", "ɣ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const THETA: Phoneme = consonant("θ", "θ");
pub(crate) const X: Phoneme = consonant("x", "x");
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
/// n before k, g and x (ŋ), and before b, p and m (m), in eSpeak NG's notation.
pub(crate) const N_VELAR: Phoneme = consonant("n", "ŋ");
pub(crate) const N_LABIAL: Phoneme = consonant("n", "m");
pub(crate) const NY: Phoneme = consonant("ɲ", "ɲ");
pub(crate) const L: Phoneme = consonant("l", "l");
/// ll in Castilian.
pub(crate) const LL: Phoneme = consonant("ʎ", "ʎ");
/// ll in Latin American Spanish, which eSpeak NG writes jj.
pub(crate) const LL_YEISMO: Phoneme = consonant("ʝ", "jj");
/// y before a vowel.
pub(crate) const Y: Phoneme = consonant("ʝ", "ʝ");
pub(crate) const TAP: Phoneme = consonant("ɾ", "ɾ");
pub(crate) const TRILL: Phoneme = consonant("r", "r");
/// i and u before the vowel of their syllable (cielo, bueno).
pub(crate) const GLIDE_I: Phoneme = consonant("j", "j");
pub(crate) const GLIDE_U: Phoneme = consonant("w", "w");
/// i and u after the vowel of their syllable (rey, euro): eSpeak NG writes ɪ and ʊ.
pub(crate) const OFFGLIDE_I: Phoneme = consonant("i̯", "ɪ");
pub(crate) const OFFGLIDE_U: Phoneme = consonant("u̯", "ʊ");
/// The i of muy, after u, which eSpeak NG writes j.
pub(crate) const OFFGLIDE_I_AFTER_U: Phoneme = consonant("i̯", "j");

/// Whether `phoneme` is a nasal consonant, after which b, d and g stay stops.
pub(crate) fn is_nasal(phoneme: Phoneme) -> bool {
    [M, N, N_VELAR, N_LABIAL, NY].contains(&phoneme)
}

/// Whether `phoneme` is a glide after a vowel, which never starts a syllable.
pub(crate) fn is_offglide(phoneme: Phoneme) -> bool {
    [OFFGLIDE_I, OFFGLIDE_U, OFFGLIDE_I_AFTER_U].contains(&phoneme)
}

/// Whether `cluster` can start a Spanish syllable: one consonant (not an offglide); a stop or f
/// with r or l (pl, br, tr; not dl); any of those followed by a glide (bueno, prueba, fiel).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let without_glide = match cluster {
        [rest @ .., GLIDE_I | GLIDE_U] => rest,
        _ => cluster,
    };
    match without_glide {
        [] => cluster.len() == 1,
        [single] => !is_offglide(*single),
        [first, TAP | L] => {
            [P, B, B_SOFT, F, K, G, G_SOFT].contains(first) || ([T, D, D_SOFT].contains(first) && cluster[1] == TAP)
        }
        _ => false,
    }
}

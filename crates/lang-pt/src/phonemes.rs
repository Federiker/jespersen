//! Portuguese phonemes in plain IPA and eSpeak NG's notation. Where the two varieties write a
//! sound differently there is one constant for each; where eSpeak NG's notation writes an
//! allophone or a habit of its own (ɾə for an r before a consonant, ɑ before l) the plain IPA
//! symbol is the phoneme.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
/// European unstressed a, and stressed a before m, n and nh.
pub(crate) const A_REDUCED: Phoneme = vowel("ɐ", "ɐ");
/// European stressed a before m, n or nh, which eSpeak NG writes nasal (ano ˈɐ̃nʊ).
pub(crate) const A_BEFORE_NASAL: Phoneme = vowel("ɐ", "ɐ̃");
/// European a before a final l (sal sˈɑl).
pub(crate) const A_BEFORE_L: Phoneme = vowel("a", "ɑ");
/// Brazilian unstressed a after the stress (casa kˈazæ in eSpeak NG's notation).
pub(crate) const A_AFTER_STRESS: Phoneme = vowel("ɐ", "æ");
/// Brazilian a starting a word before m or n (amigos ˌæmˈiɡʊs in eSpeak NG's notation).
pub(crate) const A_OPENING: Phoneme = vowel("a", "æ");
/// à, a and a fused (à praia ˌaː).
pub(crate) const A_LONG: Phoneme = vowel("a", "aː");
pub(crate) const E: Phoneme = vowel("e", "e");
pub(crate) const E_OPEN: Phoneme = vowel("ɛ", "ɛ");
/// European unstressed e.
pub(crate) const E_REDUCED: Phoneme = vowel("ɨ", "ɨ");
/// European unstressed e with a rhythmic stress, which eSpeak NG writes e (telhado tˌeʎˈadʊ).
pub(crate) const E_RHYTHMIC: Phoneme = vowel("ɨ", "e");
/// Brazilian e ending a word (leite lˈeɪtʃy in eSpeak NG's notation).
pub(crate) const E_FINAL: Phoneme = vowel("i", "y");
pub(crate) const I: Phoneme = vowel("i", "i");
pub(crate) const O: Phoneme = vowel("o", "o");
pub(crate) const O_OPEN: Phoneme = vowel("ɔ", "ɔ");
/// European unstressed o.
pub(crate) const O_REDUCED: Phoneme = vowel("u", "u");
/// o ending a word (gato ɡˈatʊ).
pub(crate) const O_FINAL: Phoneme = vowel("u", "ʊ");
pub(crate) const U: Phoneme = vowel("u", "u");

pub(crate) const A_NASAL: Phoneme = vowel("ɐ̃", "ɐ̃");
pub(crate) const E_NASAL: Phoneme = vowel("ẽ", "eɪ");
pub(crate) const I_NASAL: Phoneme = vowel("ĩ", "i");
pub(crate) const O_NASAL: Phoneme = vowel("õ", "o");
/// õ written with its tilde (põe pˈõj).
pub(crate) const O_TILDE: Phoneme = vowel("o\u{303}", "o\u{303}");
/// European unstressed nasal o (montanhas mˌuŋtˈɐ̃ɲɐʃ in eSpeak NG's notation).
pub(crate) const O_NASAL_REDUCED: Phoneme = vowel("õ", "u");
pub(crate) const U_NASAL: Phoneme = vowel("u\u{303}", "u\u{303}");
/// The nasal consonant eSpeak NG writes after a nasal vowel: ŋ, m before p and b.
pub(crate) const NASAL_CODA: Phoneme = consonant("", "ŋ");
pub(crate) const NASAL_CODA_LABIAL: Phoneme = consonant("", "m");
/// The nasal glides of ão, ãe and õe, and of a final -am and -em.
pub(crate) const GLIDE_U_NASAL: Phoneme = consonant("w̃", "ʊ̃");
pub(crate) const GLIDE_I_NASAL_EUROPEAN: Phoneme = consonant("j̃", "j");
pub(crate) const GLIDE_I_NASAL_BRAZILIAN: Phoneme = consonant("j̃", "y");
/// The glide of a final -em, which eSpeak NG writes ŋ (bem bˈeɪŋ).
pub(crate) const GLIDE_EM: Phoneme = consonant("j̃", "ŋ");

/// The second half of ai, ei, oi, ui; au, eu, iu; ou.
pub(crate) const OFFGLIDE_I: Phoneme = consonant("j", "ɪ");
pub(crate) const OFFGLIDE_U: Phoneme = consonant("w", "ʊ");
pub(crate) const OFFGLIDE_U_AFTER_O: Phoneme = consonant("w", "w");
/// i and u before a vowel: after the stress (pátio pˈatjʊ), in qu and gu (quatro, água), and a
/// final o or Brazilian e before a word that starts with a vowel.
pub(crate) const GLIDE_I: Phoneme = consonant("j", "j");
pub(crate) const GLIDE_U: Phoneme = consonant("w", "w");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const Z: Phoneme = consonant("z", "z");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const ZH: Phoneme = consonant("ʒ", "ʒ");
/// Brazilian t and d before an i sound.
pub(crate) const TJ: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const DJ: Phoneme = consonant("dʒ", "dʒ");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NY: Phoneme = consonant("ɲ", "ɲ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const LH_EUROPEAN: Phoneme = consonant("ʎ", "ʎ");
pub(crate) const LH_BRAZILIAN: Phoneme = consonant("ʎ", "lj");
/// A Brazilian l ending a syllable (Brasil brazˈiʊ); after u both varieties write w (sul sˈuw).
pub(crate) const L_VOCALISED: Phoneme = consonant("w", "ʊ");
pub(crate) const L_AFTER_U: Phoneme = consonant("w", "w");
pub(crate) const TAP: Phoneme = consonant("ɾ", "ɾ");
/// An r before a consonant, which eSpeak NG writes with a schwa after it (porta pˈɔɾətɐ).
pub(crate) const TAP_CODA: Phoneme = consonant("ɾ", "ɾə");
/// An r after a consonant (prato) or ending a word: eSpeak NG writes ɹ for Portugal, r for Brazil.
pub(crate) const R_EUROPEAN: Phoneme = consonant("ɾ", "ɹ");
pub(crate) const R_BRAZILIAN: Phoneme = consonant("ɾ", "r");
/// The strong r of rato and carro.
pub(crate) const RR_EUROPEAN: Phoneme = consonant("ʁ", "ʁ");
pub(crate) const RR_BRAZILIAN: Phoneme = consonant("x", "x");

/// Whether `phoneme` is an r that can follow a consonant in an onset (prato, livro).
pub(crate) fn is_cluster_r(phoneme: Phoneme) -> bool {
    [TAP, R_EUROPEAN, R_BRAZILIAN].contains(&phoneme)
}

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop or f or v before l
/// or r (prato, bloco, livro), or a consonant before a glide (quatro, pátio).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let obstruent = |phoneme: Phoneme| [P, B, T, D, K, G, F, V, TJ, DJ].contains(&phoneme);
    match cluster {
        [_] => true,
        [first, second] if [GLIDE_I, GLIDE_U].contains(second) => !second.is_vowel() && *first != *second,
        [first, second] => obstruent(*first) && (*second == L || is_cluster_r(*second)),
        [first, second, glide] => {
            obstruent(*first) && (*second == L || is_cluster_r(*second)) && [GLIDE_I, GLIDE_U].contains(glide)
        }
        _ => false,
    }
}

/// Whether `phoneme` is voiced, for the s before it (mesmo mˈeʒmʊ, os amigos).
pub(crate) fn is_voiced(phoneme: Phoneme) -> bool {
    phoneme.is_vowel()
        || [
            B,
            D,
            G,
            V,
            Z,
            ZH,
            DJ,
            M,
            N,
            NY,
            L,
            LH_EUROPEAN,
            LH_BRAZILIAN,
            TAP,
            R_EUROPEAN,
            R_BRAZILIAN,
            RR_EUROPEAN,
            RR_BRAZILIAN,
            GLIDE_I,
            GLIDE_U,
        ]
        .contains(&phoneme)
}

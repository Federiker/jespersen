//! Italian phonemes. Plain IPA writes long consonants doubled (ˈbokka, ˈpittsa); eSpeak NG's
//! notation writes a long stop or affricate with ː (bˈokːa, pˈitsːa) and other long consonants
//! doubled (bˈɛllo), so the first half of a long stop is written nothing there. Some phonemes are
//! allophones written only in eSpeak NG's notation, and have the plain phoneme's IPA symbol.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const E: Phoneme = vowel("e", "e");
pub(crate) const E_OPEN: Phoneme = vowel("ɛ", "ɛ");
/// The stressed e of the imperfect (-eva, -evano), closed as the standard says it and open in
/// eSpeak NG's notation (rideva ridˈɛva).
pub(crate) const E_IMPERFECT: Phoneme = vowel("e", "ɛ");
pub(crate) const I: Phoneme = vowel("i", "i");
/// Unstressed i ending a word, which eSpeak NG writes ɪ (anni ˈannɪ).
pub(crate) const I_FINAL: Phoneme = vowel("i", "ɪ");
pub(crate) const O: Phoneme = vowel("o", "o");
pub(crate) const O_OPEN: Phoneme = vowel("ɔ", "ɔ");
pub(crate) const U: Phoneme = vowel("u", "u");
/// Unstressed u, which eSpeak NG writes ʊ (futuro fʊtˈuro).
pub(crate) const U_LAX: Phoneme = vowel("u", "ʊ");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
pub(crate) const S: Phoneme = consonant("s", "s");
/// s before a voiced consonant (sbaglio ˈzbaʎo).
pub(crate) const Z: Phoneme = consonant("z", "z");
/// s between vowels, which eSpeak NG writes z as the North does (casa kˈaza).
pub(crate) const S_BETWEEN: Phoneme = consonant("s", "z");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const DJ: Phoneme = consonant("dʒ", "dʒ");
pub(crate) const TS: Phoneme = consonant("ts", "ts");
pub(crate) const DZ: Phoneme = consonant("dz", "dz");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
/// n before g, ŋ in eSpeak NG's notation (inglese iŋɡlˈeze).
pub(crate) const N_VELAR: Phoneme = consonant("n", "ŋ");
pub(crate) const NY: Phoneme = consonant("ɲ", "ɲ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const LY: Phoneme = consonant("ʎ", "ʎ");
pub(crate) const R: Phoneme = consonant("r", "r");
/// The second r of rr, which eSpeak NG writes ɾ (terra tˈɛrɾa).
pub(crate) const R_SECOND: Phoneme = consonant("r", "ɾ");
pub(crate) const GLIDE_I: Phoneme = consonant("j", "j");
pub(crate) const GLIDE_U: Phoneme = consonant("w", "w");
/// Glides after a consonant past a word's first syllable (storia), and u glides after consonants
/// other than q and g (fuoco), which eSpeak NG writes as the vowels i and ʊ.
pub(crate) const GLIDE_I_VOWEL: Phoneme = consonant("j", "i");
pub(crate) const GLIDE_U_VOWEL: Phoneme = consonant("w", "ʊ");
pub(crate) const OFFGLIDE_I: Phoneme = consonant("i̯", "i");
pub(crate) const OFFGLIDE_U: Phoneme = consonant("u̯", "ʊ");

/// The first half of a long stop or affricate: written in plain IPA, nothing in eSpeak NG's
/// notation, whose second half carries ː.
const fn first_half(ipa: &'static str) -> Phoneme {
    consonant(ipa, "")
}

pub(crate) const P_FIRST: Phoneme = first_half("p");
pub(crate) const B_FIRST: Phoneme = first_half("b");
pub(crate) const T_FIRST: Phoneme = first_half("t");
pub(crate) const D_FIRST: Phoneme = first_half("d");
pub(crate) const K_FIRST: Phoneme = first_half("k");
pub(crate) const G_FIRST: Phoneme = first_half("ɡ");
/// The first halves of long consonants, which close a syllable.
const FIRST_HALVES: [Phoneme; 6] = [P_FIRST, B_FIRST, T_FIRST, D_FIRST, K_FIRST, G_FIRST];
pub(crate) const P_LONG: Phoneme = consonant("p", "pː");
pub(crate) const B_LONG: Phoneme = consonant("b", "bː");
pub(crate) const T_LONG: Phoneme = consonant("t", "tː");
pub(crate) const D_LONG: Phoneme = consonant("d", "dː");
pub(crate) const K_LONG: Phoneme = consonant("k", "kː");
pub(crate) const G_LONG: Phoneme = consonant("ɡ", "ɡː");
pub(crate) const CH_LONG: Phoneme = consonant("tʃ", "tʃː");
pub(crate) const DJ_LONG: Phoneme = consonant("dʒ", "dʒː");
pub(crate) const TS_LONG: Phoneme = consonant("ts", "tsː");
pub(crate) const DZ_LONG: Phoneme = consonant("dz", "dzː");

/// Whether `cluster` can start an Italian syllable: one consonant; s before consonants (sp, str,
/// sbr); a stop, f or v before r or l; any of these before a glide.
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let without_glide = match cluster {
        [rest @ .., GLIDE_I | GLIDE_U | GLIDE_I_VOWEL | GLIDE_U_VOWEL] => rest,
        _ => cluster,
    };
    let without_s = match without_glide {
        // Not a double s, which is split (pas-sa-re).
        [S | Z, rest @ ..] if !rest.is_empty() && !matches!(rest[0], S | Z) => rest,
        _ => without_glide,
    };
    match without_s {
        [] => cluster.len() == 1,
        [single] => ![OFFGLIDE_I, OFFGLIDE_U].contains(single) && !FIRST_HALVES.contains(single),
        [first, R | L] => [P, B, T, D, K, G, F, V].contains(first),
        _ => false,
    }
}

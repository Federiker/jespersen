//! Irish phonemes in plain IPA and eSpeak NG's notation, which reads the southern (Munster) forms.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

/// Short a, which eSpeak NG writes A before most consonants (mac mˈAk) and a before n, r, ll and
/// m (bean bˈan), and long á.
pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const A_BACK: Phoneme = vowel("a", "A");
pub(crate) const AA: Phoneme = vowel("ɑː", "ɑː");
pub(crate) const E: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const EE: Phoneme = vowel("eː", "eː");
pub(crate) const I: Phoneme = vowel("ɪ", "ɪ");
pub(crate) const II: Phoneme = vowel("iː", "iː");
pub(crate) const O: Phoneme = vowel("ɔ", "ɔ");
pub(crate) const OO: Phoneme = vowel("oː", "oː");
pub(crate) const U: Phoneme = vowel("ʊ", "ʊ");
pub(crate) const UU: Phoneme = vowel("uː", "uː");
/// The short u of uai before g (uaidh ˈui̯ɡ).
pub(crate) const U_TENSE: Phoneme = vowel("u", "u");
/// The unstressed vowels: ə, and ɐ for an a in a closed syllable (obair ˈɔbɐr).
pub(crate) const SCHWA: Phoneme = vowel("ə", "ə");
pub(crate) const SCHWA_A: Phoneme = vowel("ə", "ɐ");
/// ia and ua, and the a or o before a lenited b or m (labhair lˈaʊər).
pub(crate) const IA: Phoneme = vowel("iə", "iə");
pub(crate) const UA: Phoneme = vowel("uə", "uə");
pub(crate) const AU: Phoneme = vowel("au", "aʊ");
/// a before dh or gh and a consonant (Tadhg t̪ˈaɪɡ).
pub(crate) const AI: Phoneme = vowel("ai", "aɪ");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
/// Broad t and d, dental: t̪ d̪.
pub(crate) const T: Phoneme = consonant("t̪", "t̪");
pub(crate) const D: Phoneme = consonant("d̪", "d̪");
/// Slender t and d: c and dʲ at the start of a word, tʃ and dʒ elsewhere (tír cˈiːr, idir
/// ˈɪdʒɪr).
pub(crate) const TJ: Phoneme = consonant("tʲ", "c");
pub(crate) const DJ: Phoneme = consonant("dʲ", "dʲ");
pub(crate) const TSH: Phoneme = consonant("tʲ", "tʃ");
pub(crate) const DZH: Phoneme = consonant("dʲ", "dʒ");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const KJ: Phoneme = consonant("c", "kʲ");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
/// The g of ag, said in Munster only before a vowel (ag ól ɛɡ ˈoːl; ag teacht ɛ ˈtʲaxt, Sjoestedt
/// 1938, § 90: e tʹαχt), and always written in eSpeak NG's notation (ag teacht ɛɡ cˈAxt̪).
pub(crate) const G_BEFORE_VOWEL: Phoneme = consonant("", "ɡ");
pub(crate) const GJ: Phoneme = consonant("ɟ", "ɡʲ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
/// bh and mh before a broad vowel at the start of a word (mhaith wˈA).
pub(crate) const W: Phoneme = consonant("w", "w");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
/// ch: broad x, slender ç (chéile çʲˈeːlʲə).
pub(crate) const X: Phoneme = consonant("x", "x");
pub(crate) const XJ: Phoneme = consonant("ç", "çʲ");
/// eSpeak NG writes a slender consonant without its ʲ in some places: l, n and d before e, é
/// and ei at the start of a word (leis lˈɛʃ, de dˈɛ), n, k, g and ch at the end of one (sin ʃˈɪn,
/// chonaic xˈɔnɪk, tháinig hˈɑːnɪɡ).
pub(crate) const LJ_PLAIN: Phoneme = consonant("ʎ", "l");
pub(crate) const NJ_PLAIN: Phoneme = consonant("ɲ", "n");
pub(crate) const DJ_PLAIN: Phoneme = consonant("dʲ", "d");
pub(crate) const KJ_PLAIN: Phoneme = consonant("c", "k");
pub(crate) const GJ_PLAIN: Phoneme = consonant("ɟ", "ɡ");
pub(crate) const XJ_PLAIN: Phoneme = consonant("ç", "x");
/// The d of d' (d'ól dˈoːl), broad.
pub(crate) const D_PLAIN: Phoneme = consonant("d̪", "d");
/// A slender labial before a back vowel, marked in eSpeak NG's notation (fiú fʲˈuː).
pub(crate) const BJ: Phoneme = consonant("bʲ", "bʲ");
pub(crate) const VJ: Phoneme = consonant("vʲ", "vʲ");
pub(crate) const FJ: Phoneme = consonant("fʲ", "fʲ");
pub(crate) const MJ: Phoneme = consonant("mʲ", "mʲ");
pub(crate) const PJ: Phoneme = consonant("pʲ", "pʲ");
/// dh and gh: broad ɣ, which eSpeak NG writes ʁ for gh (ghá ʁˈɑː; dhá ɣˈɑː), slender j; a
/// final dh χ (-adh).
pub(crate) const DH: Phoneme = consonant("ɣ", "ɣ");
pub(crate) const GH: Phoneme = consonant("ɣ", "ʁ");
pub(crate) const DH_FINAL: Phoneme = consonant("x", "χ");
pub(crate) const J: Phoneme = consonant("j", "j");
pub(crate) const H: Phoneme = consonant("h", "h");
pub(crate) const HJ: Phoneme = consonant("ç", "hʲ");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NJ: Phoneme = consonant("ɲ", "nʲ");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const LJ: Phoneme = consonant("ʎ", "lʲ");
/// r: ɹ at the start of a word and between vowels, r elsewhere.
pub(crate) const R: Phoneme = consonant("r", "r");
/// A slender r, plain in eSpeak NG's notation (fuair fˈuər).
pub(crate) const RJ: Phoneme = consonant("ɾʲ", "r");
pub(crate) const R_APPROX: Phoneme = consonant("ɾ", "ɹ");
/// The glide after a long vowel before a slender consonant (dóibh d̪ˈoːi̯v).
pub(crate) const GLIDE: Phoneme = consonant("i̯", "i̯");

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop or f before l or
/// r, s before a stop, and a stop before a glide.
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let stop = |phoneme: &Phoneme| [P, B, T, D, TJ, DJ, TSH, DZH, K, KJ, G, GJ, F, V].contains(phoneme);
    match cluster {
        [_] => true,
        [first, second] if [L, LJ, R, RJ, R_APPROX].contains(second) => stop(first),
        [first, second] => [S, SH].contains(first) && [P, T, TJ, TSH, K, KJ].contains(second),
        _ => false,
    }
}

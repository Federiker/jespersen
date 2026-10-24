//! Danish phonemes in plain IPA and eSpeak NG's notation.
//!
//! eSpeak NG writes a glottal stop before most of its Danish vowels (hus hˈʔus, ikke ˈʔekə), and a
//! question mark for it before ɑ (tak tˈ?ɑk); it is part of how those vowels are written, not the
//! stød where Danish has it, so plain IPA leaves it out.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

/// a, and ɑ before r (far fˈɑ) and before m, ng, k, g, v and j (tak tˈ?ɑk, dag is a).
pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const A_BACK: Phoneme = vowel("ɑ", "ɑ");
pub(crate) const A_DARK: Phoneme = vowel("ɑ", "?ɑ");
/// A short e, ε (Greek) in eSpeak NG's notation (men mˈεn), a long one (hele hˈʔelə), æ (være
/// ʋˈɛʔʌ) and e before r (der dˈεɐ̯).
pub(crate) const E: Phoneme = vowel("ɛ", "ε");
pub(crate) const E_LONG: Phoneme = vowel("eː", "ʔe");
pub(crate) const AE: Phoneme = vowel("ɛː", "ɛ");
/// An unstressed e.
pub(crate) const SCHWA: Phoneme = vowel("ə", "ə");
/// The unstressed -er and -re, ʔʌ in eSpeak NG's notation (efter ˈεftʔʌ, være).
pub(crate) const ER: Phoneme = vowel("ɐ", "ʔʌ");
/// i, long or short (tid, lille), and a short i said e (ikke ˈʔekə).
pub(crate) const I: Phoneme = vowel("i", "ʔi");
pub(crate) const I_LOW: Phoneme = vowel("e", "ʔe");
/// o: long (stod), short ʌ (op ˈʔʌp), and ɒ before r and in ov and og (bort, over).
pub(crate) const O: Phoneme = vowel("oː", "ʔo");
pub(crate) const O_SHORT: Phoneme = vowel("ʌ", "ʔʌ");
pub(crate) const O_OPEN: Phoneme = vowel("ɒ", "ɒ");
/// o before r and a consonant, the r in it (kort kˈɒɒt).
pub(crate) const O_RR: Phoneme = vowel("ɒː", "ɒɒ");
/// The e of be- and ge-, ʔe in eSpeak NG's notation though unstressed (begynde bʔeɡˈʔœnə), and
/// the ʌ of a -re after a long e (studere stuðˈʔeʌ).
pub(crate) const E_PREFIX: Phoneme = vowel("e", "ʔe").misaki("e:");
pub(crate) const ER_AFTER_E: Phoneme = vowel("ɐ", "ʌ");
/// The unstressed i, e, o, u and y, written without the glottal stop (universitet; but the e of
/// short i keeps it, erik ˈʔeʁʔek).
pub(crate) const I_PLAIN: Phoneme = vowel("i", "i");
pub(crate) const E_PLAIN: Phoneme = vowel("eː", "e");
pub(crate) const O_PLAIN: Phoneme = vowel("oː", "o");
pub(crate) const U_PLAIN: Phoneme = vowel("u", "u");
pub(crate) const Y_PLAIN: Phoneme = vowel("y", "y");
/// The short o of a function word left without stress at its end (og ʌ).
pub(crate) const O_SHORT_PLAIN: Phoneme = vowel("ʌ", "ʌ");
/// u, and u before n and m (kunde kˈɒnə).
pub(crate) const U: Phoneme = vowel("u", "ʔu");
pub(crate) const U_LOW: Phoneme = vowel("ɔ", "ɒ");
/// y, long (lys) and short (synd sˈʔœn); ø.
pub(crate) const Y: Phoneme = vowel("y", "ʔy");
pub(crate) const Y_SHORT: Phoneme = vowel("ø", "ʔœ");
pub(crate) const OE: Phoneme = vowel("ø", "ʔœ");
/// å, long ɒ (år ˈɒ) and short ʌ (hånd hˈʔʌn).
pub(crate) const AA: Phoneme = vowel("ɔː", "ɒ");
pub(crate) const AA_SHORT: Phoneme = vowel("ɔ", "ʔʌ");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
/// The soft d after a vowel (gade ɡˈaðə).
pub(crate) const DH: Phoneme = consonant("ð", "ð");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("ʋ", "ʋ");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const SH: Phoneme = consonant("ɕ", "ʃ");
pub(crate) const H: Phoneme = consonant("h", "h");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
/// r at the start and after a consonant, ʁ; after a vowel ɐ̯ (der dˈεɐ̯), and so after b (brød
/// bɐ̯ˈʔœð in eSpeak NG's notation).
pub(crate) const R: Phoneme = consonant("ʁ", "ʁ");
pub(crate) const R_VOCALIC: Phoneme = consonant("ɐ̯", "ɐ̯");
pub(crate) const J: Phoneme = consonant("j", "j");
/// v and g after a vowel (tyve tˈyːw, bog bˈɒw).
pub(crate) const W: Phoneme = consonant("w", "w");

/// The long vowels NST's lexicon writes that the rules do not, and their forms without stress;
/// eSpeak NG's notation writes them as the short ones.
pub(crate) const I_LONG: Phoneme = vowel("iː", "ʔi");
pub(crate) const I_LONG_PLAIN: Phoneme = vowel("iː", "i");
pub(crate) const A_LONG: Phoneme = vowel("aː", "a");
pub(crate) const A_BACK_LONG: Phoneme = vowel("ɑː", "ɑ");
pub(crate) const Y_LONG: Phoneme = vowel("yː", "ʔy");
pub(crate) const Y_LONG_PLAIN: Phoneme = vowel("yː", "y");
pub(crate) const OE_LONG: Phoneme = vowel("øː", "ʔœ");
pub(crate) const U_LONG: Phoneme = vowel("uː", "ʔu");
pub(crate) const U_LONG_PLAIN: Phoneme = vowel("uː", "u");
pub(crate) const OPEN_OE: Phoneme = vowel("œ", "ʔœ");
pub(crate) const OPEN_OE_LONG: Phoneme = vowel("œː", "ʔœ");
/// A short o (bonde), and without stress.
pub(crate) const O_CLOSE: Phoneme = vowel("o", "ʔo");
pub(crate) const O_CLOSE_PLAIN: Phoneme = vowel("o", "o");
pub(crate) const O_OPEN_LONG: Phoneme = vowel("ɒː", "ɒ");
/// The unaspirated b, d and g NST writes where the spelling has p, t and k (efter ˈɛfdɐ, ikke):
/// eSpeak NG's notation writes the letter (ˈεftʔʌ).
pub(crate) const B_SPELT_P: Phoneme = consonant("b", "p");
pub(crate) const D_SPELT_T: Phoneme = consonant("d", "t");
pub(crate) const G_SPELT_K: Phoneme = consonant("ɡ", "k");
/// r after b, ʁ, which eSpeak NG writes ɐ̯ (brød bɐ̯ˈʔœð).
pub(crate) const R_AFTER_B: Phoneme = consonant("ʁ", "ɐ̯");
/// Stød, after the vowel or the consonant that has it (hund ˈhunˀ); eSpeak NG writes none.
pub(crate) const STOD: Phoneme = consonant("ˀ", "");

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop or f before l, r
/// or j, s before a stop, m, n, l or v, str, spr and skr, and k or t before v (kvinde).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let obstruent = |phoneme: &Phoneme| [P, B, T, D, K, G, F, V].contains(phoneme);
    match cluster {
        [_] => true,
        [first, second] if [L, R, R_VOCALIC, J].contains(second) => obstruent(first),
        [first, second] if *second == V => [K, S, T, D].contains(first),
        [first, second] => *first == S && [P, T, K, M, N, L].contains(second),
        [first, second, third] => *first == S && [P, T, K].contains(second) && *third == R,
        _ => false,
    }
}

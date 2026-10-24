//! Catalan (central) phonemes in plain IPA and eSpeak NG's notation.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
/// Closed and open e, closed and open o: only under the stress.
pub(crate) const E: Phoneme = vowel("e", "e");
pub(crate) const E_OPEN: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const O: Phoneme = vowel("o", "o");
pub(crate) const O_OPEN: Phoneme = vowel("ɔ", "ɔ");
/// Unstressed a and e.
pub(crate) const SCHWA: Phoneme = vowel("ə", "ə");
pub(crate) const I: Phoneme = vowel("i", "i");
pub(crate) const U: Phoneme = vowel("u", "u");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
/// b, d and g after a vowel or a continuant: β ð ɣ in eSpeak NG's notation.
pub(crate) const B_SOFT: Phoneme = consonant("b", "β");
pub(crate) const D_SOFT: Phoneme = consonant("d", "ð");
pub(crate) const G_SOFT: Phoneme = consonant("ɡ", "ɣ");
/// b and g that stay stops after a vowel: the doubled b of poble pˈɔbblə, the g of exemple
/// əɡzˈemplə (marked apart in misaki's notation, which Catalan does not use, to tell them from b
/// and g).
pub(crate) const B_HARD: Phoneme = consonant("b", "b").misaki("b:");
pub(crate) const G_HARD: Phoneme = consonant("ɡ", "ɡ").misaki("ɡ:");
/// f voiced before a vowel or a voiced consonant starting the next word.
pub(crate) const V: Phoneme = consonant("v", "v");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const Z: Phoneme = consonant("z", "z");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const ZH: Phoneme = consonant("ʒ", "ʒ");
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const DZH: Phoneme = consonant("dʒ", "dʒ");
pub(crate) const DZ: Phoneme = consonant("dz", "dz");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NY: Phoneme = consonant("ɲ", "ɲ");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
/// n before f, ɱ in eSpeak NG's notation (en fusta əɱ fˈustə).
pub(crate) const N_LABIODENTAL: Phoneme = consonant("n", "ɱ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const LL: Phoneme = consonant("ʎ", "ʎ");
/// r trilled at the start, after n, l and s, doubled and before a consonant; a tap between
/// vowels and after a consonant.
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const R_TAP: Phoneme = consonant("ɾ", "ɾ");
pub(crate) const J: Phoneme = consonant("j", "j");
pub(crate) const W: Phoneme = consonant("w", "w");
/// The final c of -nc and p of -mp, said only before a vowel in the next word (cinc anys sˈiŋk
/// ˈaɲʃ, cinc dies sˈiŋ dˈiəs); silent, and marked apart in misaki's notation, until then.
pub(crate) const LATENT_K: Phoneme = consonant("", "").misaki("k?");
pub(crate) const LATENT_P: Phoneme = consonant("", "").misaki("p?");

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop or f before l or
/// r (llibre, platja is two), or a consonant before w (quatre, aigua).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let obstruent = |phoneme: &Phoneme| [P, B, B_SOFT, T, D, D_SOFT, K, G, G_SOFT, F].contains(phoneme);
    match cluster {
        [_] => true,
        [first, second] if *second == W => *first != J,
        [first, second] => obstruent(first) && [L, R_TAP].contains(second),
        [first, second, third] => obstruent(first) && [L, R_TAP].contains(second) && *third == W,
        _ => false,
    }
}

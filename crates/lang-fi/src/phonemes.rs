//! Finnish phonemes. A long vowel or a diphthong is one nucleus, so each is one phoneme (aː, ai,
//! uo). Plain IPA writes a as ɑ and long consonants doubled (ˈkukkɑ); eSpeak NG's notation writes a
//! as a, a long stop with ː (kˈukːa) and other long consonants doubled (kˈissa).

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("ɑ", "a");
pub(crate) const E: Phoneme = vowel("e", "e");
pub(crate) const I: Phoneme = vowel("i", "i");
/// Unstressed short i, which eSpeak NG writes ɪ (Suomi sˈuomɪ).
pub(crate) const I_LAX: Phoneme = vowel("i", "ɪ");
/// Unstressed long i, which eSpeak NG writes ɪː (Helsinkiin hˈelsɪŋkɪːn).
pub(crate) const II_LAX: Phoneme = vowel("iː", "ɪː");
pub(crate) const O: Phoneme = vowel("o", "o");
pub(crate) const U: Phoneme = vowel("u", "u");
pub(crate) const Y: Phoneme = vowel("y", "y");
pub(crate) const AE: Phoneme = vowel("æ", "æ");
pub(crate) const OE: Phoneme = vowel("ø", "ø");

/// Long vowels and diphthongs, by their letters (å is read o).
pub(crate) const NUCLEI: [(&str, Phoneme); 25] = [
    ("aa", vowel("ɑː", "aː")),
    ("ee", vowel("eː", "eː")),
    ("ii", vowel("iː", "iː")),
    ("oo", vowel("oː", "oː")),
    ("uu", vowel("uː", "uː")),
    ("yy", vowel("yː", "yː")),
    ("ää", vowel("æː", "æː")),
    ("öö", vowel("øː", "øː")),
    ("ai", vowel("ɑi", "ai")),
    ("ei", vowel("ei", "ei")),
    ("oi", vowel("oi", "oi")),
    ("ui", vowel("ui", "ui")),
    ("yi", vowel("yi", "yi")),
    ("äi", vowel("æi", "æi")),
    ("öi", vowel("øi", "øi")),
    ("au", vowel("ɑu", "au")),
    ("eu", vowel("eu", "eu")),
    ("iu", vowel("iu", "iu")),
    ("ou", vowel("ou", "ou")),
    ("ey", vowel("ey", "ey")),
    ("äy", vowel("æy", "æy")),
    ("öy", vowel("øy", "øy")),
    ("ie", vowel("ie", "ie")),
    ("uo", vowel("uo", "uo")),
    ("yö", vowel("yø", "yø")),
];

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const ZH: Phoneme = consonant("ʒ", "ʒ");
pub(crate) const TS: Phoneme = consonant("ts", "ts");
pub(crate) const H: Phoneme = consonant("h", "h");
pub(crate) const V: Phoneme = consonant("ʋ", "v");
pub(crate) const J: Phoneme = consonant("j", "j");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
/// n before k or g, also across words, ŋ in eSpeak NG's notation (talon katolla tˈaloŋ kˈatolla).
pub(crate) const N_VELAR: Phoneme = consonant("n", "ŋ");
/// n before p or b across words, m in eSpeak NG's notation (niin paljon nˈiːm pˈaljon).
pub(crate) const N_LABIAL: Phoneme = consonant("n", "m");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const R: Phoneme = consonant("r", "r");

/// The first half of a long stop, written nothing in eSpeak NG's notation, whose second half
/// carries ː.
pub(crate) const P_FIRST: Phoneme = consonant("p", "");
pub(crate) const T_FIRST: Phoneme = consonant("t", "");
pub(crate) const K_FIRST: Phoneme = consonant("k", "");
pub(crate) const P_LONG: Phoneme = consonant("p", "pː");
pub(crate) const T_LONG: Phoneme = consonant("t", "tː");
pub(crate) const K_LONG: Phoneme = consonant("k", "kː");

/// Whether `cluster` can start a syllable inside a word: one consonant, not the first half of a
/// long stop (a word's first syllable takes all its consonants, as in loans: stressi).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    matches!(cluster, [single] if ![P_FIRST, T_FIRST, K_FIRST].contains(single))
}

//! Hungarian phonemes in plain IPA and eSpeak NG's notation.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

/// a, ɑ in eSpeak NG's notation; á.
pub(crate) const A: Phoneme = vowel("ɒ", "ɑ");
pub(crate) const AA: Phoneme = vowel("aː", "aː");
pub(crate) const E: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const EE: Phoneme = vowel("eː", "eː");
pub(crate) const I: Phoneme = vowel("i", "i");
pub(crate) const II: Phoneme = vowel("iː", "iː");
pub(crate) const O: Phoneme = vowel("o", "o");
pub(crate) const OO: Phoneme = vowel("oː", "oː");
/// ö and ő.
pub(crate) const OE: Phoneme = vowel("ø", "ø");
pub(crate) const OEOE: Phoneme = vowel("øː", "øː");
pub(crate) const U: Phoneme = vowel("u", "u");
pub(crate) const UU: Phoneme = vowel("uː", "uː");
/// ü and ű.
pub(crate) const UE: Phoneme = vowel("y", "y");
pub(crate) const UEUE: Phoneme = vowel("yː", "yː");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
/// ty and gy.
pub(crate) const TY: Phoneme = consonant("c", "c");
pub(crate) const GY: Phoneme = consonant("ɟ", "ɟ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
/// sz and s, z and zs.
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const Z: Phoneme = consonant("z", "z");
pub(crate) const ZH: Phoneme = consonant("ʒ", "ʒ");
/// c, cs, dz and dzs.
pub(crate) const TS: Phoneme = consonant("ts", "ts");
pub(crate) const CH: Phoneme = consonant("tʃ", "tʃ");
pub(crate) const DZ: Phoneme = consonant("dz", "dz");
pub(crate) const DZH: Phoneme = consonant("dʒ", "dʒ");
pub(crate) const H: Phoneme = consonant("h", "h");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
/// ny.
pub(crate) const NY: Phoneme = consonant("ɲ", "ɲ");
pub(crate) const L: Phoneme = consonant("l", "l");
/// j and ly.
pub(crate) const J: Phoneme = consonant("j", "j");
pub(crate) const R: Phoneme = consonant("r", "r");
/// The length of a doubled stop or affricate, which eSpeak NG writes ː (kettő kˈɛtːøː); a doubled
/// fricative, nasal or liquid it writes twice (asszony ˈɑssoɲ).
pub(crate) const LONG: Phoneme = consonant("ː", "ː");

/// The phoneme said again for a doubled fricative, nasal or liquid: written ː in plain IPA and
/// as the consonant itself in eSpeak NG's notation.
pub(crate) fn doubled(phoneme: Phoneme) -> Phoneme {
    match phoneme {
        phoneme if phoneme == S => consonant("ː", "s"),
        phoneme if phoneme == SH => consonant("ː", "ʃ"),
        phoneme if phoneme == Z => consonant("ː", "z"),
        phoneme if phoneme == ZH => consonant("ː", "ʒ"),
        phoneme if phoneme == F => consonant("ː", "f"),
        phoneme if phoneme == V => consonant("ː", "v"),
        phoneme if phoneme == H => consonant("ː", "h"),
        phoneme if phoneme == M => consonant("ː", "m"),
        phoneme if phoneme == N => consonant("ː", "n"),
        phoneme if phoneme == NY => consonant("ː", "ɲ"),
        phoneme if phoneme == L => consonant("ː", "l"),
        phoneme if phoneme == J => consonant("ː", "j"),
        phoneme if phoneme == R => consonant("ː", "r"),
        _ => LONG,
    }
}

/// Whether `cluster` can start a syllable inside a word: one consonant, or a stop or f before r
/// or l (brindzsa, Budapest is two).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    match cluster {
        [_] => true,
        [first, second] => [P, B, T, D, K, G, F].contains(first) && [R, L].contains(second),
        _ => false,
    }
}

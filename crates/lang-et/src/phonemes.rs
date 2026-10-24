//! Estonian phonemes in plain IPA and eSpeak NG's notation.

use jespersen_phonemes::{Notation, Phoneme};

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

/// The vowels: short, long (two letters) and overlong (two letters in a word of one syllable,
/// maa mˈaaː), as plain IPA and eSpeak NG write them.
pub(crate) const VOWELS: [(char, [Phoneme; 3]); 9] = [
    ('a', [vowel("a", "a"), vowel("aː", "aː"), vowel("aːː", "aaː")]),
    ('e', [vowel("e", "e"), vowel("eː", "eː"), vowel("eːː", "eeː")]),
    ('i', [vowel("i", "i"), vowel("iː", "iː"), vowel("iːː", "iiː")]),
    ('o', [vowel("o", "o"), vowel("oː", "oː"), vowel("oːː", "ooː")]),
    ('u', [vowel("u", "u"), vowel("uː", "uː"), vowel("uːː", "uuː")]),
    ('õ', [vowel("ɤ", "ɵ"), vowel("ɤː", "ɵː"), vowel("ɤːː", "ɵɵː")]),
    ('ä', [vowel("æ", "æ"), vowel("æː", "æː"), vowel("æːː", "ææː")]),
    ('ö', [vowel("ø", "ø"), vowel("øː", "øː"), vowel("øːː", "øøː")]),
    ('ü', [vowel("y", "y"), vowel("yː", "yː"), vowel("yːː", "yyː")]),
];

/// The second vowel of a diphthong (ai, ei, õu), said within the syllable of the first: short,
/// and long at the end of a word of one syllable (ei ˌeiː).
pub(crate) const OFFGLIDES: [(char, [Phoneme; 2]); 9] = [
    ('a', [consonant("a̯", "a"), consonant("a̯ː", "aː")]),
    ('e', [consonant("e̯", "e"), consonant("e̯ː", "eː")]),
    ('i', [consonant("i̯", "i"), consonant("i̯ː", "iː")]),
    ('o', [consonant("o̯", "o"), consonant("o̯ː", "oː")]),
    ('u', [consonant("u̯", "u"), consonant("u̯ː", "uː")]),
    ('õ', [consonant("ɤ̯", "ɵ"), consonant("ɤ̯ː", "ɵː")]),
    ('ä', [consonant("æ̯", "æ"), consonant("æ̯ː", "æː")]),
    ('ö', [consonant("ø̯", "ø"), consonant("ø̯ː", "øː")]),
    ('ü', [consonant("y̯", "y"), consonant("y̯ː", "yː")]),
];

/// a, e, i and u in the second syllable after a short first one, which eSpeak NG writes ɑ, ɛ, ɪ
/// and ʊ (maja mˈajɑ, tere tˈerɛ, kirik kˈirɪk, koju kˈojʊ).
pub(crate) const A_AFTER_SHORT: Phoneme = vowel("a", "ɑ");
pub(crate) const E_AFTER_SHORT: Phoneme = vowel("e", "ɛ");
pub(crate) const I_AFTER_SHORT: Phoneme = vowel("i", "ɪ");
pub(crate) const U_AFTER_SHORT: Phoneme = vowel("u", "ʊ");

pub(crate) const P: Phoneme = consonant("p", "p");
/// b, d and g, voiceless and unaspirated in Estonian, written as eSpeak NG does.
pub(crate) const B: Phoneme = consonant("b̥", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d̥", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ̊", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const Z: Phoneme = consonant("z", "z");
pub(crate) const SH: Phoneme = consonant("ʃ", "ʃ");
pub(crate) const ZH: Phoneme = consonant("ʒ", "ʒ");
pub(crate) const H: Phoneme = consonant("h", "h");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
/// n before k and g.
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const J: Phoneme = consonant("j", "j");
/// The glide between a long u and the vowel after it (uue ˈuːwe).
pub(crate) const W: Phoneme = consonant("w", "w");
/// l, n, s, d and t palatalised before i and j (neli nˈeʎɪ, kassi kˈas^s^i, padi pˈad^ɪ).
pub(crate) const L_PALATAL: Phoneme = consonant("lʲ", "ʎ");
pub(crate) const N_PALATAL: Phoneme = consonant("nʲ", "ɲ");
pub(crate) const S_PALATAL: Phoneme = consonant("sʲ", "s^");
pub(crate) const D_PALATAL: Phoneme = consonant("d̥ʲ", "d^");
pub(crate) const T_PALATAL: Phoneme = consonant("tʲ", "t^");
/// The length of a long stop, which eSpeak NG writes ː (lõpp lˈɵpː).
pub(crate) const LONG: Phoneme = consonant("ː", "ː");

/// The phoneme said again for a long fricative, nasal or liquid: ː in plain IPA and the consonant
/// itself in eSpeak NG's notation (kalla kˈalla); a stop takes [`LONG`].
pub(crate) fn doubled(phoneme: Phoneme) -> Phoneme {
    match phoneme {
        phoneme if phoneme == S => consonant("ː", "s"),
        phoneme if phoneme == S_PALATAL => consonant("ː", "s^"),
        phoneme if phoneme == SH => consonant("ː", "ʃ"),
        phoneme if phoneme == F => consonant("ː", "f"),
        phoneme if phoneme == V => consonant("ː", "v"),
        phoneme if phoneme == H => consonant("ː", "h"),
        phoneme if phoneme == M => consonant("ː", "m"),
        phoneme if phoneme == N => consonant("ː", "n"),
        phoneme if phoneme == N_PALATAL => consonant("ː", "ɲ"),
        phoneme if phoneme == L => consonant("ː", "l"),
        phoneme if phoneme == L_PALATAL => consonant("ː", "ʎ"),
        phoneme if phoneme == R => consonant("ː", "r"),
        phoneme if phoneme == J => consonant("ː", "j"),
        _ => LONG,
    }
}

/// Whether `cluster` can start a syllable inside a word: one consonant only, the rest closing
/// the syllable before (kaks-kümmend, õp-pi-ma); a diphthong's second vowel stays with its first.
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let offglide = |phoneme: &Phoneme| OFFGLIDES.iter().any(|(_, forms)| forms.contains(phoneme));
    match cluster {
        // A length mark stays with the consonant it lengthens.
        [single] => !offglide(single) && single.symbol(Notation::Ipa) != "ː",
        _ => false,
    }
}

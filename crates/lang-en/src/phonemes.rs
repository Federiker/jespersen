//! American English phonemes, with their plain IPA (broad, phonemic) and their eSpeak NG symbols.
//! Where eSpeak NG writes an allophone (a flap, a reduced initial vowel), the phoneme has its own
//! constant whose plain IPA is the phoneme's.

use jespersen_phonemes::{Phoneme, Stress};

// Vowels.
pub const LOT: Phoneme = Phoneme::vowel("ɑ", "ɑː");
pub const TRAP: Phoneme = Phoneme::vowel("æ", "æ");
pub const STRUT: Phoneme = Phoneme::vowel("ʌ", "ʌ");
pub const SCHWA: Phoneme = Phoneme::vowel("ə", "ə");
/// A schwa eSpeak NG writes ɐ: at the start of a word (ago, about) and in some weak forms (has,
/// than).
pub const SCHWA_OPEN: Phoneme = Phoneme::vowel("ə", "ɐ");
pub const THOUGHT: Phoneme = Phoneme::vowel("ɔ", "ɔː");
pub const MOUTH: Phoneme = Phoneme::vowel("aʊ", "aʊ");
pub const PRICE: Phoneme = Phoneme::vowel("aɪ", "aɪ");
pub const DRESS: Phoneme = Phoneme::vowel("ɛ", "ɛ");
pub const NURSE: Phoneme = Phoneme::vowel("ɝ", "ɜː");
pub const LETTER: Phoneme = Phoneme::vowel("ɚ", "ɚ");
pub const FACE: Phoneme = Phoneme::vowel("eɪ", "eɪ");
pub const KIT: Phoneme = Phoneme::vowel("ɪ", "ɪ");
pub const FLEECE: Phoneme = Phoneme::vowel("i", "iː");
/// Unstressed FLEECE (happY, previous), short in eSpeak NG.
pub const HAPPY: Phoneme = Phoneme::vowel("i", "i");
pub const GOAT: Phoneme = Phoneme::vowel("oʊ", "oʊ");
pub const CHOICE: Phoneme = Phoneme::vowel("ɔɪ", "ɔɪ");
pub const FOOT: Phoneme = Phoneme::vowel("ʊ", "ʊ");
pub const GOOSE: Phoneme = Phoneme::vowel("u", "uː");

// Function words' vowels, reduced in plain IPA (the textbook weak forms) and kept full in eSpeak
// NG's notation (docs/decisions.md, decision 27); see function_words.rs.
pub const TRAP_WEAK: Phoneme = Phoneme::vowel("ə", "æ");
pub const STRUT_WEAK: Phoneme = Phoneme::vowel("ə", "ʌ");
pub const FOOT_WEAK: Phoneme = Phoneme::vowel("ə", "ʊ");
pub const NURSE_WEAK: Phoneme = Phoneme::vowel("ɚ", "ɜː");
/// The weak vowel of for and or, which eSpeak NG writes with its r.
pub const NORTH_WEAK: Phoneme = Phoneme::vowel("ɚ", "ɔːɹ");
/// The weak vowel of are.
pub const START_WEAK: Phoneme = Phoneme::vowel("ɚ", "ɑːɹ");
/// The weak vowel of your.
pub const CURE_WEAK: Phoneme = Phoneme::vowel("ɚ", "ʊɹ");
/// The weak vowel of their and there.
pub const SQUARE_WEAK: Phoneme = Phoneme::vowel("ɚ", "ɛɹ");
/// The vowel of "the" before a vowel (the apple).
pub const THE_BEFORE_VOWEL: Phoneme = Phoneme::vowel("i", "ɪ");
/// The vowel of on, which eSpeak NG writes short.
pub const ON: Phoneme = Phoneme::vowel("ɑ", "ɔ");
/// Unstressed GOOSE in into, and "to" before a vowel, which eSpeak NG writes ʊ.
pub const GOOSE_WEAK: Phoneme = Phoneme::vowel("u", "ʊ");

// Allophones eSpeak NG's notation writes in running text; see rules.rs.
/// lettER before a vowel, linked with an r (butter and, history).
pub const LETTER_LINKING: Phoneme = Phoneme::vowel("ɚ", "ɚɹ");
/// Unstressed KIT in a first syllable before stress (behind, return) and in -ted, -ded.
pub const KIT_REDUCED: Phoneme = Phoneme::vowel("ɪ", "ᵻ");
/// A schwa in -ted, -ded (folded).
pub const SCHWA_REDUCED: Phoneme = Phoneme::vowel("ə", "ᵻ");
/// Unstressed FLEECE before a consonant in the same word (anything, happiness).
pub const HAPPY_CLOSED: Phoneme = Phoneme::vowel("i", "ɪ");

// Consonants.
pub const B: Phoneme = Phoneme::consonant("b", "b");
pub const CH: Phoneme = Phoneme::consonant("tʃ", "tʃ");
pub const D: Phoneme = Phoneme::consonant("d", "d");
pub const DH: Phoneme = Phoneme::consonant("ð", "ð");
pub const F: Phoneme = Phoneme::consonant("f", "f");
pub const G: Phoneme = Phoneme::consonant("ɡ", "ɡ");
pub const H: Phoneme = Phoneme::consonant("h", "h");
pub const JH: Phoneme = Phoneme::consonant("dʒ", "dʒ");
pub const K: Phoneme = Phoneme::consonant("k", "k");
pub const L: Phoneme = Phoneme::consonant("l", "l");
pub const M: Phoneme = Phoneme::consonant("m", "m");
pub const N: Phoneme = Phoneme::consonant("n", "n");
pub const NG: Phoneme = Phoneme::consonant("ŋ", "ŋ");
pub const P: Phoneme = Phoneme::consonant("p", "p");
pub const R: Phoneme = Phoneme::consonant("ɹ", "ɹ");
pub const S: Phoneme = Phoneme::consonant("s", "s");
pub const SH: Phoneme = Phoneme::consonant("ʃ", "ʃ");
pub const T: Phoneme = Phoneme::consonant("t", "t");
/// /t/ flapped between vowels before an unstressed one (butter, party), as eSpeak NG writes it.
pub const T_FLAP: Phoneme = Phoneme::consonant("t", "ɾ");
/// /t/ before a syllabic n (button), a glottal stop in eSpeak NG's notation.
pub const T_GLOTTAL: Phoneme = Phoneme::consonant("t", "ʔ");
/// The schwa of a syllabic n (button): written in plain IPA, part of the n̩ in eSpeak NG's notation.
pub const SCHWA_SYLLABIC: Phoneme = Phoneme::vowel("ə", "");
/// A syllabic n (button).
pub const N_SYLLABIC: Phoneme = Phoneme::consonant("n", "n̩");
pub const TH: Phoneme = Phoneme::consonant("θ", "θ");
pub const V: Phoneme = Phoneme::consonant("v", "v");
pub const W: Phoneme = Phoneme::consonant("w", "w");
pub const Y: Phoneme = Phoneme::consonant("j", "j");
pub const Z: Phoneme = Phoneme::consonant("z", "z");
pub const ZH: Phoneme = Phoneme::consonant("ʒ", "ʒ");

/// The American phoneme for ARPAbet phone `index` (see `arpabet::PHONES`) with CMUdict's stress
/// digit, before any contextual rule.
pub fn american(index: usize, stress: u8) -> Option<(Phoneme, Stress)> {
    let stress = match stress {
        1 => Stress::Primary,
        2 => Stress::Secondary,
        _ => Stress::Unstressed,
    };
    let unstressed = stress == Stress::Unstressed;
    let phoneme = match index {
        0 => LOT,
        1 => TRAP,
        2 if unstressed => SCHWA,
        2 => STRUT,
        3 => THOUGHT,
        4 => MOUTH,
        5 => PRICE,
        6 => B,
        7 => CH,
        8 => D,
        9 => DH,
        10 => DRESS,
        11 if unstressed => LETTER,
        11 => NURSE,
        12 => FACE,
        13 => F,
        14 => G,
        15 => H,
        16 => KIT,
        17 if unstressed => HAPPY,
        17 => FLEECE,
        18 => JH,
        19 => K,
        20 => L,
        21 => M,
        22 => N,
        23 => NG,
        24 => GOAT,
        25 => CHOICE,
        26 => P,
        27 => R,
        28 => S,
        29 => SH,
        30 => T,
        31 => TH,
        32 => FOOT,
        33 => GOOSE,
        34 => V,
        35 => W,
        36 => Y,
        37 => Z,
        38 => ZH,
        _ => return None,
    };
    Some((phoneme, stress))
}

// British English (RP), converted from the American phonemes (british.rs; docs/decisions.md,
// decision 20): plain IPA as RP is usually written, and the symbols eSpeak NG's British voice
// writes.
pub mod british {
    use jespersen_phonemes::Phoneme;

    pub const TRAP: Phoneme = Phoneme::vowel("æ", "a");
    /// BATH by notation (decision 28): broad in plain IPA, TRAP's vowel in eSpeak NG's notation.
    pub const BATH: Phoneme = Phoneme::vowel("ɑː", "a");
    pub const BATH_BROAD: Phoneme = Phoneme::vowel("ɑː", "ɑː");
    pub const BATH_FLAT: Phoneme = Phoneme::vowel("æ", "a");
    pub const LOT: Phoneme = Phoneme::vowel("ɒ", "ɒ");
    /// PALM and START: father, calm, car.
    pub const START: Phoneme = Phoneme::vowel("ɑː", "ɑː");
    /// THOUGHT, NORTH and FORCE: law, north, four.
    pub const NORTH: Phoneme = Phoneme::vowel("ɔː", "ɔː");
    pub const GOAT: Phoneme = Phoneme::vowel("əʊ", "əʊ");
    pub const GOOSE: Phoneme = Phoneme::vowel("uː", "uː");
    pub const FLEECE: Phoneme = Phoneme::vowel("iː", "iː");
    pub const NURSE: Phoneme = Phoneme::vowel("ɜː", "ɜː");
    pub const NEAR: Phoneme = Phoneme::vowel("ɪə", "iə");
    pub const SQUARE: Phoneme = Phoneme::vowel("eə", "eə");
    pub const CURE: Phoneme = Phoneme::vowel("ʊə", "ʊə");
    /// The vowel of one (and someone, anyone): RP ʌ, eSpeak NG's British voice ɒ.
    pub const ONE: Phoneme = Phoneme::vowel("ʌ", "ɒ");

    // Function words' vowels: weak in plain IPA, as eSpeak NG's British voice writes them.
    pub const TRAP_WEAK: Phoneme = Phoneme::vowel("ə", "a");
    /// of, was, from.
    pub const LOT_WEAK: Phoneme = Phoneme::vowel("ə", "ɒ");
    pub const NURSE_WEAK: Phoneme = Phoneme::vowel("ə", "ɜː");
    /// for, or, your.
    pub const NORTH_WEAK: Phoneme = Phoneme::vowel("ə", "ɔː");
    pub const START_WEAK: Phoneme = Phoneme::vowel("ə", "ɑː");
    pub const SQUARE_WEAK: Phoneme = Phoneme::vowel("ə", "eə");

    /// The r of a non-rhotic word end (butter, car, for): silent, unless the next word starts with
    /// a vowel.
    pub const R_SILENT: Phoneme = Phoneme::consonant("", "");
    /// That r before a vowel: written in eSpeak NG's notation (bˈʌtəɹ and), not in plain IPA.
    pub const R_LINKING: Phoneme = Phoneme::consonant("", "ɹ");
}

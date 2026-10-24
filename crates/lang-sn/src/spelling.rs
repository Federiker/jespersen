//! How a Shona word is said from its spelling: b and d implosive (ɓ, ɗ) but plain after a nasal
//! and in bv and dz, bh and dh breathy, mh and nh breathy nasals, v as ʋ and vh as v, h as ɦ, the
//! whistled sibilants sv, zv, tsv and dzv, ch tʃ, sh ʃ, zh ʒ, ny ɲ, ng' ŋ, n as ŋ before k and g,
//! and prenasalised consonants starting their syllable.
//!
//! Shona is a tone language whose tones are not written; the last syllable but one is the most
//! prominent, and takes the stress. eSpeak NG has no Shona voice, so its notation keeps to the
//! symbols Kokoro reads (decision 48): the implosives and breathy sounds as their plain ones, the
//! whistled sibilants as s or z with w.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

const A: Phoneme = vowel("a", "a");
const E: Phoneme = vowel("e", "e");
const I: Phoneme = vowel("i", "i");
const O: Phoneme = vowel("o", "o");
const U: Phoneme = vowel("u", "u");

/// b and d, implosive.
const B_IMPLOSIVE: Phoneme = consonant("ɓ", "b");
const D_IMPLOSIVE: Phoneme = consonant("ɗ", "d");
/// b and d after a nasal and in bv and dz.
const B: Phoneme = consonant("b", "b");
const D: Phoneme = consonant("d", "d");
/// bh and dh, breathy.
const BH: Phoneme = consonant("b̤", "b");
const DH: Phoneme = consonant("d̤", "d");
const CH: Phoneme = consonant("tʃ", "tʃ");
const F: Phoneme = consonant("f", "f");
const G: Phoneme = consonant("ɡ", "ɡ");
const H: Phoneme = consonant("ɦ", "h");
const J: Phoneme = consonant("dʒ", "dʒ");
const K: Phoneme = consonant("k", "k");
const L: Phoneme = consonant("l", "l");
const M: Phoneme = consonant("m", "m");
/// mh and nh, breathy nasals.
const MH: Phoneme = consonant("m̤", "m");
const N: Phoneme = consonant("n", "n");
const NH: Phoneme = consonant("n̤", "n");
const NG: Phoneme = consonant("ŋ", "ŋ");
const NY: Phoneme = consonant("ɲ", "ɲ");
const P: Phoneme = consonant("p", "p");
const R: Phoneme = consonant("ɾ", "ɾ");
const S: Phoneme = consonant("s", "s");
const SH: Phoneme = consonant("ʃ", "ʃ");
/// sv and zv, whistled: s and z with w in eSpeak NG's notation.
const SV: Phoneme = consonant("s͎", "sw");
const ZV: Phoneme = consonant("z͎", "zw");
const T: Phoneme = consonant("t", "t");
/// v, an approximant; vh, a fricative.
const V: Phoneme = consonant("ʋ", "ʋ");
const VH: Phoneme = consonant("v", "v");
const W: Phoneme = consonant("w", "w");
const Y: Phoneme = consonant("j", "j");
const Z: Phoneme = consonant("z", "z");
const ZH: Phoneme = consonant("ʒ", "ʒ");

/// Every cluster can start a syllable: Shona's syllables are open, and the consonants before a
/// vowel all begin its syllable (mbudzi, nzou).
fn legal_onset(_cluster: &[Phoneme]) -> bool {
    true
}

/// The sound of the consonant letters at `at` and how many letters they take; `None` for a letter
/// Shona does not read.
fn consonant_at(letters: &[char], at: usize) -> Option<(&'static [Phoneme], usize)> {
    let rest: String = letters[at..letters.len().min(at + 3)].iter().collect();
    let after_nasal = at > 0 && matches!(letters[at - 1], 'm' | 'n');
    Some(if rest.starts_with("tsv") {
        (&[T, SV], 3)
    } else if rest.starts_with("dzv") {
        (&[D, ZV], 3)
    } else if rest.starts_with("ng'") || rest.starts_with("ng’") {
        (&[NG], 3)
    } else {
        match rest.get(..2).unwrap_or(&rest) {
            "sv" => (&[SV], 2),
            "zv" => (&[ZV], 2),
            "bh" => (&[BH], 2),
            "dh" => (&[DH], 2),
            "mh" => (&[MH], 2),
            "nh" => (&[NH], 2),
            "vh" => (&[VH], 2),
            "ch" => (&[CH], 2),
            "sh" => (&[SH], 2),
            "zh" => (&[ZH], 2),
            "ny" => (&[NY], 2),
            "bv" => (&[B, VH], 2),
            "dz" => (&[D, Z], 2),
            // n is ŋ before k and g (nguva, nkuku).
            "nk" | "ng" => (&[NG], 1),
            _ => (
                match letters[at] {
                    'b' if after_nasal => &[B],
                    'b' => &[B_IMPLOSIVE],
                    'd' if after_nasal => &[D],
                    'd' => &[D_IMPLOSIVE],
                    'f' => &[F],
                    'g' => &[G],
                    'h' => &[H],
                    'j' => &[J],
                    'k' | 'c' | 'q' => &[K],
                    'l' => &[L],
                    'm' => &[M],
                    'n' => &[N],
                    'p' => &[P],
                    'r' => &[R],
                    's' => &[S],
                    't' => &[T],
                    'v' => &[V],
                    'w' => &[W],
                    'x' => &[K, S],
                    'y' => &[Y],
                    'z' => &[Z],
                    '\'' | '’' | '-' => &[],
                    _ => return None,
                },
                1,
            ),
        }
    })
}

/// The sounds of `letters`, in order; `None` for a letter Shona does not read.
fn sounds(letters: &[char]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let vowel = match letters[at] {
            'a' | 'á' | 'à' => Some(A),
            'e' | 'é' | 'è' => Some(E),
            'i' | 'í' | 'ì' => Some(I),
            'o' | 'ó' | 'ò' => Some(O),
            'u' | 'ú' | 'ù' => Some(U),
            _ => None,
        };
        if let Some(vowel) = vowel {
            sounds.push(vowel);
            at += 1;
            continue;
        }
        let (phonemes, step) = consonant_at(letters, at)?;
        sounds.extend_from_slice(phonemes);
        at += step;
    }
    Some(sounds)
}

/// How `word` is said, stressed on its last syllable but one; `None` for a letter Shona does not
/// read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let letters: Vec<char> = word.to_lowercase().chars().collect();
    let sounds = sounds(&letters)?;
    let vowels: Vec<usize> = (0..sounds.len()).filter(|&index| sounds[index].is_vowel()).collect();
    let &stressed = vowels.get(vowels.len().saturating_sub(2))?;
    let segments: Vec<Segment> = sounds
        .iter()
        .enumerate()
        .map(|(index, phoneme)| {
            Segment::new(
                *phoneme,
                if index == stressed {
                    Stress::Primary
                } else {
                    Stress::Unstressed
                },
            )
        })
        .collect();
    Some(syllabify(&segments, legal_onset))
}

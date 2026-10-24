//! How a Kinyarwanda word is said from its spelling: one sound per letter or group (c tʃ, cy c,
//! j ʒ, jy ɟ, sh ʃ, shy ʃc, ny ɲ, pf, ts), b as β except after m, r as ɾ, a doubled vowel long,
//! a consonant before w said with a velar (rw ɾɡw, tw tkw, mw mŋw), and a vowel long before a
//! nasal and a consonant (Rwanda ɾɡwaːnda, inka iːŋka).
//!
//! Kinyarwanda is a tone language without stress, and its tones are not written. For voices that
//! need a stress, the last syllable but one takes it, as in Swahili. eSpeak NG has no Kinyarwanda
//! voice, so its notation keeps to the symbols Kokoro reads (decision 48).

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
const A_LONG: Phoneme = vowel("aː", "aː");
const E_LONG: Phoneme = vowel("eː", "eː");
const I_LONG: Phoneme = vowel("iː", "iː");
const O_LONG: Phoneme = vowel("oː", "oː");
const U_LONG: Phoneme = vowel("uː", "uː");

/// b, a fricative except after m (mb).
const BETA: Phoneme = consonant("β", "β");
const B: Phoneme = consonant("b", "b");
const C: Phoneme = consonant("tʃ", "tʃ");
/// cy, a palatal stop.
const CY: Phoneme = consonant("c", "c");
const D: Phoneme = consonant("d", "d");
const F: Phoneme = consonant("f", "f");
const G: Phoneme = consonant("ɡ", "ɡ");
const H: Phoneme = consonant("h", "h");
const J: Phoneme = consonant("ʒ", "ʒ");
/// jy, a voiced palatal stop.
const JY: Phoneme = consonant("ɟ", "ɟ");
const K: Phoneme = consonant("k", "k");
const L: Phoneme = consonant("l", "l");
const M: Phoneme = consonant("m", "m");
const N: Phoneme = consonant("n", "n");
const NG: Phoneme = consonant("ŋ", "ŋ");
const NY: Phoneme = consonant("ɲ", "ɲ");
const P: Phoneme = consonant("p", "p");
const R: Phoneme = consonant("ɾ", "ɾ");
const S: Phoneme = consonant("s", "s");
const SH: Phoneme = consonant("ʃ", "ʃ");
const T: Phoneme = consonant("t", "t");
const V: Phoneme = consonant("v", "v");
const W: Phoneme = consonant("w", "w");
const Y: Phoneme = consonant("j", "j");
const Z: Phoneme = consonant("z", "z");

/// Every cluster can start a syllable: Kinyarwanda's syllables are open, and the consonants
/// before a vowel all begin its syllable (ndwi, mpwa).
fn legal_onset(_cluster: &[Phoneme]) -> bool {
    true
}

/// A vowel letter without a tone mark, should a text write one.
fn plain_vowel(letter: char) -> Option<char> {
    Some(match letter {
        'a' | 'á' | 'à' | 'â' | 'ǎ' => 'a',
        'e' | 'é' | 'è' | 'ê' | 'ě' => 'e',
        'i' | 'í' | 'ì' | 'î' | 'ǐ' => 'i',
        'o' | 'ó' | 'ò' | 'ô' | 'ǒ' => 'o',
        'u' | 'ú' | 'ù' | 'û' | 'ǔ' => 'u',
        _ => return None,
    })
}

fn short_vowel(letter: char) -> Phoneme {
    match letter {
        'a' => A,
        'e' => E,
        'i' => I,
        'o' => O,
        _ => U,
    }
}

fn long_vowel(phoneme: Phoneme) -> Phoneme {
    match phoneme {
        phoneme if phoneme == A => A_LONG,
        phoneme if phoneme == E => E_LONG,
        phoneme if phoneme == I => I_LONG,
        phoneme if phoneme == O => O_LONG,
        phoneme if phoneme == U => U_LONG,
        phoneme => phoneme,
    }
}

/// The sounds of `letters`, in order; `None` for a letter Kinyarwanda does not read.
pub(crate) fn sounds(letters: &[char]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let letter = letters[at];
        if let Some(plain) = plain_vowel(letter) {
            // A doubled vowel is one long vowel (baaba).
            let doubled = letters.get(at + 1).copied().and_then(plain_vowel) == Some(plain);
            let short = short_vowel(plain);
            sounds.push(if doubled { long_vowel(short) } else { short });
            at += if doubled { 2 } else { 1 };
            continue;
        }
        let rest: String = letters[at..letters.len().min(at + 3)].iter().collect();
        let (phonemes, step): (&[Phoneme], usize) = if rest.starts_with("shy") {
            (&[SH, CY], 3)
        } else {
            match rest.get(..2).unwrap_or(&rest) {
                "cy" => (&[CY], 2),
                "jy" => (&[JY], 2),
                "sh" => (&[SH], 2),
                "ny" => (&[NY], 2),
                _ => (
                    match letter {
                        // b is β, but b after m (mb).
                        'b' if sounds.last() == Some(&M) => &[B],
                        'b' => &[BETA],
                        'c' => &[C],
                        'd' => &[D],
                        'f' => &[F],
                        'g' => &[G],
                        'h' => &[H],
                        'j' => &[J],
                        'k' | 'q' => &[K],
                        'l' => &[L],
                        'm' => &[M],
                        // n is ŋ before k and g (inka, ingoma).
                        'n' if matches!(letters.get(at + 1), Some('k' | 'g')) => &[NG],
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
        };
        sounds.extend_from_slice(phonemes);
        at += step;
    }
    Some(sounds)
}

/// A consonant before w said with a velar: voiced ones with ɡ (rw ɾɡw, bw βɡw), voiceless ones
/// with k (tw tkw, sw skw), m and n with ŋ (mw mŋw).
fn velar_glides(sounds: &[Phoneme]) -> Vec<Phoneme> {
    let mut out = Vec::with_capacity(sounds.len() + 2);
    for (index, &phoneme) in sounds.iter().enumerate() {
        out.push(phoneme);
        if sounds.get(index + 1) != Some(&W) {
            continue;
        }
        if [BETA, B, D, R, Z, J, V].contains(&phoneme) {
            out.push(G);
        } else if [P, T, S, SH, F, C].contains(&phoneme) {
            out.push(K);
        } else if [M, N].contains(&phoneme) {
            out.push(NG);
        }
    }
    out
}

/// A vowel before a nasal and a consonant is long (Rwanda ɾɡwaːnda, inka iːŋka).
fn lengthen_before_nasals(sounds: &mut [Phoneme]) {
    for index in 0..sounds.len() {
        let nasal = sounds.get(index + 1).is_some_and(|next| [M, N, NG, NY].contains(next));
        let consonant = sounds
            .get(index + 2)
            .is_some_and(|after| !after.is_vowel() && ![W, Y].contains(after));
        if sounds[index].is_vowel() && nasal && consonant {
            sounds[index] = long_vowel(sounds[index]);
        }
    }
}

/// The sounds of `word`, in lower case, with the velars before w and the long vowels before a
/// nasal; `None` for a letter Kinyarwanda does not read.
pub(crate) fn word_sounds(word: &str) -> Option<Vec<Phoneme>> {
    let letters: Vec<char> = word.to_lowercase().chars().collect();
    let mut sounds = sounds(&letters)?;
    // Before the velars, which would otherwise make mw a nasal and a consonant (umwana).
    lengthen_before_nasals(&mut sounds);
    Some(velar_glides(&sounds))
}

/// `sounds` as a word stressed on its last syllable but one; `None` without a vowel.
pub(crate) fn stressed(sounds: &[Phoneme]) -> Option<Pronunciation> {
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

/// How `word` is said, stressed on its last syllable but one; `None` for a letter Kinyarwanda
/// does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    stressed(&word_sounds(word)?)
}

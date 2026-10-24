//! How a Malay word is said from its spelling: one letter or digraph, one sound (ng, ny, sy, kh,
//! gh), the diphthongs ai, au and oi at the end of a word (pantai, pulau; but main ma.in), k as ʔ
//! at the end of a word (budak), a final open a as ə, as Malaysia's standard speech (Johor-Riau)
//! says it (saya), and the stress on the last syllable but one, or on the last when that one has
//! ə (besar bəˈsar). e is ə; é, and the e of jespersen's own list, is e (meja).

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::words::respelling;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

const A: Phoneme = vowel("a", "a");
/// e (pepet), ə.
pub(crate) const E: Phoneme = vowel("ə", "ə");
/// é (taling), e (meja).
const E_TALING: Phoneme = vowel("e", "e");
/// A final open a, ə; a phoneme of its own so the rules can tell it from e's ə.
const A_FINAL: Phoneme = vowel("ə", "ə").misaki("a-final");
const I: Phoneme = vowel("i", "i");
const O: Phoneme = vowel("o", "o");
const U: Phoneme = vowel("u", "u");
const AI: Phoneme = vowel("ai̯", "aɪ");
const AU: Phoneme = vowel("au̯", "aʊ");
const OI: Phoneme = vowel("oi̯", "ɔɪ");
const EI: Phoneme = vowel("ei̯", "eɪ");

const B: Phoneme = consonant("b", "b");
const C: Phoneme = consonant("tʃ", "tʃ");
const D: Phoneme = consonant("d", "d");
const F: Phoneme = consonant("f", "f");
const G: Phoneme = consonant("ɡ", "ɡ");
const GH: Phoneme = consonant("ɣ", "ɣ");
const H: Phoneme = consonant("h", "h");
const J: Phoneme = consonant("dʒ", "dʒ");
const K: Phoneme = consonant("k", "k");
/// A final k, a glottal stop (budak ˈbudaʔ).
const GLOTTAL: Phoneme = consonant("ʔ", "ʔ");
const KH: Phoneme = consonant("x", "x");
const L: Phoneme = consonant("l", "l");
const M: Phoneme = consonant("m", "m");
const N: Phoneme = consonant("n", "n");
const NG: Phoneme = consonant("ŋ", "ŋ");
const NY: Phoneme = consonant("ɲ", "ɲ");
const P: Phoneme = consonant("p", "p");
const R: Phoneme = consonant("r", "r");
const S: Phoneme = consonant("s", "s");
const SY: Phoneme = consonant("ʃ", "ʃ");
const T: Phoneme = consonant("t", "t");
const V: Phoneme = consonant("v", "v");
const W: Phoneme = consonant("w", "w");
const Y: Phoneme = consonant("j", "j");
const Z: Phoneme = consonant("z", "z");

/// Whether `cluster` can start a syllable inside a word: one consonant (a word's first syllable
/// takes all its consonants, as in loans: struktur).
fn legal_onset(cluster: &[Phoneme]) -> bool {
    cluster.len() == 1
}

/// The sounds of `letters`, one letter or digraph at a time; `None` for a letter Malay does not
/// read.
fn sounds(letters: &[char]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let pair: String = letters[at..letters.len().min(at + 2)].iter().collect();
        // The diphthongs only end a word: pantai, pulau, amboi; main and laut have two vowels.
        let last = at + 2 == letters.len();
        let (phonemes, step): (&[Phoneme], usize) = match pair.as_str() {
            "ng" => (&[NG], 2),
            "ny" => (&[NY], 2),
            "sy" => (&[SY], 2),
            "kh" => (&[KH], 2),
            "gh" => (&[GH], 2),
            "ai" if last => (&[AI], 2),
            "au" if last => (&[AU], 2),
            "oi" if last => (&[OI], 2),
            "ei" if last => (&[EI], 2),
            // A final k is a glottal stop (budak, tidak).
            "k" if at + 1 == letters.len() && at > 0 => (&[GLOTTAL], 1),
            _ => (
                match letters[at] {
                    'a' => &[A],
                    'e' => &[E],
                    'é' | 'è' => &[E_TALING],
                    'i' => &[I],
                    'o' => &[O],
                    'u' => &[U],
                    'b' => &[B],
                    'c' => &[C],
                    'd' => &[D],
                    'f' => &[F],
                    'g' => &[G],
                    'h' => &[H],
                    'j' => &[J],
                    'k' | 'q' => &[K],
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
                    '\'' | '’' => &[],
                    _ => return None,
                },
                1,
            ),
        };
        sounds.extend_from_slice(phonemes);
        at += step;
    }
    Some(sounds)
}

/// How `word` is said; `None` for a letter Malay does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let lower = word.to_lowercase();
    let spelt = respelling(&lower).unwrap_or(&lower);
    let letters: Vec<char> = spelt.chars().collect();
    let mut sounds = sounds(&letters)?;
    let vowels: Vec<usize> = (0..sounds.len()).filter(|&index| sounds[index].is_vowel()).collect();
    if vowels.is_empty() {
        return None;
    }
    // A final open a is ə in a word of two syllables or more (saya ˈsajə, but ya).
    if let Some(&last) = vowels.last()
        && vowels.len() > 1
        && last + 1 == sounds.len()
        && sounds[last] == A
    {
        sounds[last] = A_FINAL;
    }
    // The stress is on the last syllable but one, or on the last when that one has ə.
    let penult = vowels.len().saturating_sub(2);
    let stressed = if vowels.len() > 1 && sounds[vowels[penult]] == E {
        vowels[vowels.len() - 1]
    } else {
        vowels[penult]
    };
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
    let mut said = syllabify(&segments, legal_onset);
    // Words of four syllables or more take a rhythmic secondary stress on their first, unless its
    // vowel is ə (Malaysia ˌmalajˈsiə, but kerajaan).
    if said.syllables.len() >= 4
        && let Some(first) = said.syllables.first_mut()
        && first.stress == Stress::Unstressed
        && first.nucleus != [E]
    {
        first.stress = Stress::Rhythmic;
    }
    Some(said)
}

//! How an Indonesian word is said from its spelling: one letter or digraph, one sound (ng, ny,
//! sy, kh), the diphthongs ai, au and oi, and the stress on the last syllable but one. e is ɛ under
//! the stress and ə elsewhere and o before r is ɔ, as eSpeak NG reads them.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

const A: Phoneme = vowel("a", "a");
/// e, ə until the stress makes it ɛ.
pub(crate) const E: Phoneme = vowel("ə", "ə");
pub(crate) const E_STRESSED: Phoneme = vowel("ɛ", "ɛ");
const I: Phoneme = vowel("i", "i");
const O: Phoneme = vowel("o", "o");
/// o before r (sore, kantor).
const O_OPEN: Phoneme = vowel("ɔ", "ɔ");
const U: Phoneme = vowel("u", "u");
const AI: Phoneme = vowel("ai̯", "aɪ");
const AU: Phoneme = vowel("au̯", "aʊ");
const OI: Phoneme = vowel("oi̯", "ɔɪ");
const EI: Phoneme = vowel("ei̯", "ɛi");

const B: Phoneme = consonant("b", "b");
const C: Phoneme = consonant("tʃ", "tʃ");
const D: Phoneme = consonant("d", "d");
const F: Phoneme = consonant("f", "f");
const G: Phoneme = consonant("ɡ", "ɡ");
const H: Phoneme = consonant("h", "h");
const J: Phoneme = consonant("dʒ", "dʒ");
const K: Phoneme = consonant("k", "k");
const KH: Phoneme = consonant("x", "x");
const L: Phoneme = consonant("l", "l");
const M: Phoneme = consonant("m", "m");
const N: Phoneme = consonant("n", "n");
const NG: Phoneme = consonant("ŋ", "ŋ");
const NY: Phoneme = consonant("ɲ", "ɲ");
const P: Phoneme = consonant("p", "p");
const R: Phoneme = consonant("r", "r");
const S: Phoneme = consonant("s", "s");
const SY: Phoneme = consonant("ʃ", "ç");
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

/// How `word` is said, stressed on its last syllable but one; `None` for a letter Indonesian does
/// not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let letters: Vec<char> = word.to_lowercase().chars().collect();
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let pair: String = letters[at..letters.len().min(at + 2)].iter().collect();
        let (phonemes, step): (&[Phoneme], usize) = match pair.as_str() {
            "ng" => (&[NG], 2),
            "ny" => (&[NY], 2),
            "sy" => (&[SY], 2),
            "kh" => (&[KH], 2),
            "ai" => (&[AI], 2),
            "au" => (&[AU], 2),
            "oi" => (&[OI], 2),
            "ei" => (&[EI], 2),
            _ => (
                match letters[at] {
                    'a' => &[A],
                    'e' | 'é' | 'è' => &[E],
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
    let vowels: Vec<usize> = (0..sounds.len()).filter(|&index| sounds[index].is_vowel()).collect();
    if vowels.is_empty() {
        return None;
    }
    let stressed = vowels[vowels.len().saturating_sub(2)];
    if sounds[stressed] == E {
        sounds[stressed] = E_STRESSED;
    }
    // o before r is open (sore sˈɔrə, kantor kˈantɔr).
    for index in 0..sounds.len() {
        if sounds[index] == O && sounds.get(index + 1) == Some(&R) {
            sounds[index] = O_OPEN;
        }
    }
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
    // vowel is ə (Indonesia ˌindonəsˈia, but kepulauan).
    if said.syllables.len() >= 4
        && let Some(first) = said.syllables.first_mut()
        && first.stress == Stress::Unstressed
        && first.nucleus != [E]
    {
        first.stress = Stress::Rhythmic;
    }
    Some(said)
}

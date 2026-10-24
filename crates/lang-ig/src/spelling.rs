//! How an Igbo word is said from its spelling: eight vowels (ị ɪ, ọ ɔ, ụ ʊ), the digraphs ch, gb,
//! gh, gw, kp, kw, nw, ny and sh, ṅ as ŋ, and m, n and ṅ syllabic when no vowel follows them
//! (ndị, mmiri, nke, n'ụlọ).
//!
//! Igbo is a tone language: the tone marks some texts write (á high, à low) are read, whether
//! precomposed or combining, and not said, since Kokoro, which reads eSpeak NG's notation, has no
//! tones; plain IPA leaves them out too. No syllable is stressed in Igbo, but Kokoro learnt eSpeak
//! NG's notation, in which every word that is not a function word carries a stress mark, so each
//! word is marked on its last syllable but one (a syllabic nasal counting as a syllable), as the
//! other West African languages and Swahili are. eSpeak NG's notation keeps to Kokoro's symbols
//! (decision 48): ɡb and kp without the tie, ɡw, kw and ŋw for the labialised consonants, h for ɦ,
//! and a syllabic nasal as the nasal alone.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::marks::{Letter, letters};

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

const A: Phoneme = vowel("a", "a");
const E: Phoneme = vowel("e", "e");
const I: Phoneme = vowel("i", "i");
const I_OPEN: Phoneme = vowel("ɪ", "ɪ");
const O: Phoneme = vowel("o", "o");
const O_OPEN: Phoneme = vowel("ɔ", "ɔ");
const U: Phoneme = vowel("u", "u");
const U_OPEN: Phoneme = vowel("ʊ", "ʊ");
/// m, n and ṅ as a syllable of their own (ndị ˈn̩dɪ, mmiri m̩ˈmiɾi).
const M_SYLLABIC: Phoneme = vowel("m\u{329}", "m");
const N_SYLLABIC: Phoneme = vowel("n\u{329}", "n");
const NG_SYLLABIC: Phoneme = vowel("ŋ\u{329}", "ŋ");

const B: Phoneme = consonant("b", "b");
const CH: Phoneme = consonant("tʃ", "tʃ");
const D: Phoneme = consonant("d", "d");
const F: Phoneme = consonant("f", "f");
const G: Phoneme = consonant("ɡ", "ɡ");
const GB: Phoneme = consonant("ɡ\u{361}b", "ɡb");
const GH: Phoneme = consonant("ɣ", "ɣ");
const GW: Phoneme = consonant("ɡʷ", "ɡw");
const H: Phoneme = consonant("ɦ", "h");
const J: Phoneme = consonant("dʒ", "dʒ");
const K: Phoneme = consonant("k", "k");
const KP: Phoneme = consonant("k\u{361}p", "kp");
const KW: Phoneme = consonant("kʷ", "kw");
const L: Phoneme = consonant("l", "l");
const M: Phoneme = consonant("m", "m");
const N: Phoneme = consonant("n", "n");
const NG: Phoneme = consonant("ŋ", "ŋ");
const NW: Phoneme = consonant("ŋʷ", "ŋw");
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

fn is_vowel(letter: Option<&Letter>) -> bool {
    letter.is_some_and(|letter| matches!(letter.base, 'a' | 'e' | 'i' | 'o' | 'u'))
}

/// The sounds of `letters`, in order; `None` for a letter Igbo does not read.
fn sounds(letters: &[Letter]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let letter = letters[at];
        let next = letters.get(at + 1);
        let vowel_next = is_vowel(next);
        // The digraphs, their second letter a w, a y, an h or a b.
        let pair = match (letter.base, next.map(|next| next.base)) {
            ('c', Some('h')) => Some(CH),
            ('g', Some('b')) => Some(GB),
            ('g', Some('h')) => Some(GH),
            ('g', Some('w')) => Some(GW),
            ('k', Some('p')) => Some(KP),
            ('k', Some('w')) => Some(KW),
            ('n', Some('w')) if !letter.above => Some(NW),
            ('n', Some('y')) if !letter.above => Some(NY),
            ('s', Some('h')) => Some(SH),
            _ => None,
        };
        if let Some(pair) = pair {
            sounds.push(pair);
            at += 2;
            continue;
        }
        let phonemes: &[Phoneme] = match (letter.base, letter.below) {
            ('a', _) => &[A],
            ('e', _) => &[E],
            ('i', false) => &[I],
            ('i', true) => &[I_OPEN],
            ('o', false) => &[O],
            ('o', true) => &[O_OPEN],
            ('u', false) => &[U],
            ('u', true) => &[U_OPEN],
            // m, n and ṅ with no vowel after them are a syllable of their own.
            ('n', _) if letter.above && !vowel_next => &[NG_SYLLABIC],
            ('n', _) if letter.above => &[NG],
            ('n', _) if !vowel_next => &[N_SYLLABIC],
            ('m', _) if !vowel_next => &[M_SYLLABIC],
            ('n', _) => &[N],
            ('m', _) => &[M],
            ('b', _) => &[B],
            ('d', _) => &[D],
            ('f', _) => &[F],
            ('g', _) => &[G],
            ('h', _) => &[H],
            ('j', _) => &[J],
            ('k' | 'c' | 'q', _) => &[K],
            ('l', _) => &[L],
            ('p', _) => &[P],
            ('r', _) => &[R],
            ('s', _) => &[S],
            ('t', _) => &[T],
            ('v', _) => &[V],
            ('w', _) => &[W],
            ('y', _) => &[Y],
            ('z', _) => &[Z],
            ('x', _) => &[K, S],
            ('\'' | '’' | '-', _) => &[],
            _ => return None,
        };
        sounds.extend_from_slice(phonemes);
        at += 1;
    }
    Some(sounds)
}

/// Whether `cluster` can start a syllable: Igbo's syllables have one consonant at most.
fn legal_onset(cluster: &[Phoneme]) -> bool {
    cluster.len() == 1
}

/// How `word` is said, marked on its last syllable but one; `None` for a letter Igbo does not
/// read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let sounds = sounds(&letters(word))?;
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

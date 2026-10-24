//! How a Somali word is said from its official Latin spelling (1972): one letter or digraph, one
//! sound (c ʕ, x ħ, dh ɖ, kh x, sh ʃ, j dʒ, q q, ' ʔ), a doubled vowel long and a doubled
//! consonant geminate, and b, d, g and dh softened between vowels (β, ð, ɣ, ɽ), as Saeed's
//! description of the standard language has them.
//!
//! Somali marks its words with a high tone on one mora, which the spelling does not show and
//! which depends on the word's gender and case. jespersen puts the stress on the second-last mora,
//! the commonest place: on the last syllable when its vowel is long, which counts two morae
//! (khamiis xaˈmiːs), otherwise on the last syllable but one (Soomaali soːˈmaːli, magaalo
//! maˈɣaːlo).
//!
//! eSpeak NG has no Somali voice, so its notation keeps to the symbols Kokoro reads (decision 48):
//! ħ is written h and ʕ as ʔ, the nearest sounds Kokoro has.

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

const B: Phoneme = consonant("b", "b");
const D: Phoneme = consonant("d", "d");
/// dh, a voiced retroflex stop (dhul ˈɖul).
const DH: Phoneme = consonant("ɖ", "ɖ");
const F: Phoneme = consonant("f", "f");
const G: Phoneme = consonant("ɡ", "ɡ");
const H: Phoneme = consonant("h", "h");
const J: Phoneme = consonant("dʒ", "dʒ");
const K: Phoneme = consonant("k", "k");
const KH: Phoneme = consonant("x", "x");
const L: Phoneme = consonant("l", "l");
const M: Phoneme = consonant("m", "m");
const N: Phoneme = consonant("n", "n");
const P: Phoneme = consonant("p", "p");
const Q: Phoneme = consonant("q", "q");
const R: Phoneme = consonant("r", "r");
const S: Phoneme = consonant("s", "s");
const SH: Phoneme = consonant("ʃ", "ʃ");
const T: Phoneme = consonant("t", "t");
const V: Phoneme = consonant("v", "v");
const W: Phoneme = consonant("w", "w");
const Y: Phoneme = consonant("j", "j");
const Z: Phoneme = consonant("z", "z");
/// x, a voiceless pharyngeal fricative (xoog ˈħoːɡ), h in Kokoro's symbols.
const X: Phoneme = consonant("ħ", "h");
/// c, a voiced pharyngeal (cad ˈʕad), ʔ in Kokoro's symbols.
const C: Phoneme = consonant("ʕ", "ʔ");
/// The apostrophe, a glottal stop (ba'an ˈbaʔan).
const GLOTTAL: Phoneme = consonant("ʔ", "ʔ");
/// b, d, g and dh softened between vowels (aabo ˈaːβo, gabadh ˈɡaβaɖ).
const B_SOFT: Phoneme = consonant("β", "β");
const D_SOFT: Phoneme = consonant("ð", "ð");
const G_SOFT: Phoneme = consonant("ɣ", "ɣ");
const DH_SOFT: Phoneme = consonant("ɽ", "ɽ");

/// Whether `cluster` can start a syllable inside a word: one consonant only, since Somali
/// syllables begin with at most one (a loan's br, tr and the like are split).
fn legal_onset(cluster: &[Phoneme]) -> bool {
    cluster.len() == 1
}

/// A letter or digraph's sound; `None` for a letter Somali does not read.
fn sound(letters: &[char], at: usize) -> Option<(Phoneme, usize)> {
    let pair: String = letters[at..letters.len().min(at + 2)].iter().collect();
    Some(match pair.as_str() {
        "sh" => (SH, 2),
        "dh" => (DH, 2),
        "kh" => (KH, 2),
        _ => (
            match letters[at] {
                'a' => A,
                'e' => E,
                'i' => I,
                'o' => O,
                'u' => U,
                'b' => B,
                'c' => C,
                'd' => D,
                'f' => F,
                'g' => G,
                'h' => H,
                'j' => J,
                'k' => K,
                'l' => L,
                'm' => M,
                'n' => N,
                'p' => P,
                'q' => Q,
                'r' => R,
                's' => S,
                't' => T,
                'v' => V,
                'w' => W,
                'x' => X,
                'y' => Y,
                'z' => Z,
                '\'' | '’' => GLOTTAL,
                _ => return None,
            },
            1,
        ),
    })
}

/// A short vowel made long by its doubled letter.
fn lengthened(phoneme: Phoneme) -> Phoneme {
    match phoneme {
        phoneme if phoneme == A => A_LONG,
        phoneme if phoneme == E => E_LONG,
        phoneme if phoneme == I => I_LONG,
        phoneme if phoneme == O => O_LONG,
        _ => U_LONG,
    }
}

/// The sounds of `letters`, in order, a doubled vowel as one long vowel and a doubled consonant
/// as the consonant said twice; `None` for a letter Somali does not read.
fn sounds(letters: &[char]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let (phoneme, step) = sound(letters, at)?;
        if phoneme.is_vowel() && letters.get(at + 1) == Some(&letters[at]) {
            sounds.push(lengthened(phoneme));
            at += 2;
        } else {
            sounds.push(phoneme);
            at += step;
        }
    }
    Some(sounds)
}

/// Softens a single b, d, g or dh between two vowels; a geminate stays a stop.
fn soften(sounds: &mut [Phoneme]) {
    for index in 1..sounds.len().saturating_sub(1) {
        if !(sounds[index - 1].is_vowel() && sounds[index + 1].is_vowel()) {
            continue;
        }
        sounds[index] = match sounds[index] {
            phoneme if phoneme == B => B_SOFT,
            phoneme if phoneme == D => D_SOFT,
            phoneme if phoneme == G => G_SOFT,
            phoneme if phoneme == DH => DH_SOFT,
            phoneme => phoneme,
        };
    }
}

fn is_long(phoneme: Phoneme) -> bool {
    [A_LONG, E_LONG, I_LONG, O_LONG, U_LONG].contains(&phoneme)
}

/// How `word` is said, stressed on its second-last mora; `None` for a letter Somali does not
/// read, or a word without a vowel.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let letters: Vec<char> = word.to_lowercase().chars().collect();
    let mut sounds = sounds(&letters)?;
    soften(&mut sounds);
    let vowels: Vec<usize> = (0..sounds.len()).filter(|&index| sounds[index].is_vowel()).collect();
    let &last = vowels.last()?;
    let stressed = if is_long(sounds[last]) {
        last
    } else {
        vowels[vowels.len().saturating_sub(2)]
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
    Some(syllabify(&segments, legal_onset))
}

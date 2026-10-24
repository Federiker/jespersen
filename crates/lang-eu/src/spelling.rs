//! How a Basque word is said from its spelling: one letter, one sound, with tz, ts, tx, tt, dd,
//! ll and rr; h silent; l and n palatal after i before a vowel (ilargi, baino); the diphthongs
//! ai, ei, oi, au and eu, other vowels side by side apart (katua ka-tu-a); and, as eSpeak NG
//! reads it, the stress on the second syllable (on the first of a word of two that ends in a
//! vowel) with a secondary one on the last of a longer word.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, AI, AU, B, D, DD, E, EI, EU, F, G, I, J, J_STOP, K, L, LL, M, N, NY, O, OI, P, R, RR, S, SH, T, TS, TT, TX, TZ,
    U, Z, legal_onset,
};

/// Verb forms stressed on their last syllable though they end in a vowel (gara, dira).
const FINAL_STRESS: [&str; 5] = ["gara", "dira", "zara", "dela", "bada"];

/// The sounds of `word`'s letters (in lower case); `None` for a letter Basque does not read.
fn letters(word: &[char]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(word.len());
    let mut at = 0;
    let is_vowel = |c: Option<char>| matches!(c, Some('a' | 'e' | 'i' | 'o' | 'u'));
    while at < word.len() {
        let next = word.get(at + 1).copied();
        let after_i = at > 0 && matches!(word[at - 1], 'i');
        let (phonemes, step): (&[Phoneme], usize) = match (word[at], next) {
            ('t', Some('z')) => (&[TZ], 2),
            ('t', Some('s')) => (&[TS], 2),
            ('t', Some('x')) => (&[TX], 2),
            ('t', Some('t')) => (&[TT], 2),
            ('d', Some('d')) => (&[DD], 2),
            ('l', Some('l')) => (&[LL], 2),
            ('r', Some('r')) => (&[RR], 2),
            ('a', Some('i')) => (&[AI], 2),
            ('e', Some('i')) => (&[EI], 2),
            ('o', Some('i')) => (&[OI], 2),
            // eSpeak NG writes j as ɟ, but as j in jan and before a diphthong (jaitsi, joango).
            ('j', Some(_)) if word != ['j', 'a', 'n'] && !is_vowel(word.get(at + 2).copied()) => (&[J_STOP], 1),
            ('a', Some('u')) if !is_vowel(word.get(at + 2).copied()) => (&[AU], 2),
            ('e', Some('u')) if !is_vowel(word.get(at + 2).copied()) => (&[EU], 2),
            // l and n after i before a vowel are palatal (ilargi, baino).
            ('l', Some(_)) if after_i && is_vowel(next) => (&[LL], 1),
            ('n', Some(_)) if after_i && is_vowel(next) => (&[NY], 1),
            // r starting a word is trilled.
            ('r', _) if at == 0 => (&[RR], 1),
            (c, _) => (
                match c {
                    'a' => &[A],
                    'b' | 'v' => &[B],
                    'c' | 'k' | 'q' => &[K],
                    'd' => &[D],
                    'e' => &[E],
                    'f' => &[F],
                    'g' => &[G],
                    'i' | 'y' => &[I],
                    'j' => &[J],
                    'l' => &[L],
                    'm' => &[M],
                    'n' => &[N],
                    'ñ' => &[NY],
                    'o' => &[O],
                    'p' => &[P],
                    'r' => &[R],
                    's' => &[S],
                    't' => &[T],
                    'u' | 'ü' | 'w' => &[U],
                    'x' => &[SH],
                    'z' => &[Z],
                    'h' | '\'' | '’' | '-' => &[],
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

/// How `word` is said; `None` for a letter Basque does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let lower = word.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();
    let sounds = letters(&chars)?;
    if sounds.is_empty() {
        return None;
    }
    let vowels: Vec<usize> = (0..sounds.len()).filter(|&at| sounds[at].is_vowel()).collect();
    let ends_in_vowel = sounds.last().is_some_and(|sound| sound.is_vowel());
    let stressed = match vowels.len() {
        0 => None,
        1 => Some(vowels[0]),
        2 if ends_in_vowel && !FINAL_STRESS.contains(&lower.as_str()) => Some(vowels[0]),
        _ => Some(vowels[1]),
    };
    let segments: Vec<Segment> = sounds
        .iter()
        .enumerate()
        .map(|(at, &sound)| {
            let stress = if Some(at) == stressed {
                Stress::Primary
            } else {
                Stress::Unstressed
            };
            Segment::new(sound, stress)
        })
        .collect();
    let mut said = syllabify(&segments, legal_onset);
    // eSpeak NG's secondary stress on the last syllable of a word of three or more (katua katˈuˌa).
    if said.syllables.len() >= 3
        && let Some(last) = said.syllables.last_mut()
        && last.stress == Stress::Unstressed
    {
        last.stress = Stress::Rhythmic;
    }
    Some(said)
}

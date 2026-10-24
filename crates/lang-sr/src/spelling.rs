//! How a Serbian word, in either script, is said from its spelling: one letter, one sound, with dž,
//! lj and nj; r as a syllable between consonants (smrt, prst); a cluster of obstruents voiced or
//! not by its last one; and, as eSpeak NG reads Croatian, the stress on the first syllable with a
//! vowel letter, and the vowels of unstressed syllables.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, Syllable, syllabify};

use crate::{
    cyrillic::to_latin,
    phonemes::{
        A, A_BACK, A_UNSTRESSED, AE, B, CH, D, DJ, DZH, E, E_AFTER_J, F, G, H_INNER, I, I_UNSTRESSED, J, K, L, LJ, M,
        N, NG, NY, O, P, R, R_SYLLABIC, R_TAP, S, SH, T, TJ, TS, U, U_UNSTRESSED, V, V_BEFORE_VOICELESS, X, Z, ZH,
        is_obstruent, legal_onset, voices, with_voicing,
    },
};

/// The sounds of `word`'s letters (in lower case), a vowel as its stressed form; `None` for a
/// letter Serbian does not read.
fn letters(word: &[char]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(word.len());
    let mut at = 0;
    while at < word.len() {
        let next = word.get(at + 1).copied();
        let (phonemes, step): (&[Phoneme], usize) = match (word[at], next) {
            ('d', Some('ž')) => (&[DZH], 2),
            ('l', Some('j')) => (&[LJ], 2),
            ('n', Some('j')) => (&[NY], 2),
            // n is ŋ before k and g (banka).
            ('n', Some('k' | 'g')) => (&[NG], 1),
            // The ae of -naest is one syllable (jedanaest, dvanaest).
            ('a', Some('e')) if word.get(at + 2) == Some(&'s') && at > 0 && word[at - 1] == 'n' => (&[AE], 2),
            (c, _) => (
                match c {
                    'a' => &[A],
                    'b' => &[B],
                    'c' => &[TS],
                    'č' => &[CH],
                    'ć' => &[TJ],
                    'd' => &[D],
                    'đ' => &[DJ],
                    'e' => &[E],
                    'f' => &[F],
                    'g' => &[G],
                    'h' if at > 0 => &[H_INNER],
                    'h' => &[X],
                    'i' | 'y' => &[I],
                    'j' => &[J],
                    'k' => &[K],
                    'l' => &[L],
                    'm' => &[M],
                    'n' => &[N],
                    'o' => &[O],
                    'p' => &[P],
                    'q' => &[K, V],
                    'r' => &[R],
                    's' => &[S],
                    'š' => &[SH],
                    't' => &[T],
                    'u' => &[U],
                    'v' | 'w' => &[V],
                    'x' => &[K, S],
                    'z' => &[Z],
                    'ž' => &[ZH],
                    '\'' | '’' | '-' => &[],
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

/// r with no vowel beside it is the nucleus of its syllable (smrt, trg, prst); r at the start
/// of a word or after a consonant is written ɾ by eSpeak NG (redu, krovu).
fn r(sounds: &mut [Phoneme]) {
    for at in 0..sounds.len() {
        if sounds[at] != R {
            continue;
        }
        let vowel = |offset: Option<usize>| {
            offset
                .and_then(|at| sounds.get(at))
                .is_some_and(|sound| sound.is_vowel())
        };
        let before = at.checked_sub(1);
        if !vowel(before) && !vowel(Some(at + 1)) {
            sounds[at] = R_SYLLABIC;
        } else if !vowel(before) {
            sounds[at] = R_TAP;
        }
    }
}

/// Voicing inside a word: a cluster of obstruents takes the voicing of its last one other than v
/// (novčanik noftʃanik, svjedodžba).
fn assimilate(sounds: &mut [Phoneme]) {
    let mut start = 0;
    while start < sounds.len() {
        let length = sounds[start..].iter().take_while(|&&sound| is_obstruent(sound)).count();
        let cluster = &mut sounds[start..start + length];
        if let Some(last) = cluster.iter().rposition(|&sound| sound != V) {
            let voiced = voices(cluster[last]);
            for sound in &mut cluster[..last] {
                // v keeps its voice in plain IPA; eSpeak NG writes it f before a voiceless consonant.
                *sound = if *sound == V && !voiced {
                    V_BEFORE_VOICELESS
                } else {
                    with_voicing(*sound, voiced)
                };
            }
        }
        start += length.max(1);
    }
}

/// A vowel as eSpeak NG writes it by its stress and what is around it: a as æ unstressed and ɑ
/// before r and j, i and u as ɪ and ʊ unstressed, e after j, nj and lj as e. Plain IPA keeps the
/// phonemes.
fn vowel_quality(vowel: Phoneme, stressed: bool, before: Option<Phoneme>, after: Option<Phoneme>) -> Phoneme {
    match vowel {
        v if v == A && after.is_some_and(|after| [R, R_TAP, J].contains(&after)) => A_BACK,
        v if v == A && !stressed => A_UNSTRESSED,
        v if v == E && before.is_some_and(|before| [J, NY, LJ].contains(&before)) => E_AFTER_J,
        v if v == I && !stressed => I_UNSTRESSED,
        v if v == U && !stressed => U_UNSTRESSED,
        v => v,
    }
}

/// How `word` is said, stressed on its first syllable with a vowel letter (a syllabic r is passed
/// over, as eSpeak NG reads it: trg trɡ, trgu trɡˈu), with eSpeak NG's secondary stress on the
/// third syllable of a longer word; `None` for a letter Serbian does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let lower: Vec<char> = to_latin(word).chars().collect();
    let mut sounds = letters(&lower)?;
    if sounds.is_empty() {
        return None;
    }
    r(&mut sounds);
    assimilate(&mut sounds);
    let stressed = sounds.iter().position(|sound| sound.is_vowel() && *sound != R_SYLLABIC);
    let qualities: Vec<Phoneme> = (0..sounds.len())
        .map(|at| {
            if sounds[at].is_vowel() && sounds[at] != R_SYLLABIC {
                vowel_quality(
                    sounds[at],
                    Some(at) == stressed,
                    at.checked_sub(1).map(|before| sounds[before]),
                    sounds.get(at + 1).copied(),
                )
            } else {
                sounds[at]
            }
        })
        .collect();
    let segments: Vec<Segment> = qualities
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
    rhythm(&mut said);
    Some(said)
}

/// eSpeak NG's secondary stress on the third syllable of a word of four or more whose first is
/// stressed (igralištu ˈiɡɾælˌɪʃtʊ).
fn rhythm(said: &mut Pronunciation) {
    let stressed_first = said
        .syllables
        .first()
        .is_some_and(|syllable: &Syllable| syllable.stress == Stress::Primary);
    if stressed_first
        && said.syllables.len() >= 4
        && let Some(third) = said.syllables.get_mut(2)
    {
        third.stress = Stress::Rhythmic;
    }
}

/// eSpeak NG writes an unstressed syllable's vowel by its own rules: once a word loses its stress
/// in running text, its stressed vowel takes the unstressed form (je, u ʊ, i ɪ).
pub(crate) fn unstress(said: &mut Pronunciation) {
    for syllable in &mut said.syllables {
        if syllable.stress == Stress::Primary {
            continue;
        }
        for vowel in &mut syllable.nucleus {
            *vowel = match *vowel {
                v if v == I => I_UNSTRESSED,
                v if v == U => U_UNSTRESSED,
                v if v == A => A_UNSTRESSED,
                v => v,
            };
        }
    }
}

/// The voiceless obstruent that a voiced one becomes at the end of a word before a voiceless one
/// (od povrća ot), and the voiced one a voiceless one becomes before a voiced one (put do pud).
pub(crate) fn voice_end(said: &mut Pronunciation, voiced: bool) {
    let Some(last) = said.syllables.last_mut() else {
        return;
    };
    let end = if last.coda.is_empty() && last.nucleus.is_empty() {
        &mut last.onset
    } else {
        &mut last.coda
    };
    for phoneme in end.iter_mut().rev() {
        if !is_obstruent(*phoneme) {
            break;
        }
        *phoneme = with_voicing(*phoneme, voiced);
    }
}

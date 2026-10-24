//! How a Finnish word is said from its spelling, which shows it all: one letter, one sound; a
//! letter written twice is long; the stress is on the first syllable.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, AE, B, D, E, F, G, H, I, J, K, K_FIRST, K_LONG, L, M, N, NG, NUCLEI, O, OE, P, P_FIRST, P_LONG, R, S, SH, T,
    T_FIRST, T_LONG, TS, U, V, Y, ZH, legal_onset,
};

/// The vowel a letter is, `None` for a consonant.
fn vowel(c: char) -> Option<Phoneme> {
    Some(match c {
        'a' | 'á' | 'à' => A,
        'e' | 'é' | 'è' => E,
        'i' | 'í' => I,
        'o' | 'ó' | 'å' => O,
        'u' | 'ú' => U,
        'y' | 'ü' => Y,
        'ä' => AE,
        'ö' => OE,
        _ => return None,
    })
}

/// The letters of a vowel with the accents of loans taken off (é is e).
fn plain(c: char) -> char {
    match c {
        'á' | 'à' => 'a',
        'é' | 'è' => 'e',
        'í' => 'i',
        'ó' | 'å' => 'o',
        'ú' => 'u',
        'ü' => 'y',
        c => c,
    }
}

/// How `word` is said, stressed on its first syllable; `None` for a letter Finnish does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let letters: Vec<char> = word.to_lowercase().chars().collect();
    let letter = |at: usize| letters.get(at).copied();
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let c = letters[at];
        let next = letter(at + 1);
        if let Some(single) = vowel(c) {
            // A long vowel or a diphthong is one nucleus. ie, uo and yö are diphthongs in a word's first
            // syllable, and eSpeak NG makes them so in compounds too (rautatie, Rovaniemi).
            let pair: String = [plain(c), next.map_or(' ', plain)].iter().collect();
            let nucleus = NUCLEI.iter().find(|(letters, _)| *letters == pair);
            if let Some((_, phoneme)) = nucleus {
                sounds.push(*phoneme);
                at += 2;
            } else {
                sounds.push(single);
                at += 1;
            }
            continue;
        }
        let doubled = next == Some(c);
        let mut step = 1;
        match c {
            'p' | 't' | 'k' if doubled => {
                let (first, long) = match c {
                    'p' => (P_FIRST, P_LONG),
                    't' => (T_FIRST, T_LONG),
                    _ => (K_FIRST, K_LONG),
                };
                sounds.push(first);
                sounds.push(long);
                step = 2;
            }
            // ng is a long ŋ (kengät kˈeŋŋæt), n before k a short one (kenkä).
            'n' if next == Some('g') => {
                sounds.push(NG);
                sounds.push(NG);
                step = 2;
            }
            'n' if next == Some('k') => sounds.push(NG),
            'p' => sounds.push(P),
            't' => sounds.push(T),
            'c' if matches!(next, Some('e' | 'i' | 'y')) => sounds.push(S),
            'k' | 'q' | 'c' => sounds.push(K),
            'b' => sounds.push(B),
            'd' => sounds.push(D),
            'g' => sounds.push(G),
            'f' => sounds.push(F),
            's' => sounds.push(S),
            'š' => sounds.push(SH),
            'ž' => sounds.push(ZH),
            'z' => sounds.push(TS),
            'h' => sounds.push(H),
            'v' | 'w' => sounds.push(V),
            'j' => sounds.push(J),
            'l' => sounds.push(L),
            'm' => sounds.push(M),
            'n' => sounds.push(N),
            'r' => sounds.push(R),
            'x' => {
                sounds.push(K);
                sounds.push(S);
            }
            '-' | '\'' | '’' | ':' => {}
            _ => return None,
        }
        at += step;
    }
    if sounds.is_empty() {
        return None;
    }
    let first_vowel = sounds.iter().position(|phoneme| phoneme.is_vowel());
    let segments: Vec<Segment> = sounds
        .iter()
        .enumerate()
        .map(|(at, phoneme)| {
            Segment::new(
                *phoneme,
                if Some(at) == first_vowel {
                    Stress::Primary
                } else {
                    Stress::Unstressed
                },
            )
        })
        .collect();
    let mut said = syllabify(&segments, legal_onset);
    rhythm(&mut said);
    Some(said)
}

/// The rhythmic secondary stresses, which eSpeak NG's notation marks: on the third syllable, the
/// fifth and so on, but never the last (tuhansia tˈuhansˌia, but omenaa ˈomenaː).
fn rhythm(said: &mut Pronunciation) {
    let last = said.syllables.len().saturating_sub(1);
    for (number, syllable) in said.syllables.iter_mut().enumerate() {
        if number >= 2 && number % 2 == 0 && number < last && syllable.stress == Stress::Unstressed {
            syllable.stress = Stress::Rhythmic;
        }
    }
}

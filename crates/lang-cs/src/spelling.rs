//! How a Czech word is said from its spelling: one letter, one sound, with ch, dž, ou, au and eu;
//! d, t and n soft before ě and i (děti ɟɛcɪ), ě as jɛ after b, p, v and f and as ɲɛ after m; r
//! and l as syllables between consonants (krk, vlk); voicing that follows the last obstruent of a
//! cluster and is lost at the end of a word; and the stress on the first syllable.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, AA, AU, B, CH, D, DJ, DZH, E, EE, EU, F, G, H, I, II, J, J_OFFGLIDE, K, L, L_SYLLABIC, M, N, NG, NY, O, OO, OU,
    P, R, R_SYLLABIC, RZ, RZ_FINAL, RZ_VOICELESS, S, SH, T, TJ, TS, U, UU, V, V_FINAL, X, Z, ZH, final_form,
    is_obstruent, is_voiced, legal_onset, voiced_inside, with_voicing,
};

/// A vowel letter's sound.
fn vowel(letter: char) -> Option<Phoneme> {
    Some(match letter {
        'a' => A,
        'á' => AA,
        'e' | 'ě' => E,
        'é' => EE,
        'i' | 'y' => I,
        'í' | 'ý' => II,
        'o' => O,
        'ó' => OO,
        'u' => U,
        'ú' | 'ů' => UU,
        _ => return None,
    })
}

/// The sounds of `word`'s letters (in lower case); `None` for a letter Czech does not read.
fn letters(word: &[char]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(word.len() + 2);
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        let next = word.get(at + 1).copied();
        let mut step = 1;
        match (c, next) {
            ('c', Some('h')) => {
                sounds.push(X);
                step = 2;
            }
            ('d', Some('ž')) => {
                sounds.push(DZH);
                step = 2;
            }
            ('o' | 'a' | 'e', Some('u')) => {
                sounds.push(match c {
                    'o' => OU,
                    'a' => AU,
                    _ => EU,
                });
                step = 2;
            }
            // d, t and n are soft before ě, i and í (děti, tisíc, nikdy); the ě is then ɛ.
            ('d' | 't' | 'n', Some('ě' | 'i' | 'í')) => {
                sounds.push(match c {
                    'd' => DJ,
                    't' => TJ,
                    _ => NY,
                });
                if next == Some('ě') {
                    sounds.push(E);
                    step = 2;
                }
            }
            // ě after b, p, v and f is jɛ (pět), after m ɲɛ (mě).
            ('b' | 'p' | 'v' | 'f' | 'm', Some('ě')) => {
                sounds.push(match c {
                    'b' => B,
                    'p' => P,
                    'f' => F,
                    'm' => M,
                    _ => V,
                });
                sounds.extend([if c == 'm' { NY } else { J }, E]);
                step = 2;
            }
            // ě anywhere else is jɛ.
            ('ě', _) => sounds.extend([J, E]),
            // j after e before a consonant closes a diphthong (odejdeš).
            ('j', _) if sounds.last() == Some(&E) && next.is_some_and(|next| vowel(next).is_none()) => {
                sounds.push(J_OFFGLIDE);
            }
            // i before e or i has a j after it (historie ɦɪstorɪjɛ, anatomii).
            ('i' | 'í', Some('e' | 'é' | 'i' | 'í')) => sounds.extend([vowel(c).unwrap_or(I), J]),
            (c, _) if vowel(c).is_some() => sounds.extend(vowel(c)),
            _ => sounds.extend_from_slice(match c {
                'b' => &[B],
                'c' => &[TS],
                'č' => &[CH],
                'd' => &[D],
                'ď' => &[DJ],
                'f' => &[F],
                'g' => &[G],
                'h' => &[H],
                'j' => &[J],
                'k' => &[K],
                'l' => &[L],
                'm' => &[M],
                // n is ŋ before k and g (maminka).
                'n' if matches!(next, Some('k' | 'g')) => &[NG],
                'n' => &[N],
                'ň' => &[NY],
                'p' => &[P],
                'q' => &[K, V],
                'r' => &[R],
                'ř' => &[RZ],
                's' => &[S],
                'š' => &[SH],
                't' => &[T],
                'ť' => &[TJ],
                'v' | 'w' => &[V],
                'x' => &[K, S],
                'z' => &[Z],
                'ž' => &[ZH],
                '\'' | '’' | '-' => &[],
                _ => return None,
            }),
        }
        at += step;
    }
    Some(sounds)
}

/// r and l with no vowel beside them are the nucleus of their syllable (krk, vlk, bratr, Brno).
fn syllabic(sounds: &mut [Phoneme]) {
    for at in 0..sounds.len() {
        let beside_vowel = |offset: Option<usize>| {
            offset
                .and_then(|at| sounds.get(at))
                .is_some_and(|sound| sound.is_vowel())
        };
        if (sounds[at] == R || sounds[at] == L) && !beside_vowel(at.checked_sub(1)) && !beside_vowel(Some(at + 1)) {
            sounds[at] = if sounds[at] == R { R_SYLLABIC } else { L_SYLLABIC };
        }
    }
}

/// Voicing inside a word: a cluster of obstruents takes the voicing of its last one other than v and ř
/// (všechno fʃ, vždycky vʒd), ř is voiceless after a voiceless consonant (tři), and the last
/// obstruents of the word lose their voice (hrad hrat). eSpeak NG's notation shows only the loss
/// of voice inside a word; plain IPA also shows the gain (kdo ɡdo, shoda zɦoda).
fn assimilate(sounds: &mut [Phoneme]) {
    let mut start = 0;
    while start < sounds.len() {
        let length = sounds[start..].iter().take_while(|&&sound| is_obstruent(sound)).count();
        let cluster = &mut sounds[start..start + length];
        if let Some(last) = cluster.iter().rposition(|&sound| sound != V && sound != RZ) {
            let voiced = is_voiced(cluster[last]);
            for sound in &mut cluster[..last] {
                *sound = if voiced {
                    voiced_inside(*sound)
                } else {
                    with_voicing(*sound, false)
                };
            }
        }
        start += length.max(1);
    }
    for at in 1..sounds.len() {
        if sounds[at] == RZ && is_obstruent(sounds[at - 1]) && !is_voiced(sounds[at - 1]) {
            sounds[at] = RZ_VOICELESS;
        }
    }
    // A word without a vowel (the prepositions v, z) keeps its voice for the next word to settle.
    if sounds.iter().any(|sound| sound.is_vowel()) {
        for sound in sounds.iter_mut().rev() {
            if !is_obstruent(*sound) {
                break;
            }
            *sound = match *sound {
                sound if sound == V => V_FINAL,
                sound if sound == RZ => RZ_FINAL,
                sound => final_form(sound),
            };
        }
    }
}

/// How `word` is said, stressed on its first syllable, with eSpeak NG's secondary stress on the
/// third of a word of four syllables or more (rˈepublˌika); `None` for a letter Czech does not
/// read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let lower: Vec<char> = word.to_lowercase().chars().collect();
    let mut sounds = letters(&lower)?;
    if sounds.is_empty() {
        return None;
    }
    // ll is said as one l (Sulla), as eSpeak NG says it.
    sounds.dedup_by(|this, before| *this == L && *before == L);
    syllabic(&mut sounds);
    assimilate(&mut sounds);
    let first = sounds.iter().position(|sound| sound.is_vowel());
    let segments: Vec<Segment> = sounds
        .iter()
        .enumerate()
        .map(|(at, &sound)| {
            let stress = if Some(at) == first {
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
/// stressed (also once a preposition joins it: na zahradě nˈazahrˌaɟɛ).
pub(crate) fn rhythm(said: &mut Pronunciation) {
    let stressed_first = said
        .syllables
        .first()
        .is_some_and(|syllable| syllable.stress == Stress::Primary);
    for syllable in &mut said.syllables {
        if syllable.stress == Stress::Rhythmic {
            syllable.stress = Stress::Unstressed;
        }
    }
    if stressed_first
        && said.syllables.len() >= 4
        && let Some(third) = said.syllables.get_mut(2)
        && third.stress == Stress::Unstressed
    {
        third.stress = Stress::Rhythmic;
    }
}

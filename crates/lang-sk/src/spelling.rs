//! How a Slovak word is said from its spelling: one letter, one sound, with ch, dz, dž and ou;
//! the diphthongs ia, ie, iu and ô; d, t and n soft before e and i (deti ɟeci), ľ and ä; r and l
//! as syllables between consonants (krk, vlk); v after a vowel said u̯ (domov); voicing that follows
//! the last obstruent of a cluster and is lost at the end of a word; and the stress on the first
//! syllable.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, AA, AE, AU, B, CH, D, DJ, DZ, DZH, E, EE, F, G, H, H_DEVOICED, I, I_GLIDE, II, J, K, L, L_SYLLABIC,
    L_SYLLABIC_LONG, LJ, M, N, NG, NY, O, OO, OU, P, R, R_SYLLABIC, R_SYLLABIC_LONG, S, SH, T, TJ, TS, U, UO, UU, V,
    V_OFFGLIDE, X, Z, ZH, final_form, is_obstruent, is_voiced, legal_onset, voiced_inside, with_voicing,
};

/// A vowel letter's sound.
fn vowel(letter: char) -> Option<Phoneme> {
    Some(match letter {
        'a' => A,
        'á' => AA,
        'ä' => AE,
        'e' => E,
        'é' => EE,
        'i' | 'y' => I,
        'í' | 'ý' => II,
        'o' => O,
        'ó' => OO,
        'ô' => UO,
        'u' => U,
        'ú' => UU,
        _ => return None,
    })
}

/// Native words whose d, t and n stay hard before e and i, as eSpeak NG says them (ten, teraz).
const HARD: [&str; 9] = [
    "tie",
    "ten",
    "tento",
    "teraz",
    "jeden",
    "jedenásť",
    "jedenásty",
    "vtedy",
    "neho",
];

/// The sounds of `word`'s letters (in lower case); `None` for a letter Slovak does not read.
fn letters(word: &[char]) -> Option<Vec<Phoneme>> {
    let written: String = word.iter().collect();
    let hard = HARD.contains(&written.as_str());
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(word.len() + 2);
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        let next = word.get(at + 1).copied();
        let after_vowel = sounds.last().is_some_and(|sound| sound.is_vowel());
        let mut step = 1;
        match (c, next) {
            ('c', Some('h')) => {
                sounds.push(X);
                step = 2;
            }
            ('d', Some('ž' | 'z')) => {
                sounds.push(if next == Some('ž') { DZH } else { DZ });
                step = 2;
            }
            ('o', Some('u')) => {
                sounds.push(OU);
                step = 2;
            }
            ('a', Some('u')) => {
                sounds.push(AU);
                step = 2;
            }
            // ia, ie and iu are diphthongs, a glide into the vowel (piesok, ľudia).
            ('i', Some('a' | 'e' | 'u')) if at > 0 => sounds.push(I_GLIDE),
            // d, t and n are soft before e and i (deti, teplo, nikdy), but hard before é and the -ej of
            // adjectives (červené, strednej), as eSpeak NG says them.
            ('d' | 't' | 'n', Some('e' | 'i' | 'í'))
                if !hard && (next != Some('e') || word.get(at + 2) != Some(&'j') || word.len() != at + 3) =>
            {
                sounds.push(match c {
                    'd' => DJ,
                    't' => TJ,
                    _ => NY,
                });
            }
            // v after a vowel before an obstruent or at the end is u̯ (domov, pravda; but hlavná).
            ('v', _) if after_vowel && next.is_none_or(|next| vowel(next).is_none() && !"lľmnňrj".contains(next)) => {
                sounds.push(V_OFFGLIDE);
            }
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
                'ľ' => &[LJ],
                'ĺ' => &[L_SYLLABIC_LONG],
                'm' => &[M],
                // n is ŋ before k and g (banka).
                'n' if matches!(next, Some('k' | 'g')) => &[NG],
                'n' => &[N],
                'ň' => &[NY],
                'p' => &[P],
                'q' => &[K, V],
                'r' => &[R],
                'ŕ' => &[R_SYLLABIC_LONG],
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
                .is_some_and(|sound| sound.is_vowel() || *sound == I_GLIDE)
        };
        if (sounds[at] == R || sounds[at] == L) && !beside_vowel(at.checked_sub(1)) && !beside_vowel(Some(at + 1)) {
            sounds[at] = if sounds[at] == R { R_SYLLABIC } else { L_SYLLABIC };
        }
    }
}

/// Voicing inside a word: a cluster of obstruents takes the voicing of its last one other than v
/// (všetko fʃ, vždy vʒd), and the last obstruents of the word lose their voice (dub dup). eSpeak NG's notation shows only the loss
/// of voice inside a word; plain IPA also shows the gain (kde ɡɟe, shoda zɦoda).
fn assimilate(sounds: &mut [Phoneme]) {
    let mut start = 0;
    while start < sounds.len() {
        let length = sounds[start..].iter().take_while(|&&sound| is_obstruent(sound)).count();
        let cluster = &mut sounds[start..start + length];
        if let Some(last) = cluster.iter().rposition(|&sound| sound != V) {
            let voiced = is_voiced(cluster[last]);
            for sound in &mut cluster[..last] {
                *sound = if voiced {
                    voiced_inside(*sound)
                } else if *sound == H {
                    // eSpeak NG writes a devoiced h as h (ľahko ʎˈahko).
                    H_DEVOICED
                } else {
                    with_voicing(*sound, false)
                };
            }
        }
        start += length.max(1);
    }
    // A word without a vowel (the prepositions v, z) keeps its voice for the next word to settle.
    if sounds.iter().any(|sound| sound.is_vowel()) {
        for sound in sounds.iter_mut().rev() {
            if !is_obstruent(*sound) {
                break;
            }
            *sound = final_form(*sound);
        }
    }
}

/// How `word` is said, stressed on its first syllable, with eSpeak NG's secondary stress on the
/// third of a word of four syllables or more (rˈepublˌika); `None` for a letter Slovak does not
/// read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let lower: Vec<char> = word.to_lowercase().chars().collect();
    let mut sounds = letters(&lower)?;
    if sounds.is_empty() {
        return None;
    }
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
/// stressed (also once a preposition joins it: na večeru nˈavetʃˌeru).
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
    // eSpeak NG counts ô as two syllables, and the diphthongs of i inside a word (pôjdeme
    // pˈuojdʲˌeme, polievku pˈoliˌewku; but lietadlo, stretnutie).
    let last = said.syllables.len().saturating_sub(1);
    let weight = |(at, syllable): (usize, &jespersen_phonemes::Syllable)| {
        let inner_diphthong = at > 0 && at < last && syllable.onset.last() == Some(&I_GLIDE);
        1 + usize::from(syllable.nucleus == [UO] || inner_diphthong)
    };
    let total: usize = said.syllables.iter().enumerate().map(weight).sum();
    if !stressed_first || total < 4 {
        return;
    }
    let mut counted = 0;
    for (at, syllable) in said.syllables.iter_mut().enumerate() {
        let this = weight((at, syllable));
        counted += this;
        if counted >= 3 {
            if syllable.stress == Stress::Unstressed && counted - this < 3 {
                syllable.stress = Stress::Rhythmic;
            }
            break;
        }
    }
}

//! How a Slovenian word is said from its spelling: one letter, one sound, with dž and lj and nj
//! (soft before i and at the end); r as a syllable between consonants (vrt ʋərt); l and v after
//! a vowel at the end of a syllable said u̯ (bil, lev), v starting a word before a consonant said u
//! (vse); e as ə in a last syllable before r or n (dober); voicing that follows the last obstruent
//! of a cluster and is lost at the end of a word; and, as eSpeak NG reads it, the stress on the
//! last syllable but one, its vowel long.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, AA, B, CH, D, DZH, E, E_SHORT_STRESSED, EE, EE_OPEN, F, G, H_BEFORE_CONSONANT, I, II, J, K, L, LJ, M, N, NG, NY,
    O, O_SHORT_STRESSED, OO, OO_OPEN, P, R, R_SYLLABIC, S, SCHWA, SH, T, TS, U, UU, V, V_VOWEL, W, X, Z, ZH,
    is_obstruent, legal_onset, voices, with_voicing,
};

/// The sounds of `word`'s letters (in lower case), a vowel as its unstressed form; `None` for a
/// letter Slovenian does not read. In jespersen's respellings ə is a schwa, and é and ó open e and o.
fn letters(word: &[char]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(word.len());
    let mut at = 0;
    while at < word.len() {
        let next = word.get(at + 1).copied();
        let after = word.get(at + 2).copied();
        // lj and nj are one soft sound before i and at the end (knjiga, učitelj), l and n with j
        // elsewhere (ljudje, zelenjava).
        let soft = matches!(after, None | Some('i'));
        let (phonemes, step): (&[Phoneme], usize) = match (word[at], next) {
            ('d', Some('ž')) => (&[DZH], 2),
            ('l', Some('j')) if soft => (&[LJ], 2),
            ('n', Some('j')) if soft => (&[NY], 2),
            (c, _) => (
                match c {
                    'a' => &[A],
                    'b' => &[B],
                    'c' => &[TS],
                    'č' | 'ć' => &[CH],
                    'd' => &[D],
                    'e' => &[E],
                    'é' => &[EE],
                    'è' => &[E_SHORT_STRESSED],
                    'ê' => &[EE_OPEN],
                    'ə' => &[SCHWA],
                    'f' => &[F],
                    'g' => &[G],
                    // eSpeak NG writes h before a voiceless consonant, and at the end after i, h (lehko, svojih).
                    'h' if next.is_some_and(|next| "ptkcčsšf".contains(next))
                        || (next.is_none() && at > 0 && word[at - 1] == 'i') =>
                    {
                        &[H_BEFORE_CONSONANT]
                    }
                    'h' => &[X],
                    'i' | 'y' => &[I],
                    'j' => &[J],
                    'k' => &[K],
                    'l' => &[L],
                    'm' => &[M],
                    // n is ŋ before k and g (enkrat).
                    'n' if matches!(next, Some('k' | 'g')) => &[NG],
                    'n' => &[N],
                    'o' => &[O],
                    'ó' => &[OO],
                    'ò' => &[O_SHORT_STRESSED],
                    'ô' => &[OO_OPEN],
                    'á' => &[AA],
                    'í' => &[II],
                    'ú' => &[UU],
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

/// r with no vowel beside it is a syllable (vrt, trg); l and v after a vowel before a consonant or
/// at the end are u̯ (bil, volk, lev); v starting a word before a consonant is u (vse, v).
fn sonorants(sounds: &mut [Phoneme]) {
    let vowel = |sounds: &[Phoneme], at: Option<usize>| at.and_then(|at| sounds.get(at)).is_some_and(|s| s.is_vowel());
    for at in 0..sounds.len() {
        if sounds[at] == R && !vowel(sounds, at.checked_sub(1)) && !vowel(sounds, Some(at + 1)) {
            sounds[at] = R_SYLLABIC;
        }
    }
    for at in 0..sounds.len() {
        let after_vowel = vowel(sounds, at.checked_sub(1));
        let before_vowel = vowel(sounds, Some(at + 1));
        // l before j stays l (okolje), and v starting a word becomes u only before an obstruent
        // (vse, but vrata, vlak).
        let before_j = sounds.get(at + 1) == Some(&J);
        let before_liquid = matches!(sounds.get(at + 1), Some(&R | &J));
        let before_obstruent = sounds.get(at + 1).is_none_or(|&next| is_obstruent(next));
        let kept = if sounds[at] == V { before_liquid } else { before_j };
        if [L, V].contains(&sounds[at]) && after_vowel && !before_vowel && !kept {
            sounds[at] = W;
        } else if sounds[at] == V && at == 0 && before_obstruent {
            sounds[at] = V_VOWEL;
        }
    }
}

/// Voicing inside a word: a cluster of obstruents takes the voicing of its last one, and the
/// obstruents ending the word lose their voice (knjig knjik), unless it has no vowel (z).
fn assimilate(sounds: &mut [Phoneme]) {
    let mut start = 0;
    while start < sounds.len() {
        let length = sounds[start..].iter().take_while(|&&sound| is_obstruent(sound)).count();
        if length > 1 {
            let voiced = voices(sounds[start + length - 1]);
            for sound in &mut sounds[start..start + length - 1] {
                *sound = with_voicing(*sound, voiced);
            }
        }
        start += length.max(1);
    }
    if sounds.iter().any(|sound| sound.is_vowel() && *sound != V_VOWEL) {
        for sound in sounds.iter_mut().rev() {
            if !is_obstruent(*sound) {
                break;
            }
            *sound = with_voicing(*sound, false);
        }
    }
}

/// Which sound is stressed: the vowel of the last syllable but one, or of the last in words
/// ending in -aj, -al and -ač (zjutraj, pripovedoval, kovač), as eSpeak NG reads them; a schwa
/// and the u of v are passed over.
fn stressed(sounds: &[Phoneme], word: &str) -> Option<usize> {
    // A respelling's accented vowel (desêt, žógo) is stressed.
    let open =
        |sound: &Phoneme| [AA, EE, EE_OPEN, II, OO, OO_OPEN, UU, E_SHORT_STRESSED, O_SHORT_STRESSED].contains(sound);
    if let Some(at) = sounds.iter().position(open) {
        return Some(at);
    }
    let candidates: Vec<usize> = (0..sounds.len())
        .filter(|&at| sounds[at].is_vowel() && sounds[at] != SCHWA && sounds[at] != V_VOWEL)
        .collect();
    let last = candidates.len().checked_sub(1)?;
    let final_stress = ["aj", "al", "il", "ač", "oj"]
        .iter()
        .any(|ending| word.ends_with(ending));
    Some(candidates[if final_stress || last == 0 { last } else { last - 1 }])
}

/// The long form of a stressed vowel.
fn long(vowel: Phoneme) -> Phoneme {
    match vowel {
        v if v == A => AA,
        v if v == E => EE,
        v if v == I => II,
        v if v == O => OO,
        v if v == U => UU,
        v => v,
    }
}

/// How `word` is said; `None` for a letter Slovenian does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let lower = word.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();
    let mut sounds = letters(&chars)?;
    if sounds.is_empty() {
        return None;
    }
    sonorants(&mut sounds);
    assimilate(&mut sounds);
    let stress = stressed(&sounds, &lower);
    // An unstressed e in the last syllable before a final r or n is a schwa (dober, preden).
    if let Some(last) = sounds.iter().rposition(|sound| sound.is_vowel())
        && Some(last) != stress
        && sounds[last] == E
        && last + 2 == sounds.len()
        && [R, N].contains(&sounds[last + 1])
    {
        sounds[last] = SCHWA;
    }
    let segments: Vec<Segment> = sounds
        .iter()
        .enumerate()
        .map(|(at, &sound)| {
            if Some(at) == stress {
                Segment::new(long(sound), Stress::Primary)
            } else {
                Segment::new(sound, Stress::Unstressed)
            }
        })
        .collect();
    Some(syllabify(&segments, legal_onset))
}

/// The voicing a word's last obstruents take from the next word (brat dela brad).
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

/// The short vowel of a syllable that lost its stress in running text (je jɛ, bila bˌila).
pub(crate) fn shorten(said: &mut Pronunciation) {
    for syllable in &mut said.syllables {
        if syllable.stress == Stress::Primary {
            continue;
        }
        for vowel in &mut syllable.nucleus {
            *vowel = match *vowel {
                v if v == AA => A,
                v if v == EE || v == EE_OPEN || v == E_SHORT_STRESSED => E,
                v if v == II => I,
                v if v == OO || v == OO_OPEN || v == O_SHORT_STRESSED => O,
                v if v == UU => U,
                v => v,
            };
        }
    }
}

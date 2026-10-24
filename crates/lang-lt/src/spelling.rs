//! How a Lithuanian word is said from its spelling: each vowel as written, long in ą, ę, ė, į, y,
//! ū and ų; the diphthongs ie, uo, ai, au and ui, and ei as e and i; an i between a consonant and
//! a back vowel only softening the consonant (čia tɕˈe, kelionė); the consonants soft before a
//! front vowel, and t, d, n, s, z, l, š and ž before a soft consonant too; b, d, g, z and ž
//! unvoiced at the end of a word; and the stress, as eSpeak NG places it when no dictionary says
//! otherwise, on the last syllable but one (or where jespersen's list marks it).

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, A_LONG, AI, AU, B, CH, CH_BACK, D, DZ, DZH, DZH_BACK, E, E_LONG, EE, EI, EU, F, G, H, I, I_STRESSED, IE, II, J,
    K, L, L_SOFT_BACK, M, N, N_BACK, O, P, R, S, S_BACK, SH, SH_BACK, T, TS, U, UI, UO, UU, V, X, Z, ZH, ZH_BACK, hard,
    is_soft, legal_onset, soft,
};

/// What a letter, or a group of them, comes to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Unit {
    phoneme: Phoneme,
    /// Whether the letters were marked stressed (jespersen's list: penkì).
    stressed: bool,
}

/// `c` without a stress mark (an acute, grave or tilde accent, as Lithuanian dictionaries write
/// them), and whether it had one.
fn plain(c: char) -> (char, bool) {
    match c {
        'á' | 'à' | 'ã' => ('a', true),
        'é' | 'è' | 'ẽ' => ('e', true),
        'í' | 'ì' | 'ĩ' => ('i', true),
        'ó' | 'ò' | 'õ' => ('o', true),
        'ú' | 'ù' | 'ũ' => ('u', true),
        'ý' | 'ỳ' | 'ỹ' => ('y', true),
        _ => (c, false),
    }
}

fn is_vowel_letter(c: Option<char>) -> bool {
    matches!(
        c,
        Some('a' | 'ą' | 'e' | 'ę' | 'ė' | 'i' | 'į' | 'y' | 'o' | 'u' | 'ū' | 'ų')
    )
}

/// The vowel after an i that only softens the consonant before it, and how many letters it takes
/// with the i: ia is e, iu u, io o (čia, myliu, kelionė).
fn after_soft_i(rest: &[char]) -> Option<(&'static [Phoneme], usize)> {
    Some(match rest {
        ['i', 'a', 'u', ..] => (&[E, U], 3),
        // -iai is one syllable, ɛi (tūkstančiai, svečiai).
        ['i', 'a', 'i', ..] => (&[EI], 3),
        ['i', 'a' | 'ą', ..] => (if rest[1] == 'a' { &[E] } else { &[E_LONG] }, 2),
        ['i', 'u', 'o', ..] => (&[UO], 3),
        ['i', 'u', ..] => (&[U], 2),
        ['i', 'ū' | 'ų', ..] => (&[UU], 2),
        ['i', 'o', ..] => (&[O], 2),
        _ => return None,
    })
}

/// The front vowels, before which a consonant is soft.
fn is_front(phoneme: Phoneme) -> bool {
    [E, E_LONG, EE, I, I_STRESSED, II, IE, EI].contains(&phoneme)
}

/// The units of `word`'s letters (in lower case, without stress marks, which `marks` gives);
/// `None` for a letter Lithuanian does not read. A soft consonant before a back vowel is marked by
/// the i after it; the rest are softened afterwards.
fn units(word: &[char], marks: &[bool]) -> Option<(Vec<Unit>, Vec<bool>)> {
    let mut units: Vec<Unit> = Vec::with_capacity(word.len());
    // Whether each unit is a consonant an i softens before a back vowel.
    let mut softened: Vec<bool> = Vec::with_capacity(word.len());
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        let next = word.get(at + 1).copied();
        let stressed = marks[at] || marks.get(at + 1).copied().unwrap_or(false) && is_vowel_letter(next);
        let after_consonant = units.last().is_some_and(|unit| !unit.phoneme.is_vowel());
        if c == 'i'
            && after_consonant
            && let Some((vowels, length)) = after_soft_i(&word[at..])
        {
            if let Some(last) = softened.last_mut() {
                *last = true;
            }
            let marked = marks[at..at + length].iter().any(|&mark| mark);
            for (index, &vowel) in vowels.iter().enumerate() {
                units.push(Unit {
                    phoneme: vowel,
                    stressed: marked && index + 1 == vowels.len(),
                });
                softened.push(false);
            }
            at += length;
            continue;
        }
        let (phonemes, step): (&[Phoneme], usize) = match (c, next) {
            ('i', Some('e')) => (&[IE], 2),
            ('u', Some('o')) => (&[UO], 2),
            ('a', Some('i')) => (&[AI], 2),
            ('a', Some('u')) => (&[AU], 2),
            ('e', Some('u')) => (&[EU], 2),
            ('u', Some('i')) => (&[UI], 2),
            // ei is e and i, the stress on the i (eina eˈɪna).
            ('e', Some('i')) => (&[E, I], 2),
            ('a', _) => (&[A], 1),
            ('ą', _) => (&[A_LONG], 1),
            ('e', _) => (&[E], 1),
            ('ę', _) => (&[E_LONG], 1),
            ('ė', _) => (&[EE], 1),
            ('i', _) => (&[I], 1),
            ('į' | 'y', _) => (&[II], 1),
            ('o', _) => (&[O], 1),
            ('u', _) => (&[U], 1),
            ('ū' | 'ų', _) => (&[UU], 1),
            ('c', Some('h')) => (&[X], 2),
            ('d', Some('ž')) => (&[DZH], 2),
            ('d', Some('z')) => (&[DZ], 2),
            ('b', _) => (&[B], 1),
            ('c', _) => (&[TS], 1),
            ('č', _) => (&[CH], 1),
            ('d', _) => (&[D], 1),
            ('f', _) => (&[F], 1),
            ('g', _) => (&[G], 1),
            ('h', _) => (&[H], 1),
            ('j', _) => (&[J], 1),
            ('k' | 'q', _) => (&[K], 1),
            ('l', _) => (&[L], 1),
            ('m', _) => (&[M], 1),
            ('n', _) => (&[N], 1),
            ('p', _) => (&[P], 1),
            ('r', _) => (&[R], 1),
            ('s', _) => (&[S], 1),
            ('š', _) => (&[SH], 1),
            ('t', _) => (&[T], 1),
            ('v' | 'w', _) => (&[V], 1),
            ('x', _) => (&[K, S], 1),
            ('z', _) => (&[Z], 1),
            ('ž', _) => (&[ZH], 1),
            ('\'' | '’' | '-', _) => (&[], 1),
            _ => return None,
        };
        let count = phonemes.len();
        for (index, &phoneme) in phonemes.iter().enumerate() {
            // A mark on ei is the i's (eina).
            let marked = stressed && phoneme.is_vowel() && (count == 1 || index + 1 == count);
            units.push(Unit {
                phoneme,
                stressed: marked,
            });
            softened.push(false);
        }
        at += step;
    }
    Some((units, softened))
}

/// The consonants that a soft consonant after them softens too (gatvė ɡˈatʲvʲee, penki; not d: dvi).
const CORONALS: [Phoneme; 7] = [T, N, S, Z, L, SH, ZH];

/// The voiced obstruents unvoiced at the end of a word (daug dˈauk, kad).
const UNVOICED: [(Phoneme, Phoneme); 7] = [(B, P), (D, T), (G, K), (Z, S), (ZH, SH), (DZ, TS), (DZH, CH)];

/// The voiceless obstruents and their voiced pairs.
const VOICING: [(Phoneme, Phoneme); 8] = [(P, B), (T, D), (K, G), (S, Z), (SH, ZH), (TS, DZ), (CH, DZH), (X, H)];

/// An obstruent takes the voice of the obstruent after it (mėgsta mʲˈeeksta, futbolas fudbˈoːl̩as),
/// but not before v.
fn assimilate(units: &mut [Unit]) {
    for at in (0..units.len().saturating_sub(1)).rev() {
        let (phoneme, next) = (units[at].phoneme, units[at + 1].phoneme);
        let voiceless_next = VOICING.iter().any(|(voiceless, _)| *voiceless == next);
        let voiced_next = VOICING.iter().any(|(_, voiced)| *voiced == next);
        // š and ž before s and z are said as s and z (užsienis usˈienʲis).
        if [S, Z].contains(&next) && [SH, ZH].contains(&phoneme) {
            units[at].phoneme = if next == S { S } else { Z };
        } else if voiceless_next && let Some(&(voiceless, _)) = VOICING.iter().find(|(_, voiced)| *voiced == phoneme) {
            units[at].phoneme = voiceless;
        } else if voiced_next && let Some(&(_, voiced)) = VOICING.iter().find(|(voiceless, _)| *voiceless == phoneme) {
            units[at].phoneme = voiced;
        }
    }
}

/// How `word` is said; `None` for a letter Lithuanian does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    // A combining accent marks the letter before it (tū́kstantis).
    let mut chars: Vec<char> = Vec::with_capacity(word.len());
    let mut marks: Vec<bool> = Vec::with_capacity(word.len());
    for c in word.to_lowercase().chars() {
        if matches!(c, '\u{300}' | '\u{301}' | '\u{303}') {
            if let Some(mark) = marks.last_mut() {
                *mark = true;
            }
            continue;
        }
        let (plain, mark) = plain(c);
        chars.push(plain);
        marks.push(mark);
    }
    let (mut units, softened) = units(&chars, &marks)?;
    if units.is_empty() {
        return None;
    }
    if let Some(last) = units.last_mut()
        && let Some(&(_, voiceless)) = UNVOICED.iter().find(|(voiced, _)| *voiced == last.phoneme)
    {
        last.phoneme = voiceless;
    }
    assimilate(&mut units);
    // Soft before a front vowel or a softening i, and the coronals before a soft consonant.
    for at in (0..units.len()).rev() {
        let phoneme = units[at].phoneme;
        if phoneme.is_vowel() {
            continue;
        }
        let next = units.get(at + 1).map(|unit| unit.phoneme);
        let front = next.is_some_and(is_front);
        // č and dž, soft in themselves, soften nothing before them (tūkstančiai).
        let before_soft = next
            .is_some_and(|next| !next.is_vowel() && is_soft(next) && hard(next) != CH && hard(next) != DZH)
            && CORONALS.contains(&phoneme);
        if softened[at] {
            let back = next.is_some_and(|next| !is_front(next));
            units[at].phoneme = match phoneme {
                phoneme if back && phoneme == L => L_SOFT_BACK,
                phoneme if back && phoneme == SH => SH_BACK,
                phoneme if back && phoneme == CH => CH_BACK,
                phoneme if back && phoneme == DZH => DZH_BACK,
                phoneme if back && phoneme == N => N_BACK,
                phoneme if back && phoneme == S => S_BACK,
                phoneme if back && phoneme == ZH => ZH_BACK,
                phoneme => soft(phoneme),
            };
        } else if front || before_soft {
            units[at].phoneme = soft(phoneme);
        }
    }
    // A consonant said twice is said once (užsienis usˈienʲis).
    units.dedup_by(|second, first| !first.phoneme.is_vowel() && first.phoneme == second.phoneme);
    let vowels: Vec<usize> = (0..units.len()).filter(|&at| units[at].phoneme.is_vowel()).collect();
    let stressed = units.iter().position(|unit| unit.stressed).or_else(|| {
        vowels
            .len()
            .checked_sub(2)
            .map(|at| vowels[at])
            .or_else(|| vowels.first().copied())
    });
    let segments: Vec<Segment> = units
        .iter()
        .enumerate()
        .map(|(at, unit)| {
            if Some(at) == stressed {
                let phoneme = if unit.phoneme == I { I_STRESSED } else { unit.phoneme };
                Segment::new(phoneme, Stress::Primary)
            } else {
                Segment::new(unit.phoneme, Stress::Unstressed)
            }
        })
        .collect();
    Some(syllabify(&segments, legal_onset))
}

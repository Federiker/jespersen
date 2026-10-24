//! How an Albanian word is said from its spelling: the digraphs dh, gj, ll, nj, rr, sh, th, xh
//! and zh; ç, q, x and y; ë as ə; u before a vowel and j after one as glides (grua, muaj); a ə
//! before rr at the start of a word (rrugë ərˈuɡə); and, as eSpeak NG reads it, the stress on
//! the last vowel but one, with rhythmic stresses before it.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, A_FINAL, B, CH, D, DH, DZ, DZH, E, E_CLOSE, F, G, GJ, H, I, I_LAX, J, J_GLIDE, K, L, L_FINAL, LL, M, N, NJ, O,
    P, Q, R, RR, S, SCHWA, SCHWA_STRESSED, SH, T, TH, TS, U, V, W, Y, Z, ZH, legal_onset,
};

/// What a letter, or a digraph, comes to before the stress is known.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Unit {
    /// A vowel letter (a, e, ë, i, o, u, y), or ə for the one said before an rr starting a word.
    Vowel(char),
    Sound(Phoneme),
}

fn is_vowel(c: Option<char>) -> bool {
    matches!(c, Some('a' | 'e' | 'ë' | 'i' | 'o' | 'u' | 'y'))
}

/// The consonant at the start of `rest` and how many letters it takes; `None` for a letter
/// Albanian does not read.
fn consonant(rest: &[char]) -> Option<(Phoneme, usize)> {
    let next = rest.get(1).copied();
    Some(match (rest[0], next) {
        ('d', Some('h')) => (DH, 2),
        ('g', Some('j')) => (GJ, 2),
        ('l', Some('l')) if rest.len() == 2 => (L_FINAL, 2),
        ('l', Some('l')) => (LL, 2),
        ('n', Some('j')) => (NJ, 2),
        ('r', Some('r')) => (RR, 2),
        ('s', Some('h')) => (SH, 2),
        ('t', Some('h')) => (TH, 2),
        ('x', Some('h')) => (DZH, 2),
        ('z', Some('h')) => (ZH, 2),
        ('b', _) => (B, 1),
        ('c', _) => (TS, 1),
        ('ç', _) => (CH, 1),
        ('d', _) => (D, 1),
        ('f', _) => (F, 1),
        ('g', _) => (G, 1),
        ('h', _) => (H, 1),
        ('j', _) => (J, 1),
        ('k', _) => (K, 1),
        ('l', _) if rest.len() == 1 => (L_FINAL, 1),
        ('l', _) => (L, 1),
        ('m', _) => (M, 1),
        ('n', _) => (N, 1),
        ('p', _) => (P, 1),
        ('q', _) => (Q, 1),
        ('r', _) => (R, 1),
        ('s', _) => (S, 1),
        ('t', _) => (T, 1),
        ('v' | 'w', _) => (V, 1),
        ('x', _) => (DZ, 1),
        ('z', _) => (Z, 1),
        _ => return None,
    })
}

/// The units of `word`'s letters, in lower case; `None` for a letter Albanian does not read.
fn units(word: &[char]) -> Option<Vec<Unit>> {
    let mut units: Vec<Unit> = Vec::with_capacity(word.len());
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        let next = word.get(at + 1).copied();
        let after_vowel = matches!(units.last(), Some(Unit::Vowel(_)));
        match c {
            // u between a consonant and a is a glide (grua ɡrwˈɑ, shkruaj).
            'u' if !after_vowel && at > 0 && next == Some('a') => units.push(Unit::Sound(W)),
            // i before e after a consonant is j (qielli tɕjˈelli).
            'i' if !after_vowel && at > 0 && next == Some('e') => units.push(Unit::Sound(J)),
            _ if is_vowel(Some(c)) => units.push(Unit::Vowel(c)),
            // j after a, e or o, and after any vowel before a consonant or the end, is a glide (majë
            // mˈaɪə, muaj, vajza).
            'j' if after_vowel && (!is_vowel(next) || matches!(units.last(), Some(Unit::Vowel('a' | 'e' | 'o')))) => {
                units.push(Unit::Sound(J_GLIDE));
            }
            // rr at the start of a word takes a ə before it (rrugë ərˈuɡə).
            'r' if at == 0 && next == Some('r') => {
                units.extend([Unit::Vowel('ə'), Unit::Sound(RR)]);
                at += 2;
                continue;
            }
            '\'' | '’' | '-' => {}
            _ => {
                let (sound, length) = consonant(&word[at..])?;
                units.push(Unit::Sound(sound));
                at += length;
                continue;
            }
        }
        at += 1;
    }
    Some(units)
}

/// The phoneme of the vowel unit at `index`, stressed or not.
fn vowel(units: &[Unit], index: usize, letter: char, stressed: bool) -> Phoneme {
    let before = index.checked_sub(1).map(|before| units[before]);
    let last = index + 1 == units.len();
    // Whether only consonants follow, at least one (a closed last syllable).
    let closed_last = !last && units[index + 1..].iter().all(|unit| matches!(unit, Unit::Sound(_)));
    match letter {
        // a is ɑ at the end of a word, after the glide of ua, before rr and before a final r (nata,
        // grua, harro, fshatar).
        'a' if last
            || before == Some(Unit::Sound(W))
            || units.get(index + 1) == Some(&Unit::Sound(RR))
            || (index + 2 == units.len() && units[index + 1] == Unit::Sound(R)) =>
        {
            A_FINAL
        }
        'a' => A,
        // e is close after j, q, nj and sh and before sh (dhjetë, qen, njerëz, shet, shpesh,
        // mrekullueshme), and after l but unstressed in a closed last syllable (fle, lehtë,
        // problem; but malet mˈaɫɛt).
        'e' if matches!(before, Some(Unit::Sound(sound)) if [J, Q, NJ, SH].contains(&sound))
            || units.get(index + 1) == Some(&Unit::Sound(SH)) =>
        {
            E_CLOSE
        }
        'e' if before == Some(Unit::Sound(L)) && (stressed || !closed_last) => E_CLOSE,
        'e' => E,
        'ë' if stressed => SCHWA_STRESSED,
        'ë' | 'ə' => SCHWA,
        'i' if !stressed && closed_last => I_LAX,
        'i' => I,
        'o' => O,
        'u' => U,
        _ => Y,
    }
}

/// The endings of definite and plural forms, which leave the stress on the syllable before
/// (malet mˈaɫɛt, qytetin tɕytˈɛtɪn, kodrave), and of the participles in -ur (rritur).
const SUFFIXES: [&str; 6] = ["et", "in", "it", "ët", "ave", "ur"];

/// Which unit (among `units`, whose vowels are at `vowels`) is stressed, as eSpeak NG reads it:
/// the last vowel of a word ending in a consonant (telefon, profesor), else the last but one
/// (Tirana, muaj), and the last but one too before a final ë or the endings -et, -in, -it and -ët
/// (libër, malet); a ë there leaves it on the vowel before, if any (freskëta frˈɛskəta, but
/// shtëpi).
fn stressed(units: &[Unit], word: &[char], vowels: &[usize]) -> Option<usize> {
    let &last = vowels.last()?;
    let text: String = word.iter().collect();
    let consonant_end = matches!(units.last(), Some(Unit::Sound(sound)) if *sound != J_GLIDE);
    if vowels.len() == 1 {
        return Some(last);
    }
    // An ending's vowels are passed over (kodrave kˈɔdravɛ).
    if consonant_end || text.ends_with("ave") {
        if let Some(suffix) = SUFFIXES.iter().find(|suffix| text.ends_with(*suffix)) {
            let skip = suffix.chars().filter(|&c| is_vowel(Some(c))).count();
            return vowels.len().checked_sub(skip + 1).map(|at| vowels[at]);
        }
        if units[last] != Unit::Vowel('ë') && consonant_end {
            return Some(last);
        }
    }
    let mut at = vowels.len() - 2;
    if !consonant_end && units[vowels[at]] == Unit::Vowel('ë') && at > 0 && units[vowels[at - 1]] != Unit::Vowel('ə')
    {
        at -= 1;
    }
    Some(vowels[at])
}

/// How `word` is said; `None` for a letter Albanian does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let chars: Vec<char> = word.to_lowercase().chars().collect();
    let units = units(&chars)?;
    let vowels: Vec<usize> = (0..units.len())
        .filter(|&index| matches!(units[index], Unit::Vowel(_)))
        .collect();
    let stressed = stressed(&units, &chars, &vowels);
    // The rhythmic stresses: every other vowel from the first, up to two before the stress.
    let stress_position = vowels.iter().position(|&index| Some(index) == stressed).unwrap_or(0);
    let rhythmic: Vec<usize> = vowels
        .iter()
        .take(stress_position.saturating_sub(1))
        .step_by(2)
        .copied()
        .filter(|&index| units[index] != Unit::Vowel('ə'))
        .collect();
    let segments: Vec<Segment> = units
        .iter()
        .enumerate()
        .map(|(index, unit)| match *unit {
            Unit::Vowel(letter) => {
                let stress = if Some(index) == stressed {
                    Stress::Primary
                } else if rhythmic.contains(&index) {
                    Stress::Rhythmic
                } else {
                    Stress::Unstressed
                };
                Segment::new(vowel(&units, index, letter, stress != Stress::Unstressed), stress)
            }
            Unit::Sound(sound) => Segment::new(sound, Stress::Unstressed),
        })
        .collect();
    if segments.is_empty() {
        return None;
    }
    Some(syllabify(&segments, legal_onset))
}

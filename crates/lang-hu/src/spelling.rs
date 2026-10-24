//! How a Hungarian word is said from its spelling: each vowel as written, long with an accent
//! (á, é, ő); the digraphs cs, dz, dzs, gy, ly, ny, sz, ty and zs; a doubled consonant long
//! (kettő, asszony, meggy); t, d, gy, n and l before j joined to it (látja lˈaːcːɑ, hallja); and
//! the stress on the first syllable.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, AA, B, CH, D, DZ, DZH, E, EE, F, G, GY, H, I, II, J, K, L, M, N, NY, O, OE, OEOE, OO, P, R, S, SH, T, TS, TY, U,
    UE, UEUE, UU, V, Z, ZH, doubled, legal_onset,
};

/// The consonant a letter or digraph at the start of `rest` is, and how many letters it takes.
fn consonant(rest: &[char]) -> Option<(Phoneme, usize)> {
    let next = rest.get(1).copied();
    let after = rest.get(2).copied();
    Some(match (rest[0], next) {
        ('d', Some('z')) if after == Some('s') => (DZH, 3),
        ('d', Some('z')) => (DZ, 2),
        // csz is c and sz (kilencszáz).
        ('c', Some('s')) if after == Some('z') => (TS, 1),
        ('c', Some('s' | 'h')) => (CH, 2),
        ('g', Some('y')) => (GY, 2),
        ('l', Some('y')) => (J, 2),
        ('n', Some('y')) => (NY, 2),
        ('s', Some('z')) => (S, 2),
        ('t', Some('y')) => (TY, 2),
        ('t', Some('h')) => (T, 2),
        ('z', Some('s')) => (ZH, 2),
        ('b', _) => (B, 1),
        ('c', _) => (TS, 1),
        ('d', _) => (D, 1),
        ('f', _) => (F, 1),
        ('g', _) => (G, 1),
        ('h', _) => (H, 1),
        ('j', _) => (J, 1),
        ('k' | 'q', _) => (K, 1),
        ('l', _) => (L, 1),
        ('m', _) => (M, 1),
        ('n', _) => (N, 1),
        ('p', _) => (P, 1),
        ('r', _) => (R, 1),
        ('s', _) => (SH, 1),
        ('t', _) => (T, 1),
        ('v' | 'w', _) => (V, 1),
        ('z', _) => (Z, 1),
        _ => return None,
    })
}

fn vowel(c: char) -> Option<Phoneme> {
    Some(match c {
        'a' => A,
        'á' => AA,
        'e' => E,
        'é' => EE,
        'i' | 'y' => I,
        'í' => II,
        'o' => O,
        'ó' => OO,
        'ö' => OE,
        'ő' | 'ô' => OEOE,
        'u' => U,
        'ú' => UU,
        'ü' => UE,
        'ű' | 'û' => UEUE,
        _ => return None,
    })
}

/// The sounds of `word`'s letters, in lower case; `None` for a letter Hungarian does not read.
fn sounds(word: &[char]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(word.len());
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        if let Some(vowel) = vowel(c) {
            sounds.push(vowel);
            at += 1;
            continue;
        }
        if matches!(c, '\'' | '’' | '-') {
            at += 1;
            continue;
        }
        if c == 'x' {
            sounds.extend([K, S]);
            at += 1;
            continue;
        }
        let (sound, length) = consonant(&word[at..])?;
        // A doubled consonant is long (tt, ll), and so is a digraph with its first letter doubled
        // (ssz, ggy, nny).
        let (sound, length, long) = if word.get(at + 1) == Some(&c) {
            match consonant(&word[at + 1..]) {
                Some((digraph, digraph_length)) if digraph_length >= 2 => (digraph, 1 + digraph_length, true),
                Some((same, 1)) if same == sound && length == 1 => (sound, 2, true),
                _ => (sound, length, false),
            }
        } else {
            (sound, length, false)
        };
        at += length;
        // sz before s is said with it (egészség ˈɛɡeːʃeːɡ).
        if sound == S && word.get(at) == Some(&'s') && word.get(at + 1) != Some(&'z') {
            continue;
        }
        // t, d, gy, ty, n and l before j are said as one long sound (látja, adja, hallja).
        if word.get(at) == Some(&'j') {
            let joined = match sound {
                sound if sound == T || sound == TY => Some(TY),
                sound if sound == D || sound == GY => Some(GY),
                sound if sound == N => Some(NY),
                sound if sound == L || sound == J => Some(J),
                _ => None,
            };
            if let Some(joined) = joined {
                // After a consonant it is short (mondja mˈonɟɑ).
                sounds.push(joined);
                if sounds.len() < 2 || sounds[sounds.len() - 2].is_vowel() {
                    sounds.push(doubled(joined));
                }
                at += 1;
                continue;
            }
        }
        sounds.push(sound);
        // A long consonant before another consonant is said short (leszállt lˈɛsaːlt).
        let long = long && word.get(at).is_none_or(|&next| vowel(next).is_some());
        // dz between vowels is long (madzag mˈɑdzːɑɡ).
        if sound == DZ
            && sounds.len() >= 2
            && sounds[sounds.len() - 2].is_vowel()
            && word.get(at).copied().and_then(vowel).is_some()
        {
            sounds.push(doubled(sound));
        }
        // A long r at the end of a word is said short (orr).
        if long && !(sound == R && at == word.len()) {
            sounds.push(doubled(sound));
        }
    }
    unvoice(&mut sounds);
    Some(sounds)
}

/// The voiced obstruents and their voiceless pairs; v keeps its voice (könyvtár kˈøɲvtaːr).
const VOICING: [(Phoneme, Phoneme); 8] = [(B, P), (D, T), (G, K), (GY, TY), (Z, S), (ZH, SH), (DZ, TS), (DZH, CH)];

/// A voiced obstruent before a voiceless one loses its voice (azt ˈɑst, kérdezte kˈeːrdɛstɛ),
/// and a stop then said twice is one long one (mondta mˈontːɑ), as is t before c (utca ˈutsːɑ).
fn unvoice(sounds: &mut [Phoneme]) {
    let voiceless: Vec<Phoneme> = VOICING.iter().map(|(_, voiceless)| *voiceless).collect();
    for at in (0..sounds.len().saturating_sub(1)).rev() {
        if voiceless.contains(&sounds[at + 1])
            && let Some(&(_, pair)) = VOICING.iter().find(|(voiced, _)| *voiced == sounds[at])
        {
            sounds[at] = pair;
        }
    }
    for at in (0..sounds.len().saturating_sub(1)).rev() {
        let (first, second) = (sounds[at], sounds[at + 1]);
        if [P, T, K, TY, TS, CH].contains(&second) && (first == second || (first == T && second == TS)) {
            sounds[at] = second;
            sounds[at + 1] = doubled(second);
        }
    }
}

/// How `word` is said, stressed on its first syllable; `None` for a letter Hungarian does not
/// read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let chars: Vec<char> = word.to_lowercase().chars().collect();
    let sounds = sounds(&chars)?;
    if sounds.is_empty() {
        return None;
    }
    let first_vowel = sounds.iter().position(|sound| sound.is_vowel());
    let segments: Vec<Segment> = sounds
        .iter()
        .enumerate()
        .map(|(at, &sound)| {
            let stress = if Some(at) == first_vowel {
                Stress::Primary
            } else {
                Stress::Unstressed
            };
            Segment::new(sound, stress)
        })
        .collect();
    Some(syllabify(&segments, legal_onset))
}

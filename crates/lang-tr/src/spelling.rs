//! How a Turkish word is said from its spelling: one letter, one sound (ğ lengthens the vowel
//! before it, or is a j between front vowels), stressed on the last syllable unless a suffix that
//! takes no stress (-yor, -sınız, -yim, -dır) or jespersen's word list moves it earlier.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, Syllable, syllabify};

use crate::{
    phonemes::{
        A, B, BACK_I, C, CH, D, E, E_LAX, E_WIDE, F, G, G_FRONT, H, I, I_LAX, J, K, K_FIRST, K_LONG, L, L_DARK, M, N,
        O, O_LAX, OE, OE_OPEN, P, P_FIRST, P_LONG, R, R_TAP, S, SH, T, T_FIRST, T_LONG, U, U_LAX, UE, UE_LAX, V, Z, ZH,
        is_front, legal_onset, long,
    },
    words::respelling,
};

/// Lower case as Turkish has it: I is ı and İ is i.
pub(crate) fn lower(word: &str) -> String {
    word.chars()
        .flat_map(|c| match c {
            'I' => vec!['ı'],
            'İ' => vec!['i'],
            c => c.to_lowercase().collect(),
        })
        .filter(|c| *c != '\u{307}')
        .collect()
}

fn vowel(c: char) -> Option<Phoneme> {
    Some(match c {
        'a' | 'â' => A,
        'e' => E,
        'ı' => BACK_I,
        'i' | 'î' => I,
        'o' => O,
        'ö' => OE,
        'u' | 'û' => U,
        'ü' => UE,
        _ => return None,
    })
}

/// Suffixes that take no stress, which falls on the syllable before them: the present -yor, the
/// endings of person after a noun, the copula -dır.
const UNSTRESSED_SUFFIXES: [&str; 19] = [
    "sınız", "siniz", "sunuz", "sünüz", "yım", "yim", "yum", "yüm", "yız", "yiz", "yuz", "dır", "dir", "dur", "dür",
    "tır", "tir", "tur", "tür",
];

/// One sound and where its letter is.
#[derive(Clone, Copy)]
struct Sound {
    phoneme: Phoneme,
    letter: usize,
    /// A vowel written with a circumflex (kâğıt, hâlâ), which keeps g and l clear.
    palatal: bool,
}

/// How `word` is said; `None` for a letter Turkish does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let lower = lower(word);
    // A name and its suffix, written apart (Ankara'da): the name keeps its stress, on its last
    // syllable unless the list says otherwise.
    let (name, suffix) = lower.split_once(['\'', '’']).unwrap_or((&lower, ""));
    let marked_name = respelling(name).map_or_else(|| name.to_owned(), str::to_owned);
    let spelt = format!("{marked_name}{suffix}");
    let letters: Vec<char> = spelt.chars().filter(|c| *c != 'ˈ').collect();
    let sounds = sounds(&letters)?;
    let vowels: Vec<usize> = (0..sounds.len())
        .filter(|&index| sounds[index].phoneme.is_vowel())
        .collect();
    let stressed = if let Some(mark) = spelt.find('ˈ') {
        let letter = spelt[..mark].chars().count();
        vowels.iter().find(|&&index| sounds[index].letter >= letter).copied()
    } else if !suffix.is_empty() {
        let name_end = marked_name.chars().count();
        vowels
            .iter()
            .rev()
            .find(|&&index| sounds[index].letter < name_end)
            .copied()
    } else {
        before_unstressed_suffix(&letters, &sounds, &vowels)
    }
    .or_else(|| vowels.last().copied());
    let segments: Vec<Segment> = sounds
        .iter()
        .enumerate()
        .map(|(index, sound)| {
            Segment::new(
                sound.phoneme,
                if Some(index) == stressed {
                    Stress::Primary
                } else {
                    Stress::Unstressed
                },
            )
        })
        .collect();
    let palatal: Vec<bool> = sounds
        .iter()
        .filter(|sound| sound.phoneme.is_vowel())
        .map(|sound| sound.palatal)
        .collect();
    let mut said = syllabify(&segments, legal_onset);
    allophones(&mut said, &palatal);
    Some(said)
}

/// The vowel before a suffix that takes no stress (gidiyorum: the i of di), if the word has one.
fn before_unstressed_suffix(letters: &[char], sounds: &[Sound], vowels: &[usize]) -> Option<usize> {
    let text: String = letters.iter().collect();
    let byte = text.find("yor").filter(|&found| found > 0).or_else(|| {
        UNSTRESSED_SUFFIXES
            .iter()
            .filter(|suffix| text.len() > suffix.len() + 2)
            .find_map(|suffix| text.strip_suffix(suffix).map(str::len))
    })?;
    let letter = text[..byte].chars().count();
    vowels
        .iter()
        .rev()
        .find(|&&index| sounds[index].letter < letter)
        .copied()
}

/// The sounds of a word's letters (in lower case); `None` for a letter Turkish does not read.
fn sounds(letters: &[char]) -> Option<Vec<Sound>> {
    let mut sounds: Vec<Sound> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let c = letters[at];
        let next = letters.get(at + 1).copied();
        if let Some(phoneme) = vowel(c) {
            sounds.push(Sound {
                phoneme,
                letter: at,
                palatal: matches!(c, 'â' | 'î' | 'û'),
            });
            at += 1;
            continue;
        }
        if c == 'ğ' {
            soft_g(&mut sounds, next.and_then(vowel), at);
            at += 1;
            continue;
        }
        if matches!(c, 'p' | 't' | 'k') && next == Some(c) {
            let (first, long) = match c {
                'p' => (P_FIRST, P_LONG),
                't' => (T_FIRST, T_LONG),
                _ => (K_FIRST, K_LONG),
            };
            sounds.push(Sound {
                phoneme: first,
                letter: at,
                palatal: false,
            });
            sounds.push(Sound {
                phoneme: long,
                letter: at + 1,
                palatal: false,
            });
            at += 2;
            continue;
        }
        let phonemes: &[Phoneme] = match c {
            'b' => &[B],
            'c' => &[C],
            'ç' => &[CH],
            'd' => &[D],
            'f' => &[F],
            'g' => &[G],
            'h' => &[H],
            'j' => &[ZH],
            'k' | 'q' => &[K],
            'l' => &[L],
            'm' => &[M],
            'n' => &[N],
            'p' => &[P],
            'r' => &[R],
            's' => &[S],
            'ş' => &[SH],
            't' => &[T],
            'v' | 'w' => &[V],
            'y' => &[J],
            'z' => &[Z],
            'x' => &[K, S],
            '\'' | '’' | '-' => &[],
            _ => return None,
        };
        sounds.extend(phonemes.iter().map(|&phoneme| Sound {
            phoneme,
            letter: at,
            palatal: false,
        }));
        at += 1;
    }
    (!sounds.is_empty()).then_some(sounds)
}

/// ğ between front vowels is a j (değil); otherwise it lengthens the vowel before it (dağ).
fn soft_g(sounds: &mut Vec<Sound>, after: Option<Phoneme>, letter: usize) {
    let Some(before) = sounds.last().map(|sound| sound.phoneme) else {
        return;
    };
    match after {
        Some(after) if is_front(before) && is_front(after) => sounds.push(Sound {
            phoneme: J,
            letter,
            palatal: false,
        }),
        _ if before.is_vowel() => {
            if let Some(last) = sounds.last_mut() {
                last.phoneme = long(before);
            }
        }
        _ => {}
    }
}

/// eSpeak NG's allophones, written in its notation only: lax vowels in closed syllables and i and
/// e at the end of a word, g as ɟ and l as clear or dark by the vowel of its syllable, r as ɾ
/// between vowels.
fn allophones(said: &mut Pronunciation, palatal: &[bool]) {
    let last = said.syllables.len().saturating_sub(1);
    let mut before_open = false;
    for (number, syllable) in said.syllables.iter_mut().enumerate() {
        // The first half of a long stop does not close the syllable for its vowel (teşekkür teʃekːˈør).
        let closed = syllable
            .coda
            .iter()
            .any(|consonant| ![P_FIRST, T_FIRST, K_FIRST].contains(consonant));
        let final_open = number == last && !closed;
        let coda = syllable.coda.first().copied();
        let Some(nucleus) = syllable.nucleus.first().copied() else {
            continue;
        };
        let front = is_front(nucleus) || palatal.get(number).copied().unwrap_or(false);
        syllable.nucleus = vec![match nucleus {
            vowel if vowel == E && closed && matches!(coda, Some(L | M | N)) => E_WIDE,
            vowel if vowel == E && (closed || final_open) => E_LAX,
            vowel if vowel == I && (closed || final_open) => I_LAX,
            vowel if vowel == U && closed => U_LAX,
            vowel if vowel == O && closed => O_LAX,
            vowel if vowel == UE && closed => UE_LAX,
            vowel if vowel == OE && coda == Some(J) => OE_OPEN,
            vowel => vowel,
        }];
        let onset_after_vowel = before_open;
        mark_consonants(syllable, front, onset_after_vowel);
        before_open = !closed;
    }
}

/// g and l by the syllable's vowel, and r between vowels.
fn mark_consonants(syllable: &mut Syllable, front: bool, after_vowel: bool) {
    let onset_len = syllable.onset.len();
    for (index, consonant) in syllable.onset.iter_mut().chain(syllable.coda.iter_mut()).enumerate() {
        *consonant = match *consonant {
            G if front => G_FRONT,
            L if !front => L_DARK,
            R if after_vowel && index == 0 && onset_len == 1 => R_TAP,
            other => other,
        };
    }
}

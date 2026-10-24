//! How an Estonian word is said from its spelling: each vowel as written, long when doubled and
//! overlong in a word of one syllable (maa mˈaaː); the vowels after the first in its syllable a
//! diphthong's second half (päike); a doubled consonant long, and so a single p, t or k after a
//! long vowel or a diphthong (saata sˈaːtːa); l, n, s, d and t palatalised before i or j after
//! the first vowel (neli nˈeʎɪ); the vowel after a short first syllable as eSpeak NG writes it
//! (maja mˈajɑ); the stress on the first syllable, with a secondary one on the third, and on the
//! long vowel of a loanword whose first vowel is short (universiteet).

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A_AFTER_SHORT, B, D, D_PALATAL, E_AFTER_SHORT, F, G, H, I_AFTER_SHORT, J, K, L, L_PALATAL, M, N, N_PALATAL, NG,
    OFFGLIDES, P, R, S, S_PALATAL, SH, T, T_PALATAL, U_AFTER_SHORT, V, VOWELS, W, Z, ZH, doubled, legal_onset,
};

/// A syllable's vowel: its letter, whether it is doubled, and a diphthong's second letter.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Nucleus {
    letter: char,
    long: bool,
    offglide: Option<char>,
}

/// The vowels and the consonant clusters between them, in order.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Part {
    Nucleus(Nucleus),
    Cluster(Vec<char>),
}

fn is_vowel(c: char) -> bool {
    VOWELS.iter().any(|(letter, _)| *letter == c)
}

/// `c` in lower case, the older spelling's w as v and y as i.
fn plain(c: char) -> char {
    match c {
        'w' => 'v',
        'y' => 'i',
        _ => c,
    }
}

/// The parts of `word` (in lower case): two vowels make a diphthong in the first syllable (ai, õu)
/// and before i in the others (põnevaid); otherwise each vowel, or doubled vowel, is a syllable of
/// its own (filosoofia, siia).
fn parts(word: &[char]) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();
    let mut at = 0;
    while at < word.len() {
        let start = at;
        let vowel = is_vowel(word[at]);
        while at < word.len() && is_vowel(word[at]) == vowel {
            at += 1;
        }
        let run = &word[start..at];
        if !vowel {
            parts.push(Part::Cluster(run.to_vec()));
            continue;
        }
        let mut index = 0;
        while index < run.len() {
            let first = !parts.iter().any(|part| matches!(part, Part::Nucleus(_)));
            let letter = run[index];
            let long = run.get(index + 1) == Some(&letter);
            index += if long { 2 } else { 1 };
            // A short vowel and another make a diphthong in the first syllable, and before i in the
            // others (põnevaid); after a long vowel the next is a syllable of its own (siia, maailm).
            let offglide = if !long && index < run.len() && run[index] != letter && (first || run[index] == 'i') {
                index += 1;
                Some(run[index - 1])
            } else {
                None
            };
            parts.push(Part::Nucleus(Nucleus { letter, long, offglide }));
        }
    }
    parts
}

/// The consonant of a letter.
fn consonant(c: char) -> Option<Phoneme> {
    Some(match c {
        'p' => P,
        'b' => B,
        't' => T,
        'd' => D,
        'k' | 'c' | 'q' => K,
        'g' => G,
        'f' => F,
        'v' => V,
        's' => S,
        'z' => Z,
        'š' => SH,
        'ž' => ZH,
        'h' => H,
        'm' => M,
        'n' => N,
        'l' => L,
        'r' => R,
        'j' => J,
        _ => return None,
    })
}

fn palatal(phoneme: Phoneme) -> Phoneme {
    match phoneme {
        phoneme if phoneme == L => L_PALATAL,
        phoneme if phoneme == N => N_PALATAL,
        phoneme if phoneme == S => S_PALATAL,
        phoneme if phoneme == D => D_PALATAL,
        phoneme if phoneme == T => T_PALATAL,
        phoneme => phoneme,
    }
}

/// The sounds of a consonant cluster. `after_first` when it follows the first vowel (not a
/// diphthong), `before_i` when an i follows it, `lengthens` when a single stop in it is long
/// (after a long vowel or a diphthong, before a vowel).
fn cluster(letters: &[char], after_first: bool, before_i: bool, lengthens: bool) -> Option<Vec<Phoneme>> {
    // Palatalised: after the first vowel a single consonant or a doubled one before i, or one
    // before j; anywhere l, n or s before ti or di (Eesti, tundi, meelsasti).
    let palatalised = (after_first
        && match letters {
            [_] | [_, _] if before_i && (letters.len() == 1 || letters[0] == letters[1]) => true,
            [_, 'j'] => true,
            _ => false,
        })
        || (before_i && matches!(letters, ['l' | 'n' | 's', 't' | 'd']));
    // A stop before a sonorant after a short first vowel is long (ütles ˈytːles).
    let stop_before_sonorant = after_first && matches!(letters, ['p' | 't' | 'k', 'l' | 'r' | 'n' | 'm' | 'j' | 'v']);
    let mut sounds: Vec<Phoneme> = Vec::new();
    let mut at = 0;
    while at < letters.len() {
        let c = letters[at];
        if matches!(c, '\'' | '’' | '-') {
            at += 1;
            continue;
        }
        if c == 'x' {
            sounds.extend([K, S]);
            at += 1;
            continue;
        }
        let mut sound = consonant(c)?;
        if palatalised && at == 0 {
            sound = palatal(sound);
        }
        // n before k and g is ŋ (ning niŋɡ).
        if sound == N && matches!(letters.get(at + 1), Some('k' | 'g')) {
            sound = NG;
        }
        let long = letters.get(at + 1) == Some(&c);
        sounds.push(sound);
        if long || (lengthens && letters.len() == 1 && [P, T, K].contains(&sound)) || (stop_before_sonorant && at == 0)
        {
            sounds.push(doubled(sound));
        }
        at += if long { 2 } else { 1 };
    }
    Some(sounds)
}

/// How `word` is said; `None` for a letter Estonian does not read.
#[expect(clippy::too_many_lines, reason = "the steps of a word's reading, in order")]
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let chars: Vec<char> = word.to_lowercase().chars().map(plain).collect();
    let parts = parts(&chars);
    let nuclei: Vec<Nucleus> = parts
        .iter()
        .filter_map(|part| match part {
            Part::Nucleus(nucleus) => Some(*nucleus),
            Part::Cluster(_) => None,
        })
        .collect();
    let count = nuclei.len();
    // A loanword with a short first vowel is stressed on its first long one (universiteet,
    // šokolaad); other words on the first syllable.
    let first_light = nuclei
        .first()
        .is_some_and(|nucleus| !nucleus.long && nucleus.offglide.is_none());
    let stressed = if first_light {
        nuclei.iter().position(|nucleus| nucleus.long).unwrap_or(0)
    } else {
        0
    };
    // The first syllable is short when its vowel is short and one consonant follows it.
    let first_cluster = parts.iter().skip_while(|part| matches!(part, Part::Cluster(_))).nth(1);
    let short_first = first_light && matches!(first_cluster, Some(Part::Cluster(letters)) if letters.len() == 1);
    let mut segments: Vec<Segment> = Vec::new();
    let mut syllable = 0;
    for (index, part) in parts.iter().enumerate() {
        match part {
            Part::Nucleus(nucleus) => {
                let forms = VOWELS.iter().find(|(letter, _)| *letter == nucleus.letter)?.1;
                // Not when two consonants end the word after it (kõvaks) or a vowel follows
                // (vanaema).
                let closing = parts.get(index + 1);
                let after_short = short_first
                    && syllable == 1
                    && !nucleus.long
                    && !matches!(closing, Some(Part::Nucleus(_)))
                    && !matches!(closing, Some(Part::Cluster(letters)) if letters.len() > 1 && index + 2 == parts.len());
                let phoneme = match nucleus.letter {
                    'a' if after_short => A_AFTER_SHORT,
                    'e' if after_short => E_AFTER_SHORT,
                    'i' if after_short => I_AFTER_SHORT,
                    'u' if after_short => U_AFTER_SHORT,
                    // Overlong in a word of one syllable, before one consonant at most (maa, saab;
                    // but sööks).
                    _ if nucleus.long
                        && count == 1
                        && !matches!(parts.get(index + 1), Some(Part::Cluster(letters)) if letters.len() > 1) =>
                    {
                        forms[2]
                    }
                    _ if nucleus.long => forms[1],
                    _ => forms[0],
                };
                let stress = if syllable == stressed {
                    Stress::Primary
                } else {
                    Stress::Unstressed
                };
                segments.push(Segment::new(phoneme, stress));
                if let Some(offglide) = nucleus.offglide {
                    let glides = OFFGLIDES.iter().find(|(letter, _)| *letter == offglide)?.1;
                    let open_end = count == 1 && index + 1 == parts.len();
                    segments.push(Segment::new(glides[usize::from(open_end)], Stress::Unstressed));
                    // An i ending a diphthong before another vowel goes over to it as j (meie
                    // mˈeije).
                    if offglide == 'i' && matches!(parts.get(index + 1), Some(Part::Nucleus(_))) {
                        segments.push(Segment::new(J, Stress::Unstressed));
                    }
                } else if nucleus.long && matches!(parts.get(index + 1), Some(Part::Nucleus(_))) {
                    // A long i or u before another vowel glides into it (siia sˈiːja, uue ˈuːwe).
                    match nucleus.letter {
                        'i' => segments.push(Segment::new(J, Stress::Unstressed)),
                        'u' => segments.push(Segment::new(W, Stress::Unstressed)),
                        _ => {}
                    }
                }
                syllable += 1;
            }
            Part::Cluster(letters) => {
                let before = index.checked_sub(1).and_then(|before| match &parts[before] {
                    Part::Nucleus(nucleus) => Some(*nucleus),
                    Part::Cluster(_) => None,
                });
                let next_vowel = match parts.get(index + 1) {
                    Some(Part::Nucleus(nucleus)) => Some(nucleus.letter),
                    _ => None,
                };
                let after_first = syllable == 1 && before.is_some_and(|nucleus| nucleus.offglide.is_none());
                // A doubled s, or l after a front vowel, ending a word of one syllable after a short
                // vowel is palatalised, with an i glide before it (kass kˈais^s^, küll; but all).
                if count == 1
                    && index + 1 == parts.len()
                    && before.is_some_and(|nucleus| {
                        !nucleus.long
                            && nucleus.offglide.is_none()
                            && (letters[..] == ['s', 's']
                                || (letters[..] == ['l', 'l'] && matches!(nucleus.letter, 'i' | 'e' | 'ä' | 'ö' | 'ü')))
                    })
                {
                    let sound = palatal(consonant(letters[0])?);
                    segments.push(Segment::new(OFFGLIDES[2].1[0], Stress::Unstressed));
                    segments.push(Segment::new(sound, Stress::Unstressed));
                    segments.push(Segment::new(doubled(sound), Stress::Unstressed));
                    continue;
                }
                let lengthens =
                    next_vowel.is_some() && before.is_some_and(|nucleus| nucleus.long || nucleus.offglide.is_some());
                for sound in cluster(letters, after_first, next_vowel == Some('i'), lengthens)? {
                    segments.push(Segment::new(sound, Stress::Unstressed));
                }
            }
        }
    }
    if segments.is_empty() {
        return None;
    }
    let mut said = syllabify(&segments, legal_onset);
    let syllables = said.syllables.len();
    if stressed == 0 {
        // A secondary stress on the third syllable of a word of four or more ending in a vowel
        // (kirjutama kˈirjutˌama; but imeliselt).
        let open_last = said.syllables.last().is_some_and(|last| last.coda.is_empty());
        if syllables >= 4 && open_last {
            said.syllables[2].stress = Stress::Secondary;
        }
    } else {
        // Before a loanword's stress, every other syllable from the first.
        for syllable in said.syllables.iter_mut().take(stressed.saturating_sub(1)).step_by(2) {
            syllable.stress = Stress::Rhythmic;
        }
    }
    Some(said)
}

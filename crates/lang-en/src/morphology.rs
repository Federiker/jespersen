//! Words the lexicon lacks, read from words it has: a known word with an inflection or a common
//! suffix (Smithson's, unfriended, greyness), or two known words written as one (lighthousekeeper).

use jespersen_lexicon::LexiconError;
use jespersen_phonemes::{Phoneme, Segment, Stress};

use crate::phonemes::{CH, D, F, HAPPY, JH, K, KIT, L, LETTER, M, N, NG, P, S, SCHWA, SH, T, TH, Z, ZH};

/// The shortest part of a compound, in letters, so that "a" + "bout" is not one.
const SHORTEST_PART: usize = 3;

/// A suffix: how it is written and how it is said.
struct Suffix {
    written: &'static str,
    sound: Sound,
}

/// How a suffix is said.
#[derive(Clone, Copy)]
enum Sound {
    /// -s, -'s: /ɪz/ after a sibilant, /s/ after another voiceless sound, /z/ otherwise.
    Sibilant,
    /// -ed: /ɪd/ after /t/ or /d/, /t/ after another voiceless sound, /d/ otherwise.
    Past,
    /// Always these phonemes, unstressed.
    Fixed(&'static [Phoneme]),
}

const SUFFIXES: &[Suffix] = &[
    Suffix {
        written: "'s",
        sound: Sound::Sibilant,
    },
    Suffix {
        written: "’s",
        sound: Sound::Sibilant,
    },
    Suffix {
        written: "s'",
        sound: Sound::Sibilant,
    },
    Suffix {
        written: "es",
        sound: Sound::Sibilant,
    },
    Suffix {
        written: "s",
        sound: Sound::Sibilant,
    },
    Suffix {
        written: "ed",
        sound: Sound::Past,
    },
    Suffix {
        written: "ing",
        sound: Sound::Fixed(&[KIT, NG]),
    },
    // -ically: basic, basically; quixotic, quixotically.
    Suffix {
        written: "ally",
        sound: Sound::Fixed(&[L, HAPPY]),
    },
    Suffix {
        written: "ly",
        sound: Sound::Fixed(&[L, HAPPY]),
    },
    Suffix {
        written: "ness",
        sound: Sound::Fixed(&[N, SCHWA, S]),
    },
    Suffix {
        written: "less",
        sound: Sound::Fixed(&[L, SCHWA, S]),
    },
    Suffix {
        written: "ful",
        sound: Sound::Fixed(&[F, SCHWA, L]),
    },
    Suffix {
        written: "ment",
        sound: Sound::Fixed(&[M, SCHWA, N, T]),
    },
    Suffix {
        written: "er",
        sound: Sound::Fixed(&[LETTER]),
    },
    Suffix {
        written: "est",
        sound: Sound::Fixed(&[SCHWA, S, T]),
    },
];

/// The phonemes of `word` (a lexicon key) read from known words, or `None`. `lookup` finds a
/// word's phonemes in the lexicon.
///
/// # Errors
///
/// When `lookup` fails.
pub fn derive(
    word: &str,
    lookup: &impl Fn(&str) -> Result<Option<Vec<Segment>>, LexiconError>,
) -> Result<Option<Vec<Segment>>, LexiconError> {
    if let Some(segments) = inflected(word, lookup)? {
        return Ok(Some(segments));
    }
    compound(word, lookup)
}

fn inflected(
    word: &str,
    lookup: &impl Fn(&str) -> Result<Option<Vec<Segment>>, LexiconError>,
) -> Result<Option<Vec<Segment>>, LexiconError> {
    for suffix in SUFFIXES {
        let Some(stem) = word.strip_suffix(suffix.written) else {
            continue;
        };
        for candidate in stems(stem, suffix.written) {
            if candidate.chars().count() < 2 {
                continue;
            }
            if let Some(mut segments) = lookup(&candidate)? {
                let added = sound(suffix.sound, segments.last().map(|segment| segment.phoneme));
                segments.extend(added.iter().map(|&phoneme| Segment::new(phoneme, Stress::Unstressed)));
                return Ok(Some(segments));
            }
        }
    }
    Ok(None)
}

/// The spellings a stem may have had before `suffix` was added: as written, with a final e
/// restored (baked, baking), with y for i (cities, happiness), and with a doubled consonant
/// undone (stopped, running).
fn stems(stem: &str, suffix: &str) -> Vec<String> {
    let mut candidates = vec![stem.to_owned()];
    if suffix.starts_with(['e', 'i']) {
        candidates.push(format!("{stem}e"));
    }
    if let Some(before) = stem.strip_suffix('i') {
        candidates.push(format!("{before}y"));
    }
    let mut letters = stem.chars().rev();
    if let (Some(last), Some(before)) = (letters.next(), letters.next())
        && last == before
        && !"aeiouls".contains(last)
    {
        candidates.push(stem[..stem.len() - last.len_utf8()].to_owned());
    }
    candidates
}

fn sound(sound: Sound, last: Option<Phoneme>) -> Vec<Phoneme> {
    let voiceless = |phoneme| [P, T, K, F, TH, S, SH, CH].contains(&phoneme);
    match (sound, last) {
        (Sound::Sibilant, Some(last)) if [S, Z, SH, ZH, CH, JH].contains(&last) => vec![KIT, Z],
        (Sound::Sibilant, Some(last)) if voiceless(last) => vec![S],
        (Sound::Sibilant, _) => vec![Z],
        (Sound::Past, Some(last)) if last == T || last == D => vec![KIT, D],
        (Sound::Past, Some(last)) if voiceless(last) => vec![T],
        (Sound::Past, _) => vec![D],
        (Sound::Fixed(phonemes), _) => phonemes.to_vec(),
    }
}

/// Two known words written as one: the first keeps its stress, the second's primary stress
/// becomes secondary (lighthouse keeper). The longest known first part wins.
fn compound(
    word: &str,
    lookup: &impl Fn(&str) -> Result<Option<Vec<Segment>>, LexiconError>,
) -> Result<Option<Vec<Segment>>, LexiconError> {
    let boundaries: Vec<usize> = word
        .char_indices()
        .map(|(index, _)| index)
        .skip(SHORTEST_PART)
        .collect();
    for &split in boundaries.iter().rev() {
        let (first, second) = word.split_at(split);
        if second.chars().count() < SHORTEST_PART {
            continue;
        }
        let Some(mut segments) = lookup(first)? else { continue };
        let Some(rest) = lookup(second)? else { continue };
        segments.extend(rest.into_iter().enumerate().map(|(index, segment)| Segment {
            stress: segment.stress.min(Stress::Secondary),
            // Syllables do not cross into the second part (lighthouse | keeper).
            boundary: index == 0,
            ..segment
        }));
        return Ok(Some(segments));
    }
    Ok(None)
}

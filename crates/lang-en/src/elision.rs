//! Verse's elisions: a word with an apostrophe inside it for a sound it leaves out (lov'd, ow'st,
//! heav'n), read from the full word.

use jespersen_lexicon::LexiconError;
use jespersen_phonemes::{Segment, Stress};

use crate::morphology;
use crate::phonemes::{LETTER, R, S, T};
use crate::spellings::vowel_groups;

/// The vowel letters an apostrophe may stand for, the likeliest first (heav'n, fam'ly, flow'ret).
const VOWELS: [char; 5] = ['e', 'i', 'o', 'a', 'u'];

/// The phonemes of `word` (a lexicon key, so with a straight apostrophe) read as an elision of a
/// word `find` knows or one read from known words ([`morphology::derive`]), or `None`:
///
/// - -'d is -ed said without a vowel: lov'd as loved, dimm'd as dimmed, call'd as called.
/// - -'st is -st: ow'st as owe and st, wander'st as wander and st.
/// - Elsewhere the apostrophe is an unstressed vowel left out: heav'n is heaven without its
///   schwa, giv'n given. An r-coloured one leaves its r before a vowel (wand'ring: wandering
///   with ɹ for ɚ) and stays at the end (pow'r: power); a full word already said without one of
///   its vowels is kept (ev'ry: every, ɛvɹi).
///
/// Only a word with one apostrophe, neither first nor last, and not a possessive's 's. Elisions of
/// a consonant (e'en, o'er, ta'en) are lexicon entries.
///
/// # Errors
///
/// When `find` fails.
pub fn read(
    word: &str,
    find: &impl Fn(&str) -> Result<Option<Vec<Segment>>, LexiconError>,
) -> Result<Option<Vec<Segment>>, LexiconError> {
    let read = |word: &str| match find(word)? {
        Some(segments) => Ok(Some(segments)),
        None => morphology::derive(word, find),
    };
    let Some(at) = word.find('\'') else { return Ok(None) };
    if at == 0 || at + 1 == word.len() || word[at + 1..].contains('\'') || word.ends_with("'s") {
        return Ok(None);
    }
    if let Some(stem) = word.strip_suffix("'d") {
        return read(&format!("{stem}ed"));
    }
    if let Some(stem) = word.strip_suffix("'st") {
        // A known word with the e first: ow'st is owe's, com'st come's (not ow, com), but not one
        // read from parts (wander'st: wand and ere).
        let with_e = if stem.ends_with('e') {
            None
        } else {
            find(&format!("{stem}e"))?
        };
        let segments = match with_e {
            Some(segments) => Some(segments),
            None if stem.chars().count() >= 2 => read(stem)?,
            None => None,
        };
        return Ok(segments.map(|mut segments| {
            segments.extend([S, T].map(|phoneme| Segment::new(phoneme, Stress::Unstressed)));
            segments
        }));
    }
    let position = word[..at].chars().filter(char::is_ascii_lowercase).count();
    for vowel in VOWELS {
        let full = word.replacen('\'', &vowel.to_string(), 1);
        if let Some(segments) = read(&full)?
            && let Some(elided) = without_vowel(segments, &full, position)
        {
            return Ok(Some(elided));
        }
    }
    Ok(None)
}

/// `segments`, the phonemes of `full`, without the vowel its letter at `position` stands for, if
/// that letter is a vowel group of its own said unstressed; see [`read`].
fn without_vowel(mut segments: Vec<Segment>, full: &str, position: usize) -> Option<Vec<Segment>> {
    let groups = vowel_groups(full);
    let group = groups
        .iter()
        .position(|group| group.start == position && group.letters.chars().count() == 1)?;
    let vowels: Vec<usize> = (0..segments.len())
        .filter(|&index| segments[index].phoneme.is_vowel())
        .collect();
    if vowels.len() + 1 == groups.len() {
        return Some(segments);
    }
    if vowels.len() != groups.len() {
        return None;
    }
    let index = vowels[group];
    if segments[index].stress != Stress::Unstressed {
        return None;
    }
    if segments[index].phoneme == LETTER {
        if segments.get(index + 1).is_some_and(|next| next.phoneme.is_vowel()) {
            segments[index].phoneme = R;
        }
    } else {
        segments.remove(index);
    }
    Some(segments)
}

//! Words read letter by letter: initialisms (U.S.A., NHS) and a letter on its own (the letter a).

use jespersen_phonemes::{Segment, Stress};

use crate::{arpabet, phonemes};

/// The American names of the letters, in ARPAbet (jespersen's own list, CC0).
#[rustfmt::skip]
const NAMES: [&str; 26] = [
    "EY1", "B IY1", "S IY1", "D IY1", "IY1", "EH1 F", "JH IY1", "EY1 CH", "AY1", "JH EY1", "K EY1", "EH1 L",
    "EH1 M", "EH1 N", "OW1", "P IY1", "K Y UW1", "AA1 R", "EH1 S", "T IY1", "Y UW1", "V IY1",
    "D AH1 B AH0 L Y UW0", "EH1 K S", "W AY1", "Z IY1",
];

/// The letters of `word` if it is read letter by letter: letters with full stops between them
/// (U.S.A, U.S.A., A.; e.g. not: the caller expands those first), or two to six capitals with no
/// vowel letter (NHS, BBC) when `known` says the lexicon lacks the word.
pub fn spelled(word: &str, known: bool) -> Option<Vec<char>> {
    let letters: Vec<char> = word.chars().filter(|&c| c != '.').collect();
    let dotted = word.contains('.')
        && word
            .split('.')
            .filter(|part| !part.is_empty())
            .all(|part| part.chars().count() == 1)
        && letters.iter().all(char::is_ascii_alphabetic)
        && !letters.is_empty();
    let capitals = !known
        && (2..=6).contains(&letters.len())
        && letters.iter().all(char::is_ascii_uppercase)
        && !letters.iter().any(|c| "AEIOUY".contains(*c));
    (dotted || capitals).then_some(letters)
}

/// The phonemes of `letters` said one by one: each letter's name, the last stressed and the others
/// lightly (jˌuːˌɛsˈeɪ), with a boundary between letters so syllables do not cross them. British
/// English says zed for z.
pub fn spell(letters: &[char], british: bool) -> Vec<Segment> {
    let mut segments = Vec::new();
    let letters: Vec<u8> = letters
        .iter()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase() as u8)
        .collect();
    for (position, &letter) in letters.iter().enumerate() {
        let last = position + 1 == letters.len();
        let name = if british && letter == b'z' {
            "Z EH1 D"
        } else {
            NAMES[usize::from(letter - b'a')]
        };
        for (at, phone) in name.split(' ').enumerate() {
            let Some((phoneme, stress)) = arpabet::encode(phone)
                .and_then(arpabet::decode)
                .and_then(|(index, stress)| phonemes::american(index, stress))
            else {
                continue;
            };
            let stress = if stress == Stress::Primary && !last {
                Stress::Secondary
            } else {
                stress
            };
            segments.push(Segment {
                boundary: at == 0 && position > 0,
                ..Segment::new(phoneme, stress)
            });
        }
    }
    segments
}

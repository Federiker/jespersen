use crate::{Phoneme, Pronunciation, Stress, Syllable};

/// A phoneme with the stress a lexicon gives it. Only a vowel's stress counts: it becomes its
/// syllable's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Segment {
    pub phoneme: Phoneme,
    pub stress: Stress,
    /// A boundary between the parts of a word comes before this segment (the s of lighthouse
    /// keeper): no syllable starts before it and ends after it.
    pub boundary: bool,
}

impl Segment {
    /// A segment with no boundary before it.
    #[must_use]
    pub const fn new(phoneme: Phoneme, stress: Stress) -> Self {
        Self {
            phoneme,
            stress,
            boundary: false,
        }
    }
}

/// Splits a word's phonemes into syllables by the maximal onset principle: each vowel is the
/// nucleus of a syllable, and the consonants between two vowels go to the second syllable's onset
/// as far as `legal_onset` (the language's phonotactics) allows, the rest to the first's coda. An
/// onset never reaches back past a boundary.
///
/// `legal_onset` is only asked about non-empty consonant clusters taken from between two vowels.
/// A word with no vowel becomes one unstressed syllable of consonants.
pub fn syllabify(segments: &[Segment], legal_onset: impl Fn(&[Phoneme]) -> bool) -> Pronunciation {
    let vowels: Vec<usize> = segments
        .iter()
        .enumerate()
        .filter(|(_, segment)| segment.phoneme.is_vowel())
        .map(|(index, _)| index)
        .collect();
    let phonemes = |range: std::ops::Range<usize>| segments[range].iter().map(|segment| segment.phoneme).collect();

    if vowels.is_empty() {
        let syllables = if segments.is_empty() {
            Vec::new()
        } else {
            vec![Syllable {
                onset: phonemes(0..segments.len()),
                ..Syllable::default()
            }]
        };
        return Pronunciation { syllables };
    }

    let mut syllables = Vec::with_capacity(vowels.len());
    let mut onset_start = 0;
    for (position, &vowel) in vowels.iter().enumerate() {
        let coda_end = vowels.get(position + 1).map_or(segments.len(), |&next| {
            // The longest legal onset for the next syllable, taken from the end of the cluster and
            // starting no earlier than the last boundary in it.
            let cluster: Vec<Phoneme> = phonemes(vowel + 1..next);
            let earliest = (vowel + 1..=next)
                .rev()
                .find(|&index| segments[index].boundary)
                .map_or(0, |index| index - vowel - 1);
            let split = (earliest..cluster.len())
                .find(|&start| legal_onset(&cluster[start..]))
                .unwrap_or(cluster.len());
            vowel + 1 + split
        });
        syllables.push(Syllable {
            onset: phonemes(onset_start..vowel),
            nucleus: vec![segments[vowel].phoneme],
            coda: phonemes(vowel + 1..coda_end),
            stress: segments[vowel].stress,
            tone: None,
            spelling: None,
        });
        onset_start = coda_end;
    }
    Pronunciation { syllables }
}

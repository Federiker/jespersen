//! How a Macedonian word is said from its Cyrillic spelling: one letter, one sound (ѓ ɟ, ќ c, ѕ dz,
//! љ ʎ, њ ɲ, џ dʒ); л dark except before е, и and ј; р as a syllable with no vowel beside it
//! (прст, 'рѓа); a cluster of obstruents voiced or not by its last one, and a final one voiceless
//! (град ɡrat); and the stress on the third syllable from the end, or on the first of a shorter
//! word (вода ˈvɔda, планина ˈplanina, учителот uˈtʃitɛlɔt).

use jespersen_phonemes::{Notation, Phoneme, Pronunciation, Segment, Stress, Syllable, syllabify};

use crate::phonemes::{
    A, B, CH, D, DZ, DZH, E, F, G, GJ, I, J, K, KJ, L, L_DARK, LJ, M, N, NG, NY, O, P, R, R_SYLLABIC, S, SH, T, TS, U,
    V, X, Z, ZH, is_obstruent, legal_onset, voices, with_voicing,
};

/// The sounds of `word`'s letters (in lower case); `None` for a letter Macedonian does not read.
fn letters(word: &[char]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(word.len());
    for (at, &letter) in word.iter().enumerate() {
        let next = word.get(at + 1).copied();
        let sound: &[Phoneme] = match letter {
            'а' => &[A],
            'б' => &[B],
            'в' => &[V],
            'г' => &[G],
            'д' => &[D],
            'ѓ' => &[GJ],
            'е' => &[E],
            'ж' => &[ZH],
            'з' => &[Z],
            'ѕ' => &[DZ],
            'и' | 'ѝ' => &[I],
            'ј' => &[J],
            'к' => &[K],
            // л is dark (ɫ) except before е, и and ј.
            'л' if matches!(next, Some('е' | 'и' | 'ѝ' | 'ј')) => &[L],
            'л' => &[L_DARK],
            'љ' => &[LJ],
            'м' => &[M],
            // н is ŋ before к and г (банка).
            'н' if matches!(next, Some('к' | 'г')) => &[NG],
            'н' => &[N],
            'њ' => &[NY],
            'о' => &[O],
            'п' => &[P],
            'р' => &[R],
            'с' => &[S],
            'т' => &[T],
            'ќ' => &[KJ],
            'у' => &[U],
            'ф' => &[F],
            'х' => &[X],
            'ц' => &[TS],
            'ч' => &[CH],
            'џ' => &[DZH],
            'ш' => &[SH],
            '\'' | '’' | '-' => &[],
            _ => return None,
        };
        sounds.extend_from_slice(sound);
    }
    Some(sounds)
}

/// р with no vowel beside it is the nucleus of its syllable (прст, срце, 'рѓа).
fn syllabic_r(sounds: &mut [Phoneme]) {
    for at in 0..sounds.len() {
        let vowel = |offset: Option<usize>| {
            offset
                .and_then(|at| sounds.get(at))
                .is_some_and(|sound| sound.is_vowel())
        };
        if sounds[at] == R && !vowel(at.checked_sub(1)) && !vowel(Some(at + 1)) {
            sounds[at] = R_SYLLABIC;
        }
    }
}

/// Voicing inside a word: a cluster of obstruents takes the voicing of its last one other than в
/// (сватба svadba), and a word's last obstruents are voiceless (град ɡrat, нож nɔʃ, љубов ʎubɔf).
fn assimilate(sounds: &mut [Phoneme]) {
    let mut start = 0;
    while start < sounds.len() {
        let length = sounds[start..].iter().take_while(|&&sound| is_obstruent(sound)).count();
        let at_end = start + length == sounds.len();
        let cluster = &mut sounds[start..start + length];
        if at_end {
            for sound in cluster.iter_mut() {
                *sound = with_voicing(*sound, false);
            }
        } else if let Some(last) = cluster.iter().rposition(|&sound| sound != V) {
            let voiced = voices(cluster[last]);
            for sound in &mut cluster[..last] {
                *sound = with_voicing(*sound, voiced);
            }
        }
        start += length.max(1);
    }
}

/// The index of the stressed syllable among `count`: the third from the end, or the first.
pub(crate) fn antepenult(count: usize) -> usize {
    count.saturating_sub(3)
}

/// How `word` is said, stressed on its third syllable from the end; `None` for a letter Macedonian
/// does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let lower: Vec<char> = word.to_lowercase().chars().collect();
    let mut sounds = letters(&lower)?;
    syllabic_r(&mut sounds);
    assimilate(&mut sounds);
    let vowels: Vec<usize> = (0..sounds.len()).filter(|&at| sounds[at].is_vowel()).collect();
    let stressed = *vowels.get(antepenult(vowels.len()))?;
    let segments: Vec<Segment> = sounds
        .iter()
        .enumerate()
        .map(|(at, &sound)| {
            let stress = if at == stressed {
                Stress::Primary
            } else {
                Stress::Unstressed
            };
            Segment::new(sound, stress)
        })
        .collect();
    Some(syllabify(&segments, legal_onset))
}

/// The mark a clitic's vowels carry in the notation Macedonian does not write (Misaki's).
const CLITIC: &str = "clitic";

/// A clitic's vowel: the same sound, told apart so that the stress of the accentual word it joins
/// can be counted from it (не знам ˈnɛ znam).
fn clitic_vowel(vowel: Phoneme) -> Phoneme {
    vowel.misaki(CLITIC)
}

/// Whether `syllable` belongs to a clitic.
fn is_clitic(syllable: &Syllable) -> bool {
    syllable
        .nucleus
        .iter()
        .all(|vowel| vowel.symbol(Notation::Misaki) == CLITIC)
}

/// A clitic (не, ќе, го, ми, се) as said: its vowels marked as a clitic's.
pub(crate) fn as_clitic(mut said: Pronunciation) -> Pronunciation {
    for vowel in said
        .syllables
        .iter_mut()
        .flat_map(|syllable| syllable.nucleus.iter_mut())
    {
        *vowel = clitic_vowel(*vowel);
    }
    said
}

/// The stress of each accentual word in a run: clitics before a stressed word join it, and the
/// whole takes the stress on its third syllable from the end (не знам ˈnɛ znam, ќе дојдам
/// ˈcɛ dɔjdam, не разбирам nɛ ˈrazbiram).
pub(crate) fn restress(words: &mut [&mut Pronunciation]) {
    let mut start = 0;
    for host in 0..words.len() {
        let stressed = words[host]
            .syllables
            .iter()
            .any(|syllable| syllable.stress == Stress::Primary);
        let clitic = !words[host].syllables.is_empty() && words[host].syllables.iter().all(is_clitic);
        if clitic {
            continue;
        }
        if stressed && start < host {
            let count: usize = words[start..=host].iter().map(|said| said.syllables.len()).sum();
            let target = antepenult(count);
            let mut index = 0;
            for said in &mut words[start..=host] {
                for syllable in &mut said.syllables {
                    syllable.stress = if index == target {
                        Stress::Primary
                    } else {
                        Stress::Unstressed
                    };
                    index += 1;
                }
            }
        }
        start = host + 1;
    }
}

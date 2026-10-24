use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Dutch,
    numbers::is_number_word,
    phonemes::{D, E, E_LONG, H, N, R, SCHWA, T, legal_onset},
    spelling::pronounce,
    words::respelling,
};

/// Words said without stress in running text: articles, prepositions, pronouns, conjunctions and
/// the auxiliaries.
const UNSTRESSED: [&str; 46] = [
    "de", "het", "een", "en", "is", "in", "op", "te", "met", "van", "aan", "dat", "die", "er", "hij", "zij", "ze",
    "je", "we", "wij", "ik", "u", "als", "om", "bij", "naar", "of", "tot", "zijn", "heeft", "voor", "door", "maar",
    "zich", "haar", "mij", "me", "ons", "hun", "wat", "jij", "gij", "kan", "kon", "dit", "ook",
];

/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 8] = ["hebben", "worden", "werd", "waren", "zou", "deze", "niet", "hare"];

/// How each token (text and kind) of a Dutch sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings),
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Dutch, &Reader)
}

/// How Dutch says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && letter.is_uppercase()
            && let Some(name) = Dutch.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        pronounce(respelling(&lower).unwrap_or(&lower))
    }

    /// The parts of a number joined by hyphens, said as one word, the first stressed and the
    /// rest lighter, en unstressed (een-en-twintig ˈeːnɛntʋˌɪntəx, twee-honderd tʋˈeːhˌɔndərt).
    fn say_joined(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let parts: Vec<&str> = lower.split('-').filter(|part| !part.is_empty()).collect();
        if !parts.iter().all(|part| is_number_word(part)) {
            return None;
        }
        let mut joined: Vec<Segment> = Vec::new();
        for (index, part) in parts.iter().enumerate() {
            let and = ["en", "ën"].contains(part);
            let said = if and { pronounce("èn")? } else { self.say(part)? };
            for syllable in &said.syllables {
                let stress = match syllable.stress {
                    _ if and => Stress::Unstressed,
                    Stress::Primary if index > 0 => Stress::Secondary,
                    stress => stress,
                };
                for &phoneme in syllable.onset.iter().chain(&syllable.nucleus).chain(&syllable.coda) {
                    let stress = if phoneme.is_vowel() { stress } else { Stress::Unstressed };
                    joined.push(Segment::new(phoneme, stress));
                }
            }
        }
        Some(syllabify(&joined, legal_onset))
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    fn run(&self, words: &mut [&mut Pronunciation]) {
        for said in words.iter_mut() {
            reduce(said);
        }
        across(words);
    }
}

/// The e of a function word left without stress is ə: in de, je, te, een, het and er (de də, een
/// ən, het hət), not in en or met.
fn reduce(said: &mut Pronunciation) {
    if said
        .syllables
        .iter()
        .any(|syllable| syllable.stress != Stress::Unstressed)
    {
        return;
    }
    let phonemes: Vec<Phoneme> = said.phonemes().collect();
    let reduced = matches!(phonemes[..], [.., vowel] | [.., vowel, N] if vowel == E_LONG)
        || phonemes == [H, E, T]
        || phonemes == [E, R];
    if reduced {
        for vowel in said
            .syllables
            .iter_mut()
            .flat_map(|syllable| syllable.nucleus.iter_mut())
        {
            *vowel = SCHWA;
        }
    }
}

/// The consonants that meet across words: a t ending a word goes over to a t or d after it, which
/// it unvoices (het dak hə tˈɑk, niet de ni tə).
fn across(words: &mut [&mut Pronunciation]) {
    for index in 1..words.len() {
        let first = words[index].phonemes().next();
        let Some(syllable) = words[index - 1].syllables.last_mut() else {
            continue;
        };
        let Some(&last) = syllable.coda.last() else {
            continue;
        };
        if last == T && matches!(first, Some(first) if first == T || first == D) {
            syllable.coda.pop();
            if let Some(onset) = words[index]
                .syllables
                .first_mut()
                .and_then(|syllable| syllable.onset.first_mut())
            {
                *onset = T;
            }
        }
    }
}

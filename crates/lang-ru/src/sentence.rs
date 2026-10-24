use jespersen_phonemes::{Phoneme, Pronunciation};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Russian,
    phonemes::{A_CLITIC, F, SCHWA, V, devoice, is_obstruent, is_voiced, voiced_before_next},
    spelling::pronounce,
    stress::stressed,
    words::lookup,
};

/// Words said without stress in running text, as eSpeak NG says them: the prepositions в, с, к,
/// у, из, от, за and на, the conjunctions и, а, но and что, the particles бы, же, ли, не and ни,
/// and был, whose vowels are reduced (что ʃto, на nə); and для, над, есть and быть, whose
/// vowels stay as if stressed (для dɭʲɑ).
const UNSTRESSED: [&str; 22] = [
    "в", "с", "к", "у", "из", "от", "за", "на", "и", "а", "но", "что", "бы", "же", "ли", "не", "ни", "был", "для",
    "над", "есть", "быть",
];
/// The unstressed words whose vowels stay as if stressed.
const FULL: [&str; 4] = ["для", "над", "есть", "быть"];
/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 8] = ["было", "были", "может", "можете", "сможет", "будет", "будут", "или"];

/// How each token (text and kind) of a Russian sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling with their stress from jespersen's
/// list or its rules, numbers, abbreviations and symbols as words. The words between punctuation
/// marks are read together: function words lose their stress, a preposition of one consonant
/// takes the voicing of the word after it (в саду f sˈɑdu), and in plain IPA a word's last
/// obstruent is voiced before a voiced one (из дома ɪz ˈdomə).
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Russian, &Reader)
}

/// How Russian says its words, for the shared reader.
struct Reader;

/// The name of `letter`; a name of two words (твёрдый знак) is said as one.
fn letter_name(letter: char) -> Option<Pronunciation> {
    let name = Russian.letter(letter)?;
    let said = stressed(&name.replace(' ', ""));
    pronounce(&said.letters, said.stress)
}

impl Speaker for Reader {
    /// A word from its spelling and stress; a function word without stress; a letter on its own
    /// that is not a word (Н. Крупская) by its name, and letters with full stops between them
    /// (Ц.К) by theirs, as one word.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = lower.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && !"вскуиаоя".contains(letter)
        {
            return letter_name(letter);
        }
        if lower.contains('.') && lower.split('.').all(|part| part.chars().count() == 1) {
            let mut said = Pronunciation::default();
            for letter in lower.chars().filter(|&c| c != '.') {
                said.syllables.extend(letter_name(letter)?.syllables);
            }
            return Some(said);
        }
        if UNSTRESSED.contains(&lower.as_str()) && !FULL.contains(&lower.as_str()) {
            let letters = lookup(&lower).map_or_else(|| lower.clone(), |said| said.letters);
            let mut said = pronounce(&letters, None)?;
            // на is said with ə (на столе nə stʌɭʲˈe).
            if lower == "на" {
                for vowel in said.syllables.iter_mut().flat_map(|syllable| &mut syllable.nucleus) {
                    if *vowel == A_CLITIC {
                        *vowel = SCHWA;
                    }
                }
            }
            return Some(said);
        }
        let said = stressed(word);
        pronounce(&said.letters, said.stress)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    /// Voicing across words, from the last word back: a preposition of one consonant (в, с, к) is
    /// voiceless before a voiceless obstruent (в саду f sˈɑdu); before a voiced obstruent other
    /// than в, a word's last в is said v in both notations (домов за dʌmˈov), its other obstruents
    /// are voiced in plain IPA only (из дома ɪz ˈdomə, eSpeak NG ɪs dˈoma).
    fn run(&self, words: &mut [&mut Pronunciation]) {
        for index in (0..words.len()).rev() {
            let Some(next) = words.get(index + 1).and_then(|said| said.phonemes().next()) else {
                continue;
            };
            let said = &mut *words[index];
            let has_vowel = said.phonemes().any(Phoneme::is_vowel);
            let voiced_obstruent = is_obstruent(next) && is_voiced(next) && next != V;
            let Some(last) = said.syllables.last_mut().and_then(|syllable| {
                if syllable.coda.is_empty() && !has_vowel {
                    syllable.onset.last_mut()
                } else {
                    syllable.coda.last_mut()
                }
            }) else {
                continue;
            };
            if !has_vowel && is_obstruent(next) && !is_voiced(next) {
                *last = devoice(*last);
            } else if voiced_obstruent && *last == F {
                *last = V;
            } else if voiced_obstruent && is_obstruent(*last) {
                *last = voiced_before_next(*last);
            }
        }
    }
}

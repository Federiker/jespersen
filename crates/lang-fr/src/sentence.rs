use jespersen_phonemes::{Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::French,
    phonemes::{DASH, J, LIAISON_N, LIAISON_ZS, S, SCHWA, UI, W, legal_onset, liaison},
    spelling::pronounce,
    words::{is_aspirated, respelling},
};

/// Words said without stress in running text: articles, prepositions, pronouns before the verb,
/// conjunctions, and est, a and ont.
const UNSTRESSED: [&str; 61] = [
    "le", "la", "les", "un", "une", "des", "du", "de", "au", "aux", "à", "et", "ou", "en", "dans", "sur", "sous",
    "par", "pour", "sans", "chez", "vers", "ce", "cet", "cette", "ces", "mon", "ton", "son", "ma", "ta", "sa", "mes",
    "tes", "ses", "notre", "votre", "nos", "vos", "leur", "leurs", "je", "tu", "il", "elle", "on", "nous", "vous",
    "ils", "elles", "me", "te", "se", "ne", "y", "que", "qui", "est", "jusqu'à", "toute", "toutes",
];
/// Words said without stress in running text that eSpeak NG reads so too: verbs and adverbs.
const UNSTRESSED_MORE: [&str; 34] = [
    "a", "ont", "ai", "tout", "très", "mais", "comme", "quand", "plus", "c'est", "s'est", "qu'il", "qu'ils", "ni",
    "quel", "quelle", "quels", "quelles", "faut", "près", "peu", "jusqu'au", "d'un", "d'une", "pas", "ça", "où",
    "tant", "qu'on", "l'on", "m'a", "n'a", "t'a", "trop",
];
/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 14] = [
    "comment", "toujours", "avec", "avant", "autour", "lui", "depuis", "voilà", "quelque", "pourtant", "cela", "après",
    "jamais", "encore",
];
/// Words that end in an n said before a vowel (un ami œ̃n amˈi, on a, en été, bien aimé).
const LIAISON_N_WORDS: [&str; 10] = ["un", "on", "en", "mon", "ton", "son", "aucun", "bien", "rien", "commun"];
/// Words after which eSpeak NG writes a dash (lə- ʃˈa, le-z amˈi).
const DASHED: [&str; 18] = [
    "le", "la", "les", "de", "du", "des", "ma", "ta", "sa", "mes", "tes", "ses", "me", "te", "se", "ce", "je", "ne",
];

/// How each token (text and kind) of a French sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's lists of respellings
/// and aspirated h), numbers, abbreviations and symbols as words, an elided word with the word
/// after it (l'école). The words between punctuation marks are read together: function words lose
/// their stress, and a word ends in its liaison consonant before a vowel (les amis le-z amˈi).
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &French, &Reader)
}

/// How French says its words, for the shared reader.
struct Reader;

/// All the words said without stress.
static UNSTRESSED_ALL: std::sync::LazyLock<Vec<&'static str>> =
    std::sync::LazyLock::new(|| UNSTRESSED.iter().chain(&UNSTRESSED_MORE).copied().collect());

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        say(word)
    }

    /// An elided word (l', d', qu', jusqu') said as the start of the word after it: lekˈɔl.
    fn elide(&self, head: &str, word: Pronunciation) -> Option<Pronunciation> {
        let lower = head.to_lowercase();
        if lower == "aujourd" {
            return say("aujourd'hui");
        }
        let mut phonemes: Vec<Segment> = say_alone(&format!("{lower}e"))?
            .phonemes()
            .filter(|phoneme| liaison(*phoneme).is_none() && *phoneme != DASH)
            .map(|phoneme| Segment::new(phoneme, Stress::Unstressed))
            .collect();
        if phonemes.last().is_some_and(|segment| segment.phoneme == SCHWA) {
            phonemes.pop();
        }
        for syllable in &word.syllables {
            for &phoneme in syllable.onset.iter().chain(&syllable.nucleus).chain(&syllable.coda) {
                let stress = if phoneme.is_vowel() {
                    syllable.stress
                } else {
                    Stress::Unstressed
                };
                phonemes.push(Segment::new(phoneme, stress));
            }
        }
        Some(syllabify(&phonemes, legal_onset))
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED_ALL
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    /// Liaison: a word's silent final consonant is said before a word that starts with a vowel or
    /// a glide (les amis, les yeux), but not before an aspirated h (les héros) or at the end of the
    /// run.
    fn run(&self, words: &mut [&mut Pronunciation]) {
        for index in 0..words.len() {
            let next = words.get(index + 1).and_then(|said| said.phonemes().next());
            let vowel_next = next.is_some_and(|phoneme| phoneme.is_vowel() || [J, W, UI].contains(&phoneme));
            let Some(last) = words[index].syllables.last_mut() else {
                continue;
            };
            let end = if last.coda.is_empty() && last.nucleus.is_empty() {
                &mut last.onset
            } else {
                &mut last.coda
            };
            if let Some(marker) = end.last().copied()
                && let Some(consonant) = liaison(marker)
            {
                end.pop();
                if vowel_next {
                    end.push(consonant);
                } else if marker == LIAISON_ZS && next.is_none() {
                    // six and dix at the end are sis and dis.
                    end.push(S);
                }
            }
        }
    }
}

/// How a word is said on its own: from jespersen's respellings or its spelling, an aspirated h kept
/// as a silent marker.
fn say_alone(word: &str) -> Option<Pronunciation> {
    let lower = word.to_lowercase();
    let spelt = respelling(&lower).unwrap_or(&lower);
    pronounce(spelt, is_aspirated(&lower))
}

/// How a word is said in running text: on its own, with eSpeak NG's dash after le, la, de and the
/// like, before the liaison consonant (le-z).
pub(crate) fn say(word: &str) -> Option<Pronunciation> {
    let mut said = say_alone(word)?;
    let lower = word.to_lowercase();
    if LIAISON_N_WORDS.contains(&lower.as_str())
        && let Some(last) = said.syllables.last_mut()
    {
        last.coda.push(LIAISON_N);
    }
    if DASHED.contains(&lower.as_str())
        && let Some(last) = said.syllables.last_mut()
    {
        let at = last
            .coda
            .last()
            .filter(|phoneme| liaison(**phoneme).is_some())
            .map_or(last.coda.len(), |_| last.coda.len() - 1);
        last.coda.insert(at, DASH);
    }
    Some(said)
}

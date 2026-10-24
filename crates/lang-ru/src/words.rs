//! jespersen's Russian word list (data/ru/words.tsv), written from the standard language: words and
//! word forms with their stress, stems whose endings keep the stress where it is or take it, and
//! respellings of words said otherwise than written (что → што).

use std::{collections::HashMap, sync::LazyLock};

use crate::spelling::is_vowel;

const WORDS: &str = include_str!("../../../data/ru/words.tsv");

/// The mark written after a stressed vowel (молоко́).
pub(crate) const ACUTE: char = '\u{301}';

/// A word with its stress: its letters in lower case without the mark (ё kept), and its stressed
/// vowel counting from 0, `None` for a word without vowels.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Stressed {
    pub(crate) letters: String,
    pub(crate) stress: Option<usize>,
}

impl Stressed {
    /// `letters` with the stress on its last vowel (a stem said without an ending: стол).
    fn on_last_vowel(letters: String) -> Self {
        let stress = letters.chars().filter(|&c| is_vowel(c)).count().checked_sub(1);
        Self { letters, stress }
    }
}

/// The words a stem forms: nouns, adjectives or verbs, each with their own endings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Class {
    Noun,
    Adjective,
    Verb,
}

const NOUN_ENDINGS: &[&str] = &[
    "", "а", "я", "у", "ю", "ы", "и", "е", "о", "ё", "ь", "ой", "ей", "ёй", "ою", "ею", "ом", "ем", "ём", "ам", "ям",
    "ах", "ях", "ами", "ями", "ов", "ев", "ёв", "ью", "ья", "ье", "ьи", "ьё", "ьев", "ьям", "ьями", "ьях", "ий", "ия",
    "ии", "ию", "ией", "иям", "иями", "иях", "ие", "ием", "й",
];

const ADJECTIVE_ENDINGS: &[&str] = &[
    "ый",
    "ий",
    "ой",
    "ая",
    "яя",
    "ое",
    "ее",
    "ые",
    "ие",
    "ого",
    "его",
    "ому",
    "ему",
    "ым",
    "им",
    "ыми",
    "ими",
    "ых",
    "их",
    "ую",
    "юю",
    "ою",
    "ею",
    "ом",
    "ем",
    "",
    "а",
    "о",
    "ы",
    "и",
    "е",
    "у",
    "ей",
    // Nouns in -ость keep their adjective's stress (промы́шленность).
    "ость",
    "ости",
    "остью",
    "остей",
    "остям",
    "остями",
    "остях",
];

const VERB_ENDINGS: &[&str] = &[
    "ть",
    "ти",
    "чь",
    "ю",
    "у",
    "ешь",
    "ёшь",
    "ет",
    "ёт",
    "ем",
    "ём",
    "ете",
    "ёте",
    "ут",
    "ют",
    "ат",
    "ят",
    "ишь",
    "ит",
    "им",
    "ите",
    "л",
    "ла",
    "ло",
    "ли",
    "й",
    "йте",
    "и",
    "я",
    "в",
    "вши",
    "ить",
    "ать",
    "ять",
    "еть",
    "уть",
    "ыть",
    "ил",
    "ила",
    "ило",
    "или",
    "ал",
    "ала",
    "ало",
    "али",
    "ял",
    "яла",
    "яло",
    "яли",
    "ел",
    "ела",
    "ело",
    "ели",
    "ул",
    "ула",
    "уло",
    "ули",
    "ив",
    "ав",
    "ая",
    "яв",
    "ыл",
    "овать",
    "овал",
    "овала",
    "овало",
    "овали",
    "ую",
    "уешь",
    "ует",
    "уем",
    "уете",
    "уют",
    "уя",
    "уй",
    // Past participles (сде́ланный, сде́лано).
    "н",
    "на",
    "но",
    "ны",
    "нный",
    "нная",
    "нное",
    "нные",
    "нного",
    "нной",
    "нному",
    "нным",
    "нными",
    "нных",
    "нную",
];

impl Class {
    fn endings(self) -> &'static [&'static str] {
        match self {
            Self::Noun => NOUN_ENDINGS,
            Self::Adjective => ADJECTIVE_ENDINGS,
            Self::Verb => VERB_ENDINGS,
        }
    }
}

/// A stem: its letters with the stress it keeps, or `None` when its endings take the stress.
#[derive(Clone, Debug)]
struct Stem {
    stressed: Stressed,
    ending_stressed: bool,
    class: Class,
}

/// The list, read once.
struct Lexicon {
    /// Word forms by their letters, and the respelling of those said otherwise.
    forms: HashMap<String, Stressed>,
    /// The forms with ё, by their letters with е, for text that leaves ё unwritten (еще → ещё).
    forms_without_yo: HashMap<String, Stressed>,
    stems: HashMap<String, Vec<Stem>>,
}

/// A listed word with its stress mark (молоко́), or a stem ending in `-` or `+` and its class
/// (`n`, `a`, `v`); `None` for a malformed one.
fn parse_word(text: &str) -> Option<(Stressed, Option<(bool, Class)>)> {
    let (word, stem) = match text.char_indices().rev().nth(1) {
        Some((at, mark @ ('-' | '+'))) => {
            let class = match &text[at + 1..] {
                "n" => Class::Noun,
                "a" => Class::Adjective,
                "v" => Class::Verb,
                _ => return None,
            };
            (&text[..at], Some((mark == '+', class)))
        }
        _ => (text, None),
    };
    let mut letters = String::with_capacity(word.len());
    let mut vowels: usize = 0;
    let mut marked = None;
    let mut yo = None;
    for c in word.chars().flat_map(char::to_lowercase) {
        if c == ACUTE {
            marked = Some(vowels.checked_sub(1)?);
        } else {
            if c == 'ё' {
                yo = Some(vowels);
            }
            if is_vowel(c) {
                vowels += 1;
            }
            letters.push(c);
        }
    }
    let stress = match (marked.or(yo), vowels) {
        (Some(stress), _) => Some(stress),
        (None, 0) => None,
        // A word of one vowel needs no mark, nor does a stem whose endings take the stress.
        (None, 1) => Some(0),
        (None, _) if matches!(stem, Some((true, _))) => None,
        (None, _) => return None,
    };
    Some((Stressed { letters, stress }, stem))
}

/// The number of entries in the list, or the line number (from 1) of its first malformed one.
#[cfg(test)]
pub(crate) fn entries(text: &str) -> Result<usize, usize> {
    let mut count = 0;
    for (number, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split('\t');
        let word = fields.next().map(parse_word);
        let respelling = fields.next().map(parse_word);
        let valid = matches!(word, Some(Some(_)))
            && respelling.is_none_or(|said| matches!(said, Some((_, None))))
            && fields.next().is_none();
        if !valid {
            return Err(number + 1);
        }
        count += 1;
    }
    Ok(count)
}

fn read(text: &str) -> Lexicon {
    let mut lexicon = Lexicon {
        forms: HashMap::new(),
        forms_without_yo: HashMap::new(),
        stems: HashMap::new(),
    };
    for line in text.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split('\t');
        let Some(Some((word, stem))) = fields.next().map(parse_word) else {
            continue;
        };
        let respelling = fields.next().and_then(parse_word).map(|(said, _)| said);
        if let Some((ending_stressed, class)) = stem {
            let key = word.letters.clone();
            let stem = Stem {
                stressed: word,
                ending_stressed,
                class,
            };
            if key.contains('ё') {
                lexicon
                    .stems
                    .entry(key.replace('ё', "е"))
                    .or_default()
                    .push(stem.clone());
            }
            lexicon.stems.entry(key).or_default().push(stem);
        } else {
            let said = respelling.unwrap_or_else(|| word.clone());
            if word.letters.contains('ё') {
                lexicon
                    .forms_without_yo
                    .entry(word.letters.replace('ё', "е"))
                    .or_insert_with(|| said.clone());
            }
            lexicon.forms.entry(word.letters).or_insert(said);
        }
    }
    lexicon
}

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| read(WORDS));

/// The stress a stressed ending takes: the first vowel of the ending, or the stem's last when the
/// ending has none (стол, столá). A stressed е in -ешь, -ет, -ем and -ете is ё (идёт).
fn with_ending(stem: &Stem, ending: &str) -> Stressed {
    if !stem.ending_stressed {
        return Stressed {
            letters: format!("{}{ending}", stem.stressed.letters),
            stress: stem.stressed.stress,
        };
    }
    let stem_vowels = stem.stressed.letters.chars().filter(|&c| is_vowel(c)).count();
    if !ending.chars().any(is_vowel) {
        return Stressed::on_last_vowel(format!("{}{ending}", stem.stressed.letters));
    }
    let ending = match ending {
        "ешь" | "ет" | "ем" | "ете" if stem.class != Class::Adjective => ending.replacen('е', "ё", 1),
        _ => ending.to_owned(),
    };
    Stressed {
        letters: format!("{}{ending}", stem.stressed.letters),
        stress: Some(stem_vowels),
    }
}

/// `word` as a listed stem and one of its endings.
fn from_stem(lexicon: &Lexicon, word: &str) -> Option<Stressed> {
    let boundaries: Vec<usize> = word
        .char_indices()
        .map(|(at, _)| at)
        .skip(2)
        .chain([word.len()])
        .collect();
    // The longest stem first.
    for &at in boundaries.iter().rev() {
        let (stem, ending) = word.split_at(at);
        if let Some(stems) = lexicon.stems.get(stem)
            && let Some(stem) = stems.iter().find(|stem| {
                // -ость keeps only a fixed stem's stress (but мо́лодость from молодо́й).
                stem.class.endings().contains(&ending) && !(stem.ending_stressed && ending.starts_with("ост"))
            })
        {
            return Some(with_ending(stem, ending));
        }
    }
    None
}

/// `word` as a listed form or a stem with an ending.
fn listed(lexicon: &Lexicon, word: &str) -> Option<Stressed> {
    lexicon
        .forms
        .get(word)
        .or_else(|| lexicon.forms_without_yo.get(word))
        .cloned()
        .or_else(|| from_stem(lexicon, word))
}

/// Prefixes a listed word can take without moving its stress (рассказать from сказать), longest
/// first; вы- takes the stress itself (высказать).
const PREFIXES: [&str; 33] = [
    "пере", "пред", "подо", "пре", "при", "про", "раз", "рас", "без", "бес", "вос", "воз", "под", "над", "обо", "ото",
    "от", "об", "до", "за", "из", "ис", "на", "по", "вы", "вз", "вс", "со", "во", "не", "у", "о", "с",
];

/// `word` as a listed form or stem, either with -ся or -сь after it.
fn direct(lexicon: &Lexicon, word: &str) -> Option<Stressed> {
    listed(lexicon, word).or_else(|| {
        ["ся", "сь"].iter().find_map(|reflexive| {
            let base = word.strip_suffix(reflexive)?;
            let said = listed(lexicon, base)?;
            Some(Stressed {
                letters: format!("{}{reflexive}", said.letters),
                stress: said.stress,
            })
        })
    })
}

/// `word` as a listed word with up to `prefixes` prefixes before it (предоставить); вы- takes the
/// stress, and an ё after it is then е (вы́зовет).
fn with_prefixes(lexicon: &Lexicon, word: &str, prefixes: usize) -> Option<Stressed> {
    direct(lexicon, word).or_else(|| {
        if prefixes == 0 {
            return None;
        }
        PREFIXES.iter().find_map(|prefix| {
            let rest = word.strip_prefix(prefix)?;
            if rest.chars().count() < 2 || !rest.chars().any(is_vowel) {
                return None;
            }
            let said = with_prefixes(lexicon, rest, prefixes - 1)?;
            let vowels = prefix.chars().filter(|&c| is_vowel(c)).count();
            Some(if *prefix == "вы" {
                Stressed {
                    letters: format!("{prefix}{}", said.letters.replace('ё', "е")),
                    stress: Some(0),
                }
            } else {
                Stressed {
                    letters: format!("{prefix}{}", said.letters),
                    stress: said.stress.map(|stress| stress + vowels),
                }
            })
        })
    })
}

/// `word`, in lower case, from the list: as a form, a stem with an ending, either with -ся or
/// -сь after it, with up to two prefixes before it.
pub(crate) fn lookup(word: &str) -> Option<Stressed> {
    with_prefixes(&LEXICON, word, 2)
}

/// The list itself, for checking.
#[cfg(test)]
pub(crate) fn text() -> &'static str {
    WORDS
}

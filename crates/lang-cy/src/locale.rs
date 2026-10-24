use jespersen_spoken::{Gender, Locale};

use crate::numbers::cardinal;

const MONTHS: [&str; 12] = [
    "Ionawr",
    "Chwefror",
    "Mawrth",
    "Ebrill",
    "Mai",
    "Mehefin",
    "Gorffennaf",
    "Awst",
    "Medi",
    "Hydref",
    "Tachwedd",
    "Rhagfyr",
];

/// Welsh abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 6] = [
    ("dr", "doctor"),
    ("parch", "parchedig"),
    ("rh", "rhif"),
    ("st", "stryd"),
    ("ffôn", "ffôn"),
    ("etc", "et cetera"),
];

fn expansion(word: &str) -> Option<&'static str> {
    let lower = word.to_lowercase();
    ABBREVIATIONS
        .iter()
        .find(|(abbreviation, _)| *abbreviation == lower)
        .map(|(_, said)| *said)
}

/// Whether `word` (without its full stop) is an abbreviation that does not end a sentence.
#[must_use]
pub fn is_abbreviation(word: &str) -> bool {
    expansion(word).is_some()
}

/// Welsh words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Welsh;

impl Locale for Welsh {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "coma" } else { "pwynt" }
    }

    /// deg tri-deg; the full hour is deg o'r gloch.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours);
        Some(if minutes == 0 {
            format!("{hour} o'r gloch")
        } else {
            format!("{hour} {}", cardinal(minutes))
        })
    }

    /// tri Mai dau mil dau-ddeg-pedwar.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "i"
    }

    fn ratio(&self) -> &'static str {
        "rhannu â"
    }

    fn minus(&self) -> &'static str {
        "minws"
    }

    fn plus(&self) -> &'static str {
        "plws"
    }

    fn percent(&self) -> &'static str {
        "y cant"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("ewro", "ewro")),
            "$" | "USD" => Some(("doler", "doler")),
            "£" | "GBP" => Some(("punt", "punt")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("y cant"),
            "&" => Some("a"),
            "@" => Some("at"),
            "+" => Some("plus"),
            "=" => Some("hafal i"),
            "°" => Some("gradd"),
            "#" => Some("hash"),
            "€" => Some("ewro"),
            "$" => Some("doler"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "bi",
            'c' => "ec",
            'd' => "di",
            'e' => "e",
            'f' => "ef",
            'g' => "eg",
            'h' => "aets",
            'i' => "i",
            'j' => "je",
            'k' => "ce",
            'l' => "el",
            'm' => "em",
            'n' => "en",
            'o' => "o",
            'p' => "pi",
            'q' => "ciw",
            'r' => "er",
            's' => "es",
            't' => "ti",
            'u' => "u",
            'v' => "fi",
            'w' => "w",
            'x' => "ecs",
            'y' => "y",
            'z' => "sed",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("at"),
            '.' => Some("punt"),
            '/' => Some("slaes"),
            ':' => Some("colon"),
            '-' => Some("cysylltnod"),
            '_' => Some("tanlinell"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiouwy"
    }
}

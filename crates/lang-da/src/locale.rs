use jespersen_spoken::{Gender, Locale};

use crate::numbers::cardinal;

const MONTHS: [&str; 12] = [
    "januar",
    "februar",
    "marts",
    "april",
    "maj",
    "juni",
    "juli",
    "august",
    "september",
    "oktober",
    "november",
    "december",
];

/// Danish abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 10] = [
    ("dr", "doktor"),
    ("prof", "professor"),
    ("nr", "nummer"),
    ("tlf", "telefon"),
    ("bl", "blandt"),
    ("kl", "klokken"),
    ("ca", "cirka"),
    ("osv", "og så videre"),
    ("etc", "et cetera"),
    ("hr", "herr"),
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

/// Danish words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Danish;

impl Locale for Danish {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "komma" } else { "punktum" }
    }

    /// ti tredive; the full hour is klokken ti.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours);
        Some(if minutes == 0 {
            format!("klokken {hour}")
        } else {
            format!("{hour} {}", cardinal(minutes))
        })
    }

    /// tre maj to tusind og fire-og-tyve.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "til"
    }

    fn ratio(&self) -> &'static str {
        "delt med"
    }

    fn minus(&self) -> &'static str {
        "minus"
    }

    fn plus(&self) -> &'static str {
        "plus"
    }

    fn percent(&self) -> &'static str {
        "procent"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("euro", "euro")),
            "$" | "USD" => Some(("dollar", "dollar")),
            "£" | "GBP" => Some(("pund", "pund")),
            "kr" | "DKK" => Some(("krone", "kroner")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("procent"),
            "&" => Some("og"),
            "@" => Some("snabel-a"),
            "+" => Some("plus"),
            "=" => Some("lig med"),
            "°" => Some("grader"),
            "#" => Some("havelåge"),
            "€" => Some("euro"),
            "$" => Some("dollar"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "be",
            'c' => "se",
            'd' => "de",
            'e' => "e",
            'f' => "ef",
            'g' => "ge",
            'h' => "hå",
            'i' => "i",
            'j' => "jod",
            'k' => "kå",
            'l' => "el",
            'm' => "em",
            'n' => "en",
            'o' => "o",
            'p' => "pe",
            'q' => "ku",
            'r' => "er",
            's' => "es",
            't' => "te",
            'u' => "u",
            'v' => "ve",
            'w' => "dobbelt-ve",
            'x' => "eks",
            'y' => "y",
            'z' => "set",
            'æ' => "æ",
            'ø' => "ø",
            'å' => "å",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("snabel-a"),
            '.' => Some("punktum"),
            '/' => Some("skråstreg"),
            ':' => Some("kolon"),
            '-' => Some("bindestreg"),
            '_' => Some("understreg"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiouyæøå"
    }
}

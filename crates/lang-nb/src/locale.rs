use jespersen_spoken::{Gender, Locale};

use crate::numbers::cardinal;

const MONTHS: [&str; 12] = [
    "januar",
    "februar",
    "mars",
    "april",
    "mai",
    "juni",
    "juli",
    "august",
    "september",
    "oktober",
    "november",
    "desember",
];

/// Norwegian abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 10] = [
    ("dr", "doktor"),
    ("prof", "professor"),
    ("nr", "nummer"),
    ("tel", "telefon"),
    ("ref", "referanse"),
    ("bl", "blant"),
    ("kl", "klokken"),
    ("ca", "cirka"),
    ("osv", "og så videre"),
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

/// Norwegian words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Norwegian;

impl Locale for Norwegian {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "komma" } else { "punkt" }
    }

    /// ti tretti; the full hour is klokken ti.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours);
        Some(if minutes == 0 {
            format!("klokken {hour}")
        } else {
            format!("{hour} {}", cardinal(minutes))
        })
    }

    /// tre mai to tusen og tjue-fire.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "til"
    }

    fn ratio(&self) -> &'static str {
        "delt på"
    }

    fn minus(&self) -> &'static str {
        "minus"
    }

    fn plus(&self) -> &'static str {
        "plus"
    }

    fn percent(&self) -> &'static str {
        "prosent"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("euro", "euro")),
            "$" | "USD" => Some(("dollar", "dollar")),
            "£" | "GBP" => Some(("pund", "pund")),
            "kr" | "NOK" => Some(("krone", "kroner")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("prosent"),
            "&" => Some("og"),
            "@" => Some("krøllalfa"),
            "+" => Some("plus"),
            "=" => Some("er lik"),
            "°" => Some("grader"),
            "#" => Some("firkant"),
            "€" => Some("euro"),
            "$" => Some("dollar"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "be",
            'c' => "ce",
            'd' => "de",
            'e' => "e",
            'f' => "eff",
            'g' => "ge",
            'h' => "hå",
            'i' => "i",
            'j' => "je",
            'k' => "kå",
            'l' => "ell",
            'm' => "emm",
            'n' => "enn",
            'o' => "o",
            'p' => "pe",
            'q' => "ku",
            'r' => "ærr",
            's' => "ess",
            't' => "te",
            'u' => "u",
            'v' => "ve",
            'w' => "dobbelt-ve",
            'x' => "ex",
            'y' => "y",
            'z' => "sett",
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
            '@' => Some("krøllalfa"),
            '.' => Some("punt"),
            '/' => Some("skråstrek"),
            ':' => Some("kolon"),
            '-' => Some("bindestrek"),
            '_' => Some("understrek"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiouyæøå"
    }
}

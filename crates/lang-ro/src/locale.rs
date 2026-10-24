use jespersen_spoken::{Gender, Locale};

use crate::numbers::cardinal;

const MONTHS: [&str; 12] = [
    "ianuarie",
    "februarie",
    "martie",
    "aprilie",
    "mai",
    "iunie",
    "iulie",
    "august",
    "septembrie",
    "octombrie",
    "noiembrie",
    "decembrie",
];

/// Romanian abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 12] = [
    ("dr", "doctor"),
    ("prof", "profesor"),
    ("ing", "inginer"),
    ("str", "strada"),
    ("nr", "numărul"),
    ("etc", "etcetera"),
    ("dl", "domnul"),
    ("dna", "doamna"),
    ("pag", "pagina"),
    ("tel", "telefon"),
    ("ex", "exemplu"),
    ("bd", "bulevardul"),
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

/// Romanian words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Romanian;

impl Locale for Romanian {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        Some(cardinal(number, gender == Gender::Feminine))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "virgulă" } else { "punct" }
    }

    /// zece și treizeci; the full hour is ora zece.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours, true);
        Some(if minutes == 0 {
            format!("ora {hour}")
        } else {
            format!("{hour} și {}", cardinal(minutes, true))
        })
    }

    /// trei mai două mii douăzeci și patru; the first of the month is întâi.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        let day = if day == 1 {
            "întâi".to_owned()
        } else {
            cardinal(day, false)
        };
        Some(format!("{day} {month} {}", cardinal(year, false)))
    }

    fn range(&self) -> &'static str {
        "până la"
    }

    fn ratio(&self) -> &'static str {
        "supra"
    }

    fn minus(&self) -> &'static str {
        "minus"
    }

    fn plus(&self) -> &'static str {
        "plus"
    }

    fn percent(&self) -> &'static str {
        "la sută"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("euro", "euro")),
            "$" | "USD" => Some(("dolar", "dolari")),
            "£" | "GBP" => Some(("liră", "lire")),
            "lei" | "RON" => Some(("leu", "lei")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("la sută"),
            "&" => Some("și"),
            "@" => Some("arond"),
            "+" => Some("plus"),
            "=" => Some("egal"),
            "°" => Some("grade"),
            "#" => Some("diez"),
            "€" => Some("euro"),
            "$" => Some("dolari"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'ă' => "ă",
            'â' | 'î' => "î",
            'b' => "be",
            'c' => "ce",
            'd' => "de",
            'e' => "e",
            'f' => "ef",
            'g' => "ge",
            'h' => "haș",
            'i' => "i",
            'j' => "je",
            'k' => "ca",
            'l' => "el",
            'm' => "em",
            'n' => "en",
            'o' => "o",
            'p' => "pe",
            'q' => "chiu",
            'r' => "er",
            's' => "es",
            'ș' => "șe",
            't' => "te",
            'ț' => "țe",
            'u' => "u",
            'v' => "ve",
            'w' => "dublu ve",
            'x' => "ics",
            'y' => "i grec",
            'z' => "zet",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("arond"),
            '.' => Some("punct"),
            '/' => Some("bară"),
            ':' => Some("două puncte"),
            '-' => Some("cratimă"),
            '_' => Some("subliniere"),
            _ => None,
        }
    }

    /// Feminine nouns in the singular end in -ă or -e (o carte, două case); the rest are
    /// masculine.
    fn gender_of(&self, word: &str) -> Gender {
        if word.ends_with('ă') || word.ends_with("ăți") || word.ends_with("ri") {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    fn vowels(&self) -> &'static str {
        "aăâeiîou"
    }
}

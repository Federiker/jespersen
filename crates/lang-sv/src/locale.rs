use jespersen_spoken::{Gender, Locale};

use crate::numbers::cardinal;

const MONTHS: [&str; 12] = [
    "januari",
    "februari",
    "mars",
    "april",
    "maj",
    "juni",
    "juli",
    "augusti",
    "september",
    "oktober",
    "november",
    "december",
];

/// Swedish abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 10] = [
    ("dr", "doktor"),
    ("prof", "professor"),
    ("nr", "nummer"),
    ("tel", "telefon"),
    ("st", "stycken"),
    ("bl", "bland"),
    ("ca", "cirka"),
    ("kl", "klockan"),
    ("ung", "ungefär"),
    ("etc", "etcetera"),
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

/// Swedish words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Swedish;

impl Locale for Swedish {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "komma" } else { "punkt" }
    }

    /// tio trettio; the full hour is klockan tio.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours);
        Some(if minutes == 0 {
            format!("klockan {hour}")
        } else {
            format!("{hour} {}", cardinal(minutes))
        })
    }

    /// tre maj två tusen tjugo-fyra.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "till"
    }

    fn ratio(&self) -> &'static str {
        "genom"
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
            "kr" | "SEK" => Some(("krona", "kronor")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("procent"),
            "&" => Some("och"),
            "@" => Some("snabel-a"),
            "+" => Some("plus"),
            "=" => Some("lika med"),
            "°" => Some("grader"),
            "#" => Some("fyrkant"),
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
            'j' => "ji",
            'k' => "kå",
            'l' => "ell",
            'm' => "emm",
            'n' => "enn",
            'o' => "o",
            'p' => "pe",
            'q' => "ku",
            'r' => "ärr",
            's' => "ess",
            't' => "te",
            'u' => "u",
            'v' => "ve",
            'w' => "dubbel-ve",
            'x' => "ex",
            'y' => "y",
            'z' => "säta",
            'å' => "å",
            'ä' => "ä",
            'ö' => "ö",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("snabel-a"),
            '.' => Some("punt"),
            '/' => Some("snedstreck"),
            ':' => Some("kolon"),
            '-' => Some("bindestreck"),
            '_' => Some("understreck"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiouyåäö"
    }
}

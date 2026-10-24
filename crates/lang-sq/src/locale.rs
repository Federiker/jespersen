use jespersen_spoken::{Gender, Locale};

use crate::numbers::cardinal;

const MONTHS: [&str; 12] = [
    "janar", "shkurt", "mars", "prill", "maj", "qershor", "korrik", "gusht", "shtator", "tetor", "nëntor", "dhjetor",
];

/// Albanian abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 9] = [
    ("dr", "doktor"),
    ("prof", "profesor"),
    ("rr", "rruga"),
    ("nr", "numër"),
    ("etj", "etjerë"),
    ("tel", "telefon"),
    ("z", "zoti"),
    ("znj", "zonja"),
    ("sh", "shembull"),
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

/// Albanian words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Albanian;

impl Locale for Albanian {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "presje" } else { "pikë" }
    }

    /// dhjetë e tridhjetë; the full hour is ora dhjetë.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        Some(if minutes == 0 {
            format!("ora {}", cardinal(hours))
        } else {
            format!("{} e {}", cardinal(hours), cardinal(minutes))
        })
    }

    /// tre maj dy mijë e njëzet-e-katër.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "deri"
    }

    fn ratio(&self) -> &'static str {
        "me"
    }

    fn minus(&self) -> &'static str {
        "minus"
    }

    fn plus(&self) -> &'static str {
        "plus"
    }

    fn percent(&self) -> &'static str {
        "përqind"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("euro", "euro")),
            "$" | "USD" => Some(("dollar", "dollarë")),
            "£" | "GBP" => Some(("paund", "paundë")),
            "ALL" => Some(("lek", "lekë")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("përqind"),
            "&" => Some("dhe"),
            "@" => Some("et"),
            "+" => Some("plus"),
            "=" => Some("baraz"),
            "°" => Some("gradë"),
            "#" => Some("numër"),
            "€" => Some("euro"),
            "$" => Some("dollarë"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "bë",
            'c' => "cë",
            'ç' => "çë",
            'd' => "dë",
            'e' => "e",
            'ë' => "ë",
            'f' => "fë",
            'g' => "gë",
            'h' => "hë",
            'i' => "i",
            'j' => "jë",
            'k' => "kë",
            'l' => "lë",
            'm' => "më",
            'n' => "në",
            'o' => "o",
            'p' => "pë",
            'q' => "që",
            'r' => "rë",
            's' => "së",
            't' => "të",
            'u' => "u",
            'v' => "vë",
            'w' => "vë dyfish",
            'x' => "xë",
            'y' => "y",
            'z' => "zë",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("et"),
            '.' => Some("pikë"),
            '/' => Some("vijë e pjerrët"),
            ':' => Some("dy pika"),
            '-' => Some("vizë"),
            '_' => Some("nënvizë"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeëiouy"
    }
}

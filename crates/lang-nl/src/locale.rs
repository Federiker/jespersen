use jespersen_spoken::{Gender, Locale};

use crate::numbers::cardinal;

const MONTHS: [&str; 12] = [
    "januari",
    "februari",
    "maart",
    "april",
    "mei",
    "juni",
    "juli",
    "augustus",
    "september",
    "oktober",
    "november",
    "december",
];

/// Dutch abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 12] = [
    ("dr", "dokter"),
    ("prof", "professor"),
    ("mr", "meester"),
    ("dhr", "de heer"),
    ("mevr", "mevrouw"),
    ("nr", "nummer"),
    ("str", "straat"),
    ("tel", "telefoon"),
    ("enz", "enzovoort"),
    ("bijv", "bijvoorbeeld"),
    ("ca", "circa"),
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

/// Dutch words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Dutch;

impl Locale for Dutch {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "komma" } else { "punt" }
    }

    /// tien uur dertig; the full hour is tien uur.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours);
        Some(if minutes == 0 {
            format!("{hour} uur")
        } else {
            format!("{hour} uur {}", cardinal(minutes))
        })
    }

    /// drie mei twee duizend vier-en-twintig.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "tot"
    }

    fn ratio(&self) -> &'static str {
        "op"
    }

    fn minus(&self) -> &'static str {
        "min"
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
            "£" | "GBP" => Some(("pond", "pond")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("procent"),
            "&" => Some("en"),
            "@" => Some("apenstaartje"),
            "+" => Some("plus"),
            "=" => Some("is gelijk aan"),
            "°" => Some("graden"),
            "#" => Some("hekje"),
            "€" => Some("euro"),
            "$" => Some("dollar"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "aa",
            'b' => "bee",
            'c' => "cee",
            'd' => "dee",
            'e' => "ee",
            'f' => "ef",
            'g' => "gee",
            'h' => "ha",
            'i' => "ie",
            'j' => "jee",
            'k' => "ka",
            'l' => "el",
            'm' => "em",
            'n' => "en",
            'o' => "oo",
            'p' => "pee",
            'q' => "kuu",
            'r' => "er",
            's' => "es",
            't' => "tee",
            'u' => "uu",
            'v' => "vee",
            'w' => "wee",
            'x' => "iks",
            'y' => "ij",
            'z' => "zet",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("apenstaartje"),
            '.' => Some("punt"),
            '/' => Some("schuine streep"),
            ':' => Some("dubbele punt"),
            '-' => Some("streepje"),
            '_' => Some("laag streepje"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiouyéëïö"
    }
}

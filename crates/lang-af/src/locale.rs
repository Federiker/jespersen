use jespersen_spoken::{Gender, Locale};

use crate::numbers::cardinal;

const MONTHS: [&str; 12] = [
    "Januarie",
    "Februarie",
    "Maart",
    "April",
    "Mei",
    "Junie",
    "Julie",
    "Augustus",
    "September",
    "Oktober",
    "November",
    "Desember",
];

/// Afrikaans abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 11] = [
    ("dr", "dokter"),
    ("prof", "professor"),
    ("mnr", "meneer"),
    ("mev", "mevrou"),
    ("nr", "nommer"),
    ("str", "straat"),
    ("tel", "telefoon"),
    ("ens", "ensovoorts"),
    ("bv", "byvoorbeeld"),
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

/// Afrikaans words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Afrikaans;

impl Locale for Afrikaans {
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

    /// drie Mei twee duisend vier-en-twintig.
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
        "minus"
    }

    fn plus(&self) -> &'static str {
        "plus"
    }

    fn percent(&self) -> &'static str {
        "persent"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("euro", "euro")),
            "$" | "USD" => Some(("dollar", "dollar")),
            "£" | "GBP" => Some(("pond", "pond")),
            "R" | "ZAR" => Some(("rand", "rand")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("persent"),
            "&" => Some("en"),
            "@" => Some("by"),
            "+" => Some("plus"),
            "=" => Some("is gelyk aan"),
            "°" => Some("grade"),
            "#" => Some("hutsteken"),
            "€" => Some("euro"),
            "$" => Some("dollar"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "aa",
            'b' => "bee",
            'c' => "see",
            'd' => "dee",
            'e' => "ee",
            'f' => "ef",
            'g' => "gee",
            'h' => "ha",
            'i' => "i",
            'j' => "jot",
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
            'u' => "u",
            'v' => "fee",
            'w' => "wee",
            'x' => "iks",
            'y' => "y",
            'z' => "zet",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("by"),
            '.' => Some("punt"),
            '/' => Some("skuinsstreep"),
            ':' => Some("dubbelpunt"),
            '-' => Some("koppelteken"),
            '_' => Some("onderstreep"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiouyêôûëï"
    }
}

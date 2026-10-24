use jespersen_spoken::{Gender, Locale};

use crate::{
    numbers::{cardinal, ordinal},
    spelling::lower,
};

const MONTHS: [&str; 12] = [
    "ocak", "şubat", "mart", "nisan", "mayıs", "haziran", "temmuz", "ağustos", "eylül", "ekim", "kasım", "aralık",
];

/// Turkish abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 16] = [
    ("dr", "doktor"),
    ("prof", "profesör"),
    ("doç", "doçent"),
    ("vb", "ve benzeri"),
    ("vs", "vesaire"),
    ("örn", "örneğin"),
    ("bkz", "bakınız"),
    ("yy", "yüzyıl"),
    ("sf", "sayfa"),
    ("av", "avukat"),
    ("sn", "sayın"),
    ("cad", "caddesi"),
    ("sok", "sokağı"),
    ("no", "numara"),
    ("tel", "telefon"),
    ("mah", "mahallesi"),
];

fn expansion(word: &str) -> Option<&'static str> {
    let lower = lower(word);
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

/// Turkish words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Turkish;

impl Locale for Turkish {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(ordinal(number))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "virgül" } else { "nokta" }
    }

    /// üç mayıs iki bin yirmi dört.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    /// A range is the two numbers said one after the other.
    fn range(&self) -> &'static str {
        ""
    }

    fn ratio(&self) -> &'static str {
        "bölü"
    }

    fn minus(&self) -> &'static str {
        "eksi"
    }

    fn plus(&self) -> &'static str {
        "artı"
    }

    fn percent(&self) -> &'static str {
        "yüzde"
    }

    fn percent_first(&self) -> bool {
        true
    }

    /// Turkish counts with the singular (beş lira).
    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "₺" | "TL" => Some(("lira", "lira")),
            "€" => Some(("avro", "avro")),
            "$" => Some(("dolar", "dolar")),
            "£" => Some(("sterlin", "sterlin")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("yüzde"),
            "&" => Some("ve"),
            "@" => Some("et"),
            "+" => Some("artı"),
            "=" => Some("eşittir"),
            "°" => Some("derece"),
            "#" => Some("diyez"),
            "₺" => Some("lira"),
            "€" => Some("avro"),
            "$" => Some("dolar"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match lower(&letter.to_string()).chars().next()? {
            'a' => "a",
            'b' => "be",
            'c' => "ce",
            'ç' => "çe",
            'd' => "de",
            'e' => "e",
            'f' => "fe",
            'g' => "ge",
            'ğ' => "yumuşak ge",
            'h' => "he",
            'ı' => "ı",
            'i' => "i",
            'j' => "je",
            'k' => "ke",
            'l' => "le",
            'm' => "me",
            'n' => "ne",
            'o' => "o",
            'ö' => "ö",
            'p' => "pe",
            'q' => "kü",
            'r' => "re",
            's' => "se",
            'ş' => "şe",
            't' => "te",
            'u' => "u",
            'ü' => "ü",
            'v' => "ve",
            'w' => "çift ve",
            'x' => "iks",
            'y' => "ye",
            'z' => "ze",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("et"),
            '.' => Some("nokta"),
            '/' => Some("bölü"),
            ':' => Some("iki nokta"),
            '-' => Some("tire"),
            '_' => Some("alt çizgi"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeıioöuüâîû"
    }
}

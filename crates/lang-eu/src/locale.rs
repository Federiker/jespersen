use jespersen_spoken::{Gender, Locale};

use crate::numbers::cardinal;

/// The months in the genitive, as dates say them (maiatzaren hirua).
const MONTHS: [&str; 12] = [
    "urtarrilaren",
    "otsailaren",
    "martxoaren",
    "apirilaren",
    "maiatzaren",
    "ekainaren",
    "uztailaren",
    "abuztuaren",
    "irailaren",
    "urriaren",
    "azaroaren",
    "abenduaren",
];

/// Basque abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 8] = [
    ("dr", "doktore"),
    ("etab", "eta abar"),
    ("adib", "adibidez"),
    ("or", "orrialdea"),
    ("zk", "zenbakia"),
    ("tel", "telefonoa"),
    ("k", "kalea"),
    ("jn", "jauna"),
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
    word.chars().count() > 1 && expansion(word).is_some()
}

/// Basque words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Basque;

impl Locale for Basque {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "koma" } else { "puntu" }
    }

    /// The year with -ko, the month in the genitive, the day with -a: bi mila eta hogeita lauko
    /// maiatzaren hirua.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{}ko {month} {}a", cardinal(year), cardinal(day)))
    }

    fn range(&self) -> &'static str {
        "eta"
    }

    fn ratio(&self) -> &'static str {
        "zati"
    }

    fn minus(&self) -> &'static str {
        "minus"
    }

    fn plus(&self) -> &'static str {
        "plus"
    }

    fn percent(&self) -> &'static str {
        "ehuneko"
    }

    /// Basque writes the percent sign before its number (% 10 ehuneko hamar).
    fn percent_first(&self) -> bool {
        true
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("euro", "euro")),
            "$" | "USD" => Some(("dolar", "dolar")),
            "£" | "GBP" => Some(("libera", "libera")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("ehuneko"),
            "&" => Some("eta"),
            "@" => Some("a bildua"),
            "+" => Some("gehi"),
            "=" => Some("berdin"),
            "°" => Some("gradu"),
            "#" => Some("traola"),
            "€" => Some("euro"),
            "$" => Some("dolar"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "be",
            'c' => "ze",
            'd' => "de",
            'e' => "e",
            'f' => "efe",
            'g' => "ge",
            'h' => "hatxe",
            'i' => "i",
            'j' => "jota",
            'k' => "ka",
            'l' => "ele",
            'm' => "eme",
            'n' => "ene",
            'ñ' => "eñe",
            'o' => "o",
            'p' => "pe",
            'q' => "ku",
            'r' => "erre",
            's' => "ese",
            't' => "te",
            'u' => "u",
            'v' => "uve",
            'w' => "uve bikoitza",
            'x' => "ixa",
            'y' => "i grekoa",
            'z' => "zeta",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        if word.chars().count() < 2 {
            return None;
        }
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("a bildua"),
            '.' => Some("puntu"),
            '/' => Some("barra"),
            ':' => Some("bi puntu"),
            '-' => Some("marratxo"),
            '_' => Some("azpimarra"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiou"
    }
}

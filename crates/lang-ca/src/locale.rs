use jespersen_spoken::{Gender, Locale};

use crate::numbers::{Form, cardinal};

const MONTHS: [&str; 12] = [
    "gener", "febrer", "març", "abril", "maig", "juny", "juliol", "agost", "setembre", "octubre", "novembre",
    "desembre",
];

/// Catalan abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 14] = [
    ("sr", "senyor"),
    ("sra", "senyora"),
    ("srta", "senyoreta"),
    ("dr", "doctor"),
    ("dra", "doctora"),
    ("prof", "professor"),
    ("etc", "etcètera"),
    ("núm", "número"),
    ("pàg", "pàgina"),
    ("tel", "telèfon"),
    ("av", "avinguda"),
    ("pl", "plaça"),
    ("ex", "exemple"),
    ("aprox", "aproximadament"),
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

fn form(gender: Gender) -> Form {
    match gender {
        Gender::Alone => Form::Alone,
        Gender::Masculine => Form::Masculine,
        Gender::Feminine => Form::Feminine,
    }
}

/// Catalan words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Catalan;

impl Locale for Catalan {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        Some(cardinal(number, form(gender)))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "coma" } else { "punt" }
    }

    /// les deu i trenta; the full hour is les deu, one o'clock la una.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let article = if hours == 1 { "la" } else { "les" };
        let hour = cardinal(hours, Form::Feminine);
        Some(if minutes == 0 {
            format!("{article} {hour}")
        } else {
            format!("{article} {hour} i {}", cardinal(minutes, Form::Alone))
        })
    }

    /// tres de maig de dos mil vint-i-quatre; d' before a month in a vowel (u d'abril).
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        let of = if month.starts_with(['a', 'o']) { "d'" } else { "de " };
        Some(format!(
            "{} {of}{month} de {}",
            cardinal(day, Form::Alone),
            cardinal(year, Form::Alone)
        ))
    }

    fn range(&self) -> &'static str {
        "a"
    }

    fn ratio(&self) -> &'static str {
        "entre"
    }

    fn minus(&self) -> &'static str {
        "menys"
    }

    fn plus(&self) -> &'static str {
        "més"
    }

    fn percent(&self) -> &'static str {
        "per cent"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("euro", "euros")),
            "$" | "USD" => Some(("dòlar", "dòlars")),
            "£" | "GBP" => Some(("lliura", "lliures")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("per cent"),
            "&" => Some("i"),
            "@" => Some("arrova"),
            "+" => Some("més"),
            "=" => Some("igual a"),
            "°" => Some("graus"),
            "#" => Some("coixinet"),
            "€" => Some("euros"),
            "$" => Some("dòlars"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' | 'à' => "a",
            'b' => "be",
            'c' => "ce",
            'ç' => "ce trencada",
            'd' => "de",
            'e' | 'è' | 'é' => "e",
            'f' => "efa",
            'g' => "ge",
            'h' => "hac",
            'i' | 'í' | 'ï' => "i",
            'j' => "jota",
            'k' => "ca",
            'l' => "ela",
            'm' => "ema",
            'n' => "ena",
            'o' | 'ò' | 'ó' => "o",
            'p' => "pe",
            'q' => "cu",
            'r' => "erra",
            's' => "essa",
            't' => "te",
            'u' | 'ú' | 'ü' => "u",
            'v' => "ve baixa",
            'w' => "ve doble",
            'x' => "ics",
            'y' => "i grega",
            'z' => "zeta",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("arrova"),
            '.' => Some("punt"),
            '/' => Some("barra"),
            ':' => Some("dos punts"),
            '-' => Some("guionet"),
            '_' => Some("guió baix"),
            _ => None,
        }
    }

    /// Nouns in -a are feminine (una lliura); the rest masculine, plurals in -es among them (dos
    /// llibres).
    fn gender_of(&self, word: &str) -> Gender {
        if word.ends_with('a') {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    fn elides(&self) -> bool {
        true
    }

    fn vowels(&self) -> &'static str {
        "aàeèéiíïoòóuúü"
    }
}

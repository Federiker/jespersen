use jespersen_spoken::{Gender, Locale};

use crate::{
    abbreviations::{expansion, letter_name},
    numbers::{cardinal, ordinal},
};

const MONTHS: [&str; 12] = [
    "enero",
    "febrero",
    "marzo",
    "abril",
    "mayo",
    "junio",
    "julio",
    "agosto",
    "septiembre",
    "octubre",
    "noviembre",
    "diciembre",
];

/// Nouns ending in -a that are masculine, for the agreement of un and una (veintiún días).
const MASCULINE_IN_A: [&str; 12] = [
    "día", "mapa", "problema", "tema", "sistema", "programa", "idioma", "clima", "planeta", "poema", "drama", "cometa",
];

/// Words after which a Roman numeral is a cardinal (el siglo XX, el capítulo IV).
const BEFORE_ROMAN: [&str; 6] = ["siglo", "siglos", "capítulo", "tomo", "volumen", "parte"];

/// Spanish words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Spanish;

impl Locale for Spanish {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        Some(cardinal(number, gender))
    }

    fn ordinal(&self, number: u64, gender: Gender) -> Option<String> {
        ordinal(number, gender)
    }

    fn ordinal_signs(&self) -> &'static [(&'static str, Gender)] {
        &[("º", Gender::Alone), ("ª", Gender::Feminine), ("er", Gender::Masculine)]
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "coma" } else { "punto" }
    }

    /// 8:30 ocho y media, 1:00 una en punto, 9:15 nueve y cuarto.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours, if hours == 1 { Gender::Feminine } else { Gender::Alone });
        let after = match minutes {
            0 => "en punto".to_owned(),
            15 => "y cuarto".to_owned(),
            30 => "y media".to_owned(),
            minutes => format!("y {}", cardinal(minutes, Gender::Alone)),
        };
        Some(format!("{hour} {after}"))
    }

    /// Day first, as Spanish writes it: tres de mayo de dos mil veinticuatro.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!(
            "{} de {month} de {}",
            cardinal(day, Gender::Alone),
            cardinal(year, Gender::Alone)
        ))
    }

    fn range(&self) -> &'static str {
        "a"
    }

    fn ratio(&self) -> &'static str {
        "entre"
    }

    fn minus(&self) -> &'static str {
        "menos"
    }

    fn plus(&self) -> &'static str {
        "más"
    }

    fn percent(&self) -> &'static str {
        "por ciento"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" => Some(("euro", "euros")),
            "$" => Some(("dólar", "dólares")),
            "£" => Some(("libra", "libras")),
            "¥" => Some(("yen", "yenes")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("por ciento"),
            "&" => Some("y"),
            "@" => Some("arroba"),
            "+" => Some("más"),
            "=" => Some("igual a"),
            "°" | "º" => Some("grados"),
            "#" => Some("número"),
            "€" => Some("euros"),
            "$" => Some("dólares"),
            "£" => Some("libras"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        letter_name(letter)
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("arroba"),
            '.' => Some("punto"),
            '/' => Some("barra"),
            ':' => Some("dos puntos"),
            '-' => Some("guion"),
            '_' => Some("guion bajo"),
            _ => None,
        }
    }

    /// Feminine for -a and -as (but un problema, veintiún días), masculine otherwise.
    fn gender_of(&self, word: &str) -> Gender {
        let lower = word.to_lowercase();
        let singular = lower.strip_suffix('s').unwrap_or(&lower);
        if singular.ends_with('a') && !MASCULINE_IN_A.contains(&singular) {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    /// A cardinal after siglo and the like (el siglo veinte), an ordinal after a name up to ten
    /// (Felipe segundo) and a cardinal above (Alfonso trece).
    fn roman(&self, value: u64, before: Option<&str>, _after: Option<&str>) -> Option<String> {
        let before = before?;
        if BEFORE_ROMAN.contains(&before.to_lowercase().as_str()) {
            return Some(cardinal(value, Gender::Alone));
        }
        before
            .chars()
            .next()
            .is_some_and(char::is_uppercase)
            .then(|| ordinal(value, Gender::Alone).unwrap_or_else(|| cardinal(value, Gender::Alone)))
    }

    fn vowels(&self) -> &'static str {
        "aeiouáéíóúü"
    }
}

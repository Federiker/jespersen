use jespersen_spoken::{Gender, Locale};

use crate::numbers::{cardinal, form};

/// The months in the genitive, as a date says them (gegužės trys).
const MONTHS: [&str; 12] = [
    "sausio",
    "vasario",
    "kovo",
    "balandžio",
    "gegužės",
    "birželio",
    "liepos",
    "rugpjūčio",
    "rugsėjo",
    "spalio",
    "lapkričio",
    "gruodžio",
];

/// Lithuanian abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 11] = [
    ("dr", "daktaras"),
    ("prof", "profesorius"),
    ("nr", "numeris"),
    ("pvz", "pavyzdžiui"),
    ("kt", "kiti"),
    ("val", "valanda"),
    ("tel", "telefonas"),
    ("mln", "milijonai"),
    ("tūkst", "tūkstančiai"),
    ("proc", "procentai"),
    ("doc", "docentas"),
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

/// A currency's three forms: after one, after two to nine, and after ten to twenty or a round ten.
fn currency_forms(sign: &str) -> Option<[&'static str; 3]> {
    match sign {
        "€" | "EUR" => Some(["euras", "eurai", "eurų"]),
        "$" | "USD" => Some(["doleris", "doleriai", "dolerių"]),
        "£" | "GBP" => Some(["svaras", "svarai", "svarų"]),
        _ => None,
    }
}

/// Lithuanian words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Lithuanian;

impl Locale for Lithuanian {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        Some(cardinal(number, gender == Gender::Feminine))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "kablelis" } else { "taškas" }
    }

    /// dešimt trisdešimt; the full hour is just its number.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours, true);
        Some(if minutes == 0 {
            hour
        } else {
            format!("{hour} {}", cardinal(minutes, true))
        })
    }

    /// du tūkstančiai dvidešimt-keturi gegužės trys: the year, the month, the day.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(year, false), cardinal(day, false)))
    }

    fn range(&self) -> &'static str {
        "iki"
    }

    fn ratio(&self) -> &'static str {
        "iš"
    }

    fn minus(&self) -> &'static str {
        "minus"
    }

    fn plus(&self) -> &'static str {
        "plius"
    }

    fn percent(&self) -> &'static str {
        "procentų"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        let forms = currency_forms(sign)?;
        Some((forms[0], forms[1]))
    }

    /// A currency after a number in the form the number asks for (vienas euras, du eurai, dešimt
    /// eurų); after a fraction the genitive singular, which the plural genitive stands in for.
    fn currency_name(&self, sign: &str, count: Option<u64>) -> Option<&'static str> {
        let forms = currency_forms(sign)?;
        Some(forms[count.map_or(2, form)])
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("procentų"),
            "&" => Some("ir"),
            "@" => Some("eta"),
            "+" => Some("plius"),
            "=" => Some("lygu"),
            "°" => Some("laipsnių"),
            "#" => Some("grotelės"),
            "€" => Some("eurų"),
            "$" => Some("dolerių"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'ą' => "a nosinė",
            'b' => "bė",
            'c' => "cė",
            'č' => "čė",
            'd' => "dė",
            'e' => "e",
            'ę' => "e nosinė",
            'ė' => "ė",
            'f' => "ef",
            'g' => "gė",
            'h' => "ha",
            'i' => "i",
            'į' => "i nosinė",
            'y' => "ilgoji i",
            'j' => "jot",
            'k' => "ka",
            'l' => "el",
            'm' => "em",
            'n' => "en",
            'o' => "o",
            'p' => "pė",
            'q' => "ku",
            'r' => "er",
            's' => "es",
            'š' => "eš",
            't' => "tė",
            'u' => "u",
            'ų' => "u nosinė",
            'ū' => "ilgoji u",
            'v' => "vė",
            'w' => "dviguba vė",
            'x' => "iks",
            'z' => "zė",
            'ž' => "žė",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("eta"),
            '.' => Some("taškas"),
            '/' => Some("pasvirasis brūkšnys"),
            ':' => Some("dvitaškis"),
            '-' => Some("brūkšnelis"),
            '_' => Some("pabraukimas"),
            _ => None,
        }
    }

    /// Nouns in -a, -ė and their plurals -os, -ės are feminine (dvi knygos); the rest masculine.
    fn gender_of(&self, word: &str) -> Gender {
        let lower = word.to_lowercase();
        if ["a", "ė", "os", "ės"].iter().any(|ending| lower.ends_with(ending)) {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    fn vowels(&self) -> &'static str {
        "aąeęėiįyouųū"
    }
}

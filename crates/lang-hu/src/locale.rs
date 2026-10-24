use jespersen_spoken::{Gender, Locale};

use crate::numbers::cardinal;

const MONTHS: [&str; 12] = [
    "január",
    "február",
    "március",
    "április",
    "május",
    "június",
    "július",
    "augusztus",
    "szeptember",
    "október",
    "november",
    "december",
];

/// Hungarian abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 10] = [
    ("dr", "doktor"),
    ("prof", "professzor"),
    ("u", "utca"),
    ("sz", "szám"),
    ("kb", "körülbelül"),
    ("pl", "például"),
    ("stb", "satöbbi"),
    ("ill", "illetve"),
    ("tel", "telefon"),
    ("id", "idősebb"),
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

/// Hungarian words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Hungarian;

impl Locale for Hungarian {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "egész" } else { "pont" }
    }

    /// tíz harminc; the full hour is tíz óra.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours);
        Some(if minutes == 0 {
            format!("{hour} óra")
        } else {
            format!("{hour} {}", cardinal(minutes))
        })
    }

    /// kétezer huszonnégy május három: the year, the month, the day.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(year), cardinal(day)))
    }

    fn range(&self) -> &'static str {
        "tól"
    }

    fn ratio(&self) -> &'static str {
        "per"
    }

    fn minus(&self) -> &'static str {
        "mínusz"
    }

    fn plus(&self) -> &'static str {
        "plusz"
    }

    fn percent(&self) -> &'static str {
        "százalék"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("euró", "euró")),
            "$" | "USD" => Some(("dollár", "dollár")),
            "£" | "GBP" => Some(("font", "font")),
            "Ft" | "HUF" => Some(("forint", "forint")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("százalék"),
            "&" => Some("és"),
            "@" => Some("kukac"),
            "+" => Some("plusz"),
            "=" => Some("egyenlő"),
            "°" => Some("fok"),
            "#" => Some("kettőskereszt"),
            "€" => Some("euró"),
            "$" => Some("dollár"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'á' => "á",
            'b' => "bé",
            'c' => "cé",
            'd' => "dé",
            'e' => "e",
            'é' => "é",
            'f' => "ef",
            'g' => "gé",
            'h' => "há",
            'i' => "i",
            'í' => "í",
            'j' => "jé",
            'k' => "ká",
            'l' => "el",
            'm' => "em",
            'n' => "en",
            'o' => "o",
            'ó' => "ó",
            'ö' => "ö",
            'ő' => "ő",
            'p' => "pé",
            'q' => "kú",
            'r' => "er",
            's' => "es",
            't' => "té",
            'u' => "u",
            'ú' => "ú",
            'ü' => "ü",
            'ű' => "ű",
            'v' => "vé",
            'w' => "duplavé",
            'x' => "iksz",
            'y' => "ipszilon",
            'z' => "zé",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("kukac"),
            '.' => Some("pont"),
            '/' => Some("per"),
            ':' => Some("kettőspont"),
            '-' => Some("kötőjel"),
            '_' => Some("alsóvonás"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aáeéiíoóöőuúüű"
    }
}

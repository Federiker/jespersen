use jespersen_spoken::{Gender, Locale};

use crate::numbers::cardinal;

const MONTHS: [&str; 12] = [
    "Eanáir",
    "Feabhra",
    "Márta",
    "Aibreán",
    "Bealtaine",
    "Meitheamh",
    "Iúil",
    "Lúnasa",
    "Meán Fómhair",
    "Deireadh Fómhair",
    "Samhain",
    "Nollaig",
];

/// Irish abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 6] = [
    ("dr", "dochtúir"),
    ("srl", "agus araile"),
    ("uimh", "uimhir"),
    ("lch", "leathanach"),
    ("co", "contae"),
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

/// Irish words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Irish;

impl Locale for Irish {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    fn decimal_word(&self, _mark: char) -> &'static str {
        "pointe"
    }

    /// a deich tríocha; the full hour is a deich a chlog.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours);
        Some(if minutes == 0 {
            format!("{hour} a chlog")
        } else {
            format!("{hour} {}", cardinal(minutes))
        })
    }

    /// a trí Bealtaine dhá mhíle fiche a ceathair.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "go"
    }

    fn ratio(&self) -> &'static str {
        "roinnte ar"
    }

    fn minus(&self) -> &'static str {
        "lúide"
    }

    fn plus(&self) -> &'static str {
        "móide"
    }

    fn percent(&self) -> &'static str {
        "faoin gcéad"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("euro", "cent")),
            "$" | "USD" => Some(("dollar", "cent")),
            "£" | "GBP" => Some(("punt", "pingin")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("faoin gcéad"),
            "&" => Some("agus"),
            "@" => Some("ag"),
            "+" => Some("móide"),
            "=" => Some("cothrom le"),
            "°" => Some("céim"),
            "#" => Some("haischlib"),
            "€" => Some("euro"),
            "$" => Some("dollar"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' | 'á' => "á",
            'b' => "bé",
            'c' => "cé",
            'd' => "dé",
            'e' | 'é' => "é",
            'f' => "eif",
            'g' => "gé",
            'h' => "héis",
            'i' | 'í' => "í",
            'j' => "jé",
            'k' => "cá",
            'l' => "eil",
            'm' => "eim",
            'n' => "ein",
            'o' | 'ó' => "ó",
            'p' => "pé",
            'q' => "cú",
            'r' => "ear",
            's' => "eas",
            't' => "té",
            'u' | 'ú' => "ú",
            'v' => "vé",
            'w' => "wae",
            'x' => "eics",
            'y' => "uaidhe",
            'z' => "zae",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("ag"),
            '.' => Some("ponc"),
            '/' => Some("slais"),
            ':' => Some("idirstad"),
            '-' => Some("fleiscín"),
            '_' => Some("fostríoc"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiouáéíóú"
    }
}

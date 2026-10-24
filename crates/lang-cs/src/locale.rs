use jespersen_spoken::{Gender, Locale};

use crate::numbers::{Agreement, Case, cardinal, form, ordinal};

/// The months in the genitive, as dates say them (třetího května).
const MONTHS: [&str; 12] = [
    "ledna",
    "února",
    "března",
    "dubna",
    "května",
    "června",
    "července",
    "srpna",
    "září",
    "října",
    "listopadu",
    "prosince",
];

/// Czech abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 18] = [
    ("atd", "a tak dále"),
    ("např", "například"),
    ("tj", "to jest"),
    ("tzv", "takzvaný"),
    ("tzn", "to znamená"),
    ("apod", "a podobně"),
    ("mj", "mimo jiné"),
    ("resp", "respektive"),
    ("dr", "doktor"),
    ("prof", "profesor"),
    ("ing", "inženýr"),
    ("mgr", "magistr"),
    ("ul", "ulice"),
    ("č", "číslo"),
    ("str", "strana"),
    ("sv", "svatý"),
    ("tel", "telefon"),
    ("max", "maximálně"),
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

/// The names of currencies after a count of one, of two to four, and of the rest.
fn currency_forms(sign: &str) -> Option<[&'static str; 3]> {
    match sign {
        "Kč" | "CZK" => Some(["koruna", "koruny", "korun"]),
        "€" | "EUR" => Some(["euro", "eura", "eur"]),
        "$" | "USD" => Some(["dolar", "dolary", "dolarů"]),
        "£" | "GBP" => Some(["libra", "libry", "liber"]),
        _ => None,
    }
}

/// Czech words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Czech;

impl Locale for Czech {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        let agreement = match gender {
            Gender::Alone => Agreement::Counting,
            Gender::Masculine => Agreement::Masculine,
            Gender::Feminine => Agreement::Feminine,
        };
        Some(cardinal(number, agreement))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "celá" } else { "tečka" }
    }

    /// The day as an ordinal in the genitive, the month too, the year as a number: třetího května
    /// dva tisíce dvacet čtyři.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!(
            "{} {month} {}",
            ordinal(day, Case::Genitive)?,
            cardinal(year, Agreement::Counting)
        ))
    }

    fn range(&self) -> &'static str {
        "až"
    }

    fn ratio(&self) -> &'static str {
        "lomeno"
    }

    fn minus(&self) -> &'static str {
        "minus"
    }

    fn plus(&self) -> &'static str {
        "plus"
    }

    fn percent(&self) -> &'static str {
        "procent"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        currency_forms(sign).map(|[one, _, many]| (one, many))
    }

    fn currency_name(&self, sign: &str, count: Option<u64>) -> Option<&'static str> {
        let forms = currency_forms(sign)?;
        Some(count.map_or(forms[2], |count| form(count, forms)))
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("procent"),
            "&" => Some("a"),
            "@" => Some("zavináč"),
            "+" => Some("plus"),
            "=" => Some("rovná se"),
            "°" => Some("stupňů"),
            "#" => Some("mřížka"),
            "€" => Some("eur"),
            "$" => Some("dolarů"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'á' => "á",
            'b' => "bé",
            'c' => "cé",
            'č' => "čé",
            'd' => "dé",
            'ď' => "ďé",
            'e' | 'ě' => "é",
            'é' => "dlouhé é",
            'f' => "ef",
            'g' => "gé",
            'h' => "há",
            'i' | 'y' => "í",
            'í' | 'ý' => "dlouhé í",
            'j' => "jé",
            'k' => "ká",
            'l' => "el",
            'm' => "em",
            'n' => "en",
            'ň' => "eň",
            'o' | 'ó' => "ó",
            'p' => "pé",
            'q' => "kvé",
            'r' => "er",
            'ř' => "eř",
            's' => "es",
            'š' => "eš",
            't' => "té",
            'ť' => "ťé",
            'u' | 'ú' | 'ů' => "ú",
            'v' => "vé",
            'w' => "dvojité vé",
            'x' => "iks",
            'z' => "zet",
            'ž' => "žet",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("zavináč"),
            '.' => Some("tečka"),
            '/' => Some("lomítko"),
            ':' => Some("dvojtečka"),
            '-' => Some("pomlčka"),
            '_' => Some("podtržítko"),
            _ => None,
        }
    }

    /// Feminine nouns in the singular end in -a (jedna koruna); the rest take jeden.
    fn gender_of(&self, word: &str) -> Gender {
        if word.ends_with('a') {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    /// A Roman numeral before století, a century, is an ordinal in the neuter (XX. století
    /// dvacáté), after a name in the masculine (Karel IV. čtvrtý).
    fn roman(&self, value: u64, before: Option<&str>, after: Option<&str>) -> Option<String> {
        if after.is_some_and(|after| after.to_lowercase().starts_with("stolet")) {
            return ordinal(value, Case::Neuter);
        }
        if before.is_some_and(|before| before.chars().next().is_some_and(char::is_uppercase)) {
            return ordinal(value, Case::Masculine);
        }
        None
    }

    fn vowels(&self) -> &'static str {
        "aáeéěiíoóuúůyý"
    }
}

use jespersen_spoken::{Gender, Locale};

use crate::numbers::{Agreement, cardinal, day, form};

/// The months in the genitive, as dates say them (tretjega maja).
const MONTHS: [&str; 12] = [
    "januarja",
    "februarja",
    "marca",
    "aprila",
    "maja",
    "junija",
    "julija",
    "avgusta",
    "septembra",
    "oktobra",
    "novembra",
    "decembra",
];

/// Slovenian abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 14] = [
    ("dr", "doktor"),
    ("prof", "profesor"),
    ("mag", "magister"),
    ("inž", "inženir"),
    ("št", "številka"),
    ("npr", "na primer"),
    ("tj", "to je"),
    ("itd", "in tako dalje"),
    ("ipd", "in podobno"),
    ("oz", "oziroma"),
    ("ul", "ulica"),
    ("tel", "telefon"),
    ("str", "stran"),
    ("sv", "sveti"),
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

/// The names of currencies after a count of one, of two, of three or four, and of the rest.
fn currency_forms(sign: &str) -> Option<[&'static str; 4]> {
    match sign {
        "€" | "EUR" => Some(["evro", "evra", "evri", "evrov"]),
        "$" | "USD" => Some(["dolar", "dolarja", "dolarji", "dolarjev"]),
        "£" | "GBP" => Some(["funt", "funta", "funti", "funtov"]),
        _ => None,
    }
}

/// Slovenian words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Slovenian;

impl Locale for Slovenian {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        let agreement = match gender {
            Gender::Alone => Agreement::Counting,
            Gender::Masculine => Agreement::Masculine,
            Gender::Feminine => Agreement::Feminine,
        };
        Some(cardinal(number, agreement))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "cela" } else { "pika" }
    }

    /// The day as an ordinal in the genitive, the month too, the year as a number: tretjega maja
    /// dva tisoč štiriindvajset.
    fn date(&self, day_of_month: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!(
            "{} {month} {}",
            day(day_of_month)?,
            cardinal(year, Agreement::Counting)
        ))
    }

    fn range(&self) -> &'static str {
        "do"
    }

    fn ratio(&self) -> &'static str {
        "skozi"
    }

    fn minus(&self) -> &'static str {
        "minus"
    }

    fn plus(&self) -> &'static str {
        "plus"
    }

    fn percent(&self) -> &'static str {
        "odstotkov"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        currency_forms(sign).map(|[one, .., many]| (one, many))
    }

    fn currency_name(&self, sign: &str, count: Option<u64>) -> Option<&'static str> {
        let forms = currency_forms(sign)?;
        Some(count.map_or(forms[3], |count| form(count, forms)))
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("odstotkov"),
            "&" => Some("in"),
            "@" => Some("afna"),
            "+" => Some("plus"),
            "=" => Some("je enako"),
            "°" => Some("stopinj"),
            "#" => Some("lojtra"),
            "€" => Some("evrov"),
            "$" => Some("dolarjev"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "be",
            'c' => "ce",
            'č' => "če",
            'd' => "de",
            'e' => "e",
            'f' => "ef",
            'g' => "ge",
            'h' => "ha",
            'i' => "i",
            'j' => "je",
            'k' => "ka",
            'l' => "el",
            'm' => "em",
            'n' => "en",
            'o' => "o",
            'p' => "pe",
            'q' => "ku",
            'r' => "er",
            's' => "es",
            'š' => "eš",
            't' => "te",
            'u' => "u",
            'v' => "ve",
            'w' => "dvojni ve",
            'x' => "iks",
            'y' => "ipsilon",
            'z' => "ze",
            'ž' => "že",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("afna"),
            '.' => Some("pika"),
            '/' => Some("poševnica"),
            ':' => Some("dvopičje"),
            '-' => Some("vezaj"),
            '_' => Some("podčrtaj"),
            _ => None,
        }
    }

    /// Feminine nouns in the singular end in -a (ena knjiga); the rest take en.
    fn gender_of(&self, word: &str) -> Gender {
        if word.ends_with('a') {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiou"
    }
}

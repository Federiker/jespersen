use jespersen_spoken::{Gender, Locale};

use crate::numbers::{Agreement, cardinal, day, form};

/// The months in the genitive, as dates say them (trećeg svibnja).
const MONTHS: [&str; 12] = [
    "siječnja",
    "veljače",
    "ožujka",
    "travnja",
    "svibnja",
    "lipnja",
    "srpnja",
    "kolovoza",
    "rujna",
    "listopada",
    "studenoga",
    "prosinca",
];

/// Croatian abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 14] = [
    ("dr", "doktor"),
    ("prof", "profesor"),
    ("mr", "magistar"),
    ("ing", "inženjer"),
    ("br", "broj"),
    ("npr", "na primjer"),
    ("tj", "to jest"),
    ("itd", "i tako dalje"),
    ("sl", "slično"),
    ("god", "godine"),
    ("ul", "ulica"),
    ("tel", "telefon"),
    ("st", "stoljeće"),
    ("gđa", "gospođa"),
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
        "€" | "EUR" => Some(["euro", "eura", "eura"]),
        "$" | "USD" => Some(["dolar", "dolara", "dolara"]),
        "£" | "GBP" => Some(["funta", "funte", "funti"]),
        "kn" | "HRK" => Some(["kuna", "kune", "kuna"]),
        _ => None,
    }
}

/// Croatian words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Croatian;

impl Locale for Croatian {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        let agreement = if gender == Gender::Feminine {
            Agreement::Feminine
        } else {
            Agreement::Masculine
        };
        Some(cardinal(number, agreement))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "zarez" } else { "točka" }
    }

    /// deset i trideset; the full hour is deset sati.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours, Agreement::Masculine);
        Some(if minutes == 0 {
            format!("{hour} sati")
        } else {
            format!("{hour} i {}", cardinal(minutes, Agreement::Feminine))
        })
    }

    /// The day as an ordinal in the genitive, the month too, the year as a number: trećeg svibnja
    /// dvije tisuće dvadeset četiri.
    fn date(&self, day_of_month: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!(
            "{} {month} {}",
            day(day_of_month)?,
            cardinal(year, Agreement::Feminine)
        ))
    }

    fn range(&self) -> &'static str {
        "do"
    }

    fn ratio(&self) -> &'static str {
        "kroz"
    }

    fn minus(&self) -> &'static str {
        "minus"
    }

    fn plus(&self) -> &'static str {
        "plus"
    }

    fn percent(&self) -> &'static str {
        "posto"
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
            "%" => Some("posto"),
            "&" => Some("i"),
            "@" => Some("et"),
            "+" => Some("plus"),
            "=" => Some("jednako"),
            "°" => Some("stupnjeva"),
            "#" => Some("ljestve"),
            "€" => Some("eura"),
            "$" => Some("dolara"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "be",
            'c' => "ce",
            'č' => "če",
            'ć' => "će",
            'd' => "de",
            'đ' => "đe",
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
            'w' => "dvostruko ve",
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
            '@' => Some("et"),
            '.' => Some("točka"),
            '/' => Some("kosa crta"),
            ':' => Some("dvotočka"),
            '-' => Some("crtica"),
            '_' => Some("podvlaka"),
            _ => None,
        }
    }

    /// Feminine nouns in the singular end in -a (jedna knjiga); the rest take jedan.
    fn gender_of(&self, word: &str) -> Gender {
        if word.ends_with('a') || word.ends_with('u') {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiou"
    }
}

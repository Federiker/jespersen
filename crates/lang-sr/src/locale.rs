use jespersen_spoken::{Gender, Locale};

use crate::{
    cyrillic::to_latin,
    numbers::{Agreement, cardinal, day, form},
};

/// The months in the genitive, as dates say them (trećeg maja).
const MONTHS: [&str; 12] = [
    "januara",
    "februara",
    "marta",
    "aprila",
    "maja",
    "juna",
    "jula",
    "avgusta",
    "septembra",
    "oktobra",
    "novembra",
    "decembra",
];

/// Serbian abbreviations, in Latin letters, and how they are said.
const ABBREVIATIONS: [(&str, &str); 15] = [
    ("dr", "doktor"),
    ("prof", "profesor"),
    ("mr", "magistar"),
    ("inž", "inženjer"),
    ("br", "broj"),
    ("npr", "na primer"),
    ("tj", "to jest"),
    ("itd", "i tako dalje"),
    ("sl", "slično"),
    ("god", "godine"),
    ("ul", "ulica"),
    ("tel", "telefon"),
    ("gđa", "gospođa"),
    ("gđica", "gospođica"),
    ("str", "strana"),
];

/// How an abbreviation in either script is said.
fn expansion(word: &str) -> Option<&'static str> {
    let latin = to_latin(word);
    ABBREVIATIONS
        .iter()
        .find(|(abbreviation, _)| *abbreviation == latin)
        .map(|(_, said)| *said)
}

/// Whether `word` (without its full stop, in either script) is an abbreviation that does not end
/// a sentence.
#[must_use]
pub fn is_abbreviation(word: &str) -> bool {
    expansion(word).is_some()
}

/// The names of currencies after a count of one, of two to four, and of the rest.
fn currency_forms(sign: &str) -> Option<[&'static str; 3]> {
    match sign {
        "din" | "дин" | "RSD" => Some(["dinar", "dinara", "dinara"]),
        "€" | "EUR" => Some(["evro", "evra", "evra"]),
        "$" | "USD" => Some(["dolar", "dolara", "dolara"]),
        "£" | "GBP" => Some(["funta", "funte", "funti"]),
        _ => None,
    }
}

/// Serbian words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Serbian;

impl Locale for Serbian {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        let agreement = if gender == Gender::Feminine {
            Agreement::Feminine
        } else {
            Agreement::Masculine
        };
        Some(cardinal(number, agreement))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "zapeta" } else { "tačka" }
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

    /// The day as an ordinal in the genitive, the month too, the year as a number: trećeg maja
    /// dve hiljade dvadeset četiri.
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
        "odsto"
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
            "%" => Some("odsto"),
            "&" => Some("i"),
            "@" => Some("et"),
            "+" => Some("plus"),
            "=" => Some("jednako"),
            "°" => Some("stepeni"),
            "#" => Some("taraba"),
            "€" => Some("evra"),
            "$" => Some("dolara"),
            _ => None,
        }
    }

    /// A letter's name, in either script (be, ve, lje, dže).
    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match to_latin(&letter.to_string()).as_str() {
            "a" => "a",
            "b" => "be",
            "c" => "ce",
            "č" => "če",
            "ć" => "će",
            "d" => "de",
            "dž" => "dže",
            "đ" => "đe",
            "e" => "e",
            "f" => "ef",
            "g" => "ge",
            "h" => "ha",
            "i" => "i",
            "j" => "je",
            "k" => "ka",
            "l" => "el",
            "lj" => "lje",
            "m" => "em",
            "n" => "en",
            "nj" => "nje",
            "o" => "o",
            "p" => "pe",
            "q" => "ku",
            "r" => "er",
            "s" => "es",
            "š" => "še",
            "t" => "te",
            "u" => "u",
            "v" => "ve",
            "w" => "dvostruko ve",
            "x" => "iks",
            "y" => "ipsilon",
            "z" => "ze",
            "ž" => "že",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("et"),
            '.' => Some("tačka"),
            '/' => Some("kosa crta"),
            ':' => Some("dve tačke"),
            '-' => Some("crtica"),
            '_' => Some("donja crta"),
            _ => None,
        }
    }

    /// Feminine nouns in the singular end in -a (jedna knjiga); the rest take jedan.
    fn gender_of(&self, word: &str) -> Gender {
        let latin = to_latin(word);
        if latin.ends_with('a') || latin.ends_with('u') {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiouаеиоу"
    }
}

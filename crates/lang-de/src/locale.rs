use jespersen_spoken::{Gender, Locale};

use crate::numbers::{Form, cardinal};

const MONTHS: [&str; 12] = [
    "Januar",
    "Februar",
    "März",
    "April",
    "Mai",
    "Juni",
    "Juli",
    "August",
    "September",
    "Oktober",
    "November",
    "Dezember",
];

/// The ordinals of the days of a month, as a date says them (der dritte Mai).
const DAYS: [&str; 31] = [
    "erste",
    "zweite",
    "dritte",
    "vierte",
    "fünfte",
    "sechste",
    "siebte",
    "achte",
    "neunte",
    "zehnte",
    "elfte",
    "zwölfte",
    "dreizehnte",
    "vierzehnte",
    "fünfzehnte",
    "sechzehnte",
    "siebzehnte",
    "achtzehnte",
    "neunzehnte",
    "zwanzigste",
    "einundzwanzigste",
    "zweiundzwanzigste",
    "dreiundzwanzigste",
    "vierundzwanzigste",
    "fünfundzwanzigste",
    "sechsundzwanzigste",
    "siebenundzwanzigste",
    "achtundzwanzigste",
    "neunundzwanzigste",
    "dreißigste",
    "einunddreißigste",
];

/// German abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 16] = [
    ("dr", "Doktor"),
    ("prof", "Professor"),
    ("hr", "Herr"),
    ("fr", "Frau"),
    ("nr", "Nummer"),
    ("str", "Straße"),
    ("tel", "Telefon"),
    ("usw", "und so weiter"),
    ("bzw", "beziehungsweise"),
    ("ca", "circa"),
    ("etc", "et cetera"),
    ("evtl", "eventuell"),
    ("ggf", "gegebenenfalls"),
    ("inkl", "inklusive"),
    ("vgl", "vergleiche"),
    ("st", "Sankt"),
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

/// German words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct German;

impl Locale for German {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        let form = match gender {
            Gender::Alone => Form::Alone,
            Gender::Masculine => Form::Before,
            Gender::Feminine => Form::Feminine,
        };
        Some(cardinal(number, form))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "Komma" } else { "Punkt" }
    }

    /// zehn Uhr dreißig; the full hour is zehn Uhr.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours, Form::Before);
        Some(if minutes == 0 {
            format!("{hour} Uhr")
        } else {
            format!("{hour} Uhr {}", cardinal(minutes, Form::Alone))
        })
    }

    /// dritter Mai zwei tausend vier und-zwanzig.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        let day = DAYS.get(usize::try_from(day).ok()?.checked_sub(1)?)?;
        Some(format!("{day}r {month} {}", cardinal(year, Form::Alone)))
    }

    fn range(&self) -> &'static str {
        "bis"
    }

    fn ratio(&self) -> &'static str {
        "durch"
    }

    fn minus(&self) -> &'static str {
        "minus"
    }

    fn plus(&self) -> &'static str {
        "plus"
    }

    fn percent(&self) -> &'static str {
        "Prozent"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("Euro", "Euro")),
            "$" | "USD" => Some(("Dollar", "Dollar")),
            "£" | "GBP" => Some(("Pfund", "Pfund")),
            "CHF" => Some(("Franken", "Franken")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("Prozent"),
            "&" => Some("und"),
            "@" => Some("at"),
            "+" => Some("plus"),
            "=" => Some("gleich"),
            "°" => Some("Grad"),
            "#" => Some("Raute"),
            "€" => Some("Euro"),
            "$" => Some("Dollar"),
            "§" => Some("Paragraf"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'ä' => "ä",
            'b' => "be",
            'c' => "ce",
            'd' => "de",
            'e' => "e",
            'f' => "ef",
            'g' => "ge",
            'h' => "ha",
            'i' => "i",
            'j' => "jott",
            'k' => "ka",
            'l' => "el",
            'm' => "em",
            'n' => "en",
            'o' => "o",
            'ö' => "ö",
            'p' => "pe",
            'q' => "ku",
            'r' => "er",
            's' => "es",
            'ß' => "eszett",
            't' => "te",
            'u' => "u",
            'ü' => "ü",
            'v' => "vau",
            'w' => "we",
            'x' => "ix",
            'y' => "ypsilon",
            'z' => "zett",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("at"),
            '.' => Some("Punkt"),
            '/' => Some("Schrägstrich"),
            ':' => Some("Doppelpunkt"),
            '-' => Some("Bindestrich"),
            '_' => Some("Unterstrich"),
            _ => None,
        }
    }

    /// The nouns in -e, -ung, -heit, -keit and -schaft are feminine (eine Stunde, eine Zeitung);
    /// the rest are counted as masculine and neuter are (ein Euro).
    fn gender_of(&self, word: &str) -> Gender {
        let lower = word.to_lowercase();
        if ["e", "ung", "heit", "keit", "schaft", "ion"]
            .iter()
            .any(|ending| lower.ends_with(ending))
        {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    fn vowels(&self) -> &'static str {
        "aäeioöuüy"
    }
}

use jespersen_spoken::{Gender, Locale};

use crate::numbers::{Case, cardinal, form, ordinal};

/// The months in the genitive, as dates say them (trzeciego maja).
const MONTHS: [&str; 12] = [
    "stycznia",
    "lutego",
    "marca",
    "kwietnia",
    "maja",
    "czerwca",
    "lipca",
    "sierpnia",
    "września",
    "października",
    "listopada",
    "grudnia",
];

/// Polish abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 22] = [
    ("dr", "doktor"),
    ("prof", "profesor"),
    ("mgr", "magister"),
    ("inż", "inżynier"),
    ("ul", "ulica"),
    ("al", "aleja"),
    ("np", "na przykład"),
    ("itd", "i tak dalej"),
    ("itp", "i tym podobne"),
    ("tzn", "to znaczy"),
    ("tzw", "tak zwany"),
    ("nr", "numer"),
    ("tel", "telefon"),
    ("godz", "godzina"),
    ("wg", "według"),
    ("zob", "zobacz"),
    ("str", "strona"),
    ("św", "święty"),
    ("ks", "ksiądz"),
    ("tys", "tysięcy"),
    ("mln", "milionów"),
    ("mld", "miliardów"),
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
        "zł" | "PLN" => Some(["złoty", "złote", "złotych"]),
        "gr" => Some(["grosz", "grosze", "groszy"]),
        "€" | "EUR" => Some(["euro", "euro", "euro"]),
        "$" | "USD" => Some(["dolar", "dolary", "dolarów"]),
        "£" | "GBP" => Some(["funt", "funty", "funtów"]),
        _ => None,
    }
}

/// Polish words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Polish;

impl Locale for Polish {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        Some(cardinal(number, gender == Gender::Feminine))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "przecinek" } else { "kropka" }
    }

    /// The hour as a feminine ordinal and the minutes as a number: dziesiąta trzydzieści, ósma
    /// zero pięć.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = if hours == 0 {
            "zero".to_owned()
        } else {
            ordinal(hours, Case::Feminine)?
        };
        Some(match minutes {
            0 => hour,
            1..=9 => format!("{hour} zero {}", cardinal(minutes, false)),
            _ => format!("{hour} {}", cardinal(minutes, false)),
        })
    }

    /// The day and the year as ordinals in the genitive: trzeciego maja dwa tysiące dwudziestego
    /// czwartego.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!(
            "{} {month} {}",
            ordinal(day, Case::Genitive)?,
            ordinal(year, Case::Genitive)?
        ))
    }

    fn range(&self) -> &'static str {
        "do"
    }

    fn ratio(&self) -> &'static str {
        "przez"
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
        // A fraction takes the genitive singular, close enough to the plural's (1,5 złotego).
        Some(count.map_or(forms[2], |count| form(count, forms)))
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("procent"),
            "&" => Some("i"),
            "@" => Some("małpa"),
            "+" => Some("plus"),
            "=" => Some("równa się"),
            "°" => Some("stopni"),
            "#" => Some("krzyżyk"),
            "€" => Some("euro"),
            "$" => Some("dolar"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'ą' => "ą",
            'b' => "be",
            'c' => "ce",
            'ć' => "cie",
            'd' => "de",
            'e' => "e",
            'ę' => "ę",
            'f' => "ef",
            'g' => "gie",
            'h' => "ha",
            'i' => "i",
            'j' => "jot",
            'k' => "ka",
            'l' => "el",
            'ł' => "eł",
            'm' => "em",
            'n' => "en",
            'ń' => "eń",
            'o' => "o",
            'ó' => "o kreskowane",
            'p' => "pe",
            'q' => "ku",
            'r' => "er",
            's' => "es",
            'ś' => "eś",
            't' => "te",
            'u' => "u",
            'v' => "fau",
            'w' => "wu",
            'x' => "iks",
            'y' => "igrek",
            'z' => "zet",
            'ź' => "ziet",
            'ż' => "żet",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("małpa"),
            '.' => Some("kropka"),
            '/' => Some("ukośnik"),
            ':' => Some("dwukropek"),
            '-' => Some("myślnik"),
            '_' => Some("podkreślnik"),
            _ => None,
        }
    }

    /// Feminine nouns in the singular end in -a (jedna osoba); the plural and the other genders
    /// take jeden.
    fn gender_of(&self, word: &str) -> Gender {
        if word.ends_with('a') {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    /// A Roman numeral after a name (Jan Paweł II) or before wiek, a century (XX wiek, w XX
    /// wieku), as an ordinal.
    fn roman(&self, value: u64, before: Option<&str>, after: Option<&str>) -> Option<String> {
        let after = after.map(str::to_lowercase);
        let case = match after.as_deref() {
            Some("wiek") => Case::Nominative,
            Some("wieku") if before.is_some_and(|before| matches!(before.to_lowercase().as_str(), "w" | "we")) => {
                Case::Locative
            }
            Some("wieku") => Case::Genitive,
            _ if before.is_some_and(|before| before.chars().next().is_some_and(char::is_uppercase)) => Case::Nominative,
            _ => return None,
        };
        ordinal(value, case)
    }

    fn vowels(&self) -> &'static str {
        "aąeęéioóuy"
    }
}

use jespersen_spoken::{Gender, Locale};

use crate::numbers::{Agreement, Case, cardinal, form, ordinal};

/// The months in the genitive, as dates say them (tretieho mája).
const MONTHS: [&str; 12] = [
    "januára",
    "februára",
    "marca",
    "apríla",
    "mája",
    "júna",
    "júla",
    "augusta",
    "septembra",
    "októbra",
    "novembra",
    "decembra",
];

/// Slovak abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 15] = [
    ("atď", "a tak ďalej"),
    ("napr", "napríklad"),
    ("tj", "to jest"),
    ("tzv", "takzvaný"),
    ("resp", "respektíve"),
    ("dr", "doktor"),
    ("prof", "profesor"),
    ("ing", "inžinier"),
    ("mgr", "magister"),
    ("ul", "ulica"),
    ("č", "číslo"),
    ("str", "strana"),
    ("sv", "svätý"),
    ("tel", "telefón"),
    ("pod", "podobne"),
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
        "€" | "EUR" => Some(["euro", "eurá", "eur"]),
        "$" | "USD" => Some(["dolár", "doláre", "dolárov"]),
        "£" | "GBP" => Some(["libra", "libry", "libier"]),
        "Kč" | "CZK" => Some(["koruna", "koruny", "korún"]),
        _ => None,
    }
}

/// Slovak words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Slovak;

impl Locale for Slovak {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        let agreement = match gender {
            Gender::Alone => Agreement::Counting,
            Gender::Masculine => Agreement::Masculine,
            Gender::Feminine => Agreement::Feminine,
        };
        Some(cardinal(number, agreement))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "celá" } else { "bodka" }
    }

    /// The day as an ordinal in the genitive, the month too, the year as a number: tretieho mája
    /// dve tisíc dvadsať štyri.
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
        "percent"
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
            "%" => Some("percent"),
            "&" => Some("a"),
            "@" => Some("zavináč"),
            "+" => Some("plus"),
            "=" => Some("rovná sa"),
            "°" => Some("stupňov"),
            "#" => Some("mriežka"),
            "€" => Some("eur"),
            "$" => Some("dolárov"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' | 'á' => "á",
            'ä' => "široké e",
            'b' => "bé",
            'c' => "cé",
            'č' => "čé",
            'd' => "dé",
            'ď' => "ďé",
            'e' | 'é' => "é",
            'f' => "ef",
            'g' => "gé",
            'h' => "há",
            'i' | 'í' => "í",
            'j' => "jé",
            'k' => "ká",
            'l' => "el",
            'ĺ' => "dlhé el",
            'ľ' => "eľ",
            'm' => "em",
            'n' => "en",
            'ň' => "eň",
            'o' | 'ó' => "ó",
            'ô' => "uo",
            'p' => "pé",
            'q' => "kvé",
            'r' => "er",
            'ŕ' => "dlhé er",
            's' => "es",
            'š' => "eš",
            't' => "té",
            'ť' => "ťé",
            'u' | 'ú' => "ú",
            'v' => "vé",
            'w' => "dvojité vé",
            'x' => "iks",
            'y' | 'ý' => "ypsilon",
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
            '.' => Some("bodka"),
            '/' => Some("lomka"),
            ':' => Some("dvojbodka"),
            '-' => Some("pomlčka"),
            '_' => Some("podčiarkovník"),
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

    /// A Roman numeral before storočie, a century, is an ordinal in the neuter (XX. storočie
    /// dvadsiate), after a name in the masculine (Karol IV. štvrtý).
    fn roman(&self, value: u64, before: Option<&str>, after: Option<&str>) -> Option<String> {
        if after.is_some_and(|after| after.to_lowercase().starts_with("storoč")) {
            return ordinal(value, Case::Neuter);
        }
        if before.is_some_and(|before| before.chars().next().is_some_and(char::is_uppercase)) {
            return ordinal(value, Case::Masculine);
        }
        None
    }

    fn vowels(&self) -> &'static str {
        "aáäeéiíoóôuúyýĺŕ"
    }
}

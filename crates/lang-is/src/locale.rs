use jespersen_spoken::{Gender, Locale};

use crate::numbers::{Form, cardinal, day_ordinal};

const MONTHS: [&str; 12] = [
    "janúar",
    "febrúar",
    "mars",
    "apríl",
    "maí",
    "júní",
    "júlí",
    "ágúst",
    "september",
    "október",
    "nóvember",
    "desember",
];

/// Icelandic abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 9] = [
    ("dr", "doktor"),
    ("prof", "prófessor"),
    ("hr", "herra"),
    ("nr", "númer"),
    ("kl", "klukkan"),
    ("bls", "blaðsíða"),
    ("ca", "sirka"),
    ("sbr", "samanber"),
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

/// Icelandic words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Icelandic;

impl Locale for Icelandic {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        let form = match gender {
            Gender::Alone | Gender::Masculine => Form::Masculine,
            Gender::Feminine => Form::Feminine,
        };
        Some(cardinal(number, form))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "komma" } else { "punktur" }
    }

    /// tíu þrjátíu; the full hour is klukkan tíu.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours, Form::Neuter);
        Some(if minutes == 0 {
            format!("klukkan {hour}")
        } else {
            format!("{hour} {}", cardinal(minutes, Form::Neuter))
        })
    }

    /// þriðji maí tvö þúsund tuttugu-og-fjögur.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!(
            "{} {month} {}",
            day_ordinal(day)?,
            cardinal(year, Form::Neuter)
        ))
    }

    fn range(&self) -> &'static str {
        "til"
    }

    fn ratio(&self) -> &'static str {
        "á móti"
    }

    fn minus(&self) -> &'static str {
        "mínus"
    }

    fn plus(&self) -> &'static str {
        "plús"
    }

    fn percent(&self) -> &'static str {
        "prósent"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("evra", "evrur")),
            "$" | "USD" => Some(("dollari", "dollarar")),
            "£" | "GBP" => Some(("pund", "pund")),
            "kr" | "ISK" => Some(("króna", "krónur")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("prósent"),
            "&" => Some("og"),
            "@" => Some("hjá"),
            "+" => Some("plús"),
            "=" => Some("jafnt og"),
            "°" => Some("gráður"),
            "#" => Some("myllumerki"),
            "€" => Some("evrur"),
            "$" => Some("dollarar"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'á' => "á",
            'b' => "bé",
            'c' => "sé",
            'd' => "dé",
            'ð' => "eð",
            'e' => "e",
            'é' => "é",
            'f' => "eff",
            'g' => "gé",
            'h' => "há",
            'i' => "i",
            'í' => "í",
            'j' => "joð",
            'k' => "ká",
            'l' => "ell",
            'm' => "emm",
            'n' => "enn",
            'o' => "o",
            'ó' => "ó",
            'p' => "pé",
            'q' => "kú",
            'r' => "err",
            's' => "ess",
            't' => "té",
            'u' => "u",
            'ú' => "ú",
            'v' => "vaff",
            'w' => "tvöfalt vaff",
            'x' => "ex",
            'y' => "ufsilon",
            'ý' => "ufsilon í",
            'z' => "seta",
            'þ' => "þorn",
            'æ' => "æ",
            'ö' => "ö",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("hjá"),
            '.' => Some("punktur"),
            '/' => Some("skástrik"),
            ':' => Some("tvípunktur"),
            '-' => Some("bandstrik"),
            '_' => Some("undirstrik"),
            _ => None,
        }
    }

    /// Nouns in -a and their plurals in -rur and -nur are feminine (ein króna, tvær evrur, krónur);
    /// the rest are counted as masculine.
    fn gender_of(&self, word: &str) -> Gender {
        if word.ends_with('a') || word.ends_with("rur") || word.ends_with("nur") {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    fn vowels(&self) -> &'static str {
        "aáeéiíoóuúyýæö"
    }
}

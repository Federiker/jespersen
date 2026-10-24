use jespersen_spoken::{Gender, Locale};

use crate::numbers::cardinal;

const MONTHS: [&str; 12] = [
    "janvāris",
    "februāris",
    "marts",
    "aprīlis",
    "maijs",
    "jūnijs",
    "jūlijs",
    "augusts",
    "septembris",
    "oktobris",
    "novembris",
    "decembris",
];

/// Latvian abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 10] = [
    ("dr", "doktors"),
    ("prof", "profesors"),
    ("nr", "numurs"),
    ("tel", "telefons"),
    ("piem", "piemēram"),
    ("utt", "un tā tālāk"),
    ("u", "un"),
    ("g", "gads"),
    ("iel", "iela"),
    ("plkst", "pulksten"),
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

/// Latvian words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Latvian;

impl Locale for Latvian {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        Some(cardinal(number, gender == Gender::Feminine))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "komats" } else { "punkts" }
    }

    /// desmit trīsdesmit; the full hour is pulksten desmit.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours, false);
        Some(if minutes == 0 {
            format!("pulksten {hour}")
        } else {
            format!("{hour} {}", cardinal(minutes, false))
        })
    }

    /// trīs maijs divi tūkstoši divdesmit četri.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day, false), cardinal(year, false)))
    }

    fn range(&self) -> &'static str {
        "līdz"
    }

    fn ratio(&self) -> &'static str {
        "pret"
    }

    fn minus(&self) -> &'static str {
        "mīnus"
    }

    fn plus(&self) -> &'static str {
        "plus"
    }

    fn percent(&self) -> &'static str {
        "procenti"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("eiro", "eiro")),
            "$" | "USD" => Some(("dolārs", "dolāri")),
            "£" | "GBP" => Some(("mārciņa", "mārciņas")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("procenti"),
            "&" => Some("un"),
            "@" => Some("et"),
            "+" => Some("plus"),
            "=" => Some("ir vienāds ar"),
            "°" => Some("grādi"),
            "#" => Some("restīte"),
            "€" => Some("eiro"),
            "$" => Some("dolāri"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'ā' => "garais ā",
            'b' => "bē",
            'c' => "cē",
            'č' => "čē",
            'd' => "dē",
            'e' => "e",
            'ē' => "garais ē",
            'f' => "ef",
            'g' => "gā",
            'ģ' => "ģē",
            'h' => "hā",
            'i' => "i",
            'ī' => "garais ī",
            'j' => "jē",
            'k' => "kā",
            'ķ' => "ķē",
            'l' => "el",
            'ļ' => "eļ",
            'm' => "em",
            'n' => "en",
            'ņ' => "eņ",
            'o' => "o",
            'p' => "pē",
            'q' => "kū",
            'r' => "er",
            's' => "es",
            'š' => "eš",
            't' => "tē",
            'u' => "u",
            'ū' => "garais ū",
            'v' => "vē",
            'w' => "dubultvē",
            'x' => "iks",
            'y' => "igrek",
            'z' => "zē",
            'ž' => "žē",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("et"),
            '.' => Some("punkts"),
            '/' => Some("slīpsvītra"),
            ':' => Some("kols"),
            '-' => Some("defise"),
            '_' => Some("apakšsvītra"),
            _ => None,
        }
    }

    /// The nouns in -a and -e are feminine (divas mārciņas); the rest are masculine.
    fn gender_of(&self, word: &str) -> Gender {
        if ["a", "e", "as", "es"].iter().any(|ending| word.ends_with(ending)) {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    fn vowels(&self) -> &'static str {
        "aāeēiīouū"
    }
}

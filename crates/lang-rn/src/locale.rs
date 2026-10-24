use jespersen_spoken::{Gender, Locale};

/// The numbers said when counting (rimwe, kabiri…); 7, 8 and 9 never agree.
const UNITS: [&str; 10] = [
    "zeru",
    "rimwe",
    "kabiri",
    "gatatu",
    "kane",
    "gatanu",
    "gatandatu",
    "indwi",
    "umunani",
    "icenda",
];

/// The units agreeing with mirongo (tens), class 4: mirongo ibiri.
const WITH_TENS: [&str; 10] = [
    "", "", "ibiri", "itatu", "ine", "itanu", "itandatu", "indwi", "umunani", "icenda",
];

/// The units agreeing with amajana (hundreds), class 6: amajana abiri.
const WITH_HUNDREDS: [&str; 10] = [
    "", "", "abiri", "atatu", "ane", "atanu", "atandatu", "indwi", "umunani", "icenda",
];

/// The units agreeing with ibihumbi (thousands), class 8: ibihumbi bibiri.
const WITH_THOUSANDS: [&str; 10] = [
    "",
    "",
    "bibiri",
    "bitatu",
    "bine",
    "bitanu",
    "bitandatu",
    "indwi",
    "umunani",
    "icenda",
];

const MONTHS: [&str; 12] = [
    "Nzero",
    "Ruhuhuma",
    "Ntwarante",
    "Ndamukiza",
    "Rusama",
    "Ruheshi",
    "Mukakaro",
    "Myandagaro",
    "Nyakanga",
    "Gitugutu",
    "Munyonyo",
    "Kigarama",
];

/// Kirundi abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 5] = [
    ("dr", "dogiteri"),
    ("prof", "purofeseri"),
    ("nyak", "nyakubahwa"),
    ("mme", "madamu"),
    ("p", "urupapuro"),
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

fn at(table: &'static [&'static str; 10], digit: u64) -> &'static str {
    table[usize::try_from(digit % 10).unwrap_or(0)]
}

/// `number` in Kirundi words, the largest part first and na between the parts (125 ijana na
/// mirongo ibiri na gatanu, 2024 ibihumbi bibiri na mirongo ibiri na kane).
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, name) in [(1_000_000_000, "miliyari"), (1_000_000, "miliyoni")] {
        let count = number / scale % 1000;
        if count > 0 {
            parts.push(format!("{name} {}", cardinal(count)));
        }
    }
    let thousands = number / 1000 % 1000;
    match thousands {
        0 => {}
        1 => parts.push("igihumbi".to_owned()),
        2..=9 => parts.push(format!("ibihumbi {}", at(&WITH_THOUSANDS, thousands))),
        _ => parts.push(format!("ibihumbi {}", cardinal(thousands))),
    }
    match number / 100 % 10 {
        0 => {}
        1 => parts.push("ijana".to_owned()),
        hundreds => parts.push(format!("amajana {}", at(&WITH_HUNDREDS, hundreds))),
    }
    let (tens, units) = (number / 10 % 10, number % 10);
    match tens {
        0 => {}
        1 if units > 0 => parts.push("cumi".to_owned()),
        1 => parts.push("icumi".to_owned()),
        tens => parts.push(format!("mirongo {}", at(&WITH_TENS, tens))),
    }
    if units > 0 {
        parts.push(at(&UNITS, units).to_owned());
    }
    parts.join(" na ")
}

/// Kirundi words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Kirundi;

impl Locale for Kirundi {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    /// The first is mbere (wa mbere); the others are said as their cardinal (wa kabiri).
    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(if number == 1 {
            "mbere".to_owned()
        } else {
            cardinal(number)
        })
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "akitso" } else { "akadomo" }
    }

    /// gatatu Rusama ibihumbi bibiri na mirongo ibiri na kane (after itariki, which the text
    /// writes).
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "gushika"
    }

    fn ratio(&self) -> &'static str {
        "kuri"
    }

    fn minus(&self) -> &'static str {
        "ukuyemwo"
    }

    fn plus(&self) -> &'static str {
        "wongeyeko"
    }

    /// mirongo itanu ku ijana: "out of a hundred", after the number.
    fn percent(&self) -> &'static str {
        "ku ijana"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "Fbu" | "FBu" | "FBU" | "BIF" => Some(("ifaranga", "amafaranga")),
            "€" => Some(("ewuro", "ewuro")),
            "$" => Some(("idolari", "amadolari")),
            "£" => Some(("ipawundi", "amapawundi")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("ku ijana"),
            "&" => Some("na"),
            "@" => Some("aroba"),
            "+" => Some("wongeyeko"),
            "=" => Some("bingana na"),
            "°" => Some("dogere"),
            "#" => Some("inomero"),
            "€" => Some("ewuro"),
            "$" => Some("idolari"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "be",
            'c' => "ce",
            'd' => "de",
            'e' => "e",
            'f' => "fe",
            'g' => "ge",
            'h' => "he",
            'i' => "i",
            'j' => "je",
            'k' => "ke",
            'l' => "le",
            'm' => "me",
            'n' => "ne",
            'o' => "o",
            'p' => "pe",
            'q' => "kiyu",
            'r' => "re",
            's' => "se",
            't' => "te",
            'u' => "u",
            'v' => "ve",
            'w' => "we",
            'x' => "igisi",
            'y' => "ye",
            'z' => "ze",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("aroba"),
            '.' => Some("akadomo"),
            '/' => Some("umurongo uhengamye"),
            ':' => Some("utudomo tubiri"),
            '-' => Some("akanyerezo"),
            '_' => Some("umurongo wo hasi"),
            _ => None,
        }
    }

    /// n'umwana, y'u Burundi: a word elided before a vowel is said with the next one.
    fn elides(&self) -> bool {
        true
    }

    fn vowels(&self) -> &'static str {
        "aeiou"
    }
}

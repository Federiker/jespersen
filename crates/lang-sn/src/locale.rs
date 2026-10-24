use jespersen_spoken::{Gender, Locale};

/// The numbers said when counting (poshi, piri…).
const UNITS: [&str; 10] = [
    "ziro",
    "poshi",
    "piri",
    "tatu",
    "ina",
    "shanu",
    "tanhatu",
    "nomwe",
    "sere",
    "pfumbamwe",
];

/// The units after ne, joined to it (gumi nerimwe, gumi nembiri).
const ADDED: [&str; 10] = [
    "",
    "rimwe",
    "mbiri",
    "nhatu",
    "na",
    "shanu",
    "nhanhatu",
    "nomwe",
    "sere",
    "pfumbamwe",
];

/// The units agreeing with makumi and mazana, class 6: makumi maviri.
const CLASS_SIX: [&str; 10] = [
    "",
    "",
    "maviri",
    "matatu",
    "mana",
    "mashanu",
    "matanhatu",
    "manomwe",
    "masere",
    "mapfumbamwe",
];

/// The units agreeing with zviuru, class 8: zviuru zviviri.
const CLASS_EIGHT: [&str; 10] = [
    "",
    "",
    "zviviri",
    "zvitatu",
    "zvina",
    "zvishanu",
    "zvitanhatu",
    "zvinomwe",
    "zvisere",
    "zvipfumbamwe",
];

const MONTHS: [&str; 12] = [
    "Ndira",
    "Kukadzi",
    "Kurume",
    "Kubvumbi",
    "Chivabvu",
    "Chikumi",
    "Chikunguru",
    "Nyamavhuvhu",
    "Gunyana",
    "Gumiguru",
    "Mbudzi",
    "Zvita",
];

/// Shona abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 3] = [("dr", "chiremba"), ("prof", "purofesa"), ("p", "peji")];

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

/// `number` in Shona words, the largest part first, each later part joined to ne (11 gumi
/// nerimwe, 125 zana nemakumi maviri neshanu, 2024 zviuru zviviri nemakumi maviri nena).
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    match number / 1_000_000 {
        0 => {}
        1 => parts.push("miriyoni".to_owned()),
        millions => parts.push(format!("mamiriyoni {}", cardinal(millions))),
    }
    match number / 1000 % 1000 {
        0 => {}
        1 => parts.push("churu".to_owned()),
        thousands @ 2..=9 => parts.push(format!("zviuru {}", at(&CLASS_EIGHT, thousands))),
        thousands => parts.push(format!("zviuru {}", cardinal(thousands))),
    }
    match number / 100 % 10 {
        0 => {}
        1 => parts.push("zana".to_owned()),
        hundreds => parts.push(format!("mazana {}", at(&CLASS_SIX, hundreds))),
    }
    match number / 10 % 10 {
        0 => {}
        1 => parts.push("gumi".to_owned()),
        tens => parts.push(format!("makumi {}", at(&CLASS_SIX, tens))),
    }
    let units = number % 10;
    if units > 0 {
        parts.push(at(if parts.is_empty() { &UNITS } else { &ADDED }, units).to_owned());
    }
    let mut said = parts.remove(0);
    for part in parts {
        said.push_str(" ne");
        said.push_str(&part);
    }
    said
}

/// Shona words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Shona;

impl Locale for Shona {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    /// The first is kutanga; the others are said as their cardinal.
    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(if number == 1 {
            "kutanga".to_owned()
        } else {
            cardinal(number)
        })
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "koma" } else { "poindi" }
    }

    /// tatu Chivabvu zviuru zviviri nemakumi maviri nena.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "kusvika"
    }

    fn ratio(&self) -> &'static str {
        "ku"
    }

    fn minus(&self) -> &'static str {
        "bvisa"
    }

    fn plus(&self) -> &'static str {
        "wedzera"
    }

    /// makumi mashanu muzana: "in a hundred", after the number.
    fn percent(&self) -> &'static str {
        "muzana"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "$" | "US$" | "ZWL" | "ZiG" => Some(("dhora", "madhora")),
            "€" => Some(("yuro", "mayuro")),
            "£" => Some(("paundi", "mapaundi")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("muzana"),
            "&" => Some("ne"),
            "@" => Some("pa"),
            "+" => Some("wedzera"),
            "=" => Some("zvakaenzana ne"),
            "°" => Some("dhigiri"),
            "#" => Some("nhamba"),
            "€" => Some("yuro"),
            "$" => Some("dhora"),
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
            'q' => "kyu",
            'r' => "re",
            's' => "se",
            't' => "te",
            'u' => "u",
            'v' => "ve",
            'w' => "we",
            'x' => "ekisi",
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
            '@' => Some("pa"),
            '.' => Some("dhoti"),
            '/' => Some("sirashi"),
            ':' => Some("koroni"),
            '-' => Some("dhashi"),
            '_' => Some("mutsetse wepasi"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiou"
    }
}

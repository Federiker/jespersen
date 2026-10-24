use jespersen_spoken::{Gender, Locale};

/// The numbers said when counting (emu, bbiri…).
const UNITS: [&str; 10] = [
    "zeero", "emu", "bbiri", "ssatu", "nnya", "ttaano", "mukaaga", "musanvu", "munaana", "mwenda",
];

/// The tens: amakumi with its number to fifty, then words of their own (nkaaga, kyenda).
const TENS: [&str; 10] = [
    "",
    "kkumi",
    "amakumi abiri",
    "amakumi asatu",
    "amakumi ana",
    "amakumi ataano",
    "nkaaga",
    "nsanvu",
    "kinaana",
    "kyenda",
];

/// The hundreds, each a word of its own (kikumi, bibiri, lukaaga).
const HUNDREDS: [&str; 10] = [
    "", "kikumi", "bibiri", "bisatu", "bina", "bitaano", "lukaaga", "lusanvu", "lunaana", "lwenda",
];

const MONTHS: [&str; 12] = [
    "Janwaliyo",
    "Febwaliyo",
    "Marisi",
    "Apuli",
    "Maayi",
    "Juuni",
    "Julaayi",
    "Agusito",
    "Sebuttemba",
    "Okitobba",
    "Novemba",
    "Desemba",
];

/// Luganda abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 4] = [
    ("dr", "dokita"),
    ("prof", "pulofeesa"),
    ("omw", "omwami"),
    ("p", "olupapula"),
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

/// `number` in Luganda words, the largest part first and mu between the parts (21 amakumi abiri
/// mu emu, 125 kikumi mu amakumi abiri mu ttaano); the teens take na (kkumi na emu).
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, one, many) in [
        (1_000_000_000, "akawumbi kamu", "obuwumbi"),
        (1_000_000, "akakadde kamu", "obukadde"),
    ] {
        match number / scale % 1000 {
            0 => {}
            1 => parts.push(one.to_owned()),
            count => parts.push(format!("{many} {}", cardinal(count))),
        }
    }
    match number / 1000 % 1000 {
        0 => {}
        1 => parts.push("lukumi".to_owned()),
        thousands => parts.push(format!("enkumi {}", cardinal(thousands))),
    }
    let hundreds = number / 100 % 10;
    if hundreds > 0 {
        parts.push(at(&HUNDREDS, hundreds).to_owned());
    }
    let (tens, units) = (number / 10 % 10, number % 10);
    if tens == 1 && units > 0 {
        parts.push(format!("kkumi na {}", at(&UNITS, units)));
    } else {
        if tens > 0 {
            parts.push(at(&TENS, tens).to_owned());
        }
        if units > 0 {
            parts.push(at(&UNITS, units).to_owned());
        }
    }
    parts.join(" mu ")
}

/// Luganda words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Luganda;

impl Locale for Luganda {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    /// The first is okusooka; the others are said as their cardinal (ekyokubiri is left to the
    /// text).
    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(if number == 1 {
            "okusooka".to_owned()
        } else {
            cardinal(number)
        })
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "kkoma" } else { "akatonnyeze" }
    }

    /// ssatu Maayi enkumi bbiri mu amakumi abiri mu nnya.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "okutuuka"
    }

    fn ratio(&self) -> &'static str {
        "ku"
    }

    fn minus(&self) -> &'static str {
        "kuggyako"
    }

    fn plus(&self) -> &'static str {
        "gattako"
    }

    /// amakumi ataano ku kikumi: "out of a hundred", after the number.
    fn percent(&self) -> &'static str {
        "ku kikumi"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "UGX" | "USh" | "Shs" => Some(("ssiringi", "ssiringi")),
            "€" => Some(("yuro", "yuro")),
            "$" => Some(("doola", "doola")),
            "£" => Some(("pawundi", "pawundi")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("ku kikumi"),
            "&" => Some("ne"),
            "@" => Some("aroba"),
            "+" => Some("gattako"),
            "=" => Some("kyenkana"),
            "°" => Some("diguli"),
            "#" => Some("namba"),
            "€" => Some("yuro"),
            "$" => Some("doola"),
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
            'ŋ' => "ŋe",
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
            '@' => Some("aroba"),
            '.' => Some("akatonnyeze"),
            '/' => Some("akasaze"),
            ':' => Some("obutonnyeze bubiri"),
            '-' => Some("akasittale"),
            '_' => Some("olukoloboze wansi"),
            _ => None,
        }
    }

    /// n'abantu, y'omusajja: a word elided before a vowel is said with the next one.
    fn elides(&self) -> bool {
        true
    }

    fn vowels(&self) -> &'static str {
        "aeiou"
    }
}

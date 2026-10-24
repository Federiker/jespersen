use jespersen_spoken::{Gender, Locale, marks::plain};

const UNITS: [&str; 10] = [
    "sifiri", "ɗaya", "biyu", "uku", "huɗu", "biyar", "shida", "bakwai", "takwas", "tara",
];

const TENS: [&str; 10] = [
    "", "goma", "ashirin", "talatin", "arbaʼin", "hamsin", "sittin", "sabaʼin", "tamanin", "casaʼin",
];

const MONTHS: [&str; 12] = [
    "Janairu", "Fabrairu", "Maris", "Afrilu", "Mayu", "Yuni", "Yuli", "Agusta", "Satumba", "Oktoba", "Nuwamba",
    "Disamba",
];

/// Hausa abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 5] = [
    ("mal", "malam"),
    ("alh", "alhaji"),
    ("haj", "hajiya"),
    ("dr", "dakta"),
    ("farf", "farfesa"),
];

fn expansion(word: &str) -> Option<&'static str> {
    let plain = plain(word);
    ABBREVIATIONS
        .iter()
        .find(|(abbreviation, _)| *abbreviation == plain)
        .map(|(_, said)| *said)
}

/// Whether `word` (without its full stop) is an abbreviation that does not end a sentence.
#[must_use]
pub fn is_abbreviation(word: &str) -> bool {
    expansion(word).is_some()
}

fn digit(value: u64) -> &'static str {
    UNITS[usize::try_from(value % 10).unwrap_or(0)]
}

/// One to 99: goma sha and the units for eleven to nineteen, the tens and da before the units
/// (ashirin da biyar).
fn below_hundred(number: u64) -> String {
    let tens = TENS[usize::try_from(number / 10).unwrap_or(0)];
    match (number / 10, number % 10) {
        (0, units) => digit(units).to_owned(),
        (_, 0) => tens.to_owned(),
        (1, units) => format!("goma sha {}", digit(units)),
        (_, units) => format!("{tens} da {}", digit(units)),
    }
}

/// `number` in Hausa words: ɗari (hundred), dubu (thousand), miliyan and biliyan with their count
/// after them, said alone for one (dubu, not dubu ɗaya), and the parts joined with da (dubu biyu
/// da ashirin da huɗu, 2024).
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, name) in [
        (1_000_000_000, "biliyan"),
        (1_000_000, "miliyan"),
        (1000, "dubu"),
        (100, "ɗari"),
    ] {
        let count = number / scale % if scale == 100 { 10 } else { 1000 };
        match count {
            0 => {}
            1 => parts.push(name.to_owned()),
            count => parts.push(format!("{name} {}", cardinal(count))),
        }
    }
    let rest = number % 100;
    if rest > 0 {
        parts.push(below_hundred(rest));
    }
    parts.join(" da ")
}

/// Hausa words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Hausa;

impl Locale for Hausa {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    /// na farko (the first), then na and the cardinal (na biyu).
    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(if number == 1 {
            "na farko".to_owned()
        } else {
            format!("na {}", cardinal(number))
        })
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "waƙafi" } else { "digo" }
    }

    /// uku ga watan Mayu dubu biyu da ashirin da huɗu (the third of May 2024).
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} ga watan {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "zuwa"
    }

    fn minus(&self) -> &'static str {
        "ragi"
    }

    fn plus(&self) -> &'static str {
        "ƙari"
    }

    /// ashirin da biyar cikin ɗari: so many in a hundred.
    fn percent(&self) -> &'static str {
        "cikin ɗari"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "₦" | "NGN" => Some(("naira", "naira")),
            "CFA" | "FCFA" => Some(("sefa", "sefa")),
            "$" => Some(("dala", "dala")),
            "€" => Some(("yuro", "yuro")),
            "£" => Some(("fam", "fam")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("cikin ɗari"),
            "&" => Some("da"),
            "@" => Some("a"),
            "+" => Some("ƙari"),
            "=" => Some("daidai da"),
            "#" => Some("lamba"),
            "₦" => Some("naira"),
            "$" => Some("dala"),
            _ => None,
        }
    }

    /// The letters' Hausa names (ba, ɓa, ƙa), the ones outside the alphabet as English calls them.
    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "ba",
            'ɓ' => "ɓa",
            'c' => "ca",
            'd' => "da",
            'ɗ' => "ɗa",
            'e' => "e",
            'f' => "fa",
            'g' => "ga",
            'h' => "ha",
            'i' => "i",
            'j' => "ja",
            'k' => "ka",
            'ƙ' => "ƙa",
            'l' => "la",
            'm' => "ma",
            'n' => "na",
            'o' => "o",
            'p' => "fi",
            'q' => "kiyu",
            'r' => "ra",
            's' => "sa",
            't' => "ta",
            'u' => "u",
            'v' => "bi",
            'w' => "wa",
            'x' => "eks",
            'y' => "ya",
            'ƴ' => "ƴa",
            'z' => "za",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("a"),
            '.' => Some("digo"),
            '/' => Some("raba"),
            ':' => Some("ruwa biyu"),
            '-' => Some("karan tsaye"),
            '_' => Some("karan ƙasa"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiou"
    }
}

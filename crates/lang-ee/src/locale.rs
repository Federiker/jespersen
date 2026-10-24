use jespersen_spoken::{Gender, Locale, marks::plain};

/// One to nine, and zero as Ghana's Ewe says it.
const UNITS: [&str; 10] = [
    "zero", "ɖeka", "eve", "etɔ̃", "ene", "atɔ̃", "ade", "adre", "enyi", "asieke",
];

/// The Ewe months.
const MONTHS: [&str; 12] = [
    "Dzove",
    "Dzodze",
    "Tedoxe",
    "Afɔfiɛ",
    "Damɛ",
    "Masa",
    "Siamlɔm",
    "Deasiamime",
    "Anyɔnyɔ",
    "Kele",
    "Adeɛmekpɔxe",
    "Dzome",
];

/// Ewe abbreviations with a full stop, without their tone marks, and how they are said.
const ABBREVIATIONS: [(&str, &str); 2] = [("dr", "dɔkta"), ("prof", "prɔfesa")];

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

/// One to 99: ewo (ten), wui- and the units for eleven to nineteen (wuieve), bla- and the count
/// for the tens (blaeve, 20) and vɔ before their units (blaeve vɔ ɖeka).
fn below_hundred(number: u64) -> String {
    match (number / 10, number % 10) {
        (0, units) => digit(units).to_owned(),
        (1, 0) => "ewo".to_owned(),
        (1, 1) => "wuiɖekɛ".to_owned(),
        (1, units) => format!("wui{}", digit(units)),
        (tens, 0) => format!("bla{}", digit(tens)),
        (tens, units) => format!("bla{} vɔ {}", digit(tens), digit(units)),
    }
}

/// `number` in Ewe words: alafa (hundred), akpe (thousand) and miliɔn with their count after them
/// (alafa ɖeka, akpe eve), and kple between the parts (akpe eve kple blaeve vɔ ene, 2024).
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, name) in [
        (1_000_000_000, "biliɔn"),
        (1_000_000, "miliɔn"),
        (1000, "akpe"),
        (100, "alafa"),
    ] {
        let count = number / scale % if scale == 100 { 10 } else { 1000 };
        if count > 0 {
            parts.push(format!("{name} {}", cardinal(count)));
        }
    }
    let rest = number % 100;
    if rest > 0 {
        parts.push(below_hundred(rest));
    }
    parts.join(" kple ")
}

/// Ewe words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Ewe;

impl Locale for Ewe {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    /// gbãtɔ (the first), then the cardinal with -lia (evelia, the second).
    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(if number == 1 {
            "gbãtɔ".to_owned()
        } else {
            format!("{}lia", cardinal(number))
        })
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "kɔma" } else { "pɔint" }
    }

    /// ŋkeke etɔ̃ Damɛ ƒe akpe eve kple blaeve vɔ ene (the third of May 2024).
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("ŋkeke {} {month} ƒe {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "va ɖo"
    }

    fn minus(&self) -> &'static str {
        "ɖe eme"
    }

    fn plus(&self) -> &'static str {
        "kpe ɖe"
    }

    /// blaeve vɔ atɔ̃ le alafa me: so many in a hundred.
    fn percent(&self) -> &'static str {
        "le alafa me"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "₵" | "GH₵" | "GHS" => Some(("sidi", "sidi")),
            "CFA" | "FCFA" => Some(("sefa", "sefa")),
            "$" => Some(("dɔla", "dɔla")),
            "€" => Some(("euro", "euro")),
            "£" => Some(("pɔn", "pɔn")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("le alafa me"),
            "&" => Some("kple"),
            "@" => Some("le"),
            "+" => Some("kpe ɖe"),
            "=" => Some("sɔ kple"),
            "#" => Some("nɔmba"),
            "₵" => Some("sidi"),
            "$" => Some("dɔla"),
            _ => None,
        }
    }

    /// The letters' Ewe names (ba, ɖa, ƒa), the ones outside the alphabet as English calls them.
    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "ba",
            'c' => "si",
            'd' => "da",
            'ɖ' => "ɖa",
            'e' => "e",
            'ɛ' => "ɛ",
            'f' => "fa",
            'ƒ' => "ƒa",
            'g' => "ga",
            'ɣ' => "ɣa",
            'h' => "ha",
            'i' => "i",
            'j' => "dze",
            'k' => "ka",
            'l' => "la",
            'm' => "ma",
            'n' => "na",
            'ŋ' => "ŋa",
            'o' => "o",
            'ɔ' => "ɔ",
            'p' => "pa",
            'q' => "kiu",
            'r' => "ra",
            's' => "sa",
            't' => "ta",
            'u' => "u",
            'v' => "va",
            'ʋ' => "ʋa",
            'w' => "wa",
            'x' => "xa",
            'y' => "ya",
            'z' => "za",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("le"),
            '.' => Some("ɖoɖo"),
            '/' => Some("slash"),
            ':' => Some("kolon"),
            '-' => Some("fli"),
            '_' => Some("fli le te"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeɛioɔu"
    }
}

use jespersen_spoken::{Gender, Locale, marks::plain};

/// Zero to nine as counted (baako, abien).
const UNITS: [&str; 10] = [
    "hwee", "baako", "abien", "abiɛsa", "anan", "anum", "asia", "ason", "awɔtwe", "akron",
];

/// One to nine as they follow du in eleven to nineteen (dubaako, dumienu).
const TEENS: [&str; 10] = [
    "", "baako", "mienu", "miɛnsa", "nan", "num", "nsia", "nson", "nwɔtwe", "nkron",
];

const TENS: [&str; 10] = [
    "",
    "du",
    "aduonu",
    "aduasa",
    "aduanan",
    "aduonum",
    "aduosia",
    "aduoson",
    "aduowɔtwe",
    "aduokron",
];

const HUNDREDS: [&str; 10] = [
    "",
    "ɔha",
    "ahaanu",
    "ahaasa",
    "ahanan",
    "ahanum",
    "ahansia",
    "ahanson",
    "ahanwɔtwe",
    "ahankron",
];

/// The Akan months.
const MONTHS: [&str; 12] = [
    "Ɔpɛpɔn",
    "Ɔgyefuo",
    "Ɔbɛnem",
    "Oforisuo",
    "Kɔtɔnimma",
    "Ayɛwohomumu",
    "Kutawonsa",
    "Ɔsanaa",
    "Ɛbɔ",
    "Ahinime",
    "Obubuo",
    "Ɔpɛnimma",
];

/// Akan abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 4] = [
    ("dr", "dɔkta"),
    ("prof", "prɔfesa"),
    ("owu", "owura"),
    ("awu", "awuraa"),
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

fn index(number: u64) -> usize {
    usize::try_from(number % 10).unwrap_or(0)
}

/// One to 999: the hundreds by name (ɔha, ahaanu), du and the units joined for eleven to nineteen
/// (dubaako), the tens followed by their units (aduonu baako), and ne between the hundreds and the
/// rest.
fn below_thousand(number: u64) -> String {
    let (hundreds, tens, units) = (number / 100, number / 10 % 10, number % 10);
    let rest = match (tens, units) {
        (0, 0) => String::new(),
        (0, units) => UNITS[index(units)].to_owned(),
        (tens, 0) => TENS[index(tens)].to_owned(),
        (1, units) => format!("du{}", TEENS[index(units)]),
        (tens, units) => format!("{} {}", TENS[index(tens)], UNITS[index(units)]),
    };
    match (hundreds, rest.is_empty()) {
        (0, _) => rest,
        (hundreds, true) => HUNDREDS[index(hundreds)].to_owned(),
        (hundreds, false) => format!("{} ne {rest}", HUNDREDS[index(hundreds)]),
    }
}

/// `number` in Akan words: apem (a thousand), mpem with the count of thousands after it (mpem
/// abien), ɔpepem for a million, and ne between the parts (mpem abien ne aduonu anan, 2024).
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, one, many) in [
        (1_000_000_000, "ɔpepepem", "ɔpepepem"),
        (1_000_000, "ɔpepem", "ɔpepem"),
        (1000, "apem", "mpem"),
    ] {
        match number / scale % 1000 {
            0 => {}
            1 => parts.push(one.to_owned()),
            count => parts.push(format!("{many} {}", below_thousand(count))),
        }
    }
    let rest = number % 1000;
    if rest > 0 {
        parts.push(below_thousand(rest));
    }
    parts.join(" ne ")
}

/// Akan words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Akan;

impl Locale for Akan {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    /// kan (the first), then a ɛtɔ so and the cardinal (a ɛtɔ so abien, the second).
    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(if number == 1 {
            "kan".to_owned()
        } else {
            format!("a ɛtɔ so {}", cardinal(number))
        })
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "kɔma" } else { "pɔint" }
    }

    /// Kɔtɔnimma da abiɛsa afe mpem abien ne aduonu anan (the third of May 2024).
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{month} da {} afe {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "kosi"
    }

    fn minus(&self) -> &'static str {
        "yi fi"
    }

    fn plus(&self) -> &'static str {
        "ka ho"
    }

    /// aduonu anum ɔha mu: so many in a hundred.
    fn percent(&self) -> &'static str {
        "ɔha mu"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "₵" | "GH₵" | "GHS" => Some(("sidi", "sidi")),
            "$" => Some(("dɔla", "dɔla")),
            "€" => Some(("yuro", "yuro")),
            "£" => Some(("pɔn", "pɔn")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("ɔha mu"),
            "&" => Some("ne"),
            "@" => Some("wɔ"),
            "+" => Some("ka ho"),
            "=" => Some("yɛ pɛ"),
            "#" => Some("nɔma"),
            "₵" => Some("sidi"),
            "$" => Some("dɔla"),
            _ => None,
        }
    }

    /// The letters' Akan names (ba, da, ɛ), the ones outside the alphabet as English calls them.
    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "ba",
            'c' => "si",
            'd' => "da",
            'e' => "e",
            'ɛ' => "ɛ",
            'f' => "fa",
            'g' => "ga",
            'h' => "ha",
            'i' => "i",
            'j' => "gye",
            'k' => "ka",
            'l' => "la",
            'm' => "ma",
            'n' => "na",
            'o' => "o",
            'ɔ' => "ɔ",
            'p' => "pa",
            'q' => "kiu",
            'r' => "ra",
            's' => "sa",
            't' => "ta",
            'u' => "u",
            'v' => "vi",
            'w' => "wa",
            'x' => "ɛks",
            'y' => "ya",
            'z' => "zɛd",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("wɔ"),
            '.' => Some("tɔnk"),
            '/' => Some("slaas"),
            ':' => Some("kolon"),
            '-' => Some("hyɛn"),
            '_' => Some("hyɛn fam"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeɛioɔu"
    }
}

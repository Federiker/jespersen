use jespersen_spoken::{Gender, Locale, marks::plain};

const UNITS: [&str; 10] = [
    "efu", "otu", "abụọ", "atọ", "anọ", "ise", "isii", "asaa", "asatọ", "itoolu",
];

/// The months as Igbo numbers them: ọnwa mbụ (the first month) to ọnwa iri na abụọ.
const MONTHS: [&str; 12] = [
    "mbụ",
    "abụọ",
    "atọ",
    "anọ",
    "ise",
    "isii",
    "asaa",
    "asatọ",
    "itoolu",
    "iri",
    "iri na otu",
    "iri na abụọ",
];

/// Igbo abbreviations with a full stop, without their tone marks, and how they are said.
const ABBREVIATIONS: [(&str, &str); 4] = [
    ("dr", "dọkịta"),
    ("prof", "prọfesọ"),
    ("maz", "mazị"),
    ("oriak", "oriakụ"),
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

/// One to 99: iri (ten) with its count after it (iri abụọ, 20), and na before the units (iri na
/// otu, 11).
fn below_hundred(number: u64) -> String {
    match (number / 10, number % 10) {
        (0, units) => digit(units).to_owned(),
        (1, 0) => "iri".to_owned(),
        (1, units) => format!("iri na {}", digit(units)),
        (tens, 0) => format!("iri {}", digit(tens)),
        (tens, units) => format!("iri {} na {}", digit(tens), digit(units)),
    }
}

/// `number` in Igbo words as the standard language counts in tens: narị (hundred), puku
/// (thousand), nde (million) and ijeri (billion) with their count after them, one of them said
/// otu narị, otu puku; the parts join with na (puku abụọ na iri abụọ na anọ, 2024). The older
/// count in twenties (ọgụ, nnụ) is not used in writing today.
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, name) in [
        (1_000_000_000, "ijeri"),
        (1_000_000, "nde"),
        (1000, "puku"),
        (100, "narị"),
    ] {
        let count = number / scale % if scale == 100 { 10 } else { 1000 };
        match count {
            0 => {}
            1 => parts.push(format!("otu {name}")),
            count => parts.push(format!("{name} {}", cardinal(count))),
        }
    }
    let rest = number % 100;
    if rest > 0 {
        parts.push(below_hundred(rest));
    }
    parts.join(" na ")
}

/// Igbo words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Igbo;

impl Locale for Igbo {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    /// nke mbụ (the first), then nke and the cardinal (nke abụọ).
    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(if number == 1 {
            "nke mbụ".to_owned()
        } else {
            format!("nke {}", cardinal(number))
        })
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "kọma" } else { "ntụpọ" }
    }

    /// ụbọchị atọ n'ọnwa ise afọ puku abụọ na iri abụọ na anọ (the third of May 2024).
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!(
            "ụbọchị {} n'ọnwa {month} afọ {}",
            cardinal(day),
            cardinal(year)
        ))
    }

    fn range(&self) -> &'static str {
        "ruo"
    }

    fn minus(&self) -> &'static str {
        "wepụ"
    }

    fn plus(&self) -> &'static str {
        "gbakwunye"
    }

    fn percent(&self) -> &'static str {
        "pasentị"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "₦" | "NGN" => Some(("naịra", "naịra")),
            "$" => Some(("dọla", "dọla")),
            "€" => Some(("yuro", "yuro")),
            "£" => Some(("pọndụ", "pọndụ")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("pasentị"),
            "&" | "@" => Some("na"),
            "+" => Some("gbakwunye"),
            "=" => Some("hà nhata"),
            "#" => Some("nọmba"),
            "₦" => Some("naịra"),
            "$" => Some("dọla"),
            _ => None,
        }
    }

    /// The letters' Igbo names (ba, cha, gba), the ones outside the alphabet as English calls them.
    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "ba",
            'c' => "cha",
            'd' => "da",
            'e' => "e",
            'f' => "fa",
            'g' => "ga",
            'h' => "ha",
            'i' => "i",
            'ị' => "ị",
            'j' => "ja",
            'k' => "ka",
            'l' => "la",
            'm' => "ma",
            'n' => "na",
            'ṅ' => "ṅa",
            'o' => "o",
            'ọ' => "ọ",
            'p' => "pa",
            'q' => "kwu",
            'r' => "ra",
            's' => "sa",
            't' => "ta",
            'u' => "u",
            'ụ' => "ụ",
            'v' => "va",
            'w' => "wa",
            'x' => "eksị",
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
            '@' => Some("na"),
            '.' => Some("ntụpọ"),
            '/' => Some("slash"),
            ':' => Some("kọlọn"),
            '-' => Some("ahịrị"),
            '_' => Some("ahịrị n'okpuru"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiouịọụ"
    }
}

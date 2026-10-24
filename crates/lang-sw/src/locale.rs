use jespersen_spoken::{Gender, Locale};

const UNITS: [&str; 10] = [
    "sifuri", "moja", "mbili", "tatu", "nne", "tano", "sita", "saba", "nane", "tisa",
];

const TENS: [&str; 10] = [
    "",
    "kumi",
    "ishirini",
    "thelathini",
    "arobaini",
    "hamsini",
    "sitini",
    "sabini",
    "themanini",
    "tisini",
];

const MONTHS: [&str; 12] = [
    "Januari", "Februari", "Machi", "Aprili", "Mei", "Juni", "Julai", "Agosti", "Septemba", "Oktoba", "Novemba",
    "Desemba",
];

/// Swahili abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 8] = [
    ("bw", "bwana"),
    ("bi", "bibi"),
    ("dkt", "daktari"),
    ("prof", "profesa"),
    ("mh", "mheshimiwa"),
    ("mt", "mtakatifu"),
    ("sh", "shilingi"),
    ("uk", "ukurasa"),
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

fn digit(value: u64) -> &'static str {
    UNITS[usize::try_from(value % 10).unwrap_or(0)]
}

/// `number` in Swahili words, the largest part first, with na before the last part (105 mia moja
/// na tano, 125 mia moja ishirini na tano, 1250 elfu moja mia mbili na hamsini); the tens take na
/// before their units (kumi na moja).
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, name) in [(1_000_000_000, "bilioni"), (1_000_000, "milioni"), (1000, "elfu")] {
        let count = number / scale % 1000;
        if count > 0 {
            parts.push(format!("{name} {}", below_thousand(count)));
        }
    }
    let hundreds = number / 100 % 10;
    if hundreds > 0 {
        parts.push(format!("mia {}", digit(hundreds)));
    }
    let rest = number % 100;
    if rest > 0 {
        parts.push(below_hundred(rest));
    }
    // na joins the last part, unless it has its own (ishirini na tano).
    if parts.len() > 1
        && let Some(last) = parts.last_mut()
        && !last.contains(" na ")
    {
        *last = format!("na {last}");
    }
    parts.join(" ")
}

fn below_hundred(number: u64) -> String {
    match (number / 10, number % 10) {
        (0, units) => digit(units).to_owned(),
        (tens, 0) => TENS[usize::try_from(tens).unwrap_or(0)].to_owned(),
        (tens, units) => format!("{} na {}", TENS[usize::try_from(tens).unwrap_or(0)], digit(units)),
    }
}

fn below_thousand(number: u64) -> String {
    if number < 100 {
        below_hundred(number)
    } else {
        cardinal(number)
    }
}

/// Swahili words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Swahili;

impl Locale for Swahili {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    /// The first and the second have words of their own (kwanza, pili); the others are said as
    /// their cardinal (wa tatu).
    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(match number {
            1 => "kwanza".to_owned(),
            2 => "pili".to_owned(),
            number => cardinal(number),
        })
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "koma" } else { "nukta" }
    }

    /// tatu Mei elfu mbili ishirini na nne (after tarehe, which the text writes).
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "hadi"
    }

    fn ratio(&self) -> &'static str {
        "kwa"
    }

    fn minus(&self) -> &'static str {
        "hasi"
    }

    fn plus(&self) -> &'static str {
        "jumlisha"
    }

    /// asilimia hamsini: the word comes before the number.
    fn percent(&self) -> &'static str {
        "asilimia"
    }

    fn percent_first(&self) -> bool {
        true
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "TSh" | "KSh" | "USh" | "Sh" => Some(("shilingi", "shilingi")),
            "€" => Some(("yuro", "yuro")),
            "$" => Some(("dola", "dola")),
            "£" => Some(("pauni", "pauni")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("asilimia"),
            "&" => Some("na"),
            "@" => Some("et"),
            "+" => Some("jumlisha"),
            "=" => Some("sawa na"),
            "°" => Some("nyuzi"),
            "#" => Some("namba"),
            "€" => Some("yuro"),
            "$" => Some("dola"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "be",
            'c' => "che",
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
            'x' => "eksi",
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
            '@' => Some("et"),
            '.' => Some("nukta"),
            '/' => Some("mkwaju"),
            ':' => Some("nukta mbili"),
            '-' => Some("kistari"),
            '_' => Some("mstari chini"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiou"
    }
}

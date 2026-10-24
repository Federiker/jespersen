use jespersen_spoken::{Gender, Locale};

const UNITS: [&str; 10] = [
    "eber", "kow", "laba", "saddex", "afar", "shan", "lix", "toddoba", "siddeed", "sagaal",
];

const TENS: [&str; 10] = [
    "",
    "toban",
    "labaatan",
    "soddon",
    "afartan",
    "konton",
    "lixdan",
    "toddobaatan",
    "siddeetan",
    "sagaashan",
];

const MONTHS: [&str; 12] = [
    "Janaayo",
    "Febraayo",
    "Maarso",
    "Abriil",
    "Maajo",
    "Juun",
    "Luulyo",
    "Agoosto",
    "Sebtembar",
    "Oktoobar",
    "Nofembar",
    "Desembar",
];

/// Somali abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 5] = [
    ("md", "mudane"),
    ("mw", "marwo"),
    ("dr", "dhakhtar"),
    ("prof", "barfasoor"),
    ("tus", "tusaale"),
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

/// `number` in Somali words, the largest part first and every part joined by iyo; below a
/// hundred the units come before the tens (21 kow iyo labaatan), and a count of one before a
/// hundred or a thousand is left unsaid (100 boqol, 1000 kun, but 2000 laba kun).
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let scaled = |count: u64, name: &str| {
        if count == 1 {
            name.to_owned()
        } else {
            format!("{} {name}", cardinal(count))
        }
    };
    let mut parts: Vec<String> = Vec::new();
    for (scale, name) in [(1_000_000_000, "bilyan"), (1_000_000, "malyuun"), (1000, "kun")] {
        let count = number / scale % 1000;
        if count > 0 {
            parts.push(scaled(count, name));
        }
    }
    let hundreds = number / 100 % 10;
    if hundreds > 0 {
        parts.push(scaled(hundreds, "boqol"));
    }
    let rest = number % 100;
    if rest > 0 {
        parts.push(match (rest / 10, rest % 10) {
            (0, units) => digit(units).to_owned(),
            (tens, 0) => TENS[usize::try_from(tens).unwrap_or(0)].to_owned(),
            (tens, units) => format!("{} iyo {}", digit(units), TENS[usize::try_from(tens).unwrap_or(0)]),
        });
    }
    parts.join(" iyo ")
}

/// An ordinal: the cardinal's last word with -aad (saddexaad), a final a dropped (labaad) and
/// afar shortened (afraad).
fn ordinal(number: u64) -> String {
    let words = cardinal(number);
    let (head, last) = words
        .rsplit_once(' ')
        .map_or(("", words.as_str()), |(head, last)| (head, last));
    let last = match last {
        "afar" => "afraad".to_owned(),
        last => format!("{}aad", last.strip_suffix('a').unwrap_or(last)),
    };
    if head.is_empty() {
        last
    } else {
        format!("{head} {last}")
    }
}

/// Somali words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Somali;

impl Locale for Somali {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(ordinal(number))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "hakad" } else { "dhibic" }
    }

    /// saddex Maajo laba kun iyo afar iyo labaatan.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "ilaa"
    }

    fn ratio(&self) -> &'static str {
        "ka"
    }

    fn minus(&self) -> &'static str {
        "taban"
    }

    fn plus(&self) -> &'static str {
        "lagu daray"
    }

    /// boqolkiiba konton: the word comes before the number.
    fn percent(&self) -> &'static str {
        "boqolkiiba"
    }

    fn percent_first(&self) -> bool {
        true
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "Sh" | "SOS" => Some(("shilin", "shilin")),
            "$" => Some(("doolar", "doolar")),
            "€" => Some(("yuuro", "yuuro")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("boqolkiiba"),
            "&" => Some("iyo"),
            "+" => Some("lagu daray"),
            "=" => Some("la mid ah"),
            "#" => Some("lambar"),
            "$" => Some("doolar"),
            "€" => Some("yuuro"),
            _ => None,
        }
    }

    /// The letters' Somali names (ba, ta, ja), the vowels said as themselves.
    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "ba",
            'c' => "ca",
            'd' => "da",
            'e' => "e",
            'f' => "fa",
            'g' => "ga",
            'h' => "ha",
            'i' => "i",
            'j' => "ja",
            'k' => "ka",
            'l' => "la",
            'm' => "ma",
            'n' => "na",
            'o' => "o",
            'p' => "pa",
            'q' => "qa",
            'r' => "ra",
            's' => "sa",
            't' => "ta",
            'u' => "u",
            'v' => "va",
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
            '@' => Some("at"),
            '.' => Some("dhibic"),
            '/' => Some("xariiq"),
            ':' => Some("labo dhibcood"),
            '-' => Some("jiitin"),
            '_' => Some("xariiq hoose"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiou"
    }
}

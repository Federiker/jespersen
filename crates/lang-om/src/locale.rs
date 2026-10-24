use jespersen_spoken::{Gender, Locale};

const UNITS: [&str; 10] = [
    "duwwaa", "tokko", "lama", "sadii", "afur", "shan", "ja'a", "torba", "saddeet", "sagal",
];

const TENS: [&str; 10] = [
    "",
    "kudhan",
    "digdama",
    "soddoma",
    "afurtama",
    "shantama",
    "jaatama",
    "torbaatama",
    "saddeettama",
    "sagaltama",
];

/// The first ten ordinals, whose stems change (lammaffaa, sadaffaa, kurnaffaa).
const ORDINALS: [&str; 10] = [
    "tokkoffaa",
    "lammaffaa",
    "sadaffaa",
    "afraffaa",
    "shanaffaa",
    "ja'affaa",
    "torbaffaa",
    "saddeettaffaa",
    "sagalaffaa",
    "kurnaffaa",
];

const MONTHS: [&str; 12] = [
    "Amajjii",
    "Guraandhala",
    "Bitootessa",
    "Elba",
    "Caamsa",
    "Waxabajjii",
    "Adooleessa",
    "Hagayya",
    "Fulbaana",
    "Onkololeessa",
    "Sadaasa",
    "Muddee",
];

/// Oromo abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 5] = [
    ("ob", "obbo"),
    ("ad", "aadde"),
    ("dr", "doktor"),
    ("fkn", "fakkeenyaaf"),
    ("kkf", "kan kana fakkaatan"),
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

/// `number` in Oromo words, the largest part first and the parts joined by fi; a hundred, a
/// thousand and a million come before their count (dhibba lama, kuma tokko), and a ten before
/// its units, kudhan as kudha and the other tens ending in -ii (11 kudha tokko, 21 digdamii
/// tokko).
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, name) in [(1_000_000_000, "biiliyoona"), (1_000_000, "miliyoona"), (1000, "kuma")] {
        let count = number / scale % 1000;
        if count > 0 {
            parts.push(format!("{name} {}", cardinal(count)));
        }
    }
    let hundreds = number / 100 % 10;
    if hundreds > 0 {
        parts.push(format!("dhibba {}", digit(hundreds)));
    }
    let rest = number % 100;
    if rest > 0 {
        let ten = TENS[usize::try_from(rest / 10).unwrap_or(0)];
        parts.push(match (rest / 10, rest % 10) {
            (0, units) => digit(units).to_owned(),
            (_, 0) => ten.to_owned(),
            (1, units) => format!("kudha {}", digit(units)),
            (_, units) => format!("{}ii {}", ten.strip_suffix('a').unwrap_or(ten), digit(units)),
        });
    }
    parts.join(" fi ")
}

/// An ordinal: the first ten have their own stems, the others the cardinal with -ffaa.
fn ordinal(number: u64) -> String {
    match number
        .checked_sub(1)
        .and_then(|index| ORDINALS.get(usize::try_from(index).ok()?))
    {
        Some(word) => (*word).to_owned(),
        None => format!("{}ffaa", cardinal(number)),
    }
}

/// Oromo words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Oromo;

impl Locale for Oromo {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(ordinal(number))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "koomaa" } else { "tuqaa" }
    }

    /// sadii Caamsa kuma lama fi digdamii afur.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "hanga"
    }

    fn minus(&self) -> &'static str {
        "hir'isuu"
    }

    fn plus(&self) -> &'static str {
        "ida'uu"
    }

    /// dhibbantaa kudhan: the word comes before the number.
    fn percent(&self) -> &'static str {
        "dhibbantaa"
    }

    fn percent_first(&self) -> bool {
        true
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "Br" | "ETB" => Some(("birrii", "birrii")),
            "$" => Some(("doolaara", "doolaara")),
            "€" => Some(("yuuroo", "yuuroo")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("dhibbantaa"),
            "&" => Some("fi"),
            "+" => Some("ida'uu"),
            "=" => Some("wal qixa"),
            "#" => Some("lakkoofsa"),
            "$" => Some("doolaara"),
            "€" => Some("yuuroo"),
            _ => None,
        }
    }

    /// The Qubee letters' names: the vowels as themselves, the consonants with a (ba, ca, da).
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
            '.' => Some("tuqaa"),
            '/' => Some("sarara"),
            ':' => Some("tuqaa lama"),
            '-' => Some("sarara gabaabaa"),
            '_' => Some("sarara jalaa"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiou"
    }
}

use jespersen_spoken::{Gender, Locale, marks::plain};

/// The numbers one to ten as counted (ọ̀kan, èjì).
const UNITS: [&str; 11] = [
    "òdo",
    "ọ̀kan",
    "èjì",
    "ẹ̀ta",
    "ẹ̀rin",
    "àrún",
    "ẹ̀fà",
    "èje",
    "ẹ̀jọ",
    "ẹ̀sán",
    "ẹ̀wá",
];

/// One to five as they begin a compound number (ẹ̀tàlá 13, àrùndínlógún 15).
const STEMS: [&str; 6] = ["", "ọ̀kan", "èjì", "ẹ̀tà", "ẹ̀rìn", "àrùn"];

/// The tens from ten to two hundred, counted in twenties: the even ones are twenties (ogún 20,
/// ogójì 40, ọgọ́ta 60), the odd ones ten less than the next (àádọ́ta 50, from ọgọ́ta).
const TENS: [&str; 21] = [
    "",
    "ẹ̀wá",
    "ogún",
    "ọgbọ̀n",
    "ogójì",
    "àádọ́ta",
    "ọgọ́ta",
    "àádọ́rin",
    "ọgọ́rin",
    "àádọ́rùn",
    "ọgọ́rùn",
    "àádọ́fà",
    "ọgọ́fà",
    "àádóje",
    "ọgóje",
    "àádọ́jọ",
    "ọgọ́jọ",
    "àádọ́sàn",
    "ọgọ́sàn",
    "àádọ́wàá",
    "igba",
];

/// The hundreds from two hundred to nine hundred (ọ̀ọ́dúnrún 300, a hundred less than irinwó 400).
const HUNDREDS: [&str; 10] = [
    "",
    "",
    "igba",
    "ọ̀ọ́dúnrún",
    "irinwó",
    "ẹ̀ẹ́dẹ́gbẹ̀ta",
    "ẹgbẹ̀ta",
    "ẹ̀ẹ́dẹ́gbẹ̀rin",
    "ẹgbẹ̀rin",
    "ẹ̀ẹ́dẹ́gbẹ̀rún",
];

/// Two to ten as they follow what they count (ẹgbẹ̀rún méjì, two thousand).
const COUNTS: [&str; 11] = [
    "",
    "kan",
    "méjì",
    "mẹ́ta",
    "mẹ́rin",
    "márùn",
    "mẹ́fà",
    "méje",
    "mẹ́jọ",
    "mẹ́sàn",
    "mẹ́wàá",
];

/// The months by their Yoruba names.
const MONTHS: [&str; 12] = [
    "Ṣẹ́rẹ́",
    "Èrèlé",
    "Ẹrẹ̀nà",
    "Igbe",
    "Èbìbì",
    "Okúdu",
    "Agẹmọ",
    "Ògún",
    "Owewe",
    "Ọ̀wàrà",
    "Bélú",
    "Ọ̀pẹ",
];

/// Yoruba abbreviations with a full stop, without their tone marks, and how they are said.
const ABBREVIATIONS: [(&str, &str); 4] = [
    ("ọgb", "ọ̀gbẹ́ni"),
    ("dr", "dókítà"),
    ("ọjọgb", "ọ̀jọ̀gbọ́n"),
    ("prof", "ọ̀jọ̀gbọ́n"),
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
    usize::try_from(number).unwrap_or(0)
}

/// One to 199 as counted in twenties: a ten said whole, one to four more after it with lé (ọ̀kanlélógún
/// 21), five to nine as one to five less than the next ten with dín (àrùndínlógún 15); eleven to
/// fourteen join ten as -lá (ọ̀kanlá).
fn below_two_hundred(number: u64) -> String {
    let (tens, units) = (number / 10, number % 10);
    match (tens, units) {
        (0, units) => UNITS[index(units)].to_owned(),
        (tens, 0) => TENS[index(tens)].to_owned(),
        (1, units @ 1..=4) => format!("{}lá", STEMS[index(units)]),
        (tens, units @ 1..=4) => format!("{}lél{}", STEMS[index(units)], TENS[index(tens)]),
        (tens, units) => format!("{}dínl{}", STEMS[index(10 - units)], TENS[index(tens + 1)]),
    }
}

fn below_thousand(number: u64) -> String {
    if number < 200 {
        return below_two_hundred(number);
    }
    let (hundreds, rest) = (number / 100, number % 100);
    let hundreds = HUNDREDS[index(hundreds)];
    if rest == 0 {
        hundreds.to_owned()
    } else {
        format!("{hundreds} ó lé {}", below_two_hundred(rest))
    }
}

/// How many of a large unit there are: two to ten as counted after it (méjì), more as a number.
fn count(number: u64) -> String {
    COUNTS
        .get(index(number))
        .map_or_else(|| below_thousand(number), |&count| count.to_owned())
}

/// `number` in Yoruba words: up to 199 counted in twenties, the hundreds by their own names (igba
/// 200, irinwó 400), and the thousands and millions with their count after them (ẹgbẹ̀rún méjì),
/// as numbers are said today; the parts after the largest join with ó lé.
pub(crate) fn cardinal(number: u64) -> String {
    if number < 1000 {
        return below_thousand(number);
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, name) in [(1_000_000_000, "bílíọ̀nù"), (1_000_000, "mílíọ̀nù"), (1000, "ẹgbẹ̀rún")] {
        let many = number / scale % 1000;
        match many {
            0 => {}
            1 => parts.push(name.to_owned()),
            many => parts.push(format!("{name} {}", count(many))),
        }
    }
    let rest = number % 1000;
    if rest > 0 {
        parts.push(below_thousand(rest));
    }
    parts.join(" ó lé ")
}

/// Yoruba words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Yoruba;

impl Locale for Yoruba {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    /// The first ten have their own words (kínní, kejì, kẹta); the others are read as cardinals.
    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        const ORDINALS: [&str; 10] = [
            "kínní",
            "kejì",
            "kẹta",
            "kẹrin",
            "karùn",
            "kẹfà",
            "keje",
            "kẹjọ",
            "kẹsàn",
            "kẹwàá",
        ];
        ORDINALS.get(index(number).checked_sub(1)?).map(|&word| word.to_owned())
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "kọ́mà" } else { "pọ́ìntì" }
    }

    /// ọjọ́ ẹ̀ta oṣù Èbìbì ọdún ẹgbẹ̀rún méjì ó lé ẹ̀rìnlélógún (the third of May 2024).
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("ọjọ́ {} oṣù {month} ọdún {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "sí"
    }

    fn ratio(&self) -> &'static str {
        "sí"
    }

    fn minus(&self) -> &'static str {
        "àyọkúrò"
    }

    fn plus(&self) -> &'static str {
        "àfikún"
    }

    fn percent(&self) -> &'static str {
        "nínú ọgọ́rùn"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "₦" | "NGN" => Some(("náírà", "náírà")),
            "$" => Some(("dọ́là", "dọ́là")),
            "€" => Some(("yúrò", "yúrò")),
            "£" => Some(("pọ́ọ̀nù", "pọ́ọ̀nù")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("nínú ọgọ́rùn"),
            "&" => Some("àti"),
            "@" => Some("ní"),
            "+" => Some("àfikún"),
            "=" => Some("dọ́gba"),
            "#" => Some("nọ́mbà"),
            "₦" => Some("náírà"),
            "$" => Some("dọ́là"),
            _ => None,
        }
    }

    /// The letters' Yoruba names (bi, di, gbi), the ones outside the alphabet as English calls
    /// them.
    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "bi",
            'd' => "di",
            'e' => "e",
            'ẹ' => "ẹ",
            'f' | 'v' => "fi",
            'g' => "gi",
            'h' => "hi",
            'i' => "i",
            'j' => "ji",
            'k' => "ki",
            'l' => "li",
            'm' => "mi",
            'n' => "ni",
            'o' => "o",
            'ọ' => "ọ",
            'p' => "pi",
            'q' => "kíù",
            'r' => "ri",
            'c' | 's' => "si",
            'ṣ' => "ṣi",
            't' => "ti",
            'u' => "u",
            'w' => "wi",
            'x' => "ẹ́ẹ̀sì",
            'y' => "yi",
            'z' => "sẹ́dì",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("ní"),
            '.' => Some("àmì"),
            '/' => Some("ìlà"),
            ':' => Some("kólọ́nù"),
            '-' => Some("àlàfo"),
            '_' => Some("ìlà ìsàlẹ̀"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiouẹọ"
    }
}

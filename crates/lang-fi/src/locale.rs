use jespersen_spoken::{Gender, Locale};

use crate::numbers::{cardinal, ordinal};

/// The months in the partitive, as a date says them (kolmas toukokuuta).
const MONTHS: [&str; 12] = [
    "tammikuuta",
    "helmikuuta",
    "maaliskuuta",
    "huhtikuuta",
    "toukokuuta",
    "kesäkuuta",
    "heinäkuuta",
    "elokuuta",
    "syyskuuta",
    "lokakuuta",
    "marraskuuta",
    "joulukuuta",
];

/// Finnish abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 24] = [
    ("esim", "esimerkiksi"),
    ("mm", "muun muassa"),
    ("jne", "ja niin edelleen"),
    ("ks", "katso"),
    ("n", "noin"),
    ("ym", "ynnä muuta"),
    ("yms", "ynnä muuta sellaista"),
    ("tms", "tai muuta sellaista"),
    ("vrt", "vertaa"),
    ("s", "sivu"),
    ("puh", "puhelin"),
    ("os", "osoite"),
    ("ns", "niin sanottu"),
    ("eKr", "ennen Kristusta"),
    ("jKr", "jälkeen Kristuksen"),
    ("prof", "professori"),
    ("tri", "tohtori"),
    ("rva", "rouva"),
    ("hra", "herra"),
    ("nti", "neiti"),
    ("klo", "kello"),
    ("kpl", "kappaletta"),
    ("t", "terveisin"),
    ("ma", "maanantai"),
];

/// Whether `word` (without its full stop) is an abbreviation that does not end a sentence.
#[must_use]
pub fn is_abbreviation(word: &str) -> bool {
    expansion(word).is_some()
}

/// How an abbreviation is said; one listed in lower case is found in any case (esim., Esim.).
fn expansion(word: &str) -> Option<&'static str> {
    let lower = word.to_lowercase();
    ABBREVIATIONS
        .iter()
        .find(|(abbreviation, _)| {
            *abbreviation == word || (abbreviation.chars().all(char::is_lowercase) && *abbreviation == lower)
        })
        .map(|(_, said)| *said)
}

/// Finnish words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Finnish;

impl Locale for Finnish {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        ordinal(number).map(str::to_owned)
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "pilkku" } else { "piste" }
    }

    /// kolmas toukokuuta kaksi tuhatta kaksikymmentäneljä.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        let day = ordinal(day).map_or_else(|| cardinal(day), str::to_owned);
        Some(format!("{day} {month} {}", cardinal(year)))
    }

    /// A range is the two numbers said one after the other.
    fn range(&self) -> &'static str {
        ""
    }

    fn ratio(&self) -> &'static str {
        "kautta"
    }

    fn minus(&self) -> &'static str {
        "miinus"
    }

    fn plus(&self) -> &'static str {
        "plus"
    }

    fn percent(&self) -> &'static str {
        "prosenttia"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" => Some(("euro", "euroa")),
            "$" => Some(("dollari", "dollaria")),
            "£" => Some(("punta", "puntaa")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("prosenttia"),
            "&" => Some("ja"),
            "@" => Some("ät"),
            "+" => Some("plus"),
            "=" => Some("on"),
            "°" => Some("astetta"),
            "#" => Some("risuaita"),
            "€" => Some("euroa"),
            "$" => Some("dollaria"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "aa",
            'b' => "bee",
            'c' => "see",
            'd' => "dee",
            'e' => "ee",
            'f' => "äf",
            'g' => "gee",
            'h' => "hoo",
            'i' => "ii",
            'j' => "jii",
            'k' => "koo",
            'l' => "äl",
            'm' => "äm",
            'n' => "än",
            'o' | 'å' => "oo",
            'p' => "pee",
            'q' => "kuu",
            'r' => "är",
            's' => "äs",
            't' => "tee",
            'u' => "uu",
            'v' => "vee",
            'w' => "kaksoisvee",
            'x' => "äks",
            'y' => "yy",
            'z' => "tset",
            'ä' => "ää",
            'ö' => "öö",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("ät"),
            '.' => Some("piste"),
            '/' => Some("kauttaviiva"),
            ':' => Some("kaksoispiste"),
            '-' => Some("viiva"),
            '_' => Some("alaviiva"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiouyäöå"
    }
}

use jespersen_spoken::{Gender, Locale};

use crate::numbers::cardinal;

const MONTHS: [&str; 12] = [
    "jaanuar",
    "veebruar",
    "märts",
    "aprill",
    "mai",
    "juuni",
    "juuli",
    "august",
    "september",
    "oktoober",
    "november",
    "detsember",
];

/// Estonian abbreviations and how they are said.
const ABBREVIATIONS: [(&str, &str); 10] = [
    ("dr", "doktor"),
    ("prof", "professor"),
    ("hr", "härra"),
    ("pr", "proua"),
    ("nr", "number"),
    ("tn", "tänav"),
    ("nt", "näiteks"),
    ("jne", "ja nii edasi"),
    ("tel", "telefon"),
    ("u", "umbes"),
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

/// Estonian words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Estonian;

impl Locale for Estonian {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "koma" } else { "punkt" }
    }

    /// kümme kolmkümmend; the full hour is kell kümme.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours);
        Some(if minutes == 0 {
            format!("kell {hour}")
        } else {
            format!("{hour} {}", cardinal(minutes))
        })
    }

    /// kolm mai kaks tuhat kakskümmend neli.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "kuni"
    }

    fn ratio(&self) -> &'static str {
        "jagatud"
    }

    fn minus(&self) -> &'static str {
        "miinus"
    }

    fn plus(&self) -> &'static str {
        "pluss"
    }

    fn percent(&self) -> &'static str {
        "protsenti"
    }

    /// The singular after one, the partitive after more (üks euro, viis eurot).
    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("euro", "eurot")),
            "$" | "USD" => Some(("dollar", "dollarit")),
            "£" | "GBP" => Some(("nael", "naela")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("protsenti"),
            "&" => Some("ja"),
            "@" => Some("ätt"),
            "+" => Some("pluss"),
            "=" => Some("võrdub"),
            "°" => Some("kraadi"),
            "#" => Some("trellid"),
            "€" => Some("eurot"),
            "$" => Some("dollarit"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "aa",
            'b' => "bee",
            'c' => "tsee",
            'd' => "dee",
            'e' => "ee",
            'f' => "eff",
            'g' => "gee",
            'h' => "haa",
            'i' => "ii",
            'j' => "jott",
            'k' => "kaa",
            'l' => "ell",
            'm' => "emm",
            'n' => "enn",
            'o' => "oo",
            'p' => "pee",
            'q' => "kuu",
            'r' => "err",
            's' => "ess",
            'š' => "šaa",
            'z' => "zett",
            'ž' => "žee",
            't' => "tee",
            'u' => "uu",
            'v' => "vee",
            'w' => "kaksisvee",
            'õ' => "õõ",
            'ä' => "ää",
            'ö' => "öö",
            'ü' => "üü",
            'x' => "iks",
            'y' => "igrek",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("ätt"),
            '.' => Some("punkt"),
            '/' => Some("kaldkriips"),
            ':' => Some("koolon"),
            '-' => Some("sidekriips"),
            '_' => Some("alakriips"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiouõäöüy"
    }
}

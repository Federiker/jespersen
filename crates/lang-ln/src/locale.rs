use jespersen_spoken::{Gender, Locale};

/// The numbers said when counting (mɔkɔ, mibale…), tone marks left out.
const UNITS: [&str; 10] = [
    "zero", "mɔkɔ", "mibale", "misato", "minei", "mitano", "motoba", "sambo", "mwambe", "libwa",
];

/// Lingala abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 4] = [
    ("dr", "dokotolo"),
    ("prof", "pulofesɛlɛ"),
    ("mt", "mosantu"),
    ("p", "lokasa"),
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

fn unit(digit: u64) -> &'static str {
    UNITS[usize::try_from(digit % 10).unwrap_or(0)]
}

/// `number` in Lingala words, the largest part first and na between the parts (zomi na mɔkɔ,
/// ntuku mibale na mitano, nkama mibale); hundreds and tens are counted (nkama mibale, ntuku
/// misato).
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, name) in [(1_000_000_000, "miliari"), (1_000_000, "milio"), (1000, "nkoto")] {
        match number / scale % 1000 {
            0 => {}
            1 => parts.push(name.to_owned()),
            count => parts.push(format!("{name} {}", cardinal(count))),
        }
    }
    match number / 100 % 10 {
        0 => {}
        1 => parts.push("nkama".to_owned()),
        hundreds => parts.push(format!("nkama {}", unit(hundreds))),
    }
    match number / 10 % 10 {
        0 => {}
        1 => parts.push("zomi".to_owned()),
        tens => parts.push(format!("ntuku {}", unit(tens))),
    }
    if !number.is_multiple_of(10) {
        parts.push(unit(number).to_owned());
    }
    parts.join(" na ")
}

/// Lingala words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Lingala;

impl Locale for Lingala {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    /// The first is yambo (ya yambo); the others are said as their cardinal (ya mibale).
    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(if number == 1 {
            "yambo".to_owned()
        } else {
            cardinal(number)
        })
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "koma" } else { "pwɛ" }
    }

    /// misato sanza ya mitano nkoto mibale na ntuku mibale na minei: the months are counted (sanza
    /// ya yambo for January).
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = u64::try_from(month).ok().filter(|month| (1..=12).contains(month))?;
        let month = if month == 1 {
            "yambo".to_owned()
        } else {
            cardinal(month)
        };
        Some(format!("{} sanza ya {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "tii"
    }

    fn ratio(&self) -> &'static str {
        "na"
    }

    fn minus(&self) -> &'static str {
        "longola"
    }

    fn plus(&self) -> &'static str {
        "bakisa"
    }

    /// ntuku mitano na nkama: "out of a hundred", after the number.
    fn percent(&self) -> &'static str {
        "na nkama"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "FC" | "CDF" => Some(("falanga", "falanga")),
            "€" => Some(("ero", "ero")),
            "$" => Some(("dolare", "dolare")),
            "£" => Some(("livre", "livre")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("na nkama"),
            "&" => Some("na"),
            "@" => Some("arobase"),
            "+" => Some("bakisa"),
            "=" => Some("ekokani na"),
            "°" => Some("degre"),
            "#" => Some("nimero"),
            "€" => Some("ero"),
            "$" => Some("dolare"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "be",
            'd' => "de",
            'e' => "e",
            'ɛ' => "ɛ",
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
            'ɔ' => "ɔ",
            'p' => "pe",
            'q' => "ku",
            'r' => "re",
            'c' | 's' => "se",
            't' => "te",
            'u' => "u",
            'v' => "ve",
            'w' => "we",
            'x' => "iksi",
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
            '@' => Some("arobase"),
            '.' => Some("pwɛ"),
            '/' => Some("slash"),
            ':' => Some("pwɛ mibale"),
            '-' => Some("tirɛ"),
            '_' => Some("tirɛ na nse"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeɛioɔu"
    }
}

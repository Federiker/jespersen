use jespersen_spoken::{Gender, Locale};

use crate::numbers::{cardinal, ordinal};

const MONTHS: [&str; 12] = [
    "janvier",
    "février",
    "mars",
    "avril",
    "mai",
    "juin",
    "juillet",
    "août",
    "septembre",
    "octobre",
    "novembre",
    "décembre",
];

/// French abbreviations and how they are said. Those written with a capital (M., Mme) match only
/// so; the others in any case.
const ABBREVIATIONS: [(&str, &str); 20] = [
    ("M", "monsieur"),
    ("MM", "messieurs"),
    ("Mme", "madame"),
    ("Mmes", "mesdames"),
    ("Mlle", "mademoiselle"),
    ("Mlles", "mesdemoiselles"),
    ("Dr", "docteur"),
    ("Pr", "professeur"),
    ("St", "saint"),
    ("Ste", "sainte"),
    ("av", "avenue"),
    ("bd", "boulevard"),
    ("etc", "et cetera"),
    ("env", "environ"),
    ("cf", "confer"),
    ("vol", "volume"),
    ("chap", "chapitre"),
    ("tél", "téléphone"),
    ("fig", "figure"),
    ("n°", "numéro"),
];

fn expansion(word: &str) -> Option<&'static str> {
    ABBREVIATIONS
        .iter()
        .find(|(abbreviation, _)| {
            *abbreviation == word
                || (abbreviation.chars().all(char::is_lowercase) && word.to_lowercase() == *abbreviation)
        })
        .map(|(_, said)| *said)
}

/// Whether `word` (without its full stop) is an abbreviation that does not end a sentence.
#[must_use]
pub fn is_abbreviation(word: &str) -> bool {
    expansion(word).is_some()
}

/// Endings of feminine nouns, for une (vingt et une heures).
const FEMININE_ENDINGS: [&str; 16] = [
    "tion", "sion", "té", "ure", "ence", "ance", "ette", "elle", "euse", "trice", "ie", "ée", "ade", "ine", "aine",
    "esse",
];
const FEMININE_NOUNS: [&str; 12] = [
    "heure", "fois", "personne", "femme", "fille", "minute", "seconde", "page", "maison", "fleur", "nuit", "semaine",
];

/// French words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct French;

impl Locale for French {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        Some(cardinal(number, gender == Gender::Feminine))
    }

    fn ordinal(&self, number: u64, gender: Gender) -> Option<String> {
        ordinal(number, gender == Gender::Feminine)
    }

    fn ordinal_signs(&self) -> &'static [(&'static str, Gender)] {
        &[
            ("er", Gender::Masculine),
            ("re", Gender::Feminine),
            ("ère", Gender::Feminine),
            ("ème", Gender::Alone),
            ("e", Gender::Alone),
        ]
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "virgule" } else { "point" }
    }

    /// dix heures trente, une heure, midi stays a number (douze heures).
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = if hours == 1 {
            "une heure".to_owned()
        } else {
            format!("{} heures", cardinal(hours, true))
        };
        Some(if minutes == 0 {
            hour
        } else {
            format!("{hour} {}", cardinal(minutes, true))
        })
    }

    /// trois mai deux mille vingt-quatre; the first is premier (premier mai).
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        let day = if day == 1 {
            "premier".to_owned()
        } else {
            cardinal(day, false)
        };
        Some(format!("{day} {month} {}", cardinal(year, false)))
    }

    fn range(&self) -> &'static str {
        "à"
    }

    fn ratio(&self) -> &'static str {
        "sur"
    }

    fn minus(&self) -> &'static str {
        "moins"
    }

    fn plus(&self) -> &'static str {
        "plus"
    }

    fn percent(&self) -> &'static str {
        "pour cent"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("euro", "euros")),
            "$" | "USD" => Some(("dollar", "dollars")),
            "£" | "GBP" => Some(("livre", "livres")),
            "CHF" => Some(("franc", "francs")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("pour cent"),
            "&" => Some("et"),
            "@" => Some("arobase"),
            "+" => Some("plus"),
            "=" => Some("égale"),
            "°" => Some("degrés"),
            "#" => Some("dièse"),
            "€" => Some("euros"),
            "$" => Some("dollars"),
            _ => None,
        }
    }

    /// The names of the letters, respelt as the rules read them (f èffe, h ache, y igrec).
    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' | 'à' | 'â' => "a",
            'b' => "bé",
            'c' | 'ç' => "cé",
            'd' => "dé",
            'e' | 'é' | 'è' | 'ê' => "eu",
            'f' => "èffe",
            'g' => "gé",
            'h' => "ache",
            'i' | 'î' | 'ï' => "i",
            'j' => "ji",
            'k' => "ka",
            'l' => "èle",
            'm' => "ème",
            'n' => "ène",
            'o' | 'ô' => "o",
            'p' => "pé",
            'q' => "ku",
            'r' => "ère",
            's' => "èsse",
            't' => "té",
            'u' | 'ù' | 'û' => "u",
            'v' => "vé",
            'w' => "doublevé",
            'x' => "ixe",
            'y' => "igrec",
            'z' => "zède",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("arobase"),
            '.' => Some("point"),
            '/' => Some("barre oblique"),
            ':' => Some("deux-points"),
            '-' => Some("tiret"),
            '_' => Some("tiret bas"),
            _ => None,
        }
    }

    /// Feminine nouns by their ending (-tion, -té, -ure…) and the common ones that are not (heure,
    /// fois), for une.
    fn gender_of(&self, word: &str) -> Gender {
        let lower = word.to_lowercase();
        let singular = lower.strip_suffix('s').unwrap_or(&lower);
        if FEMININE_NOUNS.contains(&singular) || FEMININE_ENDINGS.iter().any(|ending| singular.ends_with(ending)) {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    /// A Roman numeral after a name is a number, the first premier (Louis XIV quatorze, François
    /// Ier).
    fn roman(&self, value: u64, before: Option<&str>, _after: Option<&str>) -> Option<String> {
        if !before?.chars().next().is_some_and(char::is_uppercase) {
            return None;
        }
        Some(if value == 1 {
            "premier".to_owned()
        } else {
            cardinal(value, false)
        })
    }

    fn elides(&self) -> bool {
        true
    }

    fn vowels(&self) -> &'static str {
        "aàâeéèêëiîïoôuùûüyœ"
    }
}

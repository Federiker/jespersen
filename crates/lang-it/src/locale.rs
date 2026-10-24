use jespersen_spoken::{Gender, Locale};

use crate::{
    abbreviations::{expansion, letter_name},
    numbers::{cardinal, ordinal},
};

const MONTHS: [&str; 12] = [
    "gennaio",
    "febbraio",
    "marzo",
    "aprile",
    "maggio",
    "giugno",
    "luglio",
    "agosto",
    "settembre",
    "ottobre",
    "novembre",
    "dicembre",
];

/// Nouns ending in -a that are masculine, for un and una (un problema).
const MASCULINE_IN_A: [&str; 8] = [
    "problema",
    "tema",
    "sistema",
    "programma",
    "poeta",
    "pianeta",
    "clima",
    "dramma",
];

/// Italian words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Italian;

impl Locale for Italian {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        Some(cardinal(number, gender))
    }

    fn ordinal(&self, number: u64, gender: Gender) -> Option<String> {
        ordinal(number, gender == Gender::Feminine)
    }

    fn ordinal_signs(&self) -> &'static [(&'static str, Gender)] {
        &[("°", Gender::Alone), ("º", Gender::Alone), ("ª", Gender::Feminine)]
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "virgola" } else { "punto" }
    }

    /// 8:30 otto e mezza, 9:15 nove e un quarto, 10:00 dieci.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours, Gender::Alone);
        Some(match minutes {
            0 => hour,
            15 => format!("{hour} e un quarto"),
            30 => format!("{hour} e mezza"),
            minutes => format!("{hour} e {}", cardinal(minutes, Gender::Alone)),
        })
    }

    /// tre maggio duemilaventiquattro, primo maggio.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        let day = if day == 1 {
            "primo".to_owned()
        } else {
            cardinal(day, Gender::Alone)
        };
        Some(format!("{day} {month} {}", cardinal(year, Gender::Alone)))
    }

    fn range(&self) -> &'static str {
        "a"
    }

    fn ratio(&self) -> &'static str {
        "su"
    }

    fn minus(&self) -> &'static str {
        "meno"
    }

    fn plus(&self) -> &'static str {
        "più"
    }

    fn percent(&self) -> &'static str {
        "per cento"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" => Some(("euro", "euro")),
            "$" => Some(("dollaro", "dollari")),
            "£" => Some(("sterlina", "sterline")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("per cento"),
            "&" => Some("e"),
            "@" => Some("chiocciola"),
            "+" => Some("più"),
            "=" => Some("uguale a"),
            "°" | "º" => Some("gradi"),
            "#" => Some("numero"),
            "€" => Some("euro"),
            "$" => Some("dollari"),
            "£" => Some("sterline"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        letter_name(letter)
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("chiocciola"),
            '.' => Some("punto"),
            '/' => Some("barra"),
            ':' => Some("due punti"),
            '-' => Some("trattino"),
            '_' => Some("trattino basso"),
            _ => None,
        }
    }

    /// Feminine for -a (but un problema) and -ione (una stazione), masculine otherwise.
    fn gender_of(&self, word: &str) -> Gender {
        let lower = word.to_lowercase();
        if (lower.ends_with('a') && !MASCULINE_IN_A.contains(&lower.as_str())) || lower.ends_with("ione") {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    /// An ordinal after a name or before secolo (Luigi quattordicesimo, il ventesimo secolo).
    fn roman(&self, value: u64, before: Option<&str>, after: Option<&str>) -> Option<String> {
        let after_name = before.is_some_and(|before| before.chars().next().is_some_and(char::is_uppercase));
        let before_century = after.is_some_and(|after| after.to_lowercase().starts_with("secol"));
        (after_name || before_century).then(|| ordinal(value, false)).flatten()
    }

    fn elides(&self) -> bool {
        true
    }

    fn vowels(&self) -> &'static str {
        "aeiouàèéìòóù"
    }
}

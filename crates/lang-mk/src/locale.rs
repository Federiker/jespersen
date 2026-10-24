use jespersen_spoken::{Gender, Locale};

use crate::numbers::{Agreement, cardinal, day};

const MONTHS: [&str; 12] = [
    "јануари",
    "февруари",
    "март",
    "април",
    "мај",
    "јуни",
    "јули",
    "август",
    "септември",
    "октомври",
    "ноември",
    "декември",
];

/// Macedonian abbreviations and how they are said; д-р and м-р are written without a full stop.
const ABBREVIATIONS: [(&str, &str); 14] = [
    ("др", "доктор"),
    ("д-р", "доктор"),
    ("м-р", "магистер"),
    ("проф", "професор"),
    ("мр", "магистер"),
    ("инж", "инженер"),
    ("бр", "број"),
    ("ул", "улица"),
    ("тел", "телефон"),
    ("итн", "и така натаму"),
    ("стр", "страница"),
    ("св", "свети"),
    ("г", "година"),
    ("год", "година"),
];

pub(crate) fn expansion(word: &str) -> Option<&'static str> {
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

/// The names of currencies after a count of one, and of more.
fn currency_forms(sign: &str) -> Option<(&'static str, &'static str)> {
    match sign {
        "ден" | "MKD" => Some(("денар", "денари")),
        "€" | "EUR" => Some(("евро", "евра")),
        "$" | "USD" => Some(("долар", "долари")),
        "£" | "GBP" => Some(("фунта", "фунти")),
        _ => None,
    }
}

/// Macedonian words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Macedonian;

impl Locale for Macedonian {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        let agreement = if gender == Gender::Feminine {
            Agreement::Feminine
        } else {
            Agreement::Masculine
        };
        Some(cardinal(number, agreement))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "запирка" } else { "точка" }
    }

    /// десет и триесет; the full hour is десет часот.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours, Agreement::Masculine);
        Some(if minutes == 0 {
            format!("{hour} часот")
        } else {
            format!("{hour} и {}", cardinal(minutes, Agreement::Feminine))
        })
    }

    /// The day as an ordinal, the month, the year as a number: трети мај две илјади дваесет и
    /// четири.
    fn date(&self, day_of_month: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!(
            "{} {month} {}",
            day(day_of_month)?,
            cardinal(year, Agreement::Feminine)
        ))
    }

    fn range(&self) -> &'static str {
        "до"
    }

    fn ratio(&self) -> &'static str {
        "на"
    }

    fn minus(&self) -> &'static str {
        "минус"
    }

    fn plus(&self) -> &'static str {
        "плус"
    }

    fn percent(&self) -> &'static str {
        "проценти"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        currency_forms(sign)
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("проценти"),
            "&" => Some("и"),
            "@" => Some("мајмунче"),
            "+" => Some("плус"),
            "=" => Some("еднакво"),
            "°" => Some("степени"),
            "#" => Some("тараба"),
            "€" => Some("евра"),
            "$" => Some("долари"),
            _ => None,
        }
    }

    /// A letter's name, Cyrillic or Latin (бе, ве, ље, дабл ју).
    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'а' | 'a' => "а",
            'б' | 'b' => "бе",
            'в' | 'v' => "ве",
            'г' | 'g' => "ге",
            'д' | 'd' => "де",
            'ѓ' => "ѓе",
            'е' | 'e' => "е",
            'ж' => "же",
            'з' | 'z' => "зе",
            'ѕ' => "ѕе",
            'и' | 'i' => "и",
            'ј' => "је",
            'к' | 'k' => "ка",
            'л' | 'l' => "ел",
            'љ' => "ље",
            'м' | 'm' => "ем",
            'н' | 'n' => "ен",
            'њ' => "ње",
            'о' | 'o' => "о",
            'п' | 'p' => "пе",
            'р' | 'r' => "ер",
            'с' | 's' => "ес",
            'т' | 't' => "те",
            'ќ' => "ќе",
            'у' | 'u' => "у",
            'ф' | 'f' => "еф",
            'х' | 'h' => "ха",
            'ц' | 'c' => "це",
            'ч' => "че",
            'џ' => "џе",
            'ш' => "ше",
            'j' => "јот",
            'q' => "кју",
            'w' => "дабл ју",
            'x' => "икс",
            'y' => "ипсилон",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("мајмунче"),
            '.' => Some("точка"),
            '/' => Some("коса црта"),
            ':' => Some("две точки"),
            '-' => Some("цртичка"),
            '_' => Some("долна црта"),
            _ => None,
        }
    }

    /// Feminine nouns in the singular end in -а (една книга); the rest take еден.
    fn gender_of(&self, word: &str) -> Gender {
        if word.to_lowercase().ends_with('а') {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    fn vowels(&self) -> &'static str {
        "аеиоу"
    }
}

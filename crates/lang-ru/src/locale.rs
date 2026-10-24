use jespersen_spoken::{Gender, Locale};

use crate::numbers::{Agreement, Case, cardinal, form, ordinal};

/// The months in the genitive, as dates say them (третье мая).
const MONTHS: [&str; 12] = [
    "января",
    "февраля",
    "марта",
    "апреля",
    "мая",
    "июня",
    "июля",
    "августа",
    "сентября",
    "октября",
    "ноября",
    "декабря",
];

/// Russian abbreviations and how they are said; т.е and т.д are one token when written without
/// a space.
const ABBREVIATIONS: [(&str, &str); 26] = [
    ("т.е", "то есть"),
    ("т.д", "так далее"),
    ("т.п", "тому подобное"),
    ("т.к", "так как"),
    ("т.н", "так называемый"),
    ("напр", "например"),
    ("см", "смотри"),
    ("стр", "страница"),
    ("гг", "годы"),
    ("руб", "рублей"),
    ("коп", "копеек"),
    ("тыс", "тысяч"),
    ("млн", "миллионов"),
    ("млрд", "миллиардов"),
    ("проф", "профессор"),
    ("тов", "товарищ"),
    ("акад", "академик"),
    ("ул", "улица"),
    ("пл", "площадь"),
    ("обл", "область"),
    ("др", "другие"),
    ("пр", "прочее"),
    ("кг", "килограммов"),
    ("км", "километров"),
    ("мин", "минут"),
    ("сек", "секунд"),
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

/// Masculine and neuter nouns in the form they take after два, три and четыре, which ends like a
/// feminine one (два дня, три года, четыре окна).
const MASCULINE_AFTER_TWO: [&str; 34] = [
    "дня",
    "года",
    "часа",
    "раза",
    "рубля",
    "доллара",
    "человека",
    "километра",
    "метра",
    "сантиметра",
    "миллиона",
    "миллиарда",
    "процента",
    "слова",
    "места",
    "окна",
    "письма",
    "дела",
    "брата",
    "сына",
    "друга",
    "ребёнка",
    "ребенка",
    "мальчика",
    "стола",
    "дома",
    "города",
    "века",
    "месяца",
    "урока",
    "вопроса",
    "этажа",
    "градуса",
    "килограмма",
];

/// The names of currencies after a count of one, of two to four, and of the rest.
fn currency_forms(sign: &str) -> Option<[&'static str; 3]> {
    match sign {
        "₽" | "RUB" => Some(["рубль", "рубля", "рублей"]),
        "$" | "USD" => Some(["доллар", "доллара", "долларов"]),
        "€" | "EUR" => Some(["евро", "евро", "евро"]),
        "£" | "GBP" => Some(["фунт", "фунта", "фунтов"]),
        _ => None,
    }
}

/// Russian words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Russian;

impl Locale for Russian {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        let agreement = match gender {
            Gender::Feminine => Agreement::Feminine,
            Gender::Alone | Gender::Masculine => Agreement::Masculine,
        };
        Some(cardinal(number, agreement))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "запятая" } else { "точка" }
    }

    /// The day as a neuter ordinal, the month in the genitive and the year as an ordinal in the
    /// genitive: третье мая две тысячи двадцать четвёртого года.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!(
            "{} {month} {} года",
            ordinal(day, Case::Neuter)?,
            ordinal(year, Case::Genitive)?
        ))
    }

    fn range(&self) -> &'static str {
        "до"
    }

    fn ratio(&self) -> &'static str {
        "дробь"
    }

    fn minus(&self) -> &'static str {
        "минус"
    }

    fn plus(&self) -> &'static str {
        "плюс"
    }

    fn percent(&self) -> &'static str {
        "процентов"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        currency_forms(sign).map(|[one, _, many]| (one, many))
    }

    fn currency_name(&self, sign: &str, count: Option<u64>) -> Option<&'static str> {
        let forms = currency_forms(sign)?;
        Some(count.map_or(forms[2], |count| form(count, forms)))
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("процентов"),
            "&" => Some("и"),
            "@" => Some("собака"),
            "+" => Some("плюс"),
            "=" => Some("равно"),
            "°" => Some("градусов"),
            "#" => Some("решётка"),
            "№" => Some("номер"),
            "€" => Some("евро"),
            "$" => Some("долларов"),
            "₽" => Some("рублей"),
            _ => None,
        }
    }

    /// The letters' names, the Latin ones too; those of two words are said as one.
    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'а' | 'a' => "а",
            'б' | 'b' => "бэ",
            'в' | 'v' => "вэ",
            'г' | 'g' => "гэ",
            'д' | 'd' => "дэ",
            'е' | 'e' => "е",
            'ё' => "ё",
            'ж' => "жэ",
            'з' => "зэ",
            'и' | 'i' => "и",
            'й' => "и кра́ткое",
            'к' | 'k' => "ка",
            'л' | 'l' => "эль",
            'м' | 'm' => "эм",
            'н' | 'n' => "эн",
            'о' | 'o' => "о",
            'п' | 'p' => "пэ",
            'р' | 'r' => "эр",
            'с' | 's' => "эс",
            'т' | 't' => "тэ",
            'у' | 'u' => "у",
            'ф' | 'f' => "эф",
            'х' => "ха",
            'ц' | 'c' => "цэ",
            'ч' => "чэ",
            'ш' => "ша",
            'щ' => "ща",
            'ъ' => "твёрдый знак",
            'ы' => "ы",
            'ь' => "мя́гкий знак",
            'э' => "э",
            'ю' => "ю",
            'я' => "я",
            'h' => "аш",
            'j' => "йот",
            'q' => "ку",
            'w' => "дубль-вэ",
            'x' => "икс",
            'y' => "и́грек",
            'z' => "зэт",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("собака"),
            '.' => Some("точка"),
            '/' => Some("слэш"),
            ':' => Some("двоеточие"),
            '-' => Some("дефис"),
            '_' => Some("подчёркивание"),
            _ => None,
        }
    }

    /// A noun after a number is feminine in -а and -я (одна книга) and in -ы and -и (две книги),
    /// but for the masculine and neuter forms after два, три and четыре (два дня, три года); the
    /// rest take один and два.
    fn gender_of(&self, word: &str) -> Gender {
        let lower = word.to_lowercase();
        if MASCULINE_AFTER_TWO.contains(&lower.as_str()) {
            Gender::Masculine
        } else if lower.ends_with(['а', 'я', 'ы', 'и']) {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    /// A Roman numeral before век, a century, is an ordinal in век's case (двадцатый век, в
    /// двадцатом веке), after a name in the masculine (Пётр I первый).
    fn roman(&self, value: u64, before: Option<&str>, after: Option<&str>) -> Option<String> {
        let after = after.map(str::to_lowercase);
        let case = match after.as_deref() {
            Some("век") => Some(Case::Masculine),
            Some("столетие") => Some(Case::Neuter),
            Some("века" | "веков") => Some(Case::Genitive),
            Some("веке") => Some(Case::Prepositional),
            _ => None,
        };
        if let Some(case) = case {
            return ordinal(value, case);
        }
        if before.is_some_and(|before| before.chars().next().is_some_and(char::is_uppercase)) {
            return ordinal(value, Case::Masculine);
        }
        None
    }

    fn vowels(&self) -> &'static str {
        "аеёиоуыэюя"
    }
}

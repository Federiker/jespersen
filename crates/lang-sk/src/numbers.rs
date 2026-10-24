//! Slovak numbers in words: cardinals with their gender (jeden, jedna; dva, dve) and their
//! millions in the form the count asks for (dva milióny, päť miliónov), and ordinals in the cases
//! dates and centuries need.

/// How a number agrees with what it counts: jeden and dva, jedna and dve, or the counting forms
/// jeden and dva.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Agreement {
    Counting,
    Masculine,
    Feminine,
}

const UNITS: [&str; 20] = [
    "nula",
    "jeden",
    "dva",
    "tri",
    "štyri",
    "päť",
    "šesť",
    "sedem",
    "osem",
    "deväť",
    "desať",
    "jedenásť",
    "dvanásť",
    "trinásť",
    "štrnásť",
    "pätnásť",
    "šestnásť",
    "sedemnásť",
    "osemnásť",
    "devätnásť",
];

const TENS: [&str; 10] = [
    "",
    "desať",
    "dvadsať",
    "tridsať",
    "štyridsať",
    "päťdesiat",
    "šesťdesiat",
    "sedemdesiat",
    "osemdesiat",
    "deväťdesiat",
];

const HUNDREDS: [&str; 10] = [
    "",
    "sto",
    "dvesto",
    "tristo",
    "štyristo",
    "päťsto",
    "šesťsto",
    "sedemsto",
    "osemsto",
    "deväťsto",
];

fn digit(value: u64) -> usize {
    usize::try_from(value % 10).unwrap_or(0)
}

/// Which of three forms goes with `count`: one, two to four, and the rest (päť eur).
pub(crate) fn form(count: u64, forms: [&str; 3]) -> &str {
    if count == 1 {
        forms[0]
    } else if (2..=4).contains(&count) {
        forms[1]
    } else {
        forms[2]
    }
}

/// `number` below a thousand, agreeing as `agreement` says in its last word.
fn below_thousand(number: u64, agreement: Agreement) -> Vec<&'static str> {
    let mut words: Vec<&'static str> = Vec::new();
    if number >= 100 {
        words.push(HUNDREDS[digit(number / 100)]);
    }
    let rest = number % 100;
    let unit = |value: u64| match (value, agreement) {
        (1, Agreement::Feminine) => "jedna",
        (2, Agreement::Feminine) => "dve",
        (value, _) => UNITS[usize::try_from(value).unwrap_or(0)],
    };
    match rest {
        0 => {}
        1..=19 => words.push(unit(rest)),
        _ => {
            words.push(TENS[digit(rest / 10)]);
            if !rest.is_multiple_of(10) {
                words.push(unit(rest % 10));
            }
        }
    }
    words
}

/// `number` in words: 21 dvadsať jeden, 2024 dve tisíc dvadsať štyri, 1000 tisíc.
pub(crate) fn cardinal(number: u64, agreement: Agreement) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut words: Vec<&str> = Vec::new();
    // Thousands count as feminine (dve tisíc), millions as masculine, milliards as feminine.
    for (scale, forms, gender) in [
        (1_000_000_000, ["miliarda", "miliardy", "miliárd"], Agreement::Feminine),
        (1_000_000, ["milión", "milióny", "miliónov"], Agreement::Masculine),
        (1000, ["tisíc", "tisíc", "tisíc"], Agreement::Feminine),
    ] {
        let count = number / scale % 1000;
        if count > 1 {
            words.extend(below_thousand(count, gender));
        }
        if count > 0 {
            words.push(form(count, forms));
        }
    }
    words.extend(below_thousand(number % 1000, agreement));
    words.join(" ")
}

/// The case and gender of an ordinal: the nominative masculine (Karol štvrtý), the nominative
/// neuter (dvadsiate storočie) and the genitive (tretieho mája).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Case {
    Masculine,
    Neuter,
    Genitive,
}

const ORDINAL_UNITS: [&str; 20] = [
    "",
    "prv",
    "druh",
    "tret",
    "štvrt",
    "piat",
    "šiest",
    "siedm",
    "ôsm",
    "deviat",
    "desiat",
    "jedenást",
    "dvanást",
    "trinást",
    "štrnást",
    "pätnást",
    "šestnást",
    "sedemnást",
    "osemnást",
    "devätnást",
];

const ORDINAL_TENS: [&str; 10] = [
    "",
    "desiat",
    "dvadsiat",
    "tridsiat",
    "štyridsiat",
    "päťdesiat",
    "šesťdesiat",
    "sedemdesiat",
    "osemdesiat",
    "deväťdesiat",
];

/// An ordinal's stem with the ending of `case`: tretí is soft (tretie, tretieho); after a long
/// syllable the ending is short (piaty, piate, piateho), else long (prvý, prvé, prvého).
fn inflect(stem: &str, case: Case) -> String {
    let long = matches!(stem, "prv" | "druh" | "štvrt");
    let ending = match (stem, long, case) {
        ("tret", _, Case::Masculine) => "í",
        ("tret", _, Case::Neuter) => "ie",
        ("tret", _, Case::Genitive) => "ieho",
        (_, true, Case::Masculine) => "ý",
        (_, true, Case::Neuter) => "é",
        (_, true, Case::Genitive) => "ého",
        (_, false, Case::Masculine) => "y",
        (_, false, Case::Neuter) => "e",
        (_, false, Case::Genitive) => "eho",
    };
    format!("{stem}{ending}")
}

/// `number` as an ordinal in `case`, below a hundred (dvadsiateho prvého); `None` beyond.
pub(crate) fn ordinal(number: u64, case: Case) -> Option<String> {
    let index = |value: u64| usize::try_from(value).unwrap_or(0);
    match number {
        1..=19 => Some(inflect(ORDINAL_UNITS[index(number)], case)),
        20..=99 => {
            let tens = inflect(ORDINAL_TENS[index(number / 10)], case);
            Some(if number.is_multiple_of(10) {
                tens
            } else {
                format!("{tens} {}", inflect(ORDINAL_UNITS[index(number % 10)], case))
            })
        }
        _ => None,
    }
}

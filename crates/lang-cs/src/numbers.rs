//! Czech numbers in words: cardinals with their gender (jeden, jedna; dva, dvě) and their
//! hundreds, thousands and millions in the form the count asks for (dvě stě, pět set, dva
//! tisíce, pět tisíc), and ordinals in the cases dates and centuries need.

/// How a number agrees with what it counts: jeden and dva, jedna and dvě, or the counting forms
/// jedna and dva.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Agreement {
    Counting,
    Masculine,
    Feminine,
}

const UNITS: [&str; 20] = [
    "nula",
    "jedna",
    "dva",
    "tři",
    "čtyři",
    "pět",
    "šest",
    "sedm",
    "osm",
    "devět",
    "deset",
    "jedenáct",
    "dvanáct",
    "třináct",
    "čtrnáct",
    "patnáct",
    "šestnáct",
    "sedmnáct",
    "osmnáct",
    "devatenáct",
];

const TENS: [&str; 10] = [
    "",
    "deset",
    "dvacet",
    "třicet",
    "čtyřicet",
    "padesát",
    "šedesát",
    "sedmdesát",
    "osmdesát",
    "devadesát",
];

const HUNDREDS: [&str; 10] = [
    "",
    "sto",
    "dvě stě",
    "tři sta",
    "čtyři sta",
    "pět set",
    "šest set",
    "sedm set",
    "osm set",
    "devět set",
];

fn digit(value: u64) -> usize {
    usize::try_from(value % 10).unwrap_or(0)
}

/// Which of three forms goes with `count`: one, two to four (also at the end of a larger number:
/// dvacet dva), and the rest (pět korun, dvacet jedna korun).
pub(crate) fn form(count: u64, forms: [&str; 3]) -> &str {
    if count == 1 {
        forms[0]
    } else if (2..=4).contains(&(count % 10)) && !(12..=14).contains(&(count % 100)) {
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
        (1, Agreement::Masculine) => "jeden",
        (2, Agreement::Feminine) => "dvě",
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

/// `number` in words: 21 dvacet jedna, 2024 dva tisíce dvacet čtyři, 1000 tisíc.
pub(crate) fn cardinal(number: u64, agreement: Agreement) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut words: Vec<&str> = Vec::new();
    // Thousands and millions count as masculine, milliards as feminine (dvě miliardy).
    for (scale, forms, gender) in [
        (1_000_000_000, ["miliarda", "miliardy", "miliard"], Agreement::Feminine),
        (1_000_000, ["milion", "miliony", "milionů"], Agreement::Masculine),
        (1000, ["tisíc", "tisíce", "tisíc"], Agreement::Counting),
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

/// The case and gender of an ordinal: the nominative masculine (Karel čtvrtý), the nominative
/// neuter (dvacáté století) and the genitive (třetího května).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Case {
    Masculine,
    Neuter,
    Genitive,
}

const ORDINAL_UNITS: [&str; 20] = [
    "",
    "prvn",
    "druh",
    "třet",
    "čtvrt",
    "pát",
    "šest",
    "sedm",
    "osm",
    "devát",
    "desát",
    "jedenáct",
    "dvanáct",
    "třináct",
    "čtrnáct",
    "patnáct",
    "šestnáct",
    "sedmnáct",
    "osmnáct",
    "devatenáct",
];

const ORDINAL_TENS: [&str; 10] = [
    "",
    "desát",
    "dvacát",
    "třicát",
    "čtyřicát",
    "padesát",
    "šedesát",
    "sedmdesát",
    "osmdesát",
    "devadesát",
];

/// An ordinal's stem with the ending of `case`: soft stems (první, třetí) take -í and -ího, hard
/// ones -ý, -é and -ého.
fn inflect(stem: &str, case: Case) -> String {
    let soft = matches!(stem, "prvn" | "třet");
    let ending = match (soft, case) {
        (true, Case::Masculine | Case::Neuter) => "í",
        (true, Case::Genitive) => "ího",
        (false, Case::Masculine) => "ý",
        (false, Case::Neuter) => "é",
        (false, Case::Genitive) => "ého",
    };
    format!("{stem}{ending}")
}

/// `number` as an ordinal in `case`, below a hundred (dvacátého prvního); `None` beyond.
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

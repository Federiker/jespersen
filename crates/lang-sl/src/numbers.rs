//! Slovenian numbers in words: cardinals with their gender (en, ena; dva, dve), the units before
//! the tens (enaindvajset), and the ordinals of dates (tretjega).

/// How a number agrees with what it counts: en and dva, ena and dve, or the counting forms ena
/// and dva.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Agreement {
    Counting,
    Masculine,
    Feminine,
}

const UNITS: [&str; 20] = [
    "nič",
    "ena",
    "dva",
    "tri",
    "štiri",
    "pet",
    "šest",
    "sedem",
    "osem",
    "devet",
    "deset",
    "enajst",
    "dvanajst",
    "trinajst",
    "štirinajst",
    "petnajst",
    "šestnajst",
    "sedemnajst",
    "osemnajst",
    "devetnajst",
];

const TENS: [&str; 10] = [
    "",
    "deset",
    "dvajset",
    "trideset",
    "štirideset",
    "petdeset",
    "šestdeset",
    "sedemdeset",
    "osemdeset",
    "devetdeset",
];

/// The units of a compound, stressed as eSpeak NG reads them (enaindvajset ˈɛːnaindʋaːjsɛt), with
/// the accents of jespersen's respellings.
const COMPOUND_UNITS: [&str; 10] = [
    "", "êna", "dvá", "trí", "štíri", "pét", "šést", "sédem", "ósem", "dévet",
];

const HUNDREDS: [&str; 10] = [
    "",
    "sto",
    "dvesto",
    "tristo",
    "štiristo",
    "petsto",
    "šeststo",
    "sedemsto",
    "osemsto",
    "devetsto",
];

fn digit(value: u64) -> usize {
    usize::try_from(value % 10).unwrap_or(0)
}

/// Which of four forms goes with `count`, by its last two digits: one (and 101), two (the dual),
/// three and four, and the rest (en evro, dva evra, trije evri, pet evrov).
pub(crate) fn form(count: u64, forms: [&str; 4]) -> &str {
    match count % 100 {
        1 => forms[0],
        2 => forms[1],
        3 | 4 => forms[2],
        _ => forms[3],
    }
}

/// `number` below a hundred: the unit before the tens, joined with in (enaindvajset).
fn below_hundred(number: u64, agreement: Agreement) -> String {
    let unit = |value: u64| match (value, agreement) {
        (1, Agreement::Masculine) => "en",
        (2, Agreement::Feminine) => "dve",
        (value, _) => UNITS[usize::try_from(value).unwrap_or(0)],
    };
    match number {
        0..=19 => unit(number).to_owned(),
        _ if number.is_multiple_of(10) => TENS[digit(number / 10)].to_owned(),
        _ => format!("{}in{}", COMPOUND_UNITS[digit(number)], TENS[digit(number / 10)]),
    }
}

/// `number` in words: 21 enaindvajset, 2024 dva tisoč štiriindvajset, 1000 tisoč.
pub(crate) fn cardinal(number: u64, agreement: Agreement) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut words: Vec<String> = Vec::new();
    for (scale, forms) in [
        (1_000_000_000, ["milijarda", "milijardi", "milijarde", "milijard"]),
        (1_000_000, ["milijon", "milijona", "milijoni", "milijonov"]),
    ] {
        let count = number / scale % 1000;
        if count > 1 {
            words.push(below_thousand(count, Agreement::Masculine));
        }
        if count > 0 {
            words.push(form(count, forms).to_owned());
        }
    }
    let thousands = number / 1000 % 1000;
    if thousands > 1 {
        words.push(below_thousand(thousands, Agreement::Counting));
    }
    if thousands > 0 {
        words.push("tisoč".to_owned());
    }
    if !number.is_multiple_of(1000) {
        words.push(below_thousand(number % 1000, agreement));
    }
    words.join(" ")
}

fn below_thousand(number: u64, agreement: Agreement) -> String {
    let rest = number % 100;
    match (number / 100, rest) {
        (0, _) => below_hundred(rest, agreement),
        (hundreds, 0) => HUNDREDS[digit(hundreds)].to_owned(),
        (hundreds, _) => format!("{} {}", HUNDREDS[digit(hundreds)], below_hundred(rest, agreement)),
    }
}

const ORDINAL_UNITS: [&str; 20] = [
    "",
    "prv",
    "drug",
    "tretj",
    "četrt",
    "pet",
    "šest",
    "sedm",
    "osm",
    "devet",
    "deset",
    "enajst",
    "dvanajst",
    "trinajst",
    "štirinajst",
    "petnajst",
    "šestnajst",
    "sedemnajst",
    "osemnajst",
    "devetnajst",
];

/// The day of a month as an ordinal in the genitive: prvega, tretjega, enaindvajsetega.
pub(crate) fn day(number: u64) -> Option<String> {
    let index = |value: u64| usize::try_from(value).unwrap_or(0);
    match number {
        1..=19 => Some(format!("{}ega", ORDINAL_UNITS[index(number)])),
        20..=31 => Some(format!("{}ega", below_hundred(number, Agreement::Counting))),
        _ => None,
    }
}

//! Lithuanian numbers in words, with their genders (vienas, viena; du, dvi), grouped as eSpeak NG
//! says them: the tens joined to their unit (dvidešimt-vienas) and the hundreds to their count
//! (du-šimtai), tūkstantis and milijonas in the form their count asks for (du tūkstančiai,
//! dešimt tūkstančių). The parts joined by hyphens are said as one word.

const UNITS: [&str; 10] = [
    "nulis", "vienas", "du", "trys", "keturi", "penki", "šeši", "septyni", "aštuoni", "devyni",
];

/// The feminine units (viena, dvi, keturios).
const FEMININE: [&str; 10] = [
    "nulis",
    "viena",
    "dvi",
    "trys",
    "keturios",
    "penkios",
    "šešios",
    "septynios",
    "aštuonios",
    "devynios",
];

const TEENS: [&str; 10] = [
    "dešimt",
    "vienuolika",
    "dvylika",
    "trylika",
    "keturiolika",
    "penkiolika",
    "šešiolika",
    "septyniolika",
    "aštuoniolika",
    "devyniolika",
];

const TENS: [&str; 10] = [
    "",
    "",
    "dvidešimt",
    "trisdešimt",
    "keturiasdešimt",
    "penkiasdešimt",
    "šešiasdešimt",
    "septyniasdešimt",
    "aštuoniasdešimt",
    "devyniasdešimt",
];

fn index(value: u64) -> usize {
    usize::try_from(value % 10).unwrap_or(0)
}

/// Which form a noun takes after `count`: 0 for the nominative singular (vienas, dvidešimt
/// vienas), 1 for the nominative plural (du, penki), 2 for the genitive plural (dešimt,
/// vienuolika, dvidešimt).
pub(crate) fn form(count: u64) -> usize {
    match (count % 10, count % 100) {
        (_, 11..=19) | (0, _) => 2,
        (1, _) => 0,
        _ => 1,
    }
}

/// `number` below a hundred.
fn below_hundred(number: u64, feminine: bool) -> String {
    let units = if feminine { &FEMININE } else { &UNITS };
    match number {
        0..=9 => units[index(number)].to_owned(),
        10..=19 => TEENS[index(number)].to_owned(),
        _ if number.is_multiple_of(10) => TENS[index(number / 10)].to_owned(),
        _ => format!("{}-{}", TENS[index(number / 10)], units[index(number)]),
    }
}

fn below_thousand(number: u64, feminine: bool) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    match hundreds {
        0 => {}
        1 => parts.push("šimtas".to_owned()),
        _ => parts.push(format!("{}-šimtai", UNITS[index(hundreds)])),
    }
    if rest > 0 || hundreds == 0 {
        parts.push(below_hundred(rest, feminine));
    }
    parts.join(" ")
}

/// `number` in words, feminine when `feminine`: 21 dvidešimt-vienas, 2024 du tūkstančiai
/// dvidešimt-keturi, 1000 tūkstantis.
pub(crate) fn cardinal(number: u64, feminine: bool) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (size, forms) in [
        (1_000_000_000, ["milijardas", "milijardai", "milijardų"]),
        (1_000_000, ["milijonas", "milijonai", "milijonų"]),
        (1000, ["tūkstantis", "tūkstančiai", "tūkstančių"]),
    ] {
        let count = number / size % 1000;
        match count {
            0 => {}
            1 => parts.push(forms[0].to_owned()),
            _ => parts.push(format!("{} {}", below_thousand(count, false), forms[form(count)])),
        }
    }
    if !number.is_multiple_of(1000) || number == 0 {
        parts.push(below_thousand(number % 1000, feminine));
    }
    parts.join(" ")
}

/// The words of a number joined by hyphens, said as one (dvidešimt-vienas, du-šimtai).
pub(crate) fn is_number_word(word: &str) -> bool {
    UNITS.contains(&word) || FEMININE.contains(&word) || TENS[2..].contains(&word) || ["šimtai"].contains(&word)
}

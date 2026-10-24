//! Polish numbers in words: cardinals, with their thousands and millions in the form the count
//! asks for (dwa tysiące, pięć tysięcy), and ordinals in the cases dates and centuries need.

const UNITS: [&str; 20] = [
    "zero",
    "jeden",
    "dwa",
    "trzy",
    "cztery",
    "pięć",
    "sześć",
    "siedem",
    "osiem",
    "dziewięć",
    "dziesięć",
    "jedenaście",
    "dwanaście",
    "trzynaście",
    "czternaście",
    "piętnaście",
    "szesnaście",
    "siedemnaście",
    "osiemnaście",
    "dziewiętnaście",
];

const TENS: [&str; 10] = [
    "",
    "dziesięć",
    "dwadzieścia",
    "trzydzieści",
    "czterdzieści",
    "pięćdziesiąt",
    "sześćdziesiąt",
    "siedemdziesiąt",
    "osiemdziesiąt",
    "dziewięćdziesiąt",
];

const HUNDREDS: [&str; 10] = [
    "",
    "sto",
    "dwieście",
    "trzysta",
    "czterysta",
    "pięćset",
    "sześćset",
    "siedemset",
    "osiemset",
    "dziewięćset",
];

/// The forms of a scale word for one, for counts ending in 2 to 4 (but not 12 to 14), and for the
/// rest.
const SCALES: [(u64, [&str; 3]); 3] = [
    (1_000_000_000, ["miliard", "miliardy", "miliardów"]),
    (1_000_000, ["milion", "miliony", "milionów"]),
    (1000, ["tysiąc", "tysiące", "tysięcy"]),
];

/// Which of three forms goes with `count`: one, a count ending in 2 to 4 but not 12 to 14 (dwa
/// złote, dwadzieścia trzy złote), and the rest (pięć złotych, dwanaście złotych).
pub(crate) fn form(count: u64, forms: [&str; 3]) -> &str {
    if count == 1 {
        forms[0]
    } else if (2..=4).contains(&(count % 10)) && !(12..=14).contains(&(count % 100)) {
        forms[1]
    } else {
        forms[2]
    }
}

fn digit(value: u64) -> usize {
    usize::try_from(value % 10).unwrap_or(0)
}

/// `number` below a thousand, without the zero.
fn below_thousand(number: u64) -> Vec<&'static str> {
    let mut words = Vec::new();
    if number >= 100 {
        words.push(HUNDREDS[digit(number / 100)]);
    }
    match number % 100 {
        0 => {}
        rest @ 1..=19 => words.push(UNITS[digit(rest % 10) + if rest >= 10 { 10 } else { 0 }]),
        rest => {
            words.push(TENS[digit(rest / 10)]);
            if rest % 10 > 0 {
                words.push(UNITS[digit(rest)]);
            }
        }
    }
    words
}

/// `number` in words: 21 dwadzieścia jeden, 2024 dwa tysiące dwadzieścia cztery, 1000 tysiąc;
/// one alone is jedna when what it counts is feminine.
pub(crate) fn cardinal(number: u64, feminine: bool) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    if number == 1 && feminine {
        return "jedna".to_owned();
    }
    let mut words: Vec<&str> = Vec::new();
    for (scale, forms) in SCALES {
        let count = number / scale % 1000;
        if count > 1 {
            words.extend(below_thousand(count));
        }
        if count > 0 {
            words.push(form(count, forms));
        }
    }
    words.extend(below_thousand(number % 1000));
    words.join(" ")
}

/// The case of an ordinal: the nominative (dwudziesty wiek), the genitive (trzeciego maja), the
/// locative (w dwudziestym wieku), and the feminine nominative of the hours (dziesiąta).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Case {
    Nominative,
    Genitive,
    Locative,
    Feminine,
}

const ORDINAL_UNITS: [&str; 20] = [
    "",
    "pierwsz",
    "drug",
    "trzec",
    "czwart",
    "piąt",
    "szóst",
    "siódm",
    "ósm",
    "dziewiąt",
    "dziesiąt",
    "jedenast",
    "dwunast",
    "trzynast",
    "czternast",
    "piętnast",
    "szesnast",
    "siedemnast",
    "osiemnast",
    "dziewiętnast",
];

const ORDINAL_TENS: [&str; 10] = [
    "",
    "dziesiąt",
    "dwudziest",
    "trzydziest",
    "czterdziest",
    "pięćdziesiąt",
    "sześćdziesiąt",
    "siedemdziesiąt",
    "osiemdziesiąt",
    "dziewięćdziesiąt",
];

const ORDINAL_HUNDREDS: [&str; 10] = [
    "",
    "setn",
    "dwusetn",
    "trzysetn",
    "czterechsetn",
    "pięćsetn",
    "sześćsetn",
    "siedemsetn",
    "osiemsetn",
    "dziewięćsetn",
];

/// An ordinal's stem with the ending of `case`: drugi, drugiego, trzecia, piątym.
fn inflect(stem: &str, case: Case) -> String {
    let ending = match (stem.chars().last(), case) {
        (Some('c' | 'g'), Case::Nominative) => "i",
        (Some('c' | 'g'), Case::Genitive) => "iego",
        (Some('c' | 'g'), Case::Locative) => "im",
        (Some('c'), Case::Feminine) => "ia",
        (_, Case::Nominative) => "y",
        (_, Case::Genitive) => "ego",
        (_, Case::Locative) => "ym",
        (_, Case::Feminine) => "a",
    };
    format!("{stem}{ending}")
}

/// `number` as an ordinal in `case`: only its last two places take the ordinal form (dwa tysiące
/// dwudziestego czwartego), a round hundred or thousand all of it (dwutysięcznego); `None` from a
/// million on.
pub(crate) fn ordinal(number: u64, case: Case) -> Option<String> {
    if number == 0 || number >= 1_000_000 {
        return None;
    }
    let rest = number % 100;
    let mut words: Vec<String> = Vec::new();
    if rest == 0 {
        let hundreds = number / 100 % 10;
        let thousands = number / 1000;
        if hundreds == 0 {
            let stem = match thousands {
                1 => "tysięczn".to_owned(),
                2 => "dwutysięczn".to_owned(),
                3 => "trzytysięczn".to_owned(),
                4 => "czterotysięczn".to_owned(),
                _ => return None,
            };
            return Some(inflect(&stem, case));
        }
        if thousands > 0 {
            words.push(cardinal(thousands * 1000, false));
        }
        words.push(inflect(ORDINAL_HUNDREDS[digit(hundreds)], case));
        return Some(words.join(" "));
    }
    if number >= 100 {
        words.push(cardinal(number - rest, false));
    }
    if rest < 20 {
        words.push(inflect(
            ORDINAL_UNITS[digit(rest % 10) + if rest >= 10 { 10 } else { 0 }],
            case,
        ));
    } else {
        words.push(inflect(ORDINAL_TENS[digit(rest / 10)], case));
        if !rest.is_multiple_of(10) {
            words.push(inflect(ORDINAL_UNITS[digit(rest)], case));
        }
    }
    Some(words.join(" "))
}

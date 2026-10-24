//! Irish numbers in words, as they are counted: a haon, a dó, the teens with déag (trí-déag,
//! dó-dhéag), the tens with their unit after them (fiche-a-haon), céad and míle lenited or
//! eclipsed after their count (dhá-chéad, seacht-gcéad, trí-mhíle). The parts joined by hyphens
//! are said as one word, as eSpeak NG says them.

/// The words of a number joined by hyphens, said as one (fiche-a-haon, dhá-chéad).
pub(crate) fn is_number_word(word: &str) -> bool {
    const PARTS: [&str; 8] = ["a", "déag", "dhéag", "aon", "deich", "dhá", "ceithre", "ocht"];
    PARTS.contains(&word)
        || UNITS.iter().any(|unit| unit.strip_prefix("a ") == Some(word))
        || TENS.contains(&word)
        || COUNTS.contains(&word)
        || ["céad", "chéad", "gcéad", "míle", "mhíle", "milliún", "mhilliún"].contains(&word)
}

const UNITS: [&str; 10] = [
    "náid",
    "a haon",
    "a dó",
    "a trí",
    "a ceathair",
    "a cúig",
    "a sé",
    "a seacht",
    "a hocht",
    "a naoi",
];

/// The teens without their a, dó lenating déag (dó-dhéag).
const TEENS: [&str; 10] = [
    "",
    "aon-déag",
    "dó-dhéag",
    "trí-déag",
    "ceathair-déag",
    "cúig-déag",
    "sé-déag",
    "seacht-déag",
    "ocht-déag",
    "naoi-déag",
];

const TENS: [&str; 10] = [
    "", "a deich", "fiche", "tríocha", "daichead", "caoga", "seasca", "seachtó", "ochtó", "nócha",
];

/// The counts before céad and míle, without their a (dhá, trí, ceithre).
const COUNTS: [&str; 10] = ["", "", "dhá", "trí", "ceithre", "cúig", "sé", "seacht", "ocht", "naoi"];

fn index(value: u64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// `number` below a hundred.
fn below_hundred(number: u64) -> String {
    let (tens, unit) = (number / 10, number % 10);
    match (tens, unit) {
        (0, _) => UNITS[index(unit)].to_owned(),
        (1, 0) => TENS[1].to_owned(),
        (1, _) => TEENS[index(unit)].to_owned(),
        (_, 0) => TENS[index(tens)].to_owned(),
        _ => format!("{}-{}", TENS[index(tens)], UNITS[index(unit)].replace(' ', "-")),
    }
}

/// `noun` (céad, míle, milliún, billiún) after `count` below ten: lenited after dhá to sé,
/// eclipsed after seacht to naoi (dhá-chéad, seacht-gcéad), and without a count for one.
fn counted(count: u64, noun: &str) -> String {
    let (lenited, eclipsed) = match noun {
        "céad" => ("chéad", "gcéad"),
        "míle" => ("mhíle", "míle"),
        "milliún" => ("mhilliún", "milliún"),
        _ => ("bhilliún", "billiún"),
    };
    match count {
        1 => noun.to_owned(),
        2..=6 => format!("{}-{lenited}", COUNTS[index(count)]),
        _ => format!("{}-{eclipsed}", COUNTS[index(count)]),
    }
}

fn below_thousand(number: u64) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    match (hundreds, rest) {
        (0, _) => below_hundred(rest),
        (_, 0) => counted(hundreds, "céad"),
        _ => format!("{} {}", counted(hundreds, "céad"), below_hundred(rest)),
    }
}

/// `number` before a noun such as míle: a count below ten loses its a.
fn count_of(number: u64, noun: &str) -> String {
    if number < 10 {
        counted(number, noun)
    } else {
        format!("{} {noun}", below_thousand(number))
    }
}

/// `number` in words: 21 fiche a haon, 2024 dhá mhíle fiche a ceathair, 1000000 milliún.
pub(crate) fn cardinal(number: u64) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (size, noun) in [(1_000_000_000, "billiún"), (1_000_000, "milliún")] {
        let count = number / size % 1000;
        if count > 0 {
            parts.push(count_of(count, noun));
        }
    }
    let thousands = number / 1000 % 1000;
    if thousands > 0 {
        parts.push(count_of(thousands, "míle"));
    }
    if !number.is_multiple_of(1000) || number == 0 {
        parts.push(below_thousand(number % 1000));
    }
    parts.join(" ")
}

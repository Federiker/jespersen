//! Welsh numbers in words, in the decimal system and grouped as eSpeak NG says them: the tens
//! joined to their count and unit (dau-ddeg, dau-ddeg-pedwar), the hundreds to their count
//! (dau-gant), mil and miliwn as words of their own. The parts joined by hyphens are said as one
//! word.

const UNITS: [&str; 10] = [
    "dim", "un", "dau", "tri", "pedwar", "pump", "chwech", "saith", "wyth", "naw",
];

/// The counts before deg, cant and mil, mutated as they mutate them.
const COUNT_DEG: [&str; 10] = [
    "",
    "un",
    "dau-ddeg",
    "tri-deg",
    "pedwar-deg",
    "pum-deg",
    "chwe-deg",
    "saith-deg",
    "wyth-deg",
    "naw-deg",
];
const COUNT_CANT: [&str; 10] = [
    "",
    "cant",
    "dau-gant",
    "tri-chant",
    "pedwar-cant",
    "pum-cant",
    "chwe-chant",
    "saith-cant",
    "wyth-cant",
    "naw-cant",
];

fn index(value: u64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// `number` below a hundred.
fn below_hundred(number: u64) -> String {
    let (tens, unit) = (number / 10, number % 10);
    match (tens, unit) {
        (0, _) => UNITS[index(unit)].to_owned(),
        (1, 0) => "deg".to_owned(),
        (1, _) => format!("un-deg-{}", UNITS[index(unit)]),
        (_, 0) => COUNT_DEG[index(tens)].to_owned(),
        _ => format!("{}-{}", COUNT_DEG[index(tens)], UNITS[index(unit)]),
    }
}

fn below_thousand(number: u64) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    if hundreds > 0 {
        parts.push(COUNT_CANT[index(hundreds)].to_owned());
    }
    if rest > 0 || hundreds == 0 {
        parts.push(below_hundred(rest));
    }
    parts.join(" ")
}

/// `number` in words: 21 dau-ddeg-un, 2024 dau mil dau-ddeg-pedwar, 1000000 un miliwn.
pub(crate) fn cardinal(number: u64) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (size, name) in [(1_000_000_000, "biliwn"), (1_000_000, "miliwn")] {
        let count = number / size % 1000;
        if count > 0 {
            parts.push(format!("{} {name}", below_thousand(count)));
        }
    }
    let thousands = number / 1000 % 1000;
    match thousands {
        0 => {}
        1 => parts.push("mil".to_owned()),
        _ => parts.push(format!("{} mil", below_thousand(thousands))),
    }
    if !number.is_multiple_of(1000) || number == 0 {
        parts.push(below_thousand(number % 1000));
    }
    parts.join(" ")
}

/// The words of a number joined by hyphens, said as one (dau-ddeg, dau-gant).
pub(crate) fn is_number_word(word: &str) -> bool {
    UNITS.contains(&word) || ["deg", "ddeg", "pum", "chwe", "cant", "gant", "chant"].contains(&word)
}

/// The parts of a number after the first that keep only a lighter stress (dau-ddeg dˈaɨðˌeːɡ).
pub(crate) fn is_lighter(word: &str) -> bool {
    ["deg", "ddeg", "cant", "gant", "chant"].contains(&word)
}

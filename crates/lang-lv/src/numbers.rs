//! Latvian numbers in words: the units with their genders (viens, viena; divi, divas), the
//! teens in -padsmit and the tens in -desmit, simts and tūkstotis with their plurals (divi simti,
//! divi tūkstoši), and miljons.

const MASCULINE: [&str; 10] = [
    "nulle", "viens", "divi", "trīs", "četri", "pieci", "seši", "septiņi", "astoņi", "deviņi",
];

const FEMININE: [&str; 10] = [
    "nulle",
    "viena",
    "divas",
    "trīs",
    "četras",
    "piecas",
    "sešas",
    "septiņas",
    "astoņas",
    "deviņas",
];

/// The stems the teens and tens are made of (div-padsmit, div-desmit).
const STEMS: [&str; 10] = [
    "", "vien", "div", "trīs", "četr", "piec", "seš", "septiņ", "astoņ", "deviņ",
];

fn index(value: u64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// `number` below a hundred, `feminine` for what it counts.
fn below_hundred(number: u64, feminine: bool) -> String {
    let units = if feminine { &FEMININE } else { &MASCULINE };
    let (tens, unit) = (number / 10, number % 10);
    match (tens, unit) {
        (0, _) => units[index(unit)].to_owned(),
        (1, 0) => "desmit".to_owned(),
        (1, _) => format!("{}padsmit", STEMS[index(unit)]),
        (_, 0) => format!("{}desmit", STEMS[index(tens)]),
        _ => format!("{}desmit {}", STEMS[index(tens)], units[index(unit)]),
    }
}

/// A scale word after its count: simts, divi simti; tūkstotis, divi tūkstoši.
fn scale(count: u64, one: &str, many: &str) -> String {
    if count == 1 {
        one.to_owned()
    } else {
        format!("{} {many}", below_thousand(count, false))
    }
}

fn below_thousand(number: u64, feminine: bool) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    if hundreds > 0 {
        parts.push(scale(hundreds, "simts", "simti"));
    }
    if rest > 0 || hundreds == 0 {
        parts.push(below_hundred(rest, feminine));
    }
    parts.join(" ")
}

/// `number` in words, `feminine` for what it counts: 21 divdesmit viens, 2024 divi tūkstoši
/// divdesmit četri, 1000000 miljons.
pub(crate) fn cardinal(number: u64, feminine: bool) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (size, one, many) in [
        (1_000_000_000, "miljards", "miljardi"),
        (1_000_000, "miljons", "miljoni"),
    ] {
        let count = number / size % 1000;
        if count > 0 {
            parts.push(scale(count, one, many));
        }
    }
    let thousands = number / 1000 % 1000;
    if thousands > 0 {
        parts.push(scale(thousands, "tūkstotis", "tūkstoši"));
    }
    if !number.is_multiple_of(1000) || number == 0 {
        parts.push(below_thousand(number % 1000, feminine));
    }
    parts.join(" ")
}

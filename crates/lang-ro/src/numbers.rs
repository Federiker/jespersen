//! Romanian numbers in words: cardinals with their gender (unu, una; doi, două), the teens in
//! -sprezece, the tens with și (douăzeci și unu), and the hundreds, thousands and millions in the
//! form the count asks for (două sute, două mii, două milioane).

const UNITS: [&str; 10] = [
    "zero", "unu", "doi", "trei", "patru", "cinci", "șase", "șapte", "opt", "nouă",
];

const TEENS: [&str; 10] = [
    "zece",
    "unsprezece",
    "doisprezece",
    "treisprezece",
    "paisprezece",
    "cincisprezece",
    "șaisprezece",
    "șaptesprezece",
    "optsprezece",
    "nouăsprezece",
];

const TENS: [&str; 10] = [
    "",
    "",
    "douăzeci",
    "treizeci",
    "patruzeci",
    "cincizeci",
    "șaizeci",
    "șaptezeci",
    "optzeci",
    "nouăzeci",
];

fn index(value: u64) -> usize {
    usize::try_from(value % 10).unwrap_or(0)
}

/// `number` below a hundred, feminine when `feminine` (una, două, douăsprezece).
fn below_hundred(number: u64, feminine: bool) -> String {
    let unit = |value: u64| match (value, feminine) {
        (1, true) => "una",
        (2, true) => "două",
        (value, _) => UNITS[index(value)],
    };
    match number {
        0..=9 => unit(number).to_owned(),
        12 if feminine => "douăsprezece".to_owned(),
        10..=19 => TEENS[index(number)].to_owned(),
        _ if number.is_multiple_of(10) => TENS[index(number / 10)].to_owned(),
        _ => format!("{} și {}", TENS[index(number / 10)], unit(number % 10)),
    }
}

/// How many of a scale word: o sută, două sute; o mie, două mii; un milion, două milioane.
fn scale(count: u64, one: &str, many: &str) -> String {
    match count {
        1 => format!("{} {one}", if one == "milion" || one == "miliard" { "un" } else { "o" }),
        // From twenty on the count takes de (douăzeci de mii).
        _ if count % 100 >= 20 || count.is_multiple_of(100) => format!("{} de {many}", below_thousand(count, true)),
        _ => format!("{} {many}", below_thousand(count, true)),
    }
}

fn below_thousand(number: u64, feminine: bool) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    if hundreds > 0 {
        parts.push(scale(hundreds, "sută", "sute"));
    }
    if rest > 0 || hundreds == 0 {
        parts.push(below_hundred(rest, feminine));
    }
    parts.join(" ")
}

/// `number` in words, feminine when `feminine`: 21 douăzeci și unu, 2024 două mii douăzeci și
/// patru, 100 o sută, 1000 o mie.
pub(crate) fn cardinal(number: u64, feminine: bool) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (size, one, many) in [
        (1_000_000_000, "miliard", "miliarde"),
        (1_000_000, "milion", "milioane"),
        (1000, "mie", "mii"),
    ] {
        let count = number / size % 1000;
        if count > 0 {
            parts.push(scale(count, one, many));
        }
    }
    if !number.is_multiple_of(1000) || number == 0 {
        parts.push(below_thousand(number % 1000, feminine));
    }
    parts.join(" ")
}

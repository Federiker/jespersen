//! Norwegian numbers in words, grouped as eSpeak NG says them: the tens joined to their units
//! (tjue-en), ett hundre and ett tusen as two words but the other hundreds joined (to-hundre), og
//! before the last part (to tusen og tjue-fire), million and milliard as words of their own. The
//! parts joined by hyphens are said as one word.

const UNITS: [&str; 20] = [
    "null", "en", "to", "tre", "fire", "fem", "seks", "sju", "åtte", "ni", "ti", "elleve", "tolv", "tretten",
    "fjorten", "femten", "seksten", "sytten", "atten", "nitten",
];

const TENS: [&str; 10] = [
    "", "", "tjue", "tretti", "førti", "femti", "seksti", "sytti", "åtti", "nitti",
];

fn index(value: u64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// `number` below a hundred.
fn below_hundred(number: u64) -> String {
    match number {
        0..=19 => UNITS[index(number)].to_owned(),
        _ if number.is_multiple_of(10) => TENS[index(number / 10)].to_owned(),
        _ => format!("{}-{}", TENS[index(number / 10)], UNITS[index(number % 10)]),
    }
}

/// The hundreds of `number` (below a thousand), without what follows them.
fn hundreds(count: u64) -> String {
    if count == 1 {
        "ett hundre".to_owned()
    } else {
        format!("{}-hundre", UNITS[index(count)])
    }
}

/// `number` in words: 21 tjue-en, 2024 to tusen og tjue-fire, 1000000 en million.
pub(crate) fn cardinal(number: u64) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (size, one, many) in [
        (1_000_000_000, "milliard", "milliarder"),
        (1_000_000, "million", "millioner"),
    ] {
        let count = number / size % 1000;
        match count {
            0 => {}
            1 => parts.push(format!("en {one}")),
            _ => parts.push(format!("{} {many}", cardinal(count))),
        }
    }
    let thousands = number / 1000 % 1000;
    match thousands {
        0 => {}
        1 => parts.push("ett tusen".to_owned()),
        _ => parts.push(format!("{} tusen", cardinal(thousands))),
    }
    let (hundred, rest) = (number % 1000 / 100, number % 100);
    if hundred > 0 {
        parts.push(hundreds(hundred));
    }
    if rest > 0 || number == 0 {
        if !parts.is_empty() {
            parts.push("og".to_owned());
        }
        parts.push(below_hundred(rest));
    }
    parts.join(" ")
}

/// The words of a number joined by hyphens, said as one (tjue-en, to-hundre).
pub(crate) fn is_number_word(word: &str) -> bool {
    UNITS.contains(&word) || TENS[2..].contains(&word) || word == "hundre"
}

//! Albanian numbers in words, grouped as eSpeak NG says them: the tens joined to their unit by e
//! (njëzet-e-një) and the teens to -mbëdhjetë (pesë-mbëdhjetë), each said as one word; the
//! hundreds written with their count (dyqind), mijë, milion and miliard as words of their own, and
//! e between the groups (dy mijë e njëzet-e-katër).

const UNITS: [&str; 10] = [
    "zero", "një", "dy", "tre", "katër", "pesë", "gjashtë", "shtatë", "tetë", "nëntë",
];

const TENS: [&str; 10] = [
    "",
    "dhjetë",
    "njëzet",
    "tridhjetë",
    "dyzet",
    "pesëdhjetë",
    "gjashtëdhjetë",
    "shtatëdhjetë",
    "tetëdhjetë",
    "nëntëdhjetë",
];

fn index(value: u64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// `number` below a hundred.
fn below_hundred(number: u64) -> String {
    let (tens, unit) = (number / 10, number % 10);
    match (tens, unit) {
        (0, _) => UNITS[index(unit)].to_owned(),
        (_, 0) => TENS[index(tens)].to_owned(),
        (1, _) => format!("{}-mbëdhjetë", UNITS[index(unit)]),
        _ => format!("{}-e-{}", TENS[index(tens)], UNITS[index(unit)]),
    }
}

fn below_thousand(number: u64) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    if hundreds > 0 {
        parts.push(format!("{}qind", UNITS[index(hundreds)]));
    }
    if rest > 0 || hundreds == 0 {
        parts.push(below_hundred(rest));
    }
    parts.join(" e ")
}

/// `number` in words: 21 njëzet-e-një, 2024 dy mijë e njëzet-e-katër, 1000000 një milion.
pub(crate) fn cardinal(number: u64) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (size, one, many) in [(1_000_000_000, "miliard", "miliardë"), (1_000_000, "milion", "milionë")] {
        let count = number / size % 1000;
        match count {
            0 => {}
            1 => parts.push(format!("një {one}")),
            _ => parts.push(format!("{} {many}", below_thousand(count))),
        }
    }
    let thousands = number / 1000 % 1000;
    if thousands > 0 {
        parts.push(format!("{} mijë", below_thousand(thousands)));
    }
    if !number.is_multiple_of(1000) || number == 0 {
        parts.push(below_thousand(number % 1000));
    }
    parts.join(" e ")
}

/// The words of a number joined by hyphens, said as one (njëzet-e-një, pesë-mbëdhjetë).
pub(crate) fn is_number_word(word: &str) -> bool {
    UNITS.contains(&word) || TENS.contains(&word) || ["e", "mbëdhjetë"].contains(&word)
}

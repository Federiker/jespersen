//! Estonian numbers in words: the teens in -teist (kaksteist), the tens in -kümmend
//! (kakskümmend), the hundreds in -sada (kakssada), and tuhat, miljon and miljard as words of
//! their own, in the partitive after a count above one (kaks miljonit).

const UNITS: [&str; 10] = [
    "null", "üks", "kaks", "kolm", "neli", "viis", "kuus", "seitse", "kaheksa", "üheksa",
];

fn index(value: u64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// `number` below a hundred.
fn below_hundred(number: u64) -> String {
    let (tens, unit) = (number / 10, number % 10);
    match (tens, unit) {
        (0, _) => UNITS[index(unit)].to_owned(),
        (1, 0) => "kümme".to_owned(),
        (1, _) => format!("{}teist", UNITS[index(unit)]),
        (_, 0) => format!("{}kümmend", UNITS[index(tens)]),
        _ => format!("{}kümmend {}", UNITS[index(tens)], UNITS[index(unit)]),
    }
}

fn below_thousand(number: u64) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    match hundreds {
        0 => {}
        1 => parts.push("sada".to_owned()),
        _ => parts.push(format!("{}sada", UNITS[index(hundreds)])),
    }
    if rest > 0 || hundreds == 0 {
        parts.push(below_hundred(rest));
    }
    parts.join(" ")
}

/// `number` in words: 21 kakskümmend üks, 2024 kaks tuhat kakskümmend neli, 1000000 miljon.
pub(crate) fn cardinal(number: u64) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (size, one, many) in [
        (1_000_000_000, "miljard", "miljardit"),
        (1_000_000, "miljon", "miljonit"),
        (1000, "tuhat", "tuhat"),
    ] {
        match number / size % 1000 {
            0 => {}
            1 => parts.push(one.to_owned()),
            count => parts.push(format!("{} {many}", below_thousand(count))),
        }
    }
    if !number.is_multiple_of(1000) || number == 0 {
        parts.push(below_thousand(number % 1000));
    }
    parts.join(" ")
}

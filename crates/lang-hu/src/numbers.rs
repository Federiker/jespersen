//! Hungarian numbers in words, grouped as eSpeak NG says them: kettő on its own and két before
//! what it counts (kétszáz), the teens and twenties in tizen- and huszon- (tizenkettő,
//! huszonegy), the hundreds joined to their count (háromszáz), and ezer, millió and milliárd as
//! words of their own (kétezer huszonnégy).

const UNITS: [&str; 10] = [
    "nulla", "egy", "kettő", "három", "négy", "öt", "hat", "hét", "nyolc", "kilenc",
];

const TENS: [&str; 10] = [
    "",
    "tíz",
    "húsz",
    "harminc",
    "negyven",
    "ötven",
    "hatvan",
    "hetven",
    "nyolcvan",
    "kilencven",
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
        (1, _) => format!("tizen{}", UNITS[index(unit)]),
        (2, _) => format!("huszon{}", UNITS[index(unit)]),
        _ => format!("{}{}", TENS[index(tens)], UNITS[index(unit)]),
    }
}

/// The count before a hundred, a thousand or a million: két for two, nothing for one.
fn count(number: u64) -> String {
    match number {
        1 => String::new(),
        2 => "két".to_owned(),
        _ => below_thousand(number),
    }
}

fn below_thousand(number: u64) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    if hundreds > 0 {
        parts.push(format!("{}száz", count(hundreds)));
    }
    if rest > 0 || hundreds == 0 {
        parts.push(below_hundred(rest));
    }
    parts.join(" ")
}

/// `number` in words: 21 huszonegy, 2024 kétezer huszonnégy, 1000000 egymillió.
pub(crate) fn cardinal(number: u64) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (size, name) in [(1_000_000_000, "milliárd"), (1_000_000, "millió")] {
        let groups = number / size % 1000;
        if groups > 0 {
            let count = if groups == 1 { "egy".to_owned() } else { count(groups) };
            parts.push(format!("{count}{name}"));
        }
    }
    let thousands = number / 1000 % 1000;
    if thousands > 0 {
        parts.push(format!("{}ezer", count(thousands)));
    }
    if !number.is_multiple_of(1000) || number == 0 {
        parts.push(below_thousand(number % 1000));
    }
    parts.join(" ")
}

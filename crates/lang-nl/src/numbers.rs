//! Dutch numbers in words, grouped as eSpeak NG says them: the units joined to the tens by en
//! (een-en-twintig, twee-ën-twintig), the hundreds to their count (twee-honderd), duizend,
//! miljoen and miljard as words of their own. The parts joined by hyphens are said as one word.

const UNITS: [&str; 20] = [
    "nul",
    "een",
    "twee",
    "drie",
    "vier",
    "vijf",
    "zes",
    "zeven",
    "acht",
    "negen",
    "tien",
    "elf",
    "twaalf",
    "dertien",
    "veertien",
    "vijftien",
    "zestien",
    "zeventien",
    "achttien",
    "negentien",
];

const TENS: [&str; 10] = [
    "", "", "twintig", "dertig", "veertig", "vijftig", "zestig", "zeventig", "tachtig", "negentig",
];

fn index(value: u64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// `number` below a hundred.
fn below_hundred(number: u64) -> String {
    match number {
        0..=19 => UNITS[index(number)].to_owned(),
        _ if number.is_multiple_of(10) => TENS[index(number / 10)].to_owned(),
        _ => {
            let unit = UNITS[index(number % 10)];
            // en is written ën after a vowel (tweeëntwintig).
            let and = if unit.ends_with('e') { "ën" } else { "en" };
            format!("{unit}-{and}-{}", TENS[index(number / 10)])
        }
    }
}

fn below_thousand(number: u64) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    if hundreds > 0 {
        let count = if hundreds == 1 {
            String::new()
        } else {
            format!("{}-", UNITS[index(hundreds)])
        };
        parts.push(format!("{count}honderd"));
    }
    if rest > 0 || hundreds == 0 {
        parts.push(below_hundred(rest));
    }
    parts.join(" ")
}

/// `number` in words: 21 een-en-twintig, 2024 twee duizend vier-en-twintig, 1000000 een miljoen.
pub(crate) fn cardinal(number: u64) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (size, one, many) in [
        (1_000_000_000_000, "biljoen", "biljoen"),
        (1_000_000_000, "miljard", "miljard"),
        (1_000_000, "miljoen", "miljoen"),
    ] {
        let count = number / size % 1000;
        match count {
            0 => {}
            1 => parts.push(format!("een {one}")),
            _ => parts.push(format!("{} {many}", below_thousand(count))),
        }
    }
    let thousands = number / 1000 % 1000;
    match thousands {
        0 => {}
        1 => parts.push("duizend".to_owned()),
        _ => parts.push(format!("{} duizend", below_thousand(thousands))),
    }
    if !number.is_multiple_of(1000) || number == 0 {
        parts.push(below_thousand(number % 1000));
    }
    parts.join(" ")
}

/// The words of a number joined by hyphens, said as one (een-en-twintig, twee-honderd).
pub(crate) fn is_number_word(word: &str) -> bool {
    UNITS.contains(&word) || TENS[2..].contains(&word) || ["en", "ën", "honderd"].contains(&word)
}

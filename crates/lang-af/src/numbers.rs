//! Afrikaans numbers in words, grouped as eSpeak NG says them: the units joined to the tens by
//! en (een-en-twintig), the hundreds to their count (twee-honderd) and en before what follows
//! them (een-honderd-en een), duisend, miljoen and miljard as words of their own. The parts
//! joined by hyphens are said as one word.

const UNITS: [&str; 20] = [
    "nul",
    "een",
    "twee",
    "drie",
    "vier",
    "vyf",
    "ses",
    "sewe",
    "agt",
    "nege",
    "tien",
    "elf",
    "twaalf",
    "dertien",
    "veertien",
    "vyftien",
    "sestien",
    "sewentien",
    "agtien",
    "negentien",
];

const TENS: [&str; 10] = [
    "", "", "twintig", "dertig", "veertig", "vyftig", "sestig", "sewentig", "tagtig", "negentig",
];

fn index(value: u64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// `number` below a hundred.
fn below_hundred(number: u64) -> String {
    match number {
        0..=19 => UNITS[index(number)].to_owned(),
        _ if number.is_multiple_of(10) => TENS[index(number / 10)].to_owned(),
        _ => format!("{}-en-{}", UNITS[index(number % 10)], TENS[index(number / 10)]),
    }
}

fn below_thousand(number: u64) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    match (hundreds, rest) {
        (0, _) => below_hundred(rest),
        (_, 0) => format!("{}-honderd", UNITS[index(hundreds)]),
        _ => format!("{}-honderd-en {}", UNITS[index(hundreds)], below_hundred(rest)),
    }
}

/// `number` in words: 21 een-en-twintig, 2024 twee duisend vier-en-twintig, 1000000 een miljoen.
pub(crate) fn cardinal(number: u64) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (size, name) in [
        (1_000_000_000_000, "biljoen"),
        (1_000_000_000, "miljard"),
        (1_000_000, "miljoen"),
    ] {
        let count = number / size % 1000;
        if count > 0 {
            parts.push(format!("{} {name}", below_thousand(count)));
        }
    }
    let thousands = number / 1000 % 1000;
    if thousands > 0 {
        parts.push(format!("{} duisend", below_thousand(thousands)));
    }
    if !number.is_multiple_of(1000) || number == 0 {
        parts.push(below_thousand(number % 1000));
    }
    parts.join(" ")
}

/// The words of a number joined by hyphens, said as one (een-en-twintig, twee-honderd).
pub(crate) fn is_number_word(word: &str) -> bool {
    UNITS.contains(&word) || TENS[2..].contains(&word) || ["en", "honderd"].contains(&word)
}

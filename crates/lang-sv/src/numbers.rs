//! Swedish numbers in words, grouped as eSpeak NG says them: the tens joined to their units
//! (tjugo-ett), the hundreds to their count (ett-hundra), tusen, miljon and miljard as words of
//! their own (två tusen tjugo-fyra). The parts joined by hyphens are said as one word.

const UNITS: [&str; 20] = [
    "noll", "ett", "två", "tre", "fyra", "fem", "sex", "sju", "åtta", "nio", "tio", "elva", "tolv", "tretton",
    "fjorton", "femton", "sexton", "sjutton", "arton", "nitton",
];

const TENS: [&str; 10] = [
    "", "", "tjugo", "trettio", "fyrtio", "femtio", "sextio", "sjuttio", "åttio", "nittio",
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

fn below_thousand(number: u64) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    if hundreds > 0 {
        parts.push(format!("{}-hundra", UNITS[index(hundreds)]));
    }
    if rest > 0 || hundreds == 0 {
        parts.push(below_hundred(rest));
    }
    parts.join(" ")
}

/// `number` in words: 21 tjugo-ett, 2024 två tusen tjugo-fyra, 1000000 en miljon.
pub(crate) fn cardinal(number: u64) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (size, one, many) in [
        (1_000_000_000, "miljard", "miljarder"),
        (1_000_000, "miljon", "miljoner"),
    ] {
        let count = number / size % 1000;
        match count {
            0 => {}
            1 => parts.push(format!("en {one}")),
            _ => parts.push(format!("{} {many}", below_thousand(count))),
        }
    }
    let thousands = number / 1000 % 1000;
    if thousands > 0 {
        parts.push(format!("{} tusen", below_thousand(thousands)));
    }
    if !number.is_multiple_of(1000) || number == 0 {
        parts.push(below_thousand(number % 1000));
    }
    parts.join(" ")
}

/// Whether `word` is one of the tens, which are lighter before their unit (tjugo-ett ɕˌʉɡɔˈɛt).
pub(crate) fn is_tens(word: &str) -> bool {
    TENS[2..].contains(&word)
}

/// The words of a number joined by hyphens, said as one (tjugo-ett, ett-hundra).
pub(crate) fn is_number_word(word: &str) -> bool {
    UNITS.contains(&word) || TENS[2..].contains(&word) || word == "hundra"
}

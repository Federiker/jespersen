//! Macedonian numbers in words: cardinals with their gender (еден, една; два, две), и before the
//! last part (дваесет и пет, сто и пет, илјада деветсто седумдесет и пет), thousands and millions
//! in the singular or the plural (илјада, две илјади), and the ordinals of dates (трети).

/// How a number agrees with what it counts: еден and два, or една and две.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Agreement {
    Masculine,
    Feminine,
}

const UNITS: [&str; 20] = [
    "нула",
    "еден",
    "два",
    "три",
    "четири",
    "пет",
    "шест",
    "седум",
    "осум",
    "девет",
    "десет",
    "единаесет",
    "дванаесет",
    "тринаесет",
    "четиринаесет",
    "петнаесет",
    "шеснаесет",
    "седумнаесет",
    "осумнаесет",
    "деветнаесет",
];

const TENS: [&str; 10] = [
    "",
    "десет",
    "дваесет",
    "триесет",
    "четириесет",
    "педесет",
    "шеесет",
    "седумдесет",
    "осумдесет",
    "деведесет",
];

const HUNDREDS: [&str; 10] = [
    "",
    "сто",
    "двесте",
    "триста",
    "четиристотини",
    "петстотини",
    "шестотини",
    "седумстотини",
    "осумстотини",
    "деветстотини",
];

fn index(value: u64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// The parts of `number` below a thousand (сто, дваесет, пет), the last agreeing as `agreement`
/// says.
fn parts_below_thousand(number: u64, agreement: Agreement) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    if number >= 100 {
        parts.push(HUNDREDS[index(number / 100)].to_owned());
    }
    let unit = |value: u64| match (value, agreement) {
        (1, Agreement::Feminine) => "една",
        (2, Agreement::Feminine) => "две",
        (value, _) => UNITS[index(value)],
    };
    match number % 100 {
        0 => {}
        rest @ 1..=19 => parts.push(unit(rest).to_owned()),
        rest => {
            parts.push(TENS[index(rest / 10)].to_owned());
            if !rest.is_multiple_of(10) {
                parts.push(unit(rest % 10).to_owned());
            }
        }
    }
    parts
}

/// `number` in words: 25 дваесет и пет, 105 сто и пет, 2024 две илјади дваесет и четири.
pub(crate) fn cardinal(number: u64, agreement: Agreement) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    // Thousands and milliards count as feminine (две илјади), millions as masculine.
    for (scale, one, many, gender) in [
        (1_000_000_000, "милијарда", "милијарди", Agreement::Feminine),
        (1_000_000, "милион", "милиони", Agreement::Masculine),
        (1000, "илјада", "илјади", Agreement::Feminine),
    ] {
        match number / scale % 1000 {
            0 => {}
            1 => parts.push(one.to_owned()),
            count => parts.push(format!("{} {many}", cardinal(count, gender))),
        }
    }
    parts.extend(parts_below_thousand(number % 1000, agreement));
    // и comes before the last part.
    if parts.len() > 1
        && let Some(last) = parts.pop()
    {
        parts.push(format!("и {last}"));
    }
    parts.join(" ")
}

const ORDINALS: [&str; 20] = [
    "",
    "први",
    "втори",
    "трети",
    "четврти",
    "петти",
    "шести",
    "седми",
    "осми",
    "деветти",
    "десетти",
    "единаесетти",
    "дванаесетти",
    "тринаесетти",
    "четиринаесетти",
    "петнаесетти",
    "шеснаесетти",
    "седумнаесетти",
    "осумнаесетти",
    "деветнаесетти",
];

/// The day of a month as an ordinal: први, трети, дваесет и први, триесетти.
pub(crate) fn day(number: u64) -> Option<String> {
    match number {
        1..=19 => Some(ORDINALS[index(number)].to_owned()),
        20 | 30 => Some(format!("{}ти", TENS[index(number / 10)])),
        21..=31 => Some(format!(
            "{} и {}",
            TENS[index(number / 10)],
            ORDINALS[index(number % 10)]
        )),
        _ => None,
    }
}

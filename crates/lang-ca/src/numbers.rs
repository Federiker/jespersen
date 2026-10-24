//! Catalan numbers in words: u, un and una, dos and dues; the tens joined to their units with a
//! hyphen (trenta-dos) and the twenties with -i- (vint-i-u); the hundreds in -cents, feminine in
//! -centes (dos-cents, dues-centes); mil, and milió with its plural.

const UNITS: [&str; 20] = [
    "zero", "u", "dos", "tres", "quatre", "cinc", "sis", "set", "vuit", "nou", "deu", "onze", "dotze", "tretze",
    "catorze", "quinze", "setze", "disset", "divuit", "dinou",
];

const TENS: [&str; 10] = [
    "",
    "",
    "vint",
    "trenta",
    "quaranta",
    "cinquanta",
    "seixanta",
    "setanta",
    "vuitanta",
    "noranta",
];

/// How a number agrees: counting on its own (u), before a masculine noun (un) or a feminine one
/// (una, dues).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Form {
    Alone,
    Masculine,
    Feminine,
}

fn index(value: u64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// `number` below a hundred.
fn below_hundred(number: u64, form: Form) -> String {
    let unit = |value: u64| match (value, form) {
        (1, Form::Masculine) => "un",
        (1, Form::Feminine) => "una",
        (2, Form::Feminine) => "dues",
        (value, _) => UNITS[index(value)],
    };
    match number {
        0..=2 => unit(number).to_owned(),
        3..=19 => UNITS[index(number)].to_owned(),
        _ if number.is_multiple_of(10) => TENS[index(number / 10)].to_owned(),
        21..=29 => format!("vint-i-{}", unit(number % 10)),
        _ => format!("{}-{}", TENS[index(number / 10)], unit(number % 10)),
    }
}

fn below_thousand(number: u64, form: Form) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    match hundreds {
        0 => {}
        1 => parts.push("cent".to_owned()),
        _ => {
            let count = below_hundred(hundreds, if form == Form::Feminine { form } else { Form::Alone });
            let cents = if form == Form::Feminine { "centes" } else { "cents" };
            parts.push(format!("{count}-{cents}"));
        }
    }
    if rest > 0 || hundreds == 0 {
        parts.push(below_hundred(rest, form));
    }
    parts.join(" ")
}

/// `number` in words: 21 vint-i-u, 2024 dos mil vint-i-quatre, 1000 mil, 3000000 tres milions.
pub(crate) fn cardinal(number: u64, form: Form) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (size, one, many) in [(1_000_000_000_000, "bilió", "bilions"), (1_000_000, "milió", "milions")] {
        let count = number / size % 1_000_000;
        match count {
            0 => {}
            1 => parts.push(format!("un {one}")),
            _ => parts.push(format!("{} {many}", cardinal(count, Form::Masculine))),
        }
    }
    let thousands = number / 1000 % 1000;
    match thousands {
        0 => {}
        1 => parts.push("mil".to_owned()),
        _ => {
            let count_form = if form == Form::Feminine { form } else { Form::Masculine };
            parts.push(format!("{} mil", below_thousand(thousands, count_form)));
        }
    }
    if !number.is_multiple_of(1000) || number == 0 {
        parts.push(below_thousand(number % 1000, form));
    }
    parts.join(" ")
}

/// Whether `word` is one of the parts of a number written with hyphens (vint, i, u, cents).
pub(crate) fn is_number_word(word: &str) -> bool {
    UNITS.contains(&word) || TENS[2..].contains(&word) || ["i", "un", "una", "dues", "cents", "centes"].contains(&word)
}

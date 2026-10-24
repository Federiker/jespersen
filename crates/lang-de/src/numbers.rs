//! German numbers in words, grouped as eSpeak NG says them: the units before und apart from the
//! tens (ein und-zwanzig), the hundreds joined to their count (zwei-hundert), tausend, Million and
//! Milliarde as words of their own (zwei tausend, zwei Millionen). The parts joined by hyphens are
//! said as one word.

const UNITS: [&str; 20] = [
    "null",
    "eins",
    "zwei",
    "drei",
    "vier",
    "fünf",
    "sechs",
    "sieben",
    "acht",
    "neun",
    "zehn",
    "elf",
    "zwölf",
    "dreizehn",
    "vierzehn",
    "fünfzehn",
    "sechzehn",
    "siebzehn",
    "achtzehn",
    "neunzehn",
];

const TENS: [&str; 10] = [
    "", "", "zwanzig", "dreißig", "vierzig", "fünfzig", "sechzig", "siebzig", "achtzig", "neunzig",
];

fn index(value: u64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// How a number agrees: counting on its own (eins), before a noun (ein), or before a feminine
/// noun (eine).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Form {
    Alone,
    Before,
    Feminine,
}

/// `number` below a hundred.
fn below_hundred(number: u64, form: Form) -> String {
    let one = match form {
        Form::Alone => "eins",
        Form::Before => "ein",
        Form::Feminine => "eine",
    };
    match number {
        1 => one.to_owned(),
        0..=19 => UNITS[index(number)].to_owned(),
        _ if number.is_multiple_of(10) => TENS[index(number / 10)].to_owned(),
        _ => {
            // ein- keeps its stress before und (a hyphen makes it a number's part, not the article).
            let unit = if number % 10 == 1 {
                "ein-"
            } else {
                UNITS[index(number % 10)]
            };
            format!("{unit} und-{}", TENS[index(number / 10)])
        }
    }
}

fn below_thousand(number: u64, form: Form) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    if hundreds > 0 {
        let count = if hundreds == 1 { "ein" } else { UNITS[index(hundreds)] };
        parts.push(format!("{count}-hundert"));
    }
    if rest > 0 || hundreds == 0 {
        parts.push(below_hundred(rest, form));
    }
    parts.join(" ")
}

/// `number` in words: 21 ein und-zwanzig, 2024 zwei tausend vier und-zwanzig, 1000000 eine
/// Million.
pub(crate) fn cardinal(number: u64, form: Form) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (size, one, many) in [
        (1_000_000_000_000, "Billion", "Billionen"),
        (1_000_000_000, "Milliarde", "Milliarden"),
        (1_000_000, "Million", "Millionen"),
    ] {
        let count = number / size % 1000;
        match count {
            0 => {}
            1 => parts.push(format!("eine {one}")),
            _ => parts.push(format!("{} {many}", below_thousand(count, Form::Feminine))),
        }
    }
    let thousands = number / 1000 % 1000;
    if thousands > 0 {
        let count = if thousands == 1 {
            "ein-".to_owned()
        } else {
            below_thousand(thousands, Form::Before)
        };
        parts.push(format!("{count} tausend"));
    }
    if !number.is_multiple_of(1000) || number == 0 {
        parts.push(below_thousand(number % 1000, form));
    }
    parts.join(" ")
}

/// The words of a number joined by hyphens, said as one (und-zwanzig, drei-hundert).
pub(crate) fn is_number_word(word: &str) -> bool {
    UNITS.contains(&word) || TENS[2..].contains(&word) || ["ein", "eine", "und", "hundert"].contains(&word)
}

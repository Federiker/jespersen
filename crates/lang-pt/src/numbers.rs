//! Portuguese numbers in words: cardinals with their gender (um, uma; duzentos, duzentas) and e
//! between their parts (trezentos e quarenta e cinco), and ordinals.

use crate::Variety;

/// One to nineteen in Portugal; Brazil says dezesseis, dezessete, dezenove and quatorze.
fn unit(number: u64, feminine: bool, variety: Variety) -> &'static str {
    let brazilian = variety == Variety::Brazilian;
    match number {
        1 if feminine => "uma",
        1 => "um",
        2 if feminine => "duas",
        2 => "dois",
        3 => "três",
        4 => "quatro",
        5 => "cinco",
        6 => "seis",
        7 => "sete",
        8 => "oito",
        9 => "nove",
        10 => "dez",
        11 => "onze",
        12 => "doze",
        13 => "treze",
        14 if brazilian => "quatorze",
        14 => "catorze",
        15 => "quinze",
        16 if brazilian => "dezesseis",
        16 => "dezasseis",
        17 if brazilian => "dezessete",
        17 => "dezassete",
        18 => "dezoito",
        19 if brazilian => "dezenove",
        19 => "dezanove",
        _ => "zero",
    }
}

const TENS: [&str; 10] = [
    "",
    "dez",
    "vinte",
    "trinta",
    "quarenta",
    "cinquenta",
    "sessenta",
    "setenta",
    "oitenta",
    "noventa",
];

/// The hundreds, whose names agree with what is counted from two hundred on (duzentas casas).
fn hundreds(number: u64, feminine: bool) -> String {
    let stem = match number {
        1 => return "cento".to_owned(),
        2 => "duzent",
        3 => "trezent",
        4 => "quatrocent",
        5 => "quinhent",
        6 => "seiscent",
        7 => "setecent",
        8 => "oitocent",
        _ => "novecent",
    };
    format!("{stem}{}", if feminine { "as" } else { "os" })
}

/// `number` below a thousand: 100 cem, 101 cento e um, 345 trezentos e quarenta e cinco.
fn below_thousand(number: u64, feminine: bool, variety: Variety) -> Vec<String> {
    if number == 100 {
        return vec!["cem".to_owned()];
    }
    let mut parts: Vec<String> = Vec::new();
    if number >= 100 {
        parts.push(hundreds(number / 100, feminine));
    }
    match number % 100 {
        0 => {}
        rest @ 1..=19 => parts.push(unit(rest, feminine, variety).to_owned()),
        rest => {
            let tens = TENS[usize::try_from(rest / 10).unwrap_or(0)];
            if rest % 10 == 0 {
                parts.push(tens.to_owned());
            } else {
                parts.push(format!("{tens} e {}", unit(rest % 10, feminine, variety)));
            }
        }
    }
    parts
}

/// `number` in words, agreeing with a feminine noun when `feminine`: 21 vinte e um, 2024 dois mil e
/// vinte e quatro, 1999 mil novecentos e noventa e nove; a thousand million is mil milhões in
/// Portugal and um bilhão in Brazil.
pub(crate) fn cardinal(number: u64, feminine: bool, variety: Variety) -> String {
    if number == 0 {
        return "zero".to_owned();
    }
    let billion = match variety {
        Variety::European => None,
        Variety::Brazilian => Some((1_000_000_000, "bilhão", "bilhões")),
    };
    let mut groups: Vec<String> = Vec::new();
    let mut rest = number;
    for (scale, one, many) in billion.into_iter().chain([(1_000_000, "milhão", "milhões")]) {
        let count = rest / scale;
        if count > 0 {
            let name = if count == 1 { one } else { many };
            groups.push(format!("{} {name}", cardinal(count, false, variety)));
            rest %= scale;
        }
    }
    let thousands = rest / 1000;
    if thousands == 1 {
        groups.push("mil".to_owned());
    } else if thousands > 1 {
        groups.push(format!(
            "{} mil",
            below_thousand(thousands, feminine, variety).join(" e ")
        ));
    }
    let last = rest % 1000;
    if last > 0 {
        let words = below_thousand(last, feminine, variety).join(" e ");
        // The last group takes e when it is below a hundred or a round hundred (mil e vinte, mil e
        // trezentos, but mil novecentos e noventa).
        if !groups.is_empty() && (last < 100 || last.is_multiple_of(100)) {
            groups.push(format!("e {words}"));
        } else {
            groups.push(words);
        }
    }
    groups.join(" ")
}

const ORDINAL_UNITS: [&str; 10] = [
    "", "primeir", "segund", "terceir", "quart", "quint", "sext", "sétim", "oitav", "non",
];
const ORDINAL_TENS: [&str; 10] = [
    "",
    "décim",
    "vigésim",
    "trigésim",
    "quadragésim",
    "quinquagésim",
    "sexagésim",
    "septuagésim",
    "octogésim",
    "nonagésim",
];

/// `number` as an ordinal, masculine or feminine (primeiro, vigésima primeira); `None` from a
/// hundred on, which is read as a number.
pub(crate) fn ordinal(number: u64, feminine: bool) -> Option<String> {
    if number == 0 || number >= 100 {
        return None;
    }
    let ending = if feminine { "a" } else { "o" };
    let digit = |value: u64| usize::try_from(value % 10).unwrap_or(0);
    let mut words: Vec<String> = Vec::new();
    if number >= 10 {
        words.push(format!("{}{ending}", ORDINAL_TENS[digit(number / 10)]));
    }
    if !number.is_multiple_of(10) {
        words.push(format!("{}{ending}", ORDINAL_UNITS[digit(number)]));
    }
    Some(words.join(" "))
}

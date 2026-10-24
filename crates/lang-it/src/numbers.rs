//! Numbers in Italian words: one word up to a million (milleduecentotrenta), with the elisions
//! Italian writes (ventuno, ventotto, centottanta) and the accent of a final tre (ventitré).

pub(crate) use jespersen_spoken::Gender;

const UNITS: [&str; 20] = [
    "zero",
    "uno",
    "due",
    "tre",
    "quattro",
    "cinque",
    "sei",
    "sette",
    "otto",
    "nove",
    "dieci",
    "undici",
    "dodici",
    "tredici",
    "quattordici",
    "quindici",
    "sedici",
    "diciassette",
    "diciotto",
    "diciannove",
];
const TENS: [&str; 10] = [
    "",
    "",
    "venti",
    "trenta",
    "quaranta",
    "cinquanta",
    "sessanta",
    "settanta",
    "ottanta",
    "novanta",
];

/// `number` in words.
pub(crate) fn cardinal(number: u64, gender: Gender) -> String {
    if number == 1 {
        return match gender {
            Gender::Alone => "uno",
            Gender::Masculine => "un",
            Gender::Feminine => "una",
        }
        .to_owned();
    }
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    let billions = number / 1_000_000_000;
    let millions = number / 1_000_000 % 1000;
    let rest = number % 1_000_000;
    if billions > 0 {
        parts.push(if billions == 1 {
            "un miliardo".to_owned()
        } else {
            format!("{} miliardi", below_million(billions))
        });
    }
    if millions > 0 {
        parts.push(if millions == 1 {
            "un milione".to_owned()
        } else {
            format!("{} milioni", below_million(millions))
        });
    }
    if rest > 0 {
        parts.push(below_million(rest));
    }
    parts.join(" ")
}

/// A number below a million, as one word.
fn below_million(number: u64) -> String {
    let (thousands, rest) = (number / 1000, number % 1000);
    let mut word = match thousands {
        0 => String::new(),
        1 => "mille".to_owned(),
        thousands => format!("{}mila", without_accent(&below_thousand(thousands))),
    };
    if rest > 0 {
        word.push_str(&below_thousand(rest));
    }
    word
}

/// A number below a thousand, as one word.
fn below_thousand(number: u64) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut word = match hundreds {
        0 => String::new(),
        1 => "cento".to_owned(),
        hundreds => format!("{}cento", UNITS[usize::try_from(hundreds).unwrap_or(0)]),
    };
    if rest > 0 {
        let tens = below_hundred(rest);
        // cento loses its o before otto and ottanta (centotto, centottanta).
        if !word.is_empty() && tens.starts_with('o') {
            word.pop();
        }
        word.push_str(&tens);
    }
    word
}

fn below_hundred(number: u64) -> String {
    let index = usize::try_from(number).unwrap_or(0);
    if index < 20 {
        return UNITS[index].to_owned();
    }
    let (tens, unit) = (TENS[index / 10], index % 10);
    match unit {
        0 => tens.to_owned(),
        // venti loses its i before uno and otto (ventuno, ventotto).
        1 | 8 => format!("{}{}", &tens[..tens.len() - 1], UNITS[unit]),
        3 => format!("{tens}tré"),
        unit => format!("{tens}{}", UNITS[unit]),
    }
}

/// A number word without the accent of a final tré, inside a longer word (ventitremila).
fn without_accent(word: &str) -> String {
    word.replace('é', "e")
}

const ORDINALS: [&str; 11] = [
    "", "primo", "secondo", "terzo", "quarto", "quinto", "sesto", "settimo", "ottavo", "nono", "decimo",
];

/// An ordinal: primo to decimo, then the cardinal with -esimo (undicesimo, ventitreesimo,
/// centesimo); `feminine` for -a. Above a million `None`.
pub(crate) fn ordinal(number: u64, feminine: bool) -> Option<String> {
    let masculine = match ORDINALS.get(usize::try_from(number).ok()?) {
        Some(word) if !word.is_empty() => (*word).to_owned(),
        _ if number == 0 || number >= 1_000_000 => return None,
        _ => {
            let cardinal = without_accent(&below_million(number));
            // ventitré keeps its e (ventitreesimo); other numbers lose their last vowel.
            let stem = if cardinal.ends_with("tre") || cardinal.ends_with("sei") {
                cardinal
            } else {
                cardinal[..cardinal.len() - 1].to_owned()
            };
            format!("{stem}esimo")
        }
    };
    Some(if feminine {
        format!("{}a", &masculine[..masculine.len() - 1])
    } else {
        masculine
    })
}

//! French numbers in words: vingt et un, soixante-dix, quatre-vingts, cent un, deux cents, mille,
//! with une for a feminine noun, and ordinals (premier, vingtième).

const UNITS: [&str; 17] = [
    "zéro", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf", "dix", "onze", "douze", "treize",
    "quatorze", "quinze", "seize",
];

const TENS: [&str; 7] = ["", "dix", "vingt", "trente", "quarante", "cinquante", "soixante"];

/// `number` below a hundred. The t of vingt is said before what follows it (vingt-deux vɛ̃tdø), so
/// it is written vinte there; one is une before a feminine noun.
fn below_hundred(number: u64, feminine: bool) -> String {
    let unit = |value: u64| -> &'static str {
        if value == 1 && feminine {
            "une"
        } else {
            UNITS[usize::try_from(value).unwrap_or(0)]
        }
    };
    match number {
        0..=16 => unit(number).to_owned(),
        17..=19 => format!("dix-{}", unit(number - 10)),
        80 => "quatre-vingts".to_owned(),
        81..=99 => format!("quatre-vingt-{}", below_hundred(number - 80, feminine)),
        _ => {
            // 70 to 79 are soixante and 10 to 19.
            let (tens, rest) = if number >= 70 {
                (6, number - 60)
            } else {
                (number / 10, number % 10)
            };
            let name = TENS[usize::try_from(tens).unwrap_or(0)];
            let name = if tens == 2 && rest > 0 { "vinte" } else { name };
            match rest {
                0 => name.to_owned(),
                1 | 11 => format!("{name} et {}", below_hundred(rest, feminine)),
                _ => format!("{name}-{}", below_hundred(rest, feminine)),
            }
        }
    }
}

/// `number` below a thousand: cent, cent un, deux cents, deux cent trois.
fn below_thousand(number: u64, feminine: bool) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let hundred = match (hundreds, rest) {
        (0, _) => String::new(),
        (1, _) => "cent".to_owned(),
        (_, 0) => format!("{} cents", UNITS[usize::try_from(hundreds).unwrap_or(0)]),
        _ => format!("{} cent", UNITS[usize::try_from(hundreds).unwrap_or(0)]),
    };
    match (hundred.is_empty(), rest) {
        (true, _) => below_hundred(rest, feminine),
        (false, 0) => hundred,
        (false, _) => format!("{hundred} {}", below_hundred(rest, feminine)),
    }
}

/// `number` in words, with une for a feminine noun: 21 vingt et un, 1999 mille neuf cent
/// quatre-vingt-dix-neuf, 2000 deux mille, 1000000 un million.
pub(crate) fn cardinal(number: u64, feminine: bool) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, one, many) in [
        (1_000_000_000, "un milliard", "milliards"),
        (1_000_000, "un million", "millions"),
    ] {
        let count = number / scale % 1000;
        if count == 1 {
            parts.push(one.to_owned());
        } else if count > 1 {
            parts.push(format!("{} {many}", below_thousand(count, false)));
        }
    }
    let thousands = number / 1000 % 1000;
    if thousands == 1 {
        parts.push("mille".to_owned());
    } else if thousands > 1 {
        // Cents takes no s before mille (deux cent mille).
        let words = below_thousand(thousands, false);
        parts.push(format!(
            "{} mille",
            words
                .strip_suffix("cents")
                .map_or(words.clone(), |stem| format!("{stem}cent"))
        ));
    }
    if !number.is_multiple_of(1000) {
        parts.push(below_thousand(number % 1000, feminine));
    }
    parts.join(" ")
}

/// `number` as an ordinal: premier or première, then the cardinal with -ième (deuxième, vingt et
/// unième, cinquième, neuvième).
pub(crate) fn ordinal(number: u64, feminine: bool) -> Option<String> {
    if number == 0 {
        return None;
    }
    if number == 1 {
        return Some(if feminine { "première" } else { "premier" }.to_owned());
    }
    let words = cardinal(number, false);
    // Cents and vingts lose their s (deux centième, quatre-vingtième).
    let words = if words.ends_with("cents") || words.ends_with("vingts") {
        words[..words.len() - 1].to_owned()
    } else {
        words
    };
    let stem = if let Some(base) = words.strip_suffix("cinq") {
        format!("{base}cinqu")
    } else if let Some(base) = words.strip_suffix("neuf") {
        format!("{base}neuv")
    } else {
        words.strip_suffix('e').map_or(words.clone(), str::to_owned)
    };
    Some(format!("{stem}ième"))
}

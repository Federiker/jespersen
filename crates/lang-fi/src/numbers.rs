//! Numbers in Finnish words: 21 kaksikymmentäyksi, 1250 tuhat kaksisataa viisikymmentä. Tens and
//! units are one word; hundreds, thousands and millions are said apart.

const UNITS: [&str; 10] = [
    "nolla",
    "yksi",
    "kaksi",
    "kolme",
    "neljä",
    "viisi",
    "kuusi",
    "seitsemän",
    "kahdeksan",
    "yhdeksän",
];

/// `number` in words.
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, one, many) in [
        (1_000_000_000, "miljardi", "miljardia"),
        (1_000_000, "miljoona", "miljoonaa"),
    ] {
        let count = number / scale % 1000;
        if count == 1 {
            parts.push(one.to_owned());
        } else if count > 1 {
            parts.push(format!("{} {many}", below_thousand(count)));
        }
    }
    let thousands = number / 1000 % 1000;
    if thousands == 1 {
        parts.push("tuhat".to_owned());
    } else if thousands > 1 {
        parts.push(format!("{} tuhatta", below_thousand(thousands)));
    }
    if !number.is_multiple_of(1000) {
        parts.push(below_thousand(number % 1000));
    }
    parts.join(" ")
}

fn below_thousand(number: u64) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    if hundreds == 1 {
        parts.push("sata".to_owned());
    } else if hundreds > 1 {
        parts.push(format!("{}sataa", unit(hundreds)));
    }
    if rest > 0 {
        parts.push(below_hundred(rest));
    }
    parts.join(" ")
}

fn below_hundred(number: u64) -> String {
    let (tens, units) = (number / 10, number % 10);
    match (tens, units) {
        (0, units) => unit(units).to_owned(),
        (1, 0) => "kymmenen".to_owned(),
        (1, units) => format!("{}toista", unit(units)),
        (tens, 0) => format!("{}kymmentä", unit(tens)),
        (tens, units) => format!("{}kymmentä{}", unit(tens), unit(units)),
    }
}

fn unit(digit: u64) -> &'static str {
    UNITS[usize::try_from(digit).unwrap_or(0) % 10]
}

const ORDINALS: [&str; 11] = [
    "",
    "ensimmäinen",
    "toinen",
    "kolmas",
    "neljäs",
    "viides",
    "kuudes",
    "seitsemäs",
    "kahdeksas",
    "yhdeksäs",
    "kymmenes",
];

/// An ordinal up to ten; beyond, `None` (the cardinal is read).
pub(crate) fn ordinal(number: u64) -> Option<&'static str> {
    ORDINALS
        .get(usize::try_from(number).ok()?)
        .copied()
        .filter(|word| !word.is_empty())
}

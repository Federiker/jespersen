//! Serbian numbers in words, as the ekavian standard says them: cardinals with their gender
//! (jedan, jedna; dva, dve) and their thousands and millions in the form the count asks for (dve
//! hiljade, pet hiljada), and the ordinals of dates (trećeg).

/// How a number agrees with what it counts: jedan and dva, or jedna and dve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Agreement {
    Masculine,
    Feminine,
}

const UNITS: [&str; 20] = [
    "nula",
    "jedan",
    "dva",
    "tri",
    "četiri",
    "pet",
    "šest",
    "sedam",
    "osam",
    "devet",
    "deset",
    "jedanaest",
    "dvanaest",
    "trinaest",
    "četrnaest",
    "petnaest",
    "šesnaest",
    "sedamnaest",
    "osamnaest",
    "devetnaest",
];

const TENS: [&str; 10] = [
    "",
    "deset",
    "dvadeset",
    "trideset",
    "četrdeset",
    "pedeset",
    "šezdeset",
    "sedamdeset",
    "osamdeset",
    "devedeset",
];

const HUNDREDS: [&str; 10] = [
    "",
    "sto",
    "dvesta",
    "trista",
    "četiristo",
    "petsto",
    "šeststo",
    "sedamsto",
    "osamsto",
    "devetsto",
];

fn digit(value: u64) -> usize {
    usize::try_from(value % 10).unwrap_or(0)
}

/// Which of three forms goes with `count`: one (and 21, 31…), two to four (and 22 to 24…), and
/// the rest (pet eura, jedanaest eura).
pub(crate) fn form(count: u64, forms: [&str; 3]) -> &str {
    let teen = (11..=14).contains(&(count % 100));
    match count % 10 {
        1 if !teen => forms[0],
        2..=4 if !teen => forms[1],
        _ => forms[2],
    }
}

/// `number` below a thousand, agreeing as `agreement` says in its last word.
fn below_thousand(number: u64, agreement: Agreement) -> Vec<&'static str> {
    let mut words: Vec<&'static str> = Vec::new();
    if number >= 100 {
        words.push(HUNDREDS[digit(number / 100)]);
    }
    let rest = number % 100;
    let unit = |value: u64| match (value, agreement) {
        (1, Agreement::Feminine) => "jedna",
        (2, Agreement::Feminine) => "dve",
        (value, _) => UNITS[usize::try_from(value).unwrap_or(0)],
    };
    match rest {
        0 => {}
        1..=19 => words.push(unit(rest)),
        _ => {
            words.push(TENS[digit(rest / 10)]);
            if !rest.is_multiple_of(10) {
                words.push(unit(rest % 10));
            }
        }
    }
    words
}

/// `number` in words: 21 dvadeset jedan, 2024 dve hiljade dvadeset četiri, 1000 hiljadu.
pub(crate) fn cardinal(number: u64, agreement: Agreement) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut words: Vec<&str> = Vec::new();
    // Thousands and milliards count as feminine (dve hiljade), millions as masculine.
    for (scale, forms, gender) in [
        (
            1_000_000_000,
            ["milijarda", "milijarde", "milijardi"],
            Agreement::Feminine,
        ),
        (1_000_000, ["milion", "miliona", "miliona"], Agreement::Masculine),
        (1000, ["hiljadu", "hiljade", "hiljada"], Agreement::Feminine),
    ] {
        let count = number / scale % 1000;
        if count > 1 {
            words.extend(below_thousand(count, gender));
        }
        if count > 0 {
            words.push(form(count, forms));
        }
    }
    words.extend(below_thousand(number % 1000, agreement));
    words.join(" ")
}

const ORDINAL_UNITS: [&str; 20] = [
    "",
    "prv",
    "drug",
    "treć",
    "četvrt",
    "pet",
    "šest",
    "sedm",
    "osm",
    "devet",
    "deset",
    "jedanaest",
    "dvanaest",
    "trinaest",
    "četrnaest",
    "petnaest",
    "šesnaest",
    "sedamnaest",
    "osamnaest",
    "devetnaest",
];

const ORDINAL_TENS: [&str; 4] = ["", "deset", "dvadeset", "trideset"];

/// The day of a month as an ordinal in the genitive: prvog, trećeg, dvadeset prvog.
pub(crate) fn day(number: u64) -> Option<String> {
    let index = |value: u64| usize::try_from(value).unwrap_or(0);
    let stem = |value: u64| {
        let stem = ORDINAL_UNITS[index(value)];
        // ć and j take -eg, the others -og (trećeg, but prvog).
        format!("{stem}{}", if stem.ends_with('ć') { "eg" } else { "og" })
    };
    match number {
        1..=19 => Some(stem(number)),
        20 | 30 => Some(format!("{}og", ORDINAL_TENS[index(number / 10)])),
        21..=31 => Some(format!("{} {}", TENS[digit(number / 10)], stem(number % 10))),
        _ => None,
    }
}

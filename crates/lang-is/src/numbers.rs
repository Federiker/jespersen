//! Icelandic numbers in words, with the genders of one to four (einn, ein, eitt; tveir, tvær,
//! tvö), the tens joined to their units by og and tíu to the units before it as eSpeak NG says
//! them (tuttugu-og-einn, fjöru-tíu), the hundreds and thousands counted in the neuter (tvö
//! hundruð, eitt þúsund), and og before the last part (hundrað og einn).

/// How a number agrees: with a masculine or feminine noun, or a neuter one (and the hundreds and
/// thousands, which are neuter); counting on its own takes the masculine.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Form {
    Masculine,
    Feminine,
    Neuter,
}

const FIVE_TO_NINETEEN: [&str; 15] = [
    "fimm",
    "sex",
    "sjö",
    "átta",
    "níu",
    "tíu",
    "ellefu",
    "tólf",
    "þrettán",
    "fjórtán",
    "fimmtán",
    "sextán",
    "sautján",
    "átján",
    "nítján",
];

const TENS: [&str; 10] = [
    "",
    "",
    "tuttugu",
    "þrjá-tíu",
    "fjöru-tíu",
    "fimm-tíu",
    "sex-tíu",
    "sjö-tíu",
    "átta-tíu",
    "níu-tíu",
];

fn index(value: u64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// `number` below twenty, agreeing with `form`.
fn unit(number: u64, form: Form) -> &'static str {
    match (number, form) {
        (0, _) => "núll",
        (1, Form::Masculine) => "einn",
        (1, Form::Feminine) => "ein",
        (1, Form::Neuter) => "eitt",
        (2, Form::Masculine) => "tveir",
        (2, Form::Feminine) => "tvær",
        (2, Form::Neuter) => "tvö",
        (3, Form::Masculine) => "þrír",
        (3, Form::Feminine) => "þrjár",
        (3, Form::Neuter) => "þrjú",
        (4, Form::Masculine) => "fjórir",
        (4, Form::Feminine) => "fjórar",
        (4, Form::Neuter) => "fjögur",
        _ => FIVE_TO_NINETEEN[index(number - 5)],
    }
}

/// `number` below a hundred.
fn below_hundred(number: u64, form: Form) -> String {
    match number {
        0..=19 => unit(number, form).to_owned(),
        _ if number.is_multiple_of(10) => TENS[index(number / 10)].to_owned(),
        _ => format!("{}-og-{}", TENS[index(number / 10)], unit(number % 10, form)),
    }
}

/// The parts of `number` below a thousand.
fn below_thousand(number: u64, form: Form) -> Vec<String> {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    match hundreds {
        0 => {}
        1 => parts.push("eitt hundrað".to_owned()),
        _ => parts.push(format!("{} hundruð", unit(hundreds, Form::Neuter))),
    }
    if rest > 0 || hundreds == 0 {
        parts.push(below_hundred(rest, form));
    }
    parts
}

/// `number` in words: 21 tuttugu-og-einn, 101 eitt hundrað-og einn, 2024 tvö þúsund
/// tuttugu-og-fjórir.
pub(crate) fn cardinal(number: u64, form: Form) -> String {
    let mut parts: Vec<String> = Vec::new();
    let millions = number / 1_000_000;
    match millions {
        0 => {}
        1 => parts.push("ein milljón".to_owned()),
        _ => parts.push(format!(
            "{} milljónir",
            below_thousand(millions % 1000, Form::Feminine).join(" ")
        )),
    }
    let thousands = number / 1000 % 1000;
    match thousands {
        0 => {}
        1 => parts.push("eitt þúsund".to_owned()),
        _ => parts.push(format!("{} þúsund", below_thousand(thousands, Form::Neuter).join(" "))),
    }
    if !number.is_multiple_of(1000) || number == 0 {
        parts.extend(below_thousand(number % 1000, form));
    }
    // og comes before the last part, unless it has one, joined to the word before it as eSpeak NG
    // says it (hundrað-og einn; hundrað tuttugu-og-fimm).
    if parts.len() > 1
        && let Some(last) = parts.pop()
    {
        if !last.contains("-og-")
            && let Some(before) = parts.last_mut()
        {
            before.push_str("-og");
        }
        parts.push(last);
    }
    parts.join(" ")
}

/// The ordinal of a day of the month (þriðji maí), masculine as dates say them.
pub(crate) fn day_ordinal(day: u64) -> Option<String> {
    const FIRST: [&str; 20] = [
        "fyrsti",
        "annar",
        "þriðji",
        "fjórði",
        "fimmti",
        "sjötti",
        "sjöundi",
        "áttundi",
        "níundi",
        "tíundi",
        "ellefti",
        "tólfti",
        "þrettándi",
        "fjórtándi",
        "fimmtándi",
        "sextándi",
        "sautjándi",
        "átjándi",
        "nítjándi",
        "tuttugasti",
    ];
    Some(match day {
        1..=20 => FIRST[index(day - 1)].to_owned(),
        21..=29 => format!("tuttugasti og {}", FIRST[index(day - 21)]),
        30 => "þrítugasti".to_owned(),
        31 => "þrítugasti og fyrsti".to_owned(),
        _ => return None,
    })
}

/// The words of a number joined by hyphens, said as one (tuttugu-og-einn, fjöru-tíu, hundrað-og).
pub(crate) fn is_number_word(word: &str) -> bool {
    TENS[2..].contains(&word)
        || ["þrjá", "fjöru"].contains(&word)
        || ["hundrað", "hundruð", "þúsund"].contains(&word)
        || FIVE_TO_NINETEEN.contains(&word)
        || [
            "og", "einn", "ein", "eitt", "tveir", "tvær", "tvö", "þrír", "þrjár", "þrjú", "fjórir", "fjórar", "fjögur",
        ]
        .contains(&word)
}

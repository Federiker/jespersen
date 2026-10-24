//! Russian numbers in words: cardinals with their gender (один, одна; два, две) and their
//! thousands and millions in the form the count asks for (две тысячи, пять тысяч, два миллиона),
//! and ordinals in the forms dates, years and centuries need (третье мая, двадцать четвёртого
//! года, двадцатый век).

/// How a cardinal agrees with what it counts: один and два, or одна and две.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Agreement {
    Masculine,
    Feminine,
}

const UNITS: [&str; 20] = [
    "ноль",
    "один",
    "два",
    "три",
    "четыре",
    "пять",
    "шесть",
    "семь",
    "восемь",
    "девять",
    "десять",
    "одиннадцать",
    "двенадцать",
    "тринадцать",
    "четырнадцать",
    "пятнадцать",
    "шестнадцать",
    "семнадцать",
    "восемнадцать",
    "девятнадцать",
];

const TENS: [&str; 10] = [
    "",
    "десять",
    "двадцать",
    "тридцать",
    "сорок",
    "пятьдесят",
    "шестьдесят",
    "семьдесят",
    "восемьдесят",
    "девяносто",
];

const HUNDREDS: [&str; 10] = [
    "",
    "сто",
    "двести",
    "триста",
    "четыреста",
    "пятьсот",
    "шестьсот",
    "семьсот",
    "восемьсот",
    "девятьсот",
];

fn index(value: u64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// Which of three forms goes with `count`: one (двадцать один рубль), two to four (три рубля),
/// and the rest (пять рублей, одиннадцать рублей).
pub(crate) fn form(count: u64, forms: [&str; 3]) -> &str {
    match (count % 10, count % 100) {
        (_, 11..=14) => forms[2],
        (1, _) => forms[0],
        (2..=4, _) => forms[1],
        _ => forms[2],
    }
}

/// `number` below a thousand, agreeing as `agreement` says in its last word.
fn below_thousand(number: u64, agreement: Agreement) -> Vec<&'static str> {
    let mut words: Vec<&'static str> = Vec::new();
    if number >= 100 {
        words.push(HUNDREDS[index(number / 100 % 10)]);
    }
    let rest = number % 100;
    let unit = |value: u64| match (value, agreement) {
        (1, Agreement::Feminine) => "одна",
        (2, Agreement::Feminine) => "две",
        (value, _) => UNITS[index(value)],
    };
    match rest {
        0 => {}
        1..=19 => words.push(unit(rest)),
        _ => {
            words.push(TENS[index(rest / 10)]);
            if !rest.is_multiple_of(10) {
                words.push(unit(rest % 10));
            }
        }
    }
    words
}

/// `number` in words: 21 двадцать один, 2024 две тысячи двадцать четыре, 1000 тысяча.
pub(crate) fn cardinal(number: u64, agreement: Agreement) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut words: Vec<&str> = Vec::new();
    for (scale, forms, gender) in [
        (
            1_000_000_000,
            ["миллиард", "миллиарда", "миллиардов"],
            Agreement::Masculine,
        ),
        (1_000_000, ["миллион", "миллиона", "миллионов"], Agreement::Masculine),
        (1000, ["тысяча", "тысячи", "тысяч"], Agreement::Feminine),
    ] {
        let count = number / scale % 1000;
        // A thousand alone is тысяча, not одна тысяча.
        if count > 1 || count == 1 && scale != 1000 {
            words.extend(below_thousand(count, gender));
        }
        if count > 0 {
            words.push(form(count, forms));
        }
    }
    words.extend(below_thousand(number % 1000, agreement));
    words.join(" ")
}

/// The form of an ordinal: the masculine (двадцатый век), its genitive (двадцать четвёртого года),
/// its prepositional (в двадцатом веке) and the neuter (третье мая).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Case {
    Masculine,
    Genitive,
    Prepositional,
    Neuter,
}

/// The ordinals' stems below twenty, and whether their ending is stressed (второй, not -ый).
const ORDINAL_UNITS: [(&str, bool); 20] = [
    ("", false),
    ("перв", false),
    ("втор", true),
    ("трет", false),
    ("четвёрт", false),
    ("пят", false),
    ("шест", true),
    ("седьм", true),
    ("восьм", true),
    ("девят", false),
    ("десят", false),
    ("одиннадцат", false),
    ("двенадцат", false),
    ("тринадцат", false),
    ("четырнадцат", false),
    ("пятнадцат", false),
    ("шестнадцат", false),
    ("семнадцат", false),
    ("восемнадцат", false),
    ("девятнадцат", false),
];

const ORDINAL_TENS: [(&str, bool); 10] = [
    ("", false),
    ("десят", false),
    ("двадцат", false),
    ("тридцат", false),
    ("сороков", true),
    ("пятидесят", false),
    ("шестидесят", false),
    ("семидесят", false),
    ("восьмидесят", false),
    ("девяност", false),
];

const ORDINAL_HUNDREDS: [&str; 10] = [
    "",
    "сот",
    "двухсот",
    "трёхсот",
    "четырёхсот",
    "пятисот",
    "шестисот",
    "семисот",
    "восьмисот",
    "девятисот",
];

/// An ordinal's stem with the ending of `case`: третий has its own (третьего, третье).
fn inflect(stem: &str, stressed_ending: bool, case: Case) -> String {
    if stem == "трет" {
        let ending = match case {
            Case::Masculine => "ий",
            Case::Genitive => "ьего",
            Case::Prepositional => "ьем",
            Case::Neuter => "ье",
        };
        return format!("{stem}{ending}");
    }
    let ending = match case {
        Case::Masculine if stressed_ending => "ой",
        Case::Masculine => "ый",
        Case::Genitive => "ого",
        Case::Prepositional => "ом",
        Case::Neuter => "ое",
    };
    format!("{stem}{ending}")
}

/// `number` as an ordinal in `case`: only its last word is one (тысяча девятьсот пятнадцатого,
/// две тысячи двадцать четвёртого, двухтысячного); `None` from a million on.
pub(crate) fn ordinal(number: u64, case: Case) -> Option<String> {
    if number == 0 || number >= 1_000_000 {
        return None;
    }
    let rest = number % 100;
    let last = if rest > 0 {
        let (stem, stressed) = if rest < 20 {
            ORDINAL_UNITS[index(rest)]
        } else if rest.is_multiple_of(10) {
            ORDINAL_TENS[index(rest / 10)]
        } else {
            ORDINAL_UNITS[index(rest % 10)]
        };
        inflect(stem, stressed, case)
    } else if !number.is_multiple_of(1000) {
        inflect(ORDINAL_HUNDREDS[index(number / 100 % 10)], false, case)
    } else {
        // Whole thousands are one word: тысячного, двухтысячного, пятитысячного.
        const COUNTS: [&str; 10] = [
            "",
            "",
            "двух",
            "трёх",
            "четырёх",
            "пяти",
            "шести",
            "семи",
            "восьми",
            "девяти",
        ];
        let count = COUNTS.get(index(number / 1000))?;
        return Some(inflect(&format!("{count}тысячн"), false, case));
    };
    // The words before the last one are cardinals: двадцать четвёртого, тысяча девятьсот
    // пятнадцатого.
    let before = if rest > 0 {
        let tens = if rest > 20 && !rest.is_multiple_of(10) {
            rest / 10 * 10
        } else {
            0
        };
        number - rest + tens
    } else {
        number - number % 1000
    };
    Some(if before == 0 {
        last
    } else {
        format!("{} {last}", cardinal(before, Agreement::Masculine))
    })
}

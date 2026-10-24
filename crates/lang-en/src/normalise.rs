//! What each token of an English sentence is read as: words for words, number names for
//! numbers, currency read after its amount ($5: five dollars), names for symbols.

use jespersen_text::TokenKind;

use crate::{
    addresses,
    numbers::{cardinal, digits, ordinal, plural, year},
    titles, units,
};

/// What a token is read as.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Spoken {
    /// These words, in order.
    Words(Vec<String>),
    /// A hyphenated word's parts, in order (well-known), which eSpeak NG's notation writes joined.
    Hyphenated(Vec<String>),
    /// Nothing: punctuation, or a currency sign read with its amount.
    Silent,
    /// Something jespersen cannot read yet.
    Unknown,
}

/// A currency's names.
struct Currency {
    one: &'static str,
    many: &'static str,
    /// The hundredth, singular and plural, if the currency has one.
    cents: Option<(&'static str, &'static str)>,
}

/// What each of `tokens` (text and kind) is read as, in English; `british` for British English.
pub fn spoken(tokens: &[(&str, TokenKind)], british: bool) -> Vec<Spoken> {
    let mut spoken: Vec<Spoken> = tokens
        .iter()
        .map(|&(text, kind)| match kind {
            TokenKind::Word => word(text, british),
            TokenKind::Punctuation => Spoken::Silent,
            TokenKind::Number => number(text, british).map_or(Spoken::Unknown, Spoken::Words),
            // Footnote marks and section breaks (* * *) are not read.
            TokenKind::Symbol if matches!(text, "*" | "†" | "‡") => Spoken::Silent,
            TokenKind::Symbol => symbol(text).map_or(Spoken::Unknown, |name| Spoken::Words(vec![name.to_owned()])),
        })
        .collect();
    for index in 0..tokens.len().saturating_sub(1) {
        // A currency code after an amount: 15 EUR is fifteen euros.
        if tokens[index].1 == TokenKind::Number
            && let Some(currency) = code(tokens[index + 1].0)
            && let Some(words) = amount(tokens[index].0, &currency, british)
        {
            spoken[index] = Spoken::Words(words);
            spoken[index + 1] = Spoken::Silent;
        }
        // "No." before a number: number five.
        if matches!(tokens[index].0, "No" | "no" | "Nos" | "nos") && tokens[index + 1].1 == TokenKind::Number {
            let word = if tokens[index].0.ends_with('s') {
                "numbers"
            } else {
                "number"
            };
            spoken[index] = Spoken::Words(vec![word.to_owned()]);
        }
        if let Some(currency) = currency(tokens[index].0)
            && tokens[index + 1].1 == TokenKind::Number
            && let Some(words) = amount(tokens[index + 1].0, &currency, british)
        {
            spoken[index] = Spoken::Silent;
            spoken[index + 1] = Spoken::Words(words);
        }
    }
    in_context(tokens, &mut spoken);
    spoken
}

/// Readings that depend on the tokens around: titles and units follow
/// their next or previous token (Dr. Jones, 5 km, 20 °C, km/h).
fn in_context(tokens: &[(&str, TokenKind)], spoken: &mut [Spoken]) {
    let texts: Vec<&str> = tokens.iter().map(|&(text, _)| text).collect();
    let is_number = |index: usize| tokens.get(index).is_some_and(|&(_, kind)| kind == TokenKind::Number);
    let one = |index: usize| matches!(texts.get(index), Some(&"1" | &"-1"));
    for index in 0..tokens.len() {
        let next = texts.get(index + 1).map(|&next| (next, is_number(index + 1)));
        if tokens[index].1 == TokenKind::Word
            && let Some(word) = titles::title(texts[index], next)
        {
            spoken[index] = Spoken::Words(vec![word.to_owned()]);
        }
        if index > 0 && is_number(index - 1) {
            if let Some((singular, plural)) = units::unit(texts[index]) {
                let name = if one(index - 1) { singular } else { plural };
                spoken[index] = Spoken::Words(owned(name.split(' ')));
            }
            // 20 °C: twenty degrees Celsius.
            if texts[index] == "°"
                && let Some(scale) = texts.get(index + 1).and_then(|&scale| match scale {
                    "C" => Some("Celsius"),
                    "F" => Some("Fahrenheit"),
                    _ => None,
                })
            {
                let degrees = if one(index - 1) { "degree" } else { "degrees" };
                spoken[index] = Spoken::Words(vec![degrees.to_owned(), scale.to_owned()]);
                spoken[index + 1] = Spoken::Silent;
            }
        }
        // A day with its month is an ordinal: January 5 (Jan. 5) is January fifth, 5 January the
        // fifth of January.
        let month = |at: usize| {
            texts.get(at).is_some_and(|&text| {
                MONTHS.contains(&text)
                    || titles::title(text, Some(("1", true))).is_some_and(|word| MONTHS.contains(&word))
            })
        };
        let day = |at: usize| is_number(at) && integer(texts[at]).is_some_and(|day| (1..=31).contains(&day));
        if day(index)
            && let Some(n) = integer(texts[index])
        {
            if index > 0 && month(index - 1) {
                spoken[index] = Spoken::Words(ordinal(n, false));
            } else if month(index + 1) {
                let mut words = vec!["the".to_owned()];
                words.extend(ordinal(n, false));
                words.push("of".to_owned());
                spoken[index] = Spoken::Words(words);
            }
        }
        // km/h: kilometres per hour.
        if texts[index] == "/"
            && index > 0
            && units::unit(texts[index - 1]).is_some()
            && let Some(per) = texts.get(index + 1).and_then(|&unit| units::per(unit))
        {
            spoken[index] = Spoken::Words(vec!["per".to_owned()]);
            spoken[index + 1] = Spoken::Words(vec![per.to_owned()]);
        }
    }
}

/// The words of a number token: 42, 1,000, 3.14, 21st, 1999 (a year), 1990s, 10:30 (a time),
/// 12/05/2024 (a date), 1990-1995 (a range), 3/4 (a fraction), and other groups of digits.
fn number(text: &str, british: bool) -> Option<Vec<String>> {
    // A sign: -5 is minus five.
    if let Some(rest) = text.strip_prefix(['-', '\u{2212}']).or_else(|| text.strip_prefix('+')) {
        let mut words = vec![(if text.starts_with('+') { "plus" } else { "minus" }).to_owned()];
        words.extend(number(rest, british)?);
        return Some(words);
    }
    // A unit written straight after the number: 20km.
    if let Some(at) = text.find(|c: char| c.is_alphabetic())
        && let Some((one, many)) = units::unit(&text[at..])
    {
        let mut words = number(&text[..at], british)?;
        words.extend(owned((if &text[..at] == "1" { one } else { many }).split(' ')));
        return Some(words);
    }
    if text.contains([':', '/', '-']) || (text.contains(',') && integer(text).is_none()) {
        return time(text, british)
            .or_else(|| date(text, british))
            .or_else(|| fraction(text, british))
            .or_else(|| groups(text, british));
    }
    if let Some(integer) = integer(text) {
        // Four digits from 1100 to 2099 are read as a year; leading zeros digit by digit.
        if text.len() == 4
            && !text.contains(',')
            && let Some(words) = year(integer, british)
        {
            return Some(owned(words));
        }
        if text.starts_with('0') && text.len() > 1 {
            return Some(owned(digits(text)));
        }
        return Some(owned(cardinal(integer, british)));
    }
    if let Some((whole, fraction)) = text.split_once('.')
        && fraction.chars().all(|c| c.is_ascii_digit())
        && !fraction.is_empty()
    {
        let mut words = owned(cardinal(integer(whole)?, british));
        words.push("point".to_owned());
        words.extend(owned(digits(fraction)));
        return Some(words);
    }
    let lower = text.to_ascii_lowercase();
    for suffix in ["st", "nd", "rd", "th"] {
        if let Some(digits) = lower.strip_suffix(suffix) {
            return Some(ordinal(integer(digits)?, british));
        }
    }
    let decade = lower
        .strip_suffix("'s")
        .or_else(|| lower.strip_suffix("’s"))
        .or_else(|| lower.strip_suffix('s'))?;
    let n = integer(decade)?;
    let words = match year(n, british) {
        Some(words) if decade.len() == 4 => owned(words),
        _ => owned(cardinal(n, british)),
    };
    Some(plural(words))
}

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// A time, hours and minutes: 10:30 is ten thirty, 10:05 ten oh five, 10:00 ten o'clock, 14:00
/// fourteen hundred.
fn time(text: &str, british: bool) -> Option<Vec<String>> {
    let (hours, minutes) = text.split_once(':')?;
    let (hour, minute): (u64, u64) = (hours.parse().ok()?, minutes.parse().ok()?);
    if hours.is_empty() || hours.len() > 2 || minutes.len() != 2 || hour > 23 || minute > 59 {
        return None;
    }
    let mut words = owned(cardinal(hour, british));
    match minute {
        0 if hour <= 12 => words.push("o'clock".to_owned()),
        0 => words.push("hundred".to_owned()),
        1..=9 => {
            words.push("oh".to_owned());
            words.extend(owned(cardinal(minute, british)));
        }
        _ => words.extend(owned(cardinal(minute, british))),
    }
    Some(words)
}

/// A date written with slashes: month first in American English (12/05/2024: December fifth,
/// twenty twenty-four), day first in British English (the twelfth of May), either way round when
/// only one order is possible.
fn date(text: &str, british: bool) -> Option<Vec<String>> {
    let parts: Vec<&str> = text.split('/').collect();
    let [first, second, year_digits] = parts[..] else {
        return None;
    };
    let (first, second): (u64, u64) = (first.parse().ok()?, second.parse().ok()?);
    let (mut day, mut month) = if british { (first, second) } else { (second, first) };
    if month > 12 && day <= 12 {
        (day, month) = (month, day);
    }
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) || !matches!(year_digits.len(), 2 | 4) {
        return None;
    }
    let year_number: u64 = year_digits.parse().ok()?;
    let month_name = MONTHS[usize::try_from(month - 1).ok()?].to_owned();
    let mut words = if british {
        let mut words = vec!["the".to_owned()];
        words.extend(ordinal(day, british));
        words.push("of".to_owned());
        words.push(month_name);
        words
    } else {
        let mut words = vec![month_name];
        words.extend(ordinal(day, british));
        words
    };
    words.extend(match year(year_number, british) {
        Some(year) if year_digits.len() == 4 => owned(year),
        _ if year_digits.starts_with('0') => owned(["oh"].into_iter().chain(digits(&year_digits[1..]))),
        _ => owned(cardinal(year_number, british)),
    });
    Some(words)
}

/// A simple fraction: 1/2 one half, 3/4 three quarters, 2/3 two thirds.
fn fraction(text: &str, british: bool) -> Option<Vec<String>> {
    let (numerator, denominator) = text.split_once('/')?;
    let (top, bottom): (u64, u64) = (numerator.parse().ok()?, denominator.parse().ok()?);
    if numerator.len() > 2 || denominator.len() > 2 || bottom < 2 || numerator.starts_with('0') {
        return None;
    }
    let mut words = owned(cardinal(top, british));
    words.push(match (bottom, top == 1) {
        (2, true) => "half".to_owned(),
        (2, false) => "halves".to_owned(),
        (4, true) => "quarter".to_owned(),
        (4, false) => "quarters".to_owned(),
        (_, one) => {
            let name = ordinal(bottom, british).join(" ");
            if one { name } else { format!("{name}s") }
        }
    });
    Some(words.join(" ").split(' ').map(str::to_owned).collect())
}

/// Other groups of digits: a range of two (1990-1995: nineteen ninety to nineteen ninety-five),
/// digit by digit when hyphens join more (555-12-34), with "slash" between slashed groups, and
/// one after the other for commas that are not thousands (2,50: two fifty).
fn groups(text: &str, british: bool) -> Option<Vec<String>> {
    let separator = [':', '/', '-', ',']
        .into_iter()
        .find(|&separator| text.contains(separator))?;
    let parts: Vec<&str> = text.split(separator).collect();
    if parts
        .iter()
        .any(|part| part.is_empty() || !part.chars().all(|c| c.is_ascii_digit()))
    {
        return None;
    }
    let read = |part: &str| -> Option<Vec<String>> {
        let n: u64 = part.parse().ok()?;
        Some(match year(n, british) {
            Some(words) if part.len() == 4 => owned(words),
            _ if part.starts_with('0') && part.len() > 1 => owned(digits(part)),
            _ => owned(cardinal(n, british)),
        })
    };
    let mut words = Vec::new();
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            match separator {
                '-' if parts.len() == 2 => words.push("to".to_owned()),
                '/' => words.push("slash".to_owned()),
                _ => {}
            }
        }
        if separator == '-' && parts.len() > 2 {
            words.extend(owned(digits(part)));
        } else {
            words.extend(read(part)?);
        }
    }
    Some(words)
}

fn owned<'a>(words: impl IntoIterator<Item = &'a str>) -> Vec<String> {
    words.into_iter().map(str::to_owned).collect()
}

/// An integer written with digits only, or with commas between groups of three (1,000,000).
fn integer(text: &str) -> Option<u64> {
    let groups: Vec<&str> = text.split(',').collect();
    let valid = !text.is_empty()
        && groups
            .iter()
            .all(|group| !group.is_empty() && group.chars().all(|c| c.is_ascii_digit()))
        && (groups.len() == 1 || (groups[0].len() <= 3 && groups[1..].iter().all(|group| group.len() == 3)));
    if !valid {
        return None;
    }
    groups.concat().parse().ok()
}

/// The words of a Latin abbreviation written with its full stops (jespersen's own list, CC0).
fn abbreviation(text: &str) -> Option<Vec<String>> {
    let words: &[&str] = match text.to_ascii_lowercase().as_str() {
        "e.g" => &["for", "example"],
        "i.e" => &["that", "is"],
        "etc" => &["et", "cetera"],
        "vs" => &["versus"],
        "approx" => &["approximately"],
        _ => return None,
    };
    Some(owned(words.iter().copied()))
}

/// A Roman numeral of I, V, X and L, two letters or more, written in the usual form (II, IV,
/// XIV, XL); a single I, V or X is left as a word or letter.
fn roman(text: &str, british: bool) -> Option<Vec<String>> {
    if text.chars().count() < 2 || !text.chars().all(|c| "IVXL".contains(c)) {
        return None;
    }
    let value = |c: char| -> i64 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            _ => 50,
        }
    };
    let values: Vec<i64> = text.chars().map(value).collect();
    let mut total = 0;
    for (index, &current) in values.iter().enumerate() {
        if values.get(index + 1).is_some_and(|&next| next > current) {
            total -= current;
        } else {
            total += current;
        }
    }
    let total = u64::try_from(total).ok().filter(|&total| total > 0)?;
    // Only the usual form: what writing the total back gives.
    (to_roman(total) == text).then(|| owned(cardinal(total, british)))
}

fn to_roman(mut n: u64) -> String {
    let mut out = String::new();
    for (value, letters) in [
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ] {
        while n >= value {
            out.push_str(letters);
            n -= value;
        }
    }
    out
}

/// What a symbol is called, when it is read on its own.
fn symbol(text: &str) -> Option<&'static str> {
    Some(match text {
        "%" => "percent",
        "&" => "and",
        "+" => "plus",
        "=" => "equals",
        "@" => "at",
        "°" => "degrees",
        "×" => "times",
        "/" => "slash",
        _ => return None,
    })
}

fn currency(text: &str) -> Option<Currency> {
    Some(match text {
        "$" => Currency {
            one: "dollar",
            many: "dollars",
            cents: Some(("cent", "cents")),
        },
        "£" => Currency {
            one: "pound",
            many: "pounds",
            cents: Some(("penny", "pence")),
        },
        "€" => Currency {
            one: "euro",
            many: "euros",
            cents: Some(("cent", "cents")),
        },
        "¥" => Currency {
            one: "yen",
            many: "yen",
            cents: None,
        },
        _ => return None,
    })
}

/// An amount of money: $5 is five dollars, $3.50 three dollars and fifty cents, $0.99 ninety-nine
/// cents, $1.5 one point five dollars.
fn amount(text: &str, currency: &Currency, british: bool) -> Option<Vec<String>> {
    let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
    let units = integer(whole)?;
    let unit = if units == 1 { currency.one } else { currency.many };
    if fraction.is_empty() {
        let mut words = owned(cardinal(units, british));
        words.push(unit.to_owned());
        return Some(words);
    }
    let cents = (fraction.len() == 2).then(|| integer(fraction)).flatten();
    let (Some(cents), Some((one, many))) = (cents, currency.cents) else {
        // Not a whole number of cents: one point five dollars.
        let mut words = number(text, british)?;
        words.push(currency.many.to_owned());
        return Some(words);
    };
    let mut words = Vec::new();
    if units > 0 || cents == 0 {
        words.extend(owned(cardinal(units, british)));
        words.push(unit.to_owned());
    }
    if cents > 0 {
        if !words.is_empty() {
            words.push("and".to_owned());
        }
        words.extend(owned(cardinal(cents, british)));
        words.push((if cents == 1 { one } else { many }).to_owned());
    }
    Some(words)
}

impl Spoken {
    /// The words the token is said as, if any.
    pub fn words(&self) -> Option<&[String]> {
        match self {
            Self::Words(words) | Self::Hyphenated(words) => Some(words),
            Self::Silent | Self::Unknown => None,
        }
    }
}

/// A word token: a Latin abbreviation or a Roman numeral in words, a hyphenated word's parts (a
/// single letter among them said as the letter: A-B), or the word itself.
fn word(text: &str, british: bool) -> Spoken {
    if let Some(words) = abbreviation(text)
        .or_else(|| roman(text, british))
        .or_else(|| addresses::read(text))
    {
        return Spoken::Words(words);
    }
    if text.contains(['-', '\u{2010}']) {
        let parts = text
            .split(['-', '\u{2010}'])
            .filter(|part| !part.is_empty())
            .map(|part| {
                if part.chars().count() == 1 {
                    format!("{part}.")
                } else {
                    part.to_owned()
                }
            })
            .collect();
        return Spoken::Hyphenated(parts);
    }
    Spoken::Words(vec![text.to_owned()])
}

/// A currency written as its ISO 4217 code after an amount.
fn code(text: &str) -> Option<Currency> {
    currency(match text {
        "USD" => "$",
        "GBP" => "£",
        "EUR" => "€",
        "JPY" => "¥",
        _ => return None,
    })
}

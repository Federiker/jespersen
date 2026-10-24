//! English number names: cardinals, ordinals, years, decades and decimals, as a careful reader
//! says them. British English says "and" after hundreds and thousands (one hundred and five);
//! American English does not.

const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const SCALES: [(u64, &str); 6] = [
    (1_000_000_000_000_000_000, "quintillion"),
    (1_000_000_000_000_000, "quadrillion"),
    (1_000_000_000_000, "trillion"),
    (1_000_000_000, "billion"),
    (1_000_000, "million"),
    (1_000, "thousand"),
];

/// The words of `n` (twenty-one is "twenty", "one": the lexicon has no hyphenated compounds).
pub fn cardinal(n: u64, british: bool) -> Vec<&'static str> {
    let mut words = Vec::new();
    if n == 0 {
        words.push(ONES[0]);
        return words;
    }
    let mut rest = n;
    for (scale, name) in SCALES {
        if rest >= scale {
            below_thousand(rest / scale, british, &mut words);
            words.push(name);
            rest %= scale;
        }
    }
    if rest > 0 {
        if british && rest < 100 && n >= 1000 {
            words.push("and");
        }
        below_thousand(rest, british, &mut words);
    }
    words
}

fn below_thousand(n: u64, british: bool, words: &mut Vec<&'static str>) {
    let (hundreds, rest) = (n / 100, n % 100);
    if hundreds > 0 {
        words.push(ONES[index(hundreds)]);
        words.push("hundred");
        if british && rest > 0 {
            words.push("and");
        }
    }
    if rest > 0 {
        below_hundred(rest, words);
    }
}

fn below_hundred(n: u64, words: &mut Vec<&'static str>) {
    if n < 20 {
        words.push(ONES[index(n)]);
    } else {
        words.push(TENS[index(n / 10)]);
        if !n.is_multiple_of(10) {
            words.push(ONES[index(n % 10)]);
        }
    }
}

/// A number below 100 as an index into the tables; the callers never pass more.
fn index(n: u64) -> usize {
    usize::try_from(n).unwrap_or_default()
}

/// The words of the ordinal `n` (twenty-first).
pub fn ordinal(n: u64, british: bool) -> Vec<String> {
    let mut words: Vec<String> = cardinal(n, british).into_iter().map(str::to_owned).collect();
    if let Some(last) = words.last_mut() {
        *last = match last.as_str() {
            "one" => "first".to_owned(),
            "two" => "second".to_owned(),
            "three" => "third".to_owned(),
            "five" => "fifth".to_owned(),
            "eight" => "eighth".to_owned(),
            "nine" => "ninth".to_owned(),
            "twelve" => "twelfth".to_owned(),
            tens if tens.ends_with('y') => format!("{}ieth", &tens[..tens.len() - 1]),
            other => format!("{other}th"),
        };
    }
    words
}

/// The words of `n` as a year, for 1100 to 2099 (nineteen ninety-nine, nineteen oh five, two
/// thousand and five, twenty ten); `None` outside those years.
pub fn year(n: u64, british: bool) -> Option<Vec<&'static str>> {
    if !(1100..=2099).contains(&n) {
        return None;
    }
    if (2000..2010).contains(&n) {
        return Some(cardinal(n, british));
    }
    let (century, rest) = (n / 100, n % 100);
    let mut words = Vec::new();
    below_hundred(century, &mut words);
    match rest {
        0 => words.push("hundred"),
        1..=9 => {
            words.push("oh");
            words.push(ONES[index(rest)]);
        }
        _ => below_hundred(rest, &mut words),
    }
    Some(words)
}

/// The plural of a number's last word, for decades (nineties, the eighteen hundreds).
pub fn plural(mut words: Vec<String>) -> Vec<String> {
    if let Some(last) = words.last_mut() {
        *last = match last.strip_suffix('y') {
            Some(stem) => format!("{stem}ies"),
            None if last.ends_with('x') => format!("{last}es"),
            None => format!("{last}s"),
        };
    }
    words
}

/// The digits of `digits` one by one (the decimals of 3.14: one four).
pub fn digits(digits: &str) -> Vec<&'static str> {
    digits
        .chars()
        .filter_map(|digit| digit.to_digit(10))
        .map(|digit| ONES[digit as usize])
        .collect()
}

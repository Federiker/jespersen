//! Numbers written in Arabic digits, written out in Chinese characters for the lexicon to read:
//! 2024 两千零二十四, 10005 一万零五, 3.14 三点一四.

const DIGITS: [char; 10] = ['零', '一', '二', '三', '四', '五', '六', '七', '八', '九'];
/// The places inside a group of four digits, from the units up.
const PLACES: [&str; 4] = ["", "十", "百", "千"];
/// The groups of four digits, from the units up.
const GROUPS: [&str; 4] = ["", "万", "亿", "万亿"];

/// Each digit of `digits` on its own (二零二四), as years, telephone numbers and decimals are read.
pub(crate) fn digit_by_digit(digits: &str) -> String {
    digits
        .bytes()
        .filter(u8::is_ascii_digit)
        .map(|digit| DIGITS[usize::from(digit - b'0')])
        .collect()
}

/// A whole number written out, `None` beyond the ten thousands of billions (16 digits).
///
/// Zeros inside are read once (一万零五); a number from 10 to 19, or whose first digit is in a
/// tens place, starts without 一 (十五, 十万); a first digit 2 in the thousands or above is 两
/// (两千, 两万), as it is said.
pub(crate) fn cardinal(digits: &str) -> Option<String> {
    let digits = digits.trim_start_matches('0');
    if digits.is_empty() {
        return Some("零".to_owned());
    }
    if digits.len() > 16 || !digits.bytes().all(|digit| digit.is_ascii_digit()) {
        return None;
    }
    let values: Vec<usize> = digits.bytes().map(|digit| usize::from(digit - b'0')).collect();
    let mut out = String::new();
    // A zero has been passed since the last digit written, to be said before the next one.
    let mut zero = false;
    for (index, &value) in values.iter().enumerate() {
        let place = values.len() - 1 - index;
        let (group, position) = (place / 4, place % 4);
        if value == 0 {
            zero |= !out.is_empty();
        } else {
            if zero {
                out.push('零');
                zero = false;
            }
            let first = out.is_empty();
            if !(first && value == 1 && position == 1) {
                out.push(
                    if first && value == 2 && (position == 3 || (position == 0 && group > 0)) {
                        '两'
                    } else {
                        DIGITS[value]
                    },
                );
            }
            out.push_str(PLACES[position]);
        }
        // The end of a group of four: its name, if any digit of it was said.
        if position == 0 && group > 0 && values[index.saturating_sub(3)..=index].iter().any(|&value| value != 0) {
            out.push_str(GROUPS[group]);
        }
        // A zero between groups is said even when the group below starts with digits.
        if position == 0 && group > 0 && values.get(index + 1) == Some(&0) {
            zero = true;
        }
    }
    Some(out)
}

/// A number with a decimal point: the whole part, 点, and each digit after it (3.14 三点一四).
pub(crate) fn decimal(number: &str) -> Option<String> {
    let (whole, fraction) = number.split_once('.')?;
    if fraction.is_empty() || !fraction.bytes().all(|digit| digit.is_ascii_digit()) {
        return None;
    }
    Some(format!("{}点{}", cardinal(whole)?, digit_by_digit(fraction)))
}

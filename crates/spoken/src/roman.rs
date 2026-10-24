/// The value of a Roman numeral in capitals written the usual way (XIV 14, but not DVD or IIII).
pub(crate) fn roman(text: &str) -> Option<u64> {
    let value = |c: char| match c {
        'I' => Some(1),
        'V' => Some(5),
        'X' => Some(10),
        'L' => Some(50),
        'C' => Some(100),
        'D' => Some(500),
        'M' => Some(1000),
        _ => None,
    };
    let values: Vec<i64> = text.chars().map(value).collect::<Option<_>>()?;
    let mut total: i64 = 0;
    for (index, &this) in values.iter().enumerate() {
        if values.get(index + 1).is_some_and(|&next| next > this) {
            total -= this;
        } else {
            total += this;
        }
    }
    let total = u64::try_from(total).ok().filter(|&total| total > 0 && total < 4000)?;
    (canonical(total) == text).then_some(total)
}

/// A number below 4000 in Roman numerals, written the usual way.
fn canonical(mut number: u64) -> String {
    const NUMERALS: [(u64, &str); 13] = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut out = String::new();
    for (value, numeral) in NUMERALS {
        while number >= value {
            out.push_str(numeral);
            number -= value;
        }
    }
    out
}

//! Numbers in Turkish words, each part a word of its own: 1250 bin iki yüz elli; 100 is yüz and
//! 1000 bin, without bir.

const UNITS: [&str; 10] = [
    "sıfır", "bir", "iki", "üç", "dört", "beş", "altı", "yedi", "sekiz", "dokuz",
];
const TENS: [&str; 10] = [
    "", "on", "yirmi", "otuz", "kırk", "elli", "altmış", "yetmiş", "seksen", "doksan",
];

/// `number` in words.
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, name) in [(1_000_000_000, "milyar"), (1_000_000, "milyon")] {
        let count = number / scale % 1000;
        if count > 0 {
            parts.push(format!("{} {name}", below_thousand(count)));
        }
    }
    let thousands = number / 1000 % 1000;
    if thousands == 1 {
        parts.push("bin".to_owned());
    } else if thousands > 1 {
        parts.push(format!("{} bin", below_thousand(thousands)));
    }
    if !number.is_multiple_of(1000) {
        parts.push(below_thousand(number % 1000));
    }
    parts.join(" ")
}

fn below_thousand(number: u64) -> String {
    let index = |digit: u64| usize::try_from(digit).unwrap_or(0) % 10;
    let (hundreds, tens, units) = (index(number / 100), index(number / 10), index(number));
    let mut parts: Vec<&str> = Vec::new();
    if hundreds == 1 {
        parts.push("yüz");
    } else if hundreds > 1 {
        parts.push(UNITS[hundreds]);
        parts.push("yüz");
    }
    if tens > 0 {
        parts.push(TENS[tens]);
    }
    if units > 0 {
        parts.push(UNITS[units]);
    }
    parts.join(" ")
}

/// An ordinal: the cardinal with -(ı)ncı in the vowel of its last syllable (birinci, ikinci,
/// dördüncü, onuncu).
pub(crate) fn ordinal(number: u64) -> String {
    let mut cardinal = cardinal(number);
    if cardinal.ends_with("dört") {
        cardinal.pop();
        cardinal.push('d');
    }
    let vowels = "aeıioöuü";
    let last_vowel = cardinal.chars().rev().find(|c| vowels.contains(*c)).unwrap_or('i');
    let harmony = match last_vowel {
        'a' | 'ı' => 'ı',
        'e' | 'i' => 'i',
        'o' | 'u' => 'u',
        _ => 'ü',
    };
    if cardinal.chars().last().is_some_and(|c| vowels.contains(c)) {
        format!("{cardinal}nc{harmony}")
    } else {
        format!("{cardinal}{harmony}nc{harmony}")
    }
}

//! Basque numbers in words, counted by twenties: hogeita bat (21), berrogei (40), laurogeita
//! hamar (90), ehun, mila.

const UNITS: [&str; 20] = [
    "zero",
    "bat",
    "bi",
    "hiru",
    "lau",
    "bost",
    "sei",
    "zazpi",
    "zortzi",
    "bederatzi",
    "hamar",
    "hamaika",
    "hamabi",
    "hamahiru",
    "hamalau",
    "hamabost",
    "hamasei",
    "hamazazpi",
    "hemezortzi",
    "hemeretzi",
];

/// The twenties: hogei, berrogei, hirurogei, laurogei.
const TWENTIES: [&str; 5] = ["", "hogei", "berrogei", "hirurogei", "laurogei"];

/// The hundreds: ehun, berrehun, hirurehun…
const HUNDREDS: [&str; 10] = [
    "",
    "ehun",
    "berrehun",
    "hirurehun",
    "laurehun",
    "bostehun",
    "seiehun",
    "zazpiehun",
    "zortziehun",
    "bederatziehun",
];

fn index(value: u64) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// `number` below a hundred: a twenty, and ta with what is left (hogeita bat, berrogeita hamar).
fn below_hundred(number: u64) -> String {
    if number < 20 {
        return UNITS[index(number)].to_owned();
    }
    let (twenties, rest) = (number / 20, number % 20);
    if rest == 0 {
        TWENTIES[index(twenties)].to_owned()
    } else {
        format!("{}ta {}", TWENTIES[index(twenties)], UNITS[index(rest)])
    }
}

/// `number` below a thousand, with eta after the hundreds (ehun eta bat).
fn below_thousand(number: u64) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    match (hundreds, rest) {
        (0, _) => below_hundred(rest),
        (_, 0) => HUNDREDS[index(hundreds)].to_owned(),
        _ => format!("{} eta {}", HUNDREDS[index(hundreds)], below_hundred(rest)),
    }
}

/// `number` in words: 2024 bi mila eta hogeita lau, 1000 mila, 1000000 milioi bat.
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    let millions = number / 1_000_000;
    if millions == 1 {
        parts.push("milioi bat".to_owned());
    } else if millions > 1 {
        parts.push(format!("{} milioi", cardinal(millions)));
    }
    let thousands = number / 1000 % 1000;
    if thousands == 1 {
        parts.push("mila".to_owned());
    } else if thousands > 1 {
        parts.push(format!("{} mila", below_thousand(thousands)));
    }
    let rest = number % 1000;
    if rest > 0 {
        // After mila the last part takes eta when it is below a hundred or a round hundred.
        let joined = !parts.is_empty() && (rest < 100 || rest.is_multiple_of(100));
        let words = below_thousand(rest);
        parts.push(if joined { format!("eta {words}") } else { words });
    }
    parts.join(" ")
}

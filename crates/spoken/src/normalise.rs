//! What each token of a sentence is said as, in the words of a [`Locale`]: numbers, ordinals,
//! times, dates, money, percentages, symbols, abbreviations, initialisms, Roman numerals,
//! addresses and elisions.

use jespersen_text::TokenKind;

use crate::{Gender, Locale, roman::roman};

/// What a token is said as.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Spoken {
    /// Words, in the language's spelling; a word ending in an apostrophe is elided onto the next
    /// (Italian l', dell').
    Words(Vec<String>),
    /// The names of letters, spelling a word out (DVD), which keep their stress.
    Spelt(Vec<String>),
    /// Nothing: punctuation, or a sign read with the token before or after it.
    Silent,
    /// Part of the token before, read with it (the . and º of 3.º).
    Covered,
    /// Something jespersen cannot read yet.
    Unreadable,
}

/// The longest whole number read as a quantity; longer ones are read digit by digit.
const LONGEST_NUMBER: usize = 15;

/// What each token of a sentence (text and kind) is said as. A number takes its sign or unit from
/// the token after it (50 %, 5 €), which is then silent; a currency sign before a number ($5) is
/// read after it; an ordinal sign after it (3.º, 1°) makes it an ordinal, the sign covered.
pub fn spoken(tokens: &[(&str, TokenKind)], locale: &impl Locale) -> Vec<Spoken> {
    let mut out: Vec<Spoken> = Vec::with_capacity(tokens.len());
    while out.len() < tokens.len() {
        let index = out.len();
        let (text, kind) = tokens[index];
        let next = tokens.get(index + 1).copied();
        let before = index.checked_sub(1).map(|before| tokens[before]);
        match kind {
            TokenKind::Punctuation => out.push(Spoken::Silent),
            TokenKind::Number => number_token(tokens, index, locale, &mut out),
            // A currency written in letters before its number (Indonesian Rp 1.250) is read after it.
            TokenKind::Word
                if locale.currency(text).is_some() && next.is_some_and(|(_, kind)| kind == TokenKind::Number) =>
            {
                out.push(Spoken::Silent);
            }
            TokenKind::Word if locale.elides() && text.chars().count() <= 3 && matches!(next, Some(("'" | "’", _))) =>
            {
                // A short word elided at its end (Italian po', mo').
                out.push(Spoken::Words(vec![format!("{text}'")]));
                out.push(Spoken::Covered);
            }
            TokenKind::Word => out.push(word(
                text,
                before.map(|(word, _)| word),
                next.map(|(word, _)| word),
                locale,
            )),
            TokenKind::Symbol => {
                let before_number = next.is_some_and(|(_, kind)| kind == TokenKind::Number);
                if before_number && locale.percent_first() && matches!(text, "%" | "％") {
                    out.push(Spoken::Words(split(locale.percent())));
                } else if locale.currency(text).is_some() && before_number {
                    out.push(Spoken::Silent);
                } else {
                    out.push(
                        locale
                            .symbol(text)
                            .map_or(Spoken::Unreadable, |words| Spoken::Words(split(words))),
                    );
                }
            }
        }
    }
    out
}

/// Words separated by spaces; an empty string is no word (a language without a word for a range).
fn split(words: &str) -> Vec<String> {
    words
        .split(' ')
        .filter(|word| !word.is_empty())
        .map(str::to_owned)
        .collect()
}

/// Reads the number token at `index`, with the tokens around it that belong to it.
fn number_token(tokens: &[(&str, TokenKind)], index: usize, locale: &impl Locale, out: &mut Vec<Spoken>) {
    let (text, _) = tokens[index];
    let next = tokens.get(index + 1).copied();
    let gender_after = |at: usize| match tokens.get(at) {
        Some((word, TokenKind::Word)) => locale.gender_of(word),
        _ => Gender::Alone,
    };
    // An ordinal sign after it, alone or after a full stop (1°, 3.º).
    let sign = |at: usize| {
        let (text, _) = tokens.get(at)?;
        locale.ordinal_signs().iter().find(|(sign, _)| sign == text).copied()
    };
    let sign_at = match (next, sign(index + 1), sign(index + 2)) {
        (_, Some(found), _) => Some((1, found)),
        (Some((".", _)), None, Some(found)) => Some((2, found)),
        _ => None,
    };
    if let Some((skip, (_, gender))) = sign_at
        && let Ok(value) = text.parse::<u64>()
        && let Some(words) = ordinal(locale, value, gender, gender_after(index + 1 + skip))
    {
        out.push(Spoken::Words(split(&words)));
        out.extend(std::iter::repeat_n(Spoken::Covered, skip));
        return;
    }
    let before = index.checked_sub(1).map(|before| tokens[before]);
    let is_percent = |sign: &str| !locale.percent_first() && matches!(sign, "%" | "％");
    let prefix = before
        .map(|(sign, _)| sign)
        .filter(|sign| locale.currency(sign).is_some() && out.last() == Some(&Spoken::Silent));
    let unit = next
        .map(|(sign, _)| sign)
        .filter(|sign| locale.currency(sign).is_some() || is_percent(sign));
    // A number agrees with its currency's name (Czech jedna koruna), a percentage is masculine.
    let gender = match prefix.or(unit) {
        Some(sign) => locale
            .currency(sign)
            .map_or(Gender::Masculine, |(one, _)| match locale.gender_of(one) {
                Gender::Alone => Gender::Masculine,
                gender => gender,
            }),
        None => gender_after(index + 1),
    };
    let Some(mut words) = number(text, gender, gender_after(index + 1), locale) else {
        out.push(Spoken::Unreadable);
        return;
    };
    if let Some(sign) = prefix.or(unit) {
        // The count a currency agrees with: the whole number, its thousands marked or not (1.250).
        let count = text.trim_start_matches(['+', '-']).replace('.', "").parse().ok();
        let name = if is_percent(sign) {
            Some(locale.percent())
        } else {
            locale.currency_name(sign, count)
        };
        words.extend(name.map(split).unwrap_or_default());
    }
    out.push(Spoken::Words(words));
    if unit.is_some() {
        out.push(Spoken::Silent);
    }
}

/// An ordinal of `gender` (from its sign), agreeing with the word after it when the sign leaves
/// it open (el tercer piso); a cardinal where the language has no ordinal.
fn ordinal(locale: &impl Locale, value: u64, gender: Gender, after: Gender) -> Option<String> {
    let gender = if gender == Gender::Alone && after == Gender::Masculine {
        Gender::Masculine
    } else {
        gender
    };
    locale
        .ordinal(value, gender)
        .or_else(|| locale.cardinal(value, Gender::Alone))
}

/// A word: an address piece by piece, an abbreviation in full, a Roman numeral where the language
/// reads one, an initialism spelt unless it can be said as a word (NATO, but DVD), an elided word
/// split at its apostrophe (l'amico: l' amico), or the word.
fn word(text: &str, before: Option<&str>, after: Option<&str>, locale: &impl Locale) -> Spoken {
    if text.contains('@') || text.contains("://") || text.starts_with("www.") {
        return address(text, locale).map_or(Spoken::Unreadable, Spoken::Words);
    }
    if let Some(said) = locale.abbreviation(text) {
        return Spoken::Words(split(said));
    }
    if let Some(value) = roman(text)
        && let Some(said) = locale.roman(value, before, after)
    {
        return Spoken::Words(split(&said));
    }
    let letters: Vec<char> = text.chars().collect();
    if letters.len() >= 2 && letters.iter().all(|c| c.is_uppercase()) && !sayable(text, locale) {
        return spell(&letters, locale).map_or(Spoken::Unreadable, Spoken::Spelt);
    }
    // A hyphenated word with an apostrophe after its hyphen (Catalan menjar-se'l) is left whole.
    if locale.elides()
        && let Some(at) = text.find(['\'', '’'])
        && at > 0
        && !text[..at].contains('-')
    {
        let apostrophe = text[at..].chars().next().map_or(1, char::len_utf8);
        let (head, tail) = (&text[..at], &text[at + apostrophe..]);
        let mut words = vec![format!("{head}'")];
        if !tail.is_empty() {
            words.push(tail.to_owned());
        }
        return Spoken::Words(words);
    }
    Spoken::Words(vec![text.to_owned()])
}

/// Letters by their names.
fn spell(letters: &[char], locale: &impl Locale) -> Option<Vec<String>> {
    let names: Vec<&str> = letters
        .iter()
        .map(|letter| locale.letter(*letter))
        .collect::<Option<_>>()?;
    Some(
        names
            .iter()
            .flat_map(|name| name.split(' '))
            .map(str::to_owned)
            .collect(),
    )
}

/// Whether an initialism can be said as a word: three letters or more, a vowel, no three
/// consonants in a row, none at its start but before r or l and none at its end (NATO, ONU; DVD,
/// PSOE and ONZ spelt).
fn sayable(text: &str, locale: &impl Locale) -> bool {
    let letters: Vec<char> = text.to_lowercase().chars().collect();
    let vowel = |c: &char| locale.vowels().contains(*c);
    let three_consonants = letters.windows(3).any(|three| !three.iter().any(vowel));
    let opens_with_cluster =
        matches!(&letters[..], [first, second, ..] if !vowel(first) && !vowel(second) && !matches!(second, 'r' | 'l'));
    let closes_with_cluster = matches!(&letters[..], [.., second, last] if !vowel(second) && !vowel(last));
    letters.len() >= 3 && letters.iter().any(vowel) && !three_consonants && !opens_with_cluster && !closes_with_cluster
}

/// A number token in words: signed numbers, times (10:30), dates (3/5/2024, 2024-05-03), ranges
/// (1990-1995) and ratios (2/3), ordinals with their sign on them (3º, 2ª, 1er), and whole
/// numbers and decimals; `gender` is that of what it counts, `after` that of the word after it.
fn number(text: &str, gender: Gender, after: Gender, locale: &impl Locale) -> Option<Vec<String>> {
    if let Some(rest) = text.strip_prefix(['-', '−']) {
        let mut words = split(locale.minus());
        words.extend(number(rest, gender, after, locale)?);
        return Some(words);
    }
    if let Some(rest) = text.strip_prefix('+') {
        let mut words = split(locale.plus());
        words.extend(number(rest, gender, after, locale)?);
        return Some(words);
    }
    for (sign, sign_gender) in locale.ordinal_signs() {
        if let Some(digits) = text.strip_suffix(sign).map(|digits| digits.trim_end_matches('.'))
            && let Ok(value) = digits.parse::<u64>()
        {
            return ordinal(locale, value, *sign_gender, after).map(|words| split(&words));
        }
    }
    if let Some((hours, minutes)) = text.split_once(':') {
        return locale
            .time(hours.parse().ok()?, minutes.parse().ok()?)
            .map(|words| split(&words));
    }
    // A date written with points (3.5.2024), as much of Europe writes it.
    let points: Vec<&str> = text.split('.').collect();
    if let [day, month, year] = points[..]
        && day.len() <= 2
        && month.len() <= 2
        && matches!(year.len(), 2 | 4)
        && let (Ok(day), Ok(month), Ok(year)) = (day.parse(), month.parse(), year.parse())
    {
        return locale.date(day, month, year).map(|words| split(&words));
    }
    let parts: Vec<&str> = text.split(['/', '-']).collect();
    if let [first, second, third] = parts[..] {
        let (day, month, year) = if first.len() == 4 {
            (third, second, first)
        } else {
            (first, second, third)
        };
        return locale
            .date(day.parse().ok()?, month.parse().ok()?, year.parse().ok()?)
            .map(|words| split(&words));
    }
    if let [low, high] = parts[..]
        && !low.is_empty()
    {
        let mut words = number(low, gender, after, locale)?;
        words.extend(split(if text.contains('/') {
            locale.ratio()
        } else {
            locale.range()
        }));
        words.extend(number(high, gender, after, locale)?);
        return Some(words);
    }
    whole_or_decimal(text, gender, locale).map(|words| split(&words))
}

/// A number with thousands marked by points (1.250) or by commas between groups of three, and a
/// decimal comma or point (3,5; 3.5).
fn whole_or_decimal(text: &str, gender: Gender, locale: &impl Locale) -> Option<String> {
    let grouped = |separator: char| {
        let groups: Vec<&str> = text.split(separator).collect();
        groups.len() > 1 && groups[1..].iter().all(|group| group.len() == 3) && (1..=3).contains(&groups[0].len())
    };
    let (whole, decimals, mark) = if grouped('.') && !text.contains(',') {
        (text.replace('.', ""), None, ' ')
    } else if let Some((whole, decimals)) = text.split_once(',') {
        (whole.replace('.', ""), Some(decimals), ',')
    } else if grouped(',') {
        (text.replace(',', ""), None, ' ')
    } else if let Some((whole, decimals)) = text.split_once('.') {
        (whole.to_owned(), Some(decimals), '.')
    } else {
        (text.to_owned(), None, ' ')
    };
    if whole.is_empty() || !whole.bytes().all(|digit| digit.is_ascii_digit()) {
        return None;
    }
    // Codes and telephone numbers: a leading zero, or more digits than a quantity has.
    if (whole.len() > 1 && whole.starts_with('0')) || whole.len() > LONGEST_NUMBER {
        return digits(&whole, locale);
    }
    let value: u64 = whole.parse().ok()?;
    match decimals {
        None => locale.cardinal(value, gender),
        Some(decimals) if !decimals.is_empty() && decimals.bytes().all(|digit| digit.is_ascii_digit()) => {
            let decimals = if decimals.starts_with('0') {
                digits(decimals, locale)?
            } else {
                locale.cardinal(decimals.parse().ok()?, Gender::Alone)?
            };
            Some(format!(
                "{} {} {decimals}",
                locale.cardinal(value, Gender::Alone)?,
                locale.decimal_word(mark)
            ))
        }
        Some(_) => None,
    }
}

/// Each digit on its own.
fn digits(digits: &str, locale: &impl Locale) -> Option<String> {
    let words: Vec<String> = digits
        .bytes()
        .map(|digit| locale.cardinal(u64::from(digit - b'0'), Gender::Alone))
        .collect::<Option<_>>()?;
    Some(words.join(" "))
}

/// A web or e-mail address, said piece by piece: its words, and its signs by name; pieces without
/// a vowel (https, www) are spelt.
fn address(text: &str, locale: &impl Locale) -> Option<Vec<String>> {
    let mut words = Vec::new();
    let mut piece = String::new();
    for c in text.chars() {
        match locale.address_sign(c) {
            Some(sign) => {
                if !piece.is_empty() {
                    words.push(std::mem::take(&mut piece));
                }
                words.extend(split(sign));
            }
            None => piece.push(c),
        }
    }
    if !piece.is_empty() {
        words.push(piece);
    }
    let mut out = Vec::new();
    for word in words {
        if word.to_lowercase().chars().any(|c| locale.vowels().contains(c)) {
            out.push(word);
        } else {
            out.extend(spell(&word.chars().collect::<Vec<_>>(), locale)?);
        }
    }
    Some(out)
}

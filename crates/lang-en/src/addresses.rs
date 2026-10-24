//! Web and e-mail addresses, and dotted names: `example.com` is "example dot com",
//! `john.smith@example.co.uk` "john dot smith at example dot c o dot u k",
//! `https://www.example.com/page` "w w w dot example dot com slash page" (the scheme is not read).

use crate::numbers::digits;

/// The words of an address (a token with `://`, `@`, or a leading `www.`) or of a dotted name
/// (example.com: parts of two characters or more, ending in letters); `None` for other words.
pub fn read(text: &str) -> Option<Vec<String>> {
    let is_address = text.contains("://") || text.contains('@') || text.to_ascii_lowercase().starts_with("www.");
    if !is_address {
        let parts: Vec<&str> = text.split('.').collect();
        let dotted = parts.len() >= 2
            && parts
                .iter()
                .all(|part| part.chars().count() >= 2 && part.chars().all(char::is_alphanumeric))
            && parts.last().is_some_and(|last| last.chars().all(char::is_alphabetic));
        return dotted.then(|| dots(text));
    }
    let rest = text.find("://").map_or(text, |at| &text[at + 3..]);
    let mut words = Vec::new();
    for (index, part) in rest.split('@').enumerate() {
        if index > 0 {
            words.push("at".to_owned());
        }
        for (index, segment) in part.split('/').filter(|segment| !segment.is_empty()).enumerate() {
            if index > 0 {
                words.push("slash".to_owned());
            }
            for piece in segment
                .split(['?', '=', '&', '#', '_', '-', '+', '%'])
                .filter(|piece| !piece.is_empty())
            {
                words.extend(dots(piece));
            }
        }
    }
    Some(words)
}

/// A name's parts with "dot" between them: a part of digits read digit by digit, a part of two
/// letters or with no vowel letter spelled (www, uk), others as words.
fn dots(name: &str) -> Vec<String> {
    let mut words = Vec::new();
    for (index, part) in name.split('.').filter(|part| !part.is_empty()).enumerate() {
        if index > 0 {
            words.push("dot".to_owned());
        }
        if part.chars().all(|c| c.is_ascii_digit()) {
            words.extend(digits(part).into_iter().map(str::to_owned));
        } else if part.chars().count() <= 2 || !part.chars().any(|c| "aeiouyAEIOUY".contains(c)) {
            // A letter with a full stop is read as the letter (see spelling.rs).
            words.extend(
                part.chars()
                    .filter(char::is_ascii_alphanumeric)
                    .map(|letter| format!("{letter}.")),
            );
        } else {
            words.push(part.to_owned());
        }
    }
    words
}

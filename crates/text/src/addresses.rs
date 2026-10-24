use std::ops::Range;

/// The byte ranges of web and e-mail addresses in `text`, so that each is one token rather than
/// the pieces word boundaries would cut it into: a scheme and `://` (`https://example.com/page`), a
/// `www.` name, or a name `@` a domain (`john.smith@example.co.uk`). Trailing punctuation that ends a
/// sentence or clause is not part of an address.
pub(crate) fn addresses(text: &str) -> Vec<Range<usize>> {
    let mut found: Vec<Range<usize>> = Vec::new();
    let starts_word = |at: usize| text[..at].chars().next_back().is_none_or(|c| !c.is_alphanumeric());
    let run_end = |from: usize| {
        text[from..]
            .find(char::is_whitespace)
            .map_or(text.len(), |space| from + space)
    };
    let trimmed = |range: Range<usize>| {
        let kept = text[range.clone()].trim_end_matches(['.', ',', ';', ':', '!', '?', ')', '"', '\'', '”', '’']);
        range.start..range.start + kept.len()
    };
    for (at, _) in text.char_indices() {
        if found.last().is_some_and(|last| at < last.end) || !starts_word(at) {
            continue;
        }
        let rest = &text[at..];
        let scheme = rest
            .find("://")
            .filter(|&colon| colon > 0 && rest[..colon].chars().all(|c| c.is_ascii_alphabetic()));
        if scheme.is_some() || rest.to_ascii_lowercase().starts_with("www.") {
            found.push(trimmed(at..run_end(at)));
            continue;
        }
        // An e-mail address: a name of letters, digits and . _ % + - before @ and a dotted domain.
        let name_end = rest
            .find(|c: char| !(c.is_alphanumeric() || "._%+-".contains(c)))
            .unwrap_or(rest.len());
        if name_end > 0 && rest[name_end..].starts_with('@') {
            let domain = &rest[name_end + 1..];
            let domain_len = domain
                .find(|c: char| !(c.is_alphanumeric() || ".-".contains(c)))
                .unwrap_or(domain.len());
            let range = trimmed(at..at + name_end + 1 + domain_len);
            if text[range.clone()]
                .split('@')
                .nth(1)
                .is_some_and(|domain| domain.contains('.'))
            {
                found.push(range);
            }
        }
    }
    found
}

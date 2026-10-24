use jespersen::{Error, Language};

/// The sentences of `prose`, one per line as the text lists want them, as jespersen splits them in
/// `language`, with runs of spaces and line breaks inside a sentence made one space. Mandarin,
/// written without spaces, has its lines joined first: a space or line break there is only
/// layout, except between two Latin letters or digits.
///
/// # Errors
///
/// When an embedded lexicon is damaged.
pub fn sentences(prose: &str, language: Language) -> Result<Vec<String>, Error> {
    let joined;
    let prose = if language.tag() == "zh" {
        joined = join_lines(prose);
        joined.as_str()
    } else {
        prose
    };
    Ok(jespersen::phonemize(prose, language)?
        .iter()
        .map(|sentence| sentence.text.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|sentence| !sentence.is_empty())
        .collect())
}

/// Chinese prose with the spaces and line breaks of its layout taken out.
fn join_lines(prose: &str) -> String {
    let mut out = String::with_capacity(prose.len());
    let mut space = false;
    for c in prose.chars() {
        if c.is_whitespace() {
            space = true;
            continue;
        }
        if space && out.chars().last().is_some_and(|last| last.is_ascii_alphanumeric()) && c.is_ascii_alphanumeric() {
            out.push(' ');
        }
        space = false;
        out.push(c);
    }
    out
}

use crate::Rule;

/// What a rule that says nothing is written as.
const SILENT: &str = "-";

/// Rules as text: `header` (each line as a `# ` comment), then one `context<TAB>phonemes` line per
/// rule, phonemes named by `name` and separated by spaces, `-` for none.
#[must_use]
pub fn write_table(rules: &[Rule], header: &str, name: impl Fn(u8) -> String) -> String {
    let mut out = String::new();
    for line in header.lines() {
        out.push_str("# ");
        out.push_str(line);
        out.push('\n');
    }
    for rule in rules {
        out.push_str(&rule.context);
        out.push('\t');
        if rule.phonemes.is_empty() {
            out.push_str(SILENT);
        } else {
            let names: Vec<String> = rule.phonemes.iter().map(|&code| name(code)).collect();
            out.push_str(&names.join(" "));
        }
        out.push('\n');
    }
    out
}

/// Reads [`write_table`]'s text back, phonemes parsed by `code`. Comment lines (`# `, a hash and a
/// space: contexts never hold a space but may start with `#`) and blank lines are
/// skipped.
///
/// # Errors
///
/// The number and text of the first line that is not `context<TAB>phonemes` with known phonemes.
pub fn read_table(text: &str, code: impl Fn(&str) -> Option<u8>) -> Result<Vec<Rule>, String> {
    let mut rules = Vec::new();
    for (number, line) in text.lines().enumerate() {
        if line.is_empty() || line == "#" || line.starts_with("# ") {
            continue;
        }
        let bad = || format!("line {}: {line:?}", number + 1);
        let (context, said) = line.split_once('\t').ok_or_else(bad)?;
        let phonemes = if said == SILENT {
            Vec::new()
        } else {
            said.split(' ')
                .map(|name| code(name).ok_or_else(bad))
                .collect::<Result<_, _>>()?
        };
        rules.push(Rule {
            context: context.to_owned(),
            phonemes,
        });
    }
    Ok(rules)
}

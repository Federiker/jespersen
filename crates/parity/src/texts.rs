/// The texts of a list: one per line; blank lines and lines starting with `#` are left out.
#[must_use]
pub fn read_texts(list: &str) -> Vec<String> {
    list.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect()
}

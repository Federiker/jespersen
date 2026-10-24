/// jespersen's Czech word list (data/cs/words.tsv): loanwords respelt for the hard d, t and n the
/// rules would soften.
const WORDS: &str = include_str!("../../../data/cs/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (univerzita → unyverzita).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

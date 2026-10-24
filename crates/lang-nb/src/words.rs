/// jespersen's Norwegian word list (data/nb/words.tsv): words respelt for what the spelling rules
/// miss (og → å, jeg → jei).
const WORDS: &str = include_str!("../../../data/nb/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (og → å, jeg → jei).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

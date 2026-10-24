/// jespersen's Swedish word list (data/sv/words.tsv): words respelt for what the spelling rules
/// miss (och → ock, miljon → miljón).
const WORDS: &str = include_str!("../../../data/sv/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (och → ock, miljon → miljón).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

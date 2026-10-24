/// jespersen's Portuguese word list (data/pt/words.tsv): words respelt for the open e and o and the x
/// the rules would miss.
const WORDS: &str = include_str!("../../../data/pt/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (ela → éla, táxi → táksi).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

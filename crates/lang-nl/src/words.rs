/// jespersen's Dutch word list (data/nl/words.tsv): words respelt for what the spelling rules
/// miss (waarom → waaróm).
const WORDS: &str = include_str!("../../../data/nl/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (waarom → waaróm).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

/// jespersen's Catalan word list (data/ca/words.tsv): words respelt for what the spelling rules
/// miss (tot → tót, amor → amorr).
const WORDS: &str = include_str!("../../../data/ca/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (no → nó).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

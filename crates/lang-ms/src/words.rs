/// jespersen's Malay word list (data/ms/words.tsv): words whose e is e (taling), respelt with é.
const WORDS: &str = include_str!("../../../data/ms/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (meja → méja).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

/// jespersen's Romanian word list (data/ro/words.tsv): words respelt for the j that starts the forms
/// of a fi and the pronouns in e.
const WORDS: &str = include_str!("../../../data/ro/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (este → ieste).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

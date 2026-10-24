/// jespersen's Slovenian word list (data/sl/words.tsv): words respelt for the schwa and the open e
/// and o the rules would miss.
const WORDS: &str = include_str!("../../../data/sl/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (sem → səm).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

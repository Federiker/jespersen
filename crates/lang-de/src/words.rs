/// jespersen's German word list (data/de/words.tsv): words respelt for what the spelling rules
/// miss (Erde → Ehrde, gegen → gégen).
const WORDS: &str = include_str!("../../../data/de/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (Erde → Ehrde).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

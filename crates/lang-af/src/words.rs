/// jespersen's Afrikaans word list (data/af/words.tsv): words respelt for what the spelling rules
/// miss (kombuis → kombúis).
const WORDS: &str = include_str!("../../../data/af/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (kombuis → kombúis).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

/// jespersen's Danish word list (data/da/words.tsv): words respelt for what the spelling rules
/// miss (det → de, af → a).
const WORDS: &str = include_str!("../../../data/da/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (det → de, af → a).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

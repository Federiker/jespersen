/// jespersen's Welsh word list (data/cy/words.tsv): words respelt for what the spelling rules
/// miss (Cymraeg → Cymráeg).
const WORDS: &str = include_str!("../../../data/cy/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (Cymraeg → Cymráeg).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

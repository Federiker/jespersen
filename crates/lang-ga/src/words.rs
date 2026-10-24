/// jespersen's Irish word list (data/ga/words.tsv): words respelt for what the spelling rules
/// miss (amach → aˈmach).
const WORDS: &str = include_str!("../../../data/ga/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (amach → aˈmach).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

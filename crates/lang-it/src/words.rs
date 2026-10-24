/// jespersen's Italian word list (data/it/words.tsv): words respelt with the accents Italian
/// dictionaries use, for the stress and the open or closed e and o the rules would miss.
const WORDS: &str = include_str!("../../../data/it/words.tsv");

/// The respelling of `word` (in lower case), if the list has one (tavolo → tàvolo, bene → bène).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt.split('\t').next().unwrap_or(respelt))
}

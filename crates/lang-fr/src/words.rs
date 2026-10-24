/// jespersen's French word list (data/fr/words.tsv): words respelt for the final consonants, vowels
/// and digraphs the rules would miss.
const WORDS: &str = include_str!("../../../data/fr/words.tsv");
/// jespersen's list of words with an aspirated h (data/fr/aspirated.txt).
const ASPIRATED: &str = include_str!("../../../data/fr/aspirated.txt");

/// The respelling of `word` (in lower case), if the list has one (fils → fisse, femme → famme).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

/// Whether `word` (in lower case) starts with an aspirated h, or is one of the words that block
/// liaison as one does (onze, huit, oui); plurals and feminines included (héros, hautes).
pub(crate) fn is_aspirated(word: &str) -> bool {
    let stem = word.trim_end_matches(['s', 'x']);
    ASPIRATED
        .lines()
        .filter(|line| !line.starts_with('#'))
        .any(|entry| entry == word || entry == stem || stem.strip_suffix('e') == Some(entry))
}

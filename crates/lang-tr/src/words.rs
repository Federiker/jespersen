/// jespersen's Turkish word list (data/tr/words.tsv): words not stressed on their last syllable,
/// with ˈ before the stressed vowel.
const WORDS: &str = include_str!("../../../data/tr/words.tsv");

/// The word with its stress marked, if the list has it (ankara → ˈankara).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    WORDS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .find(|(entry, _)| *entry == word)
        .map(|(_, marked)| marked)
}

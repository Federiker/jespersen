//! The letters of a word grouped by the vowels it is said with, for rules the spelling decides.

/// A run of vowel letters in a word's spelling, with what surrounds it.
#[derive(Debug)]
pub struct Group {
    pub letters: String,
    /// Where the group starts, counting the spelling's letters only.
    pub start: usize,
    pub before: Option<char>,
    /// The letter after the group (the w of new, for ew).
    pub after: Option<char>,
    /// The group follows qu (quality: the u is part of the consonant).
    pub after_qu: bool,
    /// An rr follows (hurry, worry).
    pub doubled_r: bool,
}

/// The vowel letter groups of a lower-case spelling, one per vowel the word is said with when the
/// spelling is regular: y counts as a vowel letter except at the start, u after q does not, and a
/// final silent e (rate) or the e of -es and -ed after a consonant they do not sound with (hopes,
/// baked) is left out.
pub fn vowel_groups(spelling: &str) -> Vec<Group> {
    let letters: Vec<char> = spelling.chars().filter(char::is_ascii_lowercase).collect();
    let is_vowel = |index: usize| {
        let letter = letters[index];
        "aeiou".contains(letter) && !(letter == 'u' && index > 0 && letters[index - 1] == 'q')
            || (letter == 'y' && index > 0)
    };
    let mut groups: Vec<Group> = Vec::new();
    let mut index = 0;
    while index < letters.len() {
        if !is_vowel(index) {
            index += 1;
            continue;
        }
        let start = index;
        while index < letters.len() && is_vowel(index) {
            index += 1;
        }
        groups.push(Group {
            letters: letters[start..index].iter().collect(),
            start,
            before: start.checked_sub(1).map(|before| letters[before]),
            after: letters.get(index).copied(),
            after_qu: start >= 2 && letters[start - 2] == 'q' && letters[start - 1] == 'u',
            doubled_r: letters.get(index) == Some(&'r') && letters.get(index + 1) == Some(&'r'),
        });
    }
    let silent_e = |groups: &[Group]| {
        let n = letters.len();
        let Some(last) = groups.last() else { return false };
        if groups.len() < 2 || last.letters != "e" {
            return false;
        }
        match &letters[..] {
            [.., consonant, 'e'] if !"aeiouy".contains(*consonant) => true,
            [.., before, 'e', 's'] => !"szxhcgj".contains(*before),
            [.., before, 'e', 'd'] => !"td".contains(*before) && n > 3,
            _ => false,
        }
    };
    if silent_e(&groups) {
        groups.pop();
    }
    groups
}

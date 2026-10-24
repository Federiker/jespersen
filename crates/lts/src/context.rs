/// The contexts a letter is looked up in, from the least to the most specific: how many letters
/// to its left and to its right. A rule's context is one of these.
pub const CONTEXTS: [(usize, usize); 11] = [
    (0, 0),
    (1, 0),
    (1, 1),
    (2, 1),
    (2, 2),
    (3, 2),
    (3, 3),
    (4, 3),
    (4, 4),
    (5, 4),
    (5, 5),
];

/// What stands for the space beyond either end of the word in a context.
pub const BOUNDARY: char = '#';

/// The key of letter `index` of `letters` in context `(left, right)`: the letters around it, with
/// the letter itself in brackets (`ph[o]n`, `#[c]at`).
///
/// # Panics
///
/// When `index` is not a letter of `letters`.
#[must_use]
pub fn key(letters: &[char], index: usize, (left, right): (usize, usize)) -> String {
    let mut key = String::with_capacity(left + right + 3);
    for offset in (1..=left).rev() {
        key.push(index.checked_sub(offset).map_or(BOUNDARY, |at| letters[at]));
    }
    key.push('[');
    key.push(letters[index]);
    key.push(']');
    for offset in 1..=right {
        key.push(letters.get(index + offset).copied().unwrap_or(BOUNDARY));
    }
    key
}

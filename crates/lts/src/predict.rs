use crate::{CONTEXTS, key};

/// The phonemes of `letters`, each letter read with the most specific rule `lookup` finds for it
/// (`lookup` returns the rule's phonemes for a context key). `None` when a letter has no rule at
/// all, not even a context-free one.
///
/// # Errors
///
/// When `lookup` fails.
pub fn predict<E>(letters: &[char], lookup: impl Fn(&str) -> Result<Option<Vec<u8>>, E>) -> Result<Option<Vec<u8>>, E> {
    let mut phonemes = Vec::with_capacity(letters.len());
    for index in 0..letters.len() {
        let mut found = None;
        for &context in CONTEXTS.iter().rev() {
            if let Some(said) = lookup(&key(letters, index, context))? {
                found = Some(said);
                break;
            }
        }
        let Some(said) = found else { return Ok(None) };
        phonemes.extend(said);
    }
    Ok(Some(phonemes))
}

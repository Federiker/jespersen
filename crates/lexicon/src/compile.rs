use std::collections::BTreeMap;

use crate::{
    LexiconError,
    format::{HEADER_LEN, MAGIC, VERSION},
};

/// Compiles entries into a lexicon's bytes. A key may come several times: its values are kept in
/// the order they came, duplicates included.
///
/// # Errors
///
/// When a key is empty or contains NUL, a value is longer than 255 bytes, or the result would not
/// fit the format's 32-bit counts.
pub fn compile<K, V>(entries: impl IntoIterator<Item = (K, V)>) -> Result<Vec<u8>, LexiconError>
where
    K: Into<String>,
    V: Into<Vec<u8>>,
{
    let mut sorted: BTreeMap<String, Vec<Vec<u8>>> = BTreeMap::new();
    for (key, value) in entries {
        let key = key.into();
        let value = value.into();
        if key.is_empty() || key.contains('\0') {
            return Err(LexiconError::InvalidKey(key));
        }
        if value.len() > usize::from(u8::MAX) {
            return Err(LexiconError::ValueTooLong { key, len: value.len() });
        }
        sorted.entry(key).or_default().push(value);
    }

    let count = u32::try_from(sorted.len()).map_err(|_| LexiconError::TooLarge)?;
    let mut offsets = Vec::with_capacity(sorted.len());
    let mut entries = Vec::new();
    for (key, values) in &sorted {
        offsets.push(u32::try_from(entries.len()).map_err(|_| LexiconError::TooLarge)?);
        entries.extend_from_slice(key.as_bytes());
        entries.push(0);
        for value in values {
            // Checked above: at most 255 bytes.
            entries.push(u8::try_from(value.len()).map_err(|_| LexiconError::TooLarge)?);
            entries.extend_from_slice(value);
        }
    }
    u32::try_from(entries.len()).map_err(|_| LexiconError::TooLarge)?;

    let mut bytes = Vec::with_capacity(HEADER_LEN + offsets.len() * 4 + entries.len());
    bytes.extend_from_slice(MAGIC);
    bytes.push(VERSION);
    bytes.extend_from_slice(&count.to_le_bytes());
    for offset in offsets {
        bytes.extend_from_slice(&offset.to_le_bytes());
    }
    bytes.extend_from_slice(&entries);
    Ok(bytes)
}

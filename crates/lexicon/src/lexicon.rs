use std::cmp::Ordering;

use crate::{
    LexiconError,
    format::{HEADER_LEN, MAGIC, VERSION},
};

/// A compiled lexicon, searched in place.
#[derive(Clone, Copy, Debug)]
pub struct Lexicon<'a> {
    offsets: &'a [u8],
    entries: &'a [u8],
    count: usize,
}

impl<'a> Lexicon<'a> {
    /// Opens compiled bytes (see [`crate::compile`]).
    ///
    /// # Errors
    ///
    /// [`LexiconError::NotALexicon`] for other bytes or another version, [`LexiconError::Damaged`]
    /// when the offset table does not fit.
    pub fn new(bytes: &'a [u8]) -> Result<Self, LexiconError> {
        if bytes.len() < HEADER_LEN || &bytes[..4] != MAGIC || bytes[4] != VERSION {
            return Err(LexiconError::NotALexicon);
        }
        let count = usize::try_from(read_u32(&bytes[5..9])).map_err(|_| LexiconError::Damaged)?;
        let table_end = count
            .checked_mul(4)
            .and_then(|len| len.checked_add(HEADER_LEN))
            .filter(|&end| end <= bytes.len())
            .ok_or(LexiconError::Damaged)?;
        Ok(Self {
            offsets: &bytes[HEADER_LEN..table_end],
            entries: &bytes[table_end..],
            count,
        })
    }

    /// How many distinct keys the lexicon holds.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.count
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// The values of `key`, in the order they were compiled; `None` when the key is absent.
    ///
    /// # Errors
    ///
    /// [`LexiconError::Damaged`] when an offset or a value's length points outside the bytes.
    pub fn get(&self, key: &str) -> Result<Option<Vec<&'a [u8]>>, LexiconError> {
        let (mut low, mut high) = (0, self.count);
        while low < high {
            let middle = low + (high - low) / 2;
            let (entry_key, values) = self.entry(middle)?;
            match entry_key.cmp(key.as_bytes()) {
                Ordering::Less => low = middle + 1,
                Ordering::Greater => high = middle,
                Ordering::Equal => return split_values(values).map(Some),
            }
        }
        Ok(None)
    }

    /// Entry `index`: its key and its encoded values.
    fn entry(&self, index: usize) -> Result<(&'a [u8], &'a [u8]), LexiconError> {
        let start = self.offset(index)?;
        let end = if index + 1 < self.count {
            self.offset(index + 1)?
        } else {
            self.entries.len()
        };
        let entry = self.entries.get(start..end).ok_or(LexiconError::Damaged)?;
        let nul = entry.iter().position(|&byte| byte == 0).ok_or(LexiconError::Damaged)?;
        Ok((&entry[..nul], &entry[nul + 1..]))
    }

    fn offset(&self, index: usize) -> Result<usize, LexiconError> {
        let at = index * 4;
        let raw = self.offsets.get(at..at + 4).ok_or(LexiconError::Damaged)?;
        usize::try_from(read_u32(raw)).map_err(|_| LexiconError::Damaged)
    }
}

fn split_values(mut bytes: &[u8]) -> Result<Vec<&[u8]>, LexiconError> {
    let mut values = Vec::new();
    while let Some((&len, rest)) = bytes.split_first() {
        let len = usize::from(len);
        let value = rest.get(..len).ok_or(LexiconError::Damaged)?;
        values.push(value);
        bytes = &rest[len..];
    }
    Ok(values)
}

fn read_u32(bytes: &[u8]) -> u32 {
    let mut array = [0; 4];
    array.copy_from_slice(&bytes[..4]);
    u32::from_le_bytes(array)
}

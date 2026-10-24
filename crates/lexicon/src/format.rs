//! The compiled form, version 1, all integers little-endian:
//!
//! ```text
//! "PKLX" | version: u8 | count: u32 | offsets: count × u32 | entries
//! entry = key bytes | 0x00 | (length: u8 | value bytes)*
//! ```
//!
//! Entries are sorted by key bytes; each offset is where an entry starts, counted from the start of
//! the entries. An entry ends where the next one starts (the last at the end of the bytes).

pub(crate) const MAGIC: &[u8; 4] = b"PKLX";
pub(crate) const VERSION: u8 = 1;
/// Magic, version and count.
pub(crate) const HEADER_LEN: usize = 4 + 1 + 4;

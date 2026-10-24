//! Compiles the embedded lexicons: CMUdict (data/upstream/cmudict, pinned in
//! data/upstream/sources.toml), each word's pronunciations as one byte per phone
//! (src/arpabet.rs), and the letter-to-sound rules learned from it
//! (data/derived/en-us/letters.tsv), each context's phonemes the same way. jespersen's own entries
//! (data/en-us/lexicon.tsv) replace CMUdict's for their words; the British ones
//! (data/en-gb/lexicon.tsv) are a lexicon of their own.

use std::{env, error::Error, fs, path::PathBuf};

#[path = "src/arpabet.rs"]
#[allow(dead_code, reason = "the build script only encodes")]
mod arpabet;

fn main() -> Result<(), Box<dyn Error>> {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let out = PathBuf::from(env::var("OUT_DIR")?);
    let cmudict = manifest.join("../../data/upstream/cmudict/cmudict.dict");
    let letters = manifest.join("../../data/derived/en-us/letters.tsv");
    let own = manifest.join("../../data/en-us/lexicon.tsv");
    let british = manifest.join("../../data/en-gb/lexicon.tsv");
    for path in [&cmudict, &letters, &own, &british] {
        println!("cargo::rerun-if-changed={}", path.display());
    }
    println!("cargo::rerun-if-changed=src/arpabet.rs");

    // jespersen's own entries replace CMUdict's for their words.
    let own = own_entries(&fs::read_to_string(&own)?)?;
    let mut entries: Entries = cmudict_entries(&fs::read_to_string(&cmudict)?)?
        .into_iter()
        .filter(|(word, _)| !own.iter().any(|(own_word, _)| own_word == word))
        .collect();
    entries.extend(own);
    fs::write(out.join("cmudict.lex"), jespersen_lexicon::compile(entries)?)?;

    fs::write(
        out.join("british.lex"),
        jespersen_lexicon::compile(own_entries(&fs::read_to_string(&british)?)?)?,
    )?;

    let rules = jespersen_lts::read_table(&fs::read_to_string(&letters)?, arpabet::encode)
        .map_err(|error| format!("letters.tsv, {error}"))?;
    fs::write(
        out.join("letters.lex"),
        jespersen_lexicon::compile(rules.into_iter().map(|rule| (rule.context, rule.phonemes)))?,
    )?;
    Ok(())
}

/// A lexicon's words and their encoded pronunciations.
type Entries = Vec<(String, Vec<u8>)>;

fn cmudict_entries(text: &str) -> Result<Entries, Box<dyn Error>> {
    let mut entries = Vec::new();
    for (number, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap_or_default().trim();
        if line.is_empty() {
            continue;
        }
        let mut fields = line.split_whitespace();
        let Some(head) = fields.next() else { continue };
        // Alternative pronunciations are written `word(2)`, `word(3)`…
        let word = head.split_once('(').map_or(head, |(word, _)| word);
        let codes = fields
            .map(|phone| {
                arpabet::encode(phone)
                    .ok_or_else(|| format!("cmudict.dict line {}: unknown phone {phone:?}", number + 1))
            })
            .collect::<Result<Vec<u8>, _>>()?;
        entries.push((word.to_owned(), codes));
    }
    if entries.len() < 100_000 {
        return Err(format!("cmudict.dict gave only {} entries", entries.len()).into());
    }
    Ok(entries)
}

/// jespersen's own entries: `word<TAB>phonemes` lines, `#` comments and blank lines skipped.
fn own_entries(text: &str) -> Result<Entries, Box<dyn Error>> {
    let mut entries = Vec::new();
    for (number, line) in text.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let bad = || format!("lexicon.tsv line {}: {line:?}", number + 1);
        let (word, phonemes) = line.split_once('\t').ok_or_else(bad)?;
        let codes = phonemes
            .split(' ')
            .map(|phone| arpabet::encode(phone).ok_or_else(bad))
            .collect::<Result<Vec<u8>, _>>()?;
        entries.push((word.to_owned(), codes));
    }
    Ok(entries)
}

//! Compiles NST's Danish lexicon (data/derived/da/nst.tsv, from the archive pinned in
//! data/upstream/sources.toml): each word in lower case and one transcription in NST's SAMPA,
//! checked here. A word with several is given the one of its part of speech that comes first in
//! running text: a function word's (man the pronoun, den the article), then a verb's, an
//! adjective's or a noun's, and a name's last; among those of one part of speech, the first in
//! nst.tsv, which is sorted.

use std::{collections::BTreeMap, env, error::Error, fs, path::PathBuf};

#[path = "src/sampa.rs"]
#[allow(dead_code, reason = "the build script only checks transcriptions")]
mod sampa;

/// The parts of speech in the order a word's transcription is chosen; any other comes after them.
const ORDER: [&str; 12] = ["PN", "DT", "PP", "KN", "IN", "RG", "AB", "VB", "JJ", "NN", "", "PM"];

fn main() -> Result<(), Box<dyn Error>> {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let out = PathBuf::from(env::var("OUT_DIR")?);
    let nst = manifest.join("../../data/derived/da/nst.tsv");
    println!("cargo::rerun-if-changed={}", nst.display());
    println!("cargo::rerun-if-changed=src/sampa.rs");

    let mut chosen: BTreeMap<String, (usize, String)> = BTreeMap::new();
    for (number, line) in fs::read_to_string(&nst)?.lines().enumerate() {
        if line.starts_with('#') {
            continue;
        }
        let at = || format!("nst.tsv line {}", number + 1);
        let [word, pos, transcription] = line.split('\t').collect::<Vec<_>>()[..] else {
            return Err(format!("{}: not three fields", at()).into());
        };
        if !transcription.is_ascii() || sampa::symbols(transcription).is_none() {
            return Err(format!("{}: cannot read {transcription:?}", at()).into());
        }
        let rank = ORDER.iter().position(|&known| known == pos).unwrap_or(ORDER.len() - 2);
        let entry = chosen.entry(word.to_lowercase()).or_insert((usize::MAX, String::new()));
        if rank < entry.0 {
            *entry = (rank, transcription.to_owned());
        }
    }
    fs::write(
        out.join("nst.lex"),
        jespersen_lexicon::compile(
            chosen
                .into_iter()
                .map(|(word, (_, transcription))| (word, transcription)),
        )?,
    )?;
    Ok(())
}

//! Compiles NST's Swedish lexicon (data/derived/sv/nst.tsv, from the archive pinned in
//! data/upstream/sources.toml; NST's own entries only, decision 42): one transcription in NST's
//! SAMPA, checked here, for each noun, adjective, verb and number in lower case, and each name as
//! written (Hans, not the pronoun hans). A word NST also lists as a function word is left to the
//! rules (vad, sedan): NST says those as Swedes do (dem dOm), eSpeak NG from their spelling, and
//! jespersen's notation follows it. Acronyms and Roman numerals (VI, OSS) are left out. A word with
//! several transcriptions is given a number's, then a verb's, an adjective's or a noun's; among
//! those of one part of speech, the first in nst.tsv, which is sorted.

use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    error::Error,
    fs,
    path::PathBuf,
};

#[path = "src/sampa.rs"]
#[allow(dead_code, reason = "the build script only checks transcriptions")]
mod sampa;

/// The parts of speech whose words NST gives, in the order a word's transcription is chosen.
const CONTENT: [&str; 4] = ["RG", "VB", "JJ", "NN"];

fn main() -> Result<(), Box<dyn Error>> {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let out = PathBuf::from(env::var("OUT_DIR")?);
    let nst = manifest.join("../../data/derived/sv/nst.tsv");
    println!("cargo::rerun-if-changed={}", nst.display());
    println!("cargo::rerun-if-changed=src/sampa.rs");

    let mut chosen: BTreeMap<String, (usize, String)> = BTreeMap::new();
    let mut function_words: BTreeSet<String> = BTreeSet::new();
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
        if word.chars().skip(1).any(char::is_uppercase) {
            continue;
        }
        let (key, rank) = if pos == "PM" {
            (word.to_owned(), 0)
        } else if let Some(rank) = CONTENT.iter().position(|&content| content == pos) {
            (word.to_lowercase(), rank)
        } else {
            function_words.insert(word.to_lowercase());
            continue;
        };
        let entry = chosen.entry(key).or_insert((usize::MAX, String::new()));
        if rank < entry.0 {
            *entry = (rank, transcription.to_owned());
        }
    }
    fs::write(
        out.join("nst.lex"),
        jespersen_lexicon::compile(
            chosen
                .into_iter()
                .filter(|(word, _)| !function_words.contains(word))
                .map(|(word, (_, transcription))| (word, transcription)),
        )?,
    )?;
    Ok(())
}

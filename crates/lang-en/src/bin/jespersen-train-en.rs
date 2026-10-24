//! Learns American English letter-to-sound rules from CMUdict (docs/decisions.md, decision 29).
//!
//! `jespersen-train-en evaluate` trains on 98% of CMUdict and reports how many of the other 2% it
//! reads right; `jespersen-train-en write OUT` trains on all of it and writes the rule table that
//! the build script compiles (`mise run en:letters` writes data/derived/en-us/letters.tsv).

use std::{env, error::Error, fs, path::Path, process::ExitCode};

use jespersen_lts::{Entry, Rule, read_table, train, write_table};

#[path = "../arpabet.rs"]
#[allow(dead_code, reason = "the trainer uses only part of it")]
mod arpabet;

/// A rule's context must be seen this often to be kept (1 reads unseen words best: 46.9% exactly
/// right against 41.0% with 2, on 23 October 2026).
const MIN_COUNT: usize = 1;
/// Every this many words, one is held out for `evaluate`.
const HOLD_OUT_EVERY: usize = 50;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        ["evaluate"] => evaluate(),
        ["write", out] => write(Path::new(out)),
        _ => Err("usage: jespersen-train-en evaluate | write OUT".into()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("jespersen-train-en: {error}");
            ExitCode::FAILURE
        }
    }
}

/// CMUdict's words of letters and apostrophes only, each with its first pronunciation.
fn entries() -> Result<Vec<Entry>, Box<dyn Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/upstream/cmudict/cmudict.dict");
    let mut entries = Vec::new();
    for line in fs::read_to_string(path)?.lines() {
        let line = line.split('#').next().unwrap_or_default().trim();
        let mut fields = line.split_whitespace();
        let Some(word) = fields.next() else { continue };
        if word.contains('(') || !word.chars().all(|c| c.is_ascii_lowercase() || c == '\'') {
            continue;
        }
        let phonemes = fields
            .map(|phone| arpabet::encode(phone).ok_or_else(|| format!("unknown phone {phone:?} in {word:?}")))
            .collect::<Result<Vec<u8>, _>>()?;
        entries.push(Entry {
            letters: word.chars().collect(),
            phonemes,
        });
    }
    Ok(entries)
}

/// The phoneme a code is for, whatever its stress.
fn base(code: u8) -> u8 {
    code / 3
}

fn evaluate() -> Result<(), Box<dyn Error>> {
    let entries = entries()?;
    let (held, kept): (Vec<_>, Vec<_>) = entries
        .iter()
        .cloned()
        .enumerate()
        .partition(|(index, _)| index % HOLD_OUT_EVERY == 0);
    let kept: Vec<Entry> = kept.into_iter().map(|(_, entry)| entry).collect();
    let rules = train(&kept, base, MIN_COUNT);
    let table: std::collections::HashMap<&str, &[u8]> = rules
        .iter()
        .map(|rule| (rule.context.as_str(), rule.phonemes.as_slice()))
        .collect();
    let (mut exact, mut without_stress) = (0usize, 0usize);
    for (_, entry) in &held {
        let lookup = |context: &str| Ok::<_, ()>(table.get(context).map(|said| said.to_vec()));
        let Ok(Some(mut said)) = jespersen_lts::predict(&entry.letters, lookup) else {
            continue;
        };
        arpabet::one_primary(&mut said);
        if said == entry.phonemes {
            exact += 1;
        }
        if said
            .iter()
            .map(|&code| base(code))
            .eq(entry.phonemes.iter().map(|&code| base(code)))
        {
            without_stress += 1;
        }
    }
    println!(
        "{} rules from {} words; of {} held-out words, {} exactly right, {} right but for stress",
        rules.len(),
        kept.len(),
        held.len(),
        percent(exact, held.len()),
        percent(without_stress, held.len())
    );
    Ok(())
}

fn write(out: &Path) -> Result<(), Box<dyn Error>> {
    let rules: Vec<Rule> = train(&entries()?, base, MIN_COUNT);
    let header = "American English letter-to-sound rules, learned by jespersen-train-en from CMUdict\n\
                  (data/upstream/cmudict, pinned in data/upstream/sources.toml). Generated: do not edit;\n\
                  regenerate with `mise run en:letters`. CMUdict's licence and notice apply:\n\
                  data/upstream/cmudict/LICENSE.\n\
                  Each line: a letter in context (letters left, [letter], letters right, # beyond the\n\
                  word), a tab, and the ARPAbet phonemes it says (- for none).";
    let text = write_table(&rules, header, |code| {
        let index = usize::from(code / 3);
        if arpabet::is_vowel(index) {
            format!("{}{}", arpabet::PHONES[index], code % 3)
        } else {
            arpabet::PHONES[index].to_owned()
        }
    });
    // The table must read back as written.
    if read_table(&text, arpabet::encode)? != rules {
        return Err("the written table does not read back".into());
    }
    if let Some(folder) = out.parent() {
        fs::create_dir_all(folder)?;
    }
    fs::write(out, text)?;
    println!("{} rules written to {}", rules.len(), out.display());
    Ok(())
}

fn percent(part: usize, whole: usize) -> String {
    #[expect(clippy::cast_precision_loss, reason = "counts far below 2^52")]
    let share = part as f64 * 100.0 / whole.max(1) as f64;
    format!("{share:.1}%")
}

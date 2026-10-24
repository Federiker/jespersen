//! `jespersen-nst LANGUAGE OUT PRON` reads NST's lexicon PRON for LANGUAGE (da, sv or nb, from the
//! archive pinned in data/upstream/sources.toml) and writes the table jespersen compiles to OUT
//! (`mise run data:nst:da`, `data:nst:sv` and `data:nst:nb` write data/derived/<language>/nst.tsv).

use std::{error::Error, fs, path::Path, process::ExitCode};

use jespersen_nst::{Language, extract, latin1};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.as_slice() {
        [language, out, pron] => run(language, Path::new(out), Path::new(pron)),
        _ => Err("usage: jespersen-nst da|sv|nb OUT PRON".into()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("jespersen-nst: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(language: &str, out: &Path, pron: &Path) -> Result<(), Box<dyn Error>> {
    let language: Language = language.parse()?;
    let text = latin1(&fs::read(pron).map_err(|error| format!("{}: {error}", pron.display()))?);
    fs::write(out, extract(&text, language)?)?;
    Ok(())
}

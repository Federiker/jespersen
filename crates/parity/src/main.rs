//! `jespersen-parity capture LANGUAGE TEXTS OUT` runs the oracle named by the environment
//! (`JESPERSEN_ORACLE`: the program; `JESPERSEN_ORACLE_DATA`: its data folder) on a text list and writes
//! its answers to OUT. `jespersen-parity report LANGUAGE CAPTURE [--all]` compares jespersen with them
//! (`--all` lists every difference, not only the most frequent); it
//! says so and succeeds when there is no capture, so CI never needs one. `jespersen-parity
//! sentences LANGUAGE PROSE OUT` splits prose into a text list, one sentence per line. `jespersen-parity
//! coverage LANGUAGE TEXTS` says how much of a text list jespersen reads, with no oracle.

use std::{env, error::Error, fs, path::Path, process::ExitCode};

use jespersen::{Language, PortugueseVariety, SpanishVariety};

/// How many differing words the report lists.
const SHOWN_DIFFERENCES: usize = 25;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        ["capture", language, texts, out] => capture(language, Path::new(texts), Path::new(out)),
        ["report", language, capture] => report(language, Path::new(capture), SHOWN_DIFFERENCES),
        ["report", language, capture, "--all"] => report(language, Path::new(capture), usize::MAX),
        ["sentences", language, prose, out] => sentences(language, Path::new(prose), Path::new(out)),
        ["coverage", language, texts] => coverage(language, Path::new(texts)),
        _ => Err("usage: jespersen-parity capture LANGUAGE TEXTS OUT | report LANGUAGE CAPTURE [--all] | sentences LANGUAGE PROSE OUT | coverage LANGUAGE TEXTS".into()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("jespersen-parity: {error}");
            ExitCode::FAILURE
        }
    }
}

/// The oracle's voice name for a language; Mandarin has none: Kokoro's Chinese voices learnt misaki's
/// notation, not eSpeak NG's, so Mandarin is compared with misaki instead.
fn voice(language: Language) -> Result<&'static str, Box<dyn Error>> {
    match language {
        Language::AmericanEnglish => Ok("en-us"),
        Language::BritishEnglish(_) => Ok("en"),
        Language::Spanish(SpanishVariety::Castilian) => Ok("es"),
        Language::Spanish(SpanishVariety::LatinAmerican) => Ok("es-419"),
        Language::Portuguese(PortugueseVariety::European) => Ok("pt"),
        Language::Portuguese(PortugueseVariety::Brazilian) => Ok("pt-br"),
        Language::Italian => Ok("it"),
        Language::Oromo => Ok("om"),
        Language::Somali => Ok("so"),
        Language::Greek => Ok("el"),
        Language::Swahili => Ok("sw"),
        Language::Macedonian => Ok("mk"),
        Language::Serbian => Ok("sr"),
        Language::Malay => Ok("ms"),
        Language::Russian => Ok("ru"),
        Language::Irish => Ok("ga"),
        Language::Welsh => Ok("cy"),
        Language::Icelandic => Ok("is"),
        Language::Danish => Ok("da"),
        Language::Norwegian => Ok("nb"),
        Language::Swedish => Ok("sv"),
        Language::Albanian => Ok("sq"),
        Language::Lithuanian => Ok("lt"),
        Language::Afrikaans => Ok("af"),
        Language::Latvian => Ok("lv"),
        Language::Estonian => Ok("et"),
        Language::Hungarian => Ok("hu"),
        Language::Dutch => Ok("nl"),
        Language::German => Ok("de"),
        Language::Catalan => Ok("ca"),
        Language::Romanian => Ok("ro"),
        Language::Basque => Ok("eu"),
        Language::Slovenian => Ok("sl"),
        Language::Croatian => Ok("hr"),
        Language::Slovak => Ok("sk"),
        Language::Czech => Ok("cs"),
        Language::French => Ok("fr"),
        Language::Polish => Ok("pl"),
        Language::Indonesian => Ok("id"),
        Language::Turkish => Ok("tr"),
        Language::Finnish => Ok("fi"),
        Language::Mandarin(_) => Err("Mandarin is not compared with eSpeak NG (see tmp/docs/README.md)".into()),
        Language::Kinyarwanda
        | Language::Kirundi
        | Language::Luganda
        | Language::Lingala
        | Language::Shona
        | Language::Yoruba
        | Language::Igbo
        | Language::Hausa
        | Language::Ewe
        | Language::Akan => Err("eSpeak NG has no voice for this language (decision 48)".into()),
    }
}

fn capture(language: &str, texts: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
    let language: Language = language.parse()?;
    let program = env::var_os("JESPERSEN_ORACLE").ok_or("set JESPERSEN_ORACLE to the oracle program")?;
    let data = env::var_os("JESPERSEN_ORACLE_DATA").ok_or("set JESPERSEN_ORACLE_DATA to its data folder")?;
    let texts = jespersen_parity::read_texts(&fs::read_to_string(texts)?);
    let capture = jespersen_parity::capture(Path::new(&program), voice(language)?, Path::new(&data), &texts)?;
    if let Some(folder) = out.parent() {
        fs::create_dir_all(folder)?;
    }
    fs::write(out, jespersen_parity::write_capture(&capture))?;
    println!(
        "{}: {} answers written to {}",
        capture.version,
        capture.answers.len(),
        out.display()
    );
    Ok(())
}

fn report(language: &str, capture: &Path, shown: usize) -> Result<(), Box<dyn Error>> {
    let language: Language = language.parse()?;
    voice(language)?;
    if !capture.exists() {
        println!(
            "No capture at {}: nothing to compare (see tmp/docs/README.md).",
            capture.display()
        );
        return Ok(());
    }
    let capture = jespersen_parity::read_capture(&fs::read_to_string(capture)?);
    let report = jespersen_parity::compare(&capture.answers, language);
    println!(
        "jespersen {} against {} ({language})",
        jespersen::VERSION,
        capture.version
    );
    println!(
        "Texts: {} ({} not readable yet); exact: {} ({:.1}%)",
        report.lines,
        report.unreadable_lines,
        report.exact_lines,
        report.exact_percent()
    );
    println!(
        "Words: {} of {} ({:.1}%)",
        report.matching_words,
        report.words,
        report.word_percent()
    );
    if !report.differences.is_empty() {
        println!("Most frequent differences (jespersen → oracle):");
        for ((ours, theirs), count) in report.differences.iter().take(shown) {
            println!("  {count:>4}  {ours} → {theirs}");
        }
    }
    Ok(())
}

/// Splits a prose file into a text list, one sentence per line.
fn sentences(language: &str, prose: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
    let language: Language = language.parse()?;
    let sentences = jespersen_parity::sentences(&fs::read_to_string(prose)?, language)?;
    fs::write(out, sentences.join("\n") + "\n")?;
    println!("{} sentences written to {}", sentences.len(), out.display());
    Ok(())
}

fn coverage(language: &str, texts: &Path) -> Result<(), Box<dyn Error>> {
    let language: Language = language.parse()?;
    let texts = jespersen_parity::read_texts(&fs::read_to_string(texts)?);
    let coverage = jespersen_parity::coverage(&texts, language);
    println!("jespersen {} ({language})", jespersen::VERSION);
    println!("Texts: {}; read: {:.1}%", coverage.lines, coverage.readable_percent());
    if coverage.han_characters > 0 {
        println!(
            "Chinese characters: {}; read inside words of two or more: {:.1}%",
            coverage.han_characters,
            coverage.in_words_percent()
        );
    }
    if !coverage.single_pairs.is_empty() {
        let pairs: Vec<String> = coverage
            .single_pairs
            .iter()
            .take(40)
            .map(|(pair, count)| format!("{pair} {count}"))
            .collect();
        println!(
            "Most frequent characters read one by one side by side: {}",
            pairs.join(", ")
        );
    }
    for (text, why) in &coverage.unreadable {
        println!("  {text}\n    {why}");
    }
    Ok(())
}

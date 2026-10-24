//! The `jespersen-nst` program.

use std::{fs, path::PathBuf, process::Command};

#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn nst(args: &[&str]) -> (Option<i32>, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_jespersen-nst"))
        .args(args)
        .output()
        .unwrap();
    (output.status.code(), String::from_utf8(output.stderr).unwrap())
}

fn scratch(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name)
}

#[test]
fn wrong_arguments_are_a_usage_error() {
    let (code, err) = nst(&["da"]);
    assert_eq!(code, Some(1));
    assert!(err.contains("usage: jespersen-nst da|sv|nb OUT PRON"));
}

#[test]
fn an_unknown_language_or_missing_file_fails() {
    let out = scratch("never.tsv");
    let (code, err) = nst(&["fi", out.to_str().unwrap(), "missing.pron"]);
    assert_eq!(code, Some(1));
    assert!(err.contains("no NST lexicon for \"fi\""));
    let (code, err) = nst(&["da", out.to_str().unwrap(), "missing.pron"]);
    assert_eq!(code, Some(1));
    assert!(err.contains("missing.pron"));
}

#[test]
fn a_lexicon_is_written_as_a_table() {
    let mut fields = vec![""; 51];
    fields[0] = "og";
    fields[1] = "KN";
    fields[5] = "LEX";
    fields[11] = "\"O:";
    fields[29] = "ref_dic";
    let line = fields.join(";");
    let pron = scratch("tiny.pron");
    let lines: Vec<String> = (0..90_000).map(|n| line.replacen("og", &format!("og{n}"), 1)).collect();
    fs::write(&pron, lines.join("\r\n")).unwrap();
    let out = scratch("tiny.tsv");
    let (code, err) = nst(&["nb", out.to_str().unwrap(), pron.to_str().unwrap()]);
    assert_eq!((code, err.as_str()), (Some(0), ""));
    let table = fs::read_to_string(out).unwrap();
    assert!(table.contains("\nog0\tKN\t\"O:\n"));
}

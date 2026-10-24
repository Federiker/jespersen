//! The `jespersen-parity` program, without an oracle.

use std::{fs, path::PathBuf, process::Command};

#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn parity(args: &[&str]) -> (Option<i32>, String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_jespersen-parity"))
        .args(args)
        .env_remove("JESPERSEN_ORACLE")
        .env_remove("JESPERSEN_ORACLE_DATA")
        .output()
        .unwrap();
    (
        output.status.code(),
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    )
}

fn scratch(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name)
}

#[test]
fn a_missing_capture_is_skipped_not_failed() {
    let missing = scratch("no-such-capture.tsv");
    let (code, out, _) = parity(&["report", "en-US", missing.to_str().unwrap()]);
    assert_eq!(code, Some(0));
    assert!(out.starts_with("No capture at"));
}

#[test]
fn a_capture_is_reported() {
    let capture = scratch("capture.tsv");
    fs::write(&capture, "# oracle 9\nHello butter.\thəlˈoʊ bˈʌtɚ\n").unwrap();
    let (code, out, _) = parity(&["report", "en-US", capture.to_str().unwrap()]);
    assert_eq!(code, Some(0));
    assert!(out.contains("against oracle 9 (en-US)"));
    assert!(out.contains("Words: 1 of 2 (50.0%)"));
    assert!(out.contains("bˈʌɾɚ → bˈʌtɚ"));
}

#[test]
fn usage_language_and_oracle_errors_fail() {
    let (code, _, err) = parity(&[]);
    assert_eq!(code, Some(1));
    assert!(err.contains("usage: jespersen-parity"));
    let (code, _, err) = parity(&["report", "xx", "anything.tsv"]);
    assert_eq!(code, Some(1));
    assert!(err.contains("\"xx\""));
    let (code, _, err) = parity(&["capture", "en-US", "texts.txt", "out.tsv"]);
    assert_eq!(code, Some(1));
    assert!(err.contains("set JESPERSEN_ORACLE"));
    // A language eSpeak NG has no voice for is refused before the oracle is needed.
    let (code, _, err) = parity(&["report", "yo", "anything.tsv"]);
    assert_eq!(code, Some(1));
    assert!(err.contains("no voice"));
}

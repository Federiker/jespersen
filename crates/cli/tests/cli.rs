use std::{
    io::Write,
    process::{Command, Stdio},
};

/// Runs the command in memory with `input` as standard input: (exit code, output, messages).
#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn run_with(args: &[&str], input: &str) -> (u8, String, String) {
    let mut out = Vec::new();
    let mut err = Vec::new();
    let code = jespersen_cli::run(args, &mut input.as_bytes(), &mut out, &mut err).unwrap();
    (code, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap())
}

fn run(args: &[&str]) -> (u8, String, String) {
    run_with(args, "")
}

#[test]
fn no_arguments_and_help_show_the_help() {
    for args in [&[][..], &["-h"], &["--help"]] {
        let (code, out, err) = run(args);
        assert_eq!(code, 0, "{args:?}");
        assert!(out.contains("Usage: jespersen <command>"), "{args:?}");
        assert!(err.is_empty());
    }
}

#[test]
fn version_flags_show_the_version() {
    for flag in ["-V", "--version"] {
        let (code, out, _) = run(&[flag]);
        assert_eq!(code, 0, "{flag}");
        assert_eq!(out, format!("jespersen {}\n", jespersen::VERSION), "{flag}");
    }
}

#[test]
fn unknown_commands_and_extra_arguments_are_usage_errors() {
    let (code, out, err) = run(&["--frobnicate"]);
    assert_eq!(code, 2);
    assert!(out.is_empty());
    assert!(err.contains("unknown command \"--frobnicate\""));
    let (code, _, err) = run(&["--help", "--version"]);
    assert_eq!(code, 2);
    assert!(err.contains("too many arguments"));
}

#[test]
fn phonemes_of_the_arguments() {
    let (code, out, err) = run(&["phonemes", "Hello,", "world."]);
    assert_eq!((code, out.as_str(), err.as_str()), (0, "həˈloʊ, ˈwɝld.\n", ""));
    let (_, out, _) = run(&["phonemes", "--espeak", "-v", "en-us", "Hello, world."]);
    assert_eq!(out, "həlˈoʊ, wˈɜːld.\n");
    let (_, out, _) = run(&["phonemes", "--espeak", "--no-punctuation", "--ipa", "--", "-Hello"]);
    assert_eq!(out, "həˈloʊ\n", "the last notation wins; -- ends the options");
}

#[test]
fn phonemes_of_each_input_line() {
    let (code, out, err) = run_with(&["phonemes", "--espeak"], "hello\n\nbutter world\n");
    assert_eq!((code, err.as_str()), (0, ""));
    assert_eq!(out, "həlˈoʊ\n\nbˈʌɾɚ wˈɜːld\n");
}

#[test]
fn lines_jespersen_cannot_read_are_reported_and_left_empty() {
    let (code, out, err) = run_with(&["phonemes"], "hello\nжук §\nworld\n");
    assert_eq!(code, 1);
    assert_eq!(out, "həˈloʊ\n\nˈwɝld\n");
    assert_eq!(err, "jespersen: line 2: jespersen cannot pronounce yet: жук, §\n");
    let (code, out, err) = run(&["phonemes", "жук"]);
    assert_eq!((code, out.as_str()), (1, ""));
    assert!(err.contains("жук"));
}

#[test]
fn phonemes_options_are_checked() {
    let (code, out, _) = run(&["phonemes", "--help"]);
    assert_eq!(code, 0);
    assert!(out.contains("Usage: jespersen phonemes"));
    let (code, _, err) = run(&["phonemes", "-v"]);
    assert_eq!(code, 2);
    assert!(err.contains("-v needs a language"));
    let (code, _, err) = run(&["phonemes", "-v", "xx", "hello"]);
    assert_eq!(code, 1);
    assert!(err.contains("does not read the language \"xx\""));
    let (code, _, err) = run(&["phonemes", "--loud", "hello"]);
    assert_eq!(code, 2);
    assert!(err.contains("unknown option \"--loud\""));
}

#[test]
fn a_closed_output_is_reported() {
    struct Closed;
    impl std::io::Write for Closed {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::ErrorKind::BrokenPipe.into())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    for args in [
        &["--version"][..],
        &["--nope"],
        &["phonemes", "hello"],
        &["phonemes", "жук"],
    ] {
        assert!(
            jespersen_cli::run(args, &mut "".as_bytes(), &mut Closed, &mut Closed).is_err(),
            "{args:?}"
        );
    }
    assert!(jespersen_cli::run(["phonemes"], &mut "hello\n".as_bytes(), &mut Closed, &mut Closed).is_err());
}

#[test]
fn the_binary_reads_standard_input_and_passes_its_exit_code() {
    let ok = Command::new(env!("CARGO_BIN_EXE_jespersen"))
        .arg("--version")
        .output()
        .unwrap();
    assert!(ok.status.success());
    assert_eq!(
        String::from_utf8(ok.stdout).unwrap(),
        format!("jespersen {}\n", jespersen::VERSION)
    );

    let mut child = Command::new(env!("CARGO_BIN_EXE_jespersen"))
        .args(["phonemes", "--espeak"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all("Hello, world.\n".as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "həlˈoʊ, wˈɜːld.\n");

    let bad = Command::new(env!("CARGO_BIN_EXE_jespersen"))
        .arg("--nope")
        .output()
        .unwrap();
    assert_eq!(bad.status.code(), Some(2));
}

#[test]
fn british_english_and_its_bath_vowel() {
    let (code, out, _) = run(&["phonemes", "-v", "en-GB", "--espeak", "a fast car"]);
    assert_eq!((code, out.as_str()), (0, "ɐ fˈast kˈɑː\n"));
    let (_, out, _) = run(&["phonemes", "-v", "en-GB", "--bath", "broad", "--espeak", "fast"]);
    assert_eq!(out, "fˈɑːst\n");
    let (_, out, _) = run(&["phonemes", "-v", "en-GB", "--bath", "flat", "fast"]);
    assert_eq!(out, "ˈfæst\n");
    let (code, _, err) = run(&["phonemes", "--bath", "broad", "fast"]);
    assert_eq!(code, 2);
    assert!(err.contains("--bath applies to British English"));
    let (code, _, err) = run(&["phonemes", "-v", "en-GB", "--bath", "wide", "fast"]);
    assert_eq!(code, 2);
    assert!(err.contains("--bath needs broad, flat or notation"));
}

#[test]
fn finnish() {
    let (code, out, _) = run(&["phonemes", "-v", "fi", "--espeak", "kissa"]);
    assert_eq!(
        (code, out.as_str()),
        (
            0, "kˈissa
"
        )
    );
}

#[test]
fn italian() {
    let (code, out, _) = run(&["phonemes", "-v", "it", "--espeak", "la pizza"]);
    assert_eq!(
        (code, out.as_str()),
        (
            0,
            "la pˈitsːa
"
        )
    );
}

#[test]
fn spanish_and_its_varieties() {
    let (code, out, _) = run(&["phonemes", "-v", "es", "--espeak", "la calle"]);
    assert_eq!(
        (code, out.as_str()),
        (
            0,
            "la kˈaʎe
"
        )
    );
    let (_, out, _) = run(&["phonemes", "-v", "es-419", "--espeak", "la calle"]);
    assert_eq!(
        out,
        "la kˈajje
"
    );
}

#[test]
fn mandarin_and_its_tones() {
    let (code, out, _) = run(&["phonemes", "-v", "zh", "--misaki", "--tones", "written", "你好，世界！"]);
    assert_eq!((code, out.as_str()), (0, "ni↓ xau↓, ʂɨ↘ʨje↘!\n"));
    let (_, out, _) = run(&["phonemes", "-v", "cmn", "--tones", "spoken", "你好"]);
    assert_eq!(out, "ni˧˥ xau˨˩˦\n");
    let (code, _, err) = run(&["phonemes", "-v", "zh", "--espeak", "你好"]);
    assert_eq!(code, 1);
    assert!(err.contains("does not write zh in the Espeak notation"), "{err}");
    let (code, _, err) = run(&["phonemes", "--tones", "spoken", "hello"]);
    assert_eq!(code, 2);
    assert!(err.contains("--tones applies to Mandarin"));
    let (_, out, _) = run(&["phonemes", "-v", "zh", "--pinyin", "银行"]);
    assert_eq!(
        out,
        "yínháng
"
    );
    let (_, out, _) = run(&["phonemes", "-v", "zh", "--zhuyin", "银行"]);
    assert_eq!(
        out,
        "ㄧㄣˊㄏㄤˊ
"
    );
    let (code, _, err) = run(&["phonemes", "-v", "zh", "--tones", "sung", "你好"]);
    assert_eq!(code, 2);
    assert!(err.contains("--tones needs spoken or written"));
}

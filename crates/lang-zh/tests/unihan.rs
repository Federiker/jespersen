//! The Unihan extractor's command line, on a made-up file in Unihan's format (the real one is not
//! committed; `mise run zh:characters` runs it).

use std::{env, fmt::Write as _, fs, path::PathBuf, process::Command};

fn extractor() -> Command {
    Command::new(env!("CARGO_BIN_EXE_jespersen-unihan-zh"))
}

/// A scratch folder of its own for `name`.
#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn scratch(name: &str) -> PathBuf {
    let folder = env::temp_dir().join(format!("jespersen-unihan-{name}-{}", std::process::id()));
    fs::create_dir_all(&folder).unwrap();
    folder
}

#[test]
fn readings_are_extracted_frequent_first() {
    let folder = scratch("readings");
    let mut readings = String::from("# Unihan_Readings.txt\n# Unicode Version 99.0.0\n");
    // Enough characters to pass the extractor's check, all read yī.
    for code in 0x2_0000..0x2_0000 + 40_000 {
        writeln!(readings, "U+{code:04X}\tkMandarin\tyī").unwrap();
    }
    for line in [
        "U+4E00\tkMandarin\tyī",
        // kHanyuPinlu wins: 得's particle is neutral but kMandarin has no neutral reading, so the
        // most frequent toned one is taken; 地 has the neutral tone in kMandarin too.
        "U+F900\tkHanyuPinlu\tde(5096) dé(1496) děi(637)",
        "U+F900\tkMandarin\tdé",
        "U+F901\tkHanyuPinlu\tde(7394) dì(4976)",
        "U+F901\tkMandarin\tde dì",
        "U+F902\tkMandarin\tlǜ",
        "U+F903\tkMandarin\tm̄",
        "U+F904\tkDefinition\tnot a reading",
    ] {
        writeln!(readings, "{line}").unwrap();
    }
    let input = folder.join("Unihan_Readings.txt");
    let output = folder.join("characters.tsv");
    fs::write(&input, readings).unwrap();
    let status = extractor().arg(&output).arg(&input).status().unwrap();
    assert!(status.success());
    let table = fs::read_to_string(&output).unwrap();
    assert!(table.starts_with("# Mandarin readings of Chinese characters from Unihan (Unicode 99.0.0)"));
    for line in [
        "一\tyi1",
        "\u{F900}\tde2",
        "\u{F901}\tde5",
        "\u{F902}\tlv4",
        "\u{F903}\tm1",
    ] {
        assert!(table.lines().any(|row| row == line), "{line}");
    }
    fs::remove_dir_all(folder).unwrap();
}

#[test]
fn bad_input_is_reported() {
    let folder = scratch("bad");
    let usage = extractor().arg("only-one").output().unwrap();
    assert_eq!(usage.status.code(), Some(1));
    assert!(
        String::from_utf8(usage.stderr)
            .unwrap()
            .contains("usage: jespersen-unihan-zh OUT UNIHAN_READINGS")
    );
    let input = folder.join("readings.txt");
    for (text, message) in [
        ("U+4E00\tkMandarin\tyī\n", "no '# Unicode Version' line"),
        ("# Unicode Version 1\nU+4E00\tkMandarin\tyī\n", "only 1 readings found"),
        ("# Unicode Version 1\nU+4E00\tkMandarin\ty1ī\n", "cannot read"),
        ("# Unicode Version 1\nU+4E00\tkHanyuPinlu\tyī\n", "cannot read"),
        ("# Unicode Version 1\nU+D800\tkMandarin\tyī\n", "is no character"),
    ] {
        fs::write(&input, text).unwrap();
        let output = extractor().arg(folder.join("out.tsv")).arg(&input).output().unwrap();
        assert_eq!(output.status.code(), Some(1), "{text}");
        let error = String::from_utf8(output.stderr).unwrap();
        assert!(error.contains(message), "{text}: {error}");
    }
    fs::remove_dir_all(folder).unwrap();
}

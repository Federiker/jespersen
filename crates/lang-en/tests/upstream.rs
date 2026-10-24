//! The committed upstream data still matches the SHA-256 pinned in data/upstream/sources.toml
//! (docs/decisions.md, decision 21).

use std::{fmt::Write, fs, path::Path};

use sha2::{Digest, Sha256};

#[test]
fn upstream_files_match_their_pinned_hashes() {
    let upstream = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/upstream");
    let sources = fs::read_to_string(upstream.join("sources.toml")).unwrap();

    // `[name.files]` sections list `"file" = "sha256"` lines; the file is upstream/name/file.
    let mut source = None;
    let mut checked = 0;
    for line in sources.lines().map(str::trim) {
        if let Some(section) = line.strip_prefix('[').and_then(|line| line.strip_suffix(']')) {
            source = section.strip_suffix(".files");
            continue;
        }
        let (Some(source), Some((file, hash))) = (source, line.split_once(" = ")) else {
            continue;
        };
        let file = file.trim_matches('"');
        let expected = hash.trim_matches('"');
        let bytes = fs::read(upstream.join(source).join(file)).unwrap();
        let mut actual = String::new();
        for byte in Sha256::digest(&bytes) {
            write!(actual, "{byte:02x}").unwrap();
        }
        assert_eq!(actual, expected, "{source}/{file}");
        checked += 1;
    }
    assert!(
        checked >= 5,
        "only {checked} files checked: is sources.toml still in this format?"
    );
}

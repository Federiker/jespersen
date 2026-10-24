//! Coverage needs no oracle, so CI keeps the public-domain Chinese text readable.

use std::{fs, path::Path};

use jespersen::{Language, ToneSandhi};
use jespersen_parity::{coverage, read_texts};

#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn texts(name: &str) -> Vec<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("texts").join(name);
    read_texts(&fs::read_to_string(path).unwrap())
}

#[test]
fn lu_xun_is_read_whole() {
    let coverage = coverage(&texts("zh-luxun.txt"), Language::Mandarin(ToneSandhi::Spoken));
    assert_eq!(coverage.lines, 300);
    assert!(coverage.unreadable.is_empty(), "{:?}", coverage.unreadable);
    assert!((coverage.readable_percent() - 100.0).abs() < f64::EPSILON);
    // Measured at 38.1% on 23 October 2026: more of the text read as whole words is better.
    assert!(coverage.in_words_percent() > 37.0, "{}", coverage.in_words_percent());
    assert!(!coverage.single_pairs.is_empty());
}

#[test]
fn unreadable_lines_and_empty_lists_are_counted() {
    let coverage = coverage(&["Hello §.".to_owned()], Language::AmericanEnglish);
    assert_eq!(coverage.unreadable.len(), 1);
    assert!(coverage.readable_percent().abs() < f64::EPSILON);
    assert!(coverage.in_words_percent().abs() < f64::EPSILON);
}

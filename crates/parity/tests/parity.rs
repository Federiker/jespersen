//! The comparison's arithmetic, against made-up answers (no oracle needed).

use jespersen::Language;
use jespersen_parity::{Capture, compare, read_capture, read_texts, write_capture};

fn answers(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    pairs
        .iter()
        .map(|(text, answer)| ((*text).to_owned(), (*answer).to_owned()))
        .collect()
}

#[test]
fn exact_lines_and_matching_words_are_counted() {
    let report = compare(
        &answers(&[
            ("Hello, world.", "həlˈoʊ wˈɜːld"),
            ("Hello butter.", "həlˈoʊ bˈʌtɚ"),
            ("Five жук.", "fˈaɪv"),
        ]),
        Language::AmericanEnglish,
    );
    assert_eq!((report.lines, report.exact_lines, report.unreadable_lines), (3, 1, 1));
    assert_eq!((report.matching_words, report.words), (3, 4));
    assert_eq!(report.differences, vec![(("bˈʌɾɚ".to_owned(), "bˈʌtɚ".to_owned()), 1)]);
    assert!((report.exact_percent() - 50.0).abs() < 1e-9);
    assert!((report.word_percent() - 75.0).abs() < 1e-9);
}

#[test]
fn words_are_aligned_around_insertions_and_deletions() {
    // Before the shared last word the two sides differ: their words are paired one to one.
    let report = compare(
        &answers(&[("hello world butter", "həlˈoʊwˈɜːld xx bˈʌɾɚ")]),
        Language::AmericanEnglish,
    );
    assert_eq!((report.matching_words, report.words), (1, 3));
    assert_eq!(report.differences.len(), 2);
    assert!(
        report
            .differences
            .contains(&(("həlˈoʊ".to_owned(), "həlˈoʊwˈɜːld".to_owned()), 1))
    );
    assert!(
        report
            .differences
            .contains(&(("wˈɜːld".to_owned(), "xx".to_owned()), 1))
    );

    // A word only one side has is not paired with anything.
    let report = compare(
        &answers(&[("hello butter", "həlˈoʊ xx bˈʌɾɚ")]),
        Language::AmericanEnglish,
    );
    assert_eq!((report.matching_words, report.words), (2, 3));
    assert!(report.differences.is_empty());
}

#[test]
fn differences_are_sorted_by_frequency() {
    let report = compare(
        &answers(&[("butter", "a"), ("butter", "a"), ("hello", "b")]),
        Language::AmericanEnglish,
    );
    assert_eq!(report.differences[0], (("bˈʌɾɚ".to_owned(), "a".to_owned()), 2));
    assert_eq!(report.differences[1].1, 1);
}

#[test]
fn an_empty_comparison_is_zero_not_a_division_by_zero() {
    let report = compare(&[], Language::AmericanEnglish);
    assert!(report.exact_percent().abs() < f64::EPSILON);
    assert!(report.word_percent().abs() < f64::EPSILON);
}

#[test]
fn captures_round_trip_through_text() {
    let capture = Capture {
        version: "oracle 1.0".into(),
        answers: answers(&[("a b", "x y"), ("c", "")]),
    };
    let text = write_capture(&capture);
    assert_eq!(text, "# oracle 1.0\na b\tx y\nc\t\n");
    assert_eq!(read_capture(&text), capture);
    assert_eq!(read_capture("no tab here\n").answers, vec![]);
}

#[test]
fn text_lists_skip_comments_and_blank_lines() {
    assert_eq!(
        read_texts("# a list\n\nOne.\n  Two.  \n#Three.\n"),
        vec!["One.", "Two."]
    );
}

#[test]
fn capture_refuses_texts_with_line_breaks() {
    let error = jespersen_parity::capture(
        std::path::Path::new("does-not-matter"),
        "en-us",
        std::path::Path::new("."),
        &["one\ntwo".to_owned()],
    )
    .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
fn capture_reports_a_missing_program() {
    let result = jespersen_parity::capture(
        std::path::Path::new("jespersen-no-such-oracle"),
        "en-us",
        std::path::Path::new("."),
        &["hello".to_owned()],
    );
    assert!(result.is_err());
}

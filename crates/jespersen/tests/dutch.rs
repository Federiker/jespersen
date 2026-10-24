#![cfg(feature = "nl")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn dutch_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize(
                "De kinderen spelen op het plein, 10 % meer dan gisteren.",
                Language::Dutch,
            )
            .unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("nl-BE".parse::<Language>().unwrap(), Language::Dutch);
    assert_eq!(Language::Dutch.to_string(), "nl");
    assert_eq!(
        phonemize("Dr. Jansen is gekomen. Hij is thuis.", Language::Dutch)
            .unwrap()
            .len(),
        2
    );
}

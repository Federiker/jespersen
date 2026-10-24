#![cfg(feature = "hr")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn croatian_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Djeca se igraju na moru, imam 21 €.", Language::Croatian).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("hr-HR".parse::<Language>().unwrap(), Language::Croatian);
    assert_eq!(Language::Croatian.to_string(), "hr");
    assert_eq!(
        phonemize("Dr. Horvat je došao. Prof. Kovačić također.", Language::Croatian)
            .unwrap()
            .len(),
        2
    );
}

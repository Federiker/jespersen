#![cfg(feature = "cs")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn czech_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Děti si hrají na zahradě, mám 21 Kč.", Language::Czech).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("cs-CZ".parse::<Language>().unwrap(), Language::Czech);
    assert_eq!(Language::Czech.to_string(), "cs");
    assert_eq!(
        phonemize("Dr. Novák přišel. Prof. Svoboda také.", Language::Czech)
            .unwrap()
            .len(),
        2
    );
}

#![cfg(feature = "ro")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn romanian_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Copiii se joacă în curte, cu 10 % mai mult.", Language::Romanian).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("ro-MD".parse::<Language>().unwrap(), Language::Romanian);
    assert_eq!(Language::Romanian.to_string(), "ro");
    assert_eq!(
        phonemize("Dr. Popescu a venit. Este acasă.", Language::Romanian)
            .unwrap()
            .len(),
        2
    );
}

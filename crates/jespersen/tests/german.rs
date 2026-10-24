#![cfg(feature = "de")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn german_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Die Kinder spielen im Hof, 10 % mehr als gestern.", Language::German).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("de-AT".parse::<Language>().unwrap(), Language::German);
    assert_eq!(Language::German.to_string(), "de");
    assert_eq!(
        phonemize("Dr. Müller ist gekommen. Er ist zu Hause.", Language::German)
            .unwrap()
            .len(),
        2
    );
}

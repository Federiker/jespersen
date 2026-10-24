#![cfg(feature = "nb")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn norwegian_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Barna leker i hagen, 10 % mer enn i går.", Language::Norwegian).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("no".parse::<Language>().unwrap(), Language::Norwegian);
    assert_eq!(Language::Norwegian.to_string(), "nb");
    assert_eq!(
        phonemize("Dr. Hansen har kommet. Han er hjemme.", Language::Norwegian)
            .unwrap()
            .len(),
        2
    );
}

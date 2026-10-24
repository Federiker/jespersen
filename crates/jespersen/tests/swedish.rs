#![cfg(feature = "sv")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn swedish_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Barnen leker på gården, 10 % mer än i går.", Language::Swedish).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("sv-FI".parse::<Language>().unwrap(), Language::Swedish);
    assert_eq!(Language::Swedish.to_string(), "sv");
    assert_eq!(
        phonemize("Dr. Lindberg har kommit. Han är hemma.", Language::Swedish)
            .unwrap()
            .len(),
        2
    );
}

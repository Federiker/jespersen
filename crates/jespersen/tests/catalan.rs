#![cfg(feature = "ca")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn catalan_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Els nens juguen al pati, un 10 % més que ahir.", Language::Catalan).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("ca-ES".parse::<Language>().unwrap(), Language::Catalan);
    assert_eq!(Language::Catalan.to_string(), "ca");
    assert_eq!(
        phonemize("El Sr. Puig ha vingut. És a casa.", Language::Catalan)
            .unwrap()
            .len(),
        2
    );
}

#![cfg(feature = "is")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn icelandic_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Börnin leika sér í garðinum, 10 % meira en í gær.", Language::Icelandic).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("is-IS".parse::<Language>().unwrap(), Language::Icelandic);
    assert_eq!(Language::Icelandic.to_string(), "is");
    assert_eq!(
        phonemize("Dr. Jónsson er kominn. Hann er heima.", Language::Icelandic)
            .unwrap()
            .len(),
        2
    );
}

#![cfg(feature = "cy")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn welsh_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize(
                "Mae'r plant yn chwarae yn yr ardd, 10 % yn fwy na ddoe.",
                Language::Welsh,
            )
            .unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("cy-GB".parse::<Language>().unwrap(), Language::Welsh);
    assert_eq!(Language::Welsh.to_string(), "cy");
    assert_eq!(
        phonemize("Mae Dr. Jones wedi dod. Mae e gartref.", Language::Welsh)
            .unwrap()
            .len(),
        2
    );
}

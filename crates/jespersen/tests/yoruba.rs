#![cfg(feature = "yo")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn yoruba_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Ọmọ náà ń ka ìwé, ó san ₦500.", Language::Yoruba).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak).split(' ').next(), Some("ˈɔmɔ"));
    assert_eq!(read(Notation::Ipa).split(' ').next(), Some("ˈɔmɔ"));
    // ₦500 is said as ẹ̀ẹ́dẹ́gbẹ̀ta náírà.
    assert!(
        read(Notation::Ipa).contains("ɛɛdɛˈɡ\u{361}bɛta naˈiɾa"),
        "{}",
        read(Notation::Ipa)
    );
    assert_eq!("yo-NG".parse::<Language>().unwrap(), Language::Yoruba);
    assert_eq!(Language::Yoruba.to_string(), "yo");
    assert_eq!(phonemize("Dr. Adé ti dé.", Language::Yoruba).unwrap().len(), 1);
}

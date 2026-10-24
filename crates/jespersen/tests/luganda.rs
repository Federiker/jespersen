#![cfg(feature = "lg")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn luganda_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Omusajja asoma ekitabo n'essanyu, kigula UGX 5.000.", Language::Luganda).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak).split(' ').next(), Some("omusˈaɟɟa"));
    assert_eq!(read(Notation::Ipa).split(' ').next(), Some("omuˈsaɟɟa"));
    assert!(read(Notation::Espeak).contains("nessˈaɲu"));
    assert_eq!("lg-UG".parse::<Language>().unwrap(), Language::Luganda);
    assert_eq!(Language::Luganda.to_string(), "lg");
    assert_eq!(phonemize("Dr. Nakato azze.", Language::Luganda).unwrap().len(), 1);
}

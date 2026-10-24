#![cfg(feature = "sn")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn shona_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Baba vari kuverenga bhuku, rinodhura $5.", Language::Shona).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak).split(' ').next(), Some("bˈaba"));
    assert_eq!(read(Notation::Ipa).split(' ').next(), Some("ˈɓaɓa"));
    assert!(read(Notation::Espeak).contains("madˈoɾa"));
    assert_eq!("sn-ZW".parse::<Language>().unwrap(), Language::Shona);
    assert_eq!(Language::Shona.to_string(), "sn");
    assert_eq!(phonemize("Dr. Moyo vasvika.", Language::Shona).unwrap().len(), 1);
}

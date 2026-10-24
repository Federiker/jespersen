#![cfg(feature = "ak")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn akan_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Kwame kɔ sukuu, na ne nhoma bo yɛ ₵20.", Language::Akan).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak).split(' ').next(), Some("kwˈame"));
    assert_eq!(read(Notation::Ipa).split(' ').next(), Some("ˈkʷame"));
    // ₵20 is said as aduonu sidi.
    assert!(read(Notation::Ipa).contains("aduˈonu ˈsidi"), "{}", read(Notation::Ipa));
    assert_eq!("tw".parse::<Language>().unwrap(), Language::Akan);
    assert_eq!(Language::Akan.to_string(), "ak");
    assert_eq!(phonemize("Owu. Mensah aba.", Language::Akan).unwrap().len(), 1);
}

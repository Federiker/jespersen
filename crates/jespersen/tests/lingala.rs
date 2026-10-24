#![cfg(feature = "ln")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn lingala_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Mwana azali kotánga búku, ntalo na yango 5.000 FC.", Language::Lingala).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak).split(' ').next(), Some("mwˈana"));
    assert_eq!(read(Notation::Ipa).split(' ').next(), Some("ˈmwana"));
    assert!(read(Notation::Espeak).contains("falˈaŋɡa"));
    assert_eq!("ln-CD".parse::<Language>().unwrap(), Language::Lingala);
    assert_eq!(Language::Lingala.to_string(), "ln");
    assert_eq!(phonemize("Dr. Mukwege ayei.", Language::Lingala).unwrap().len(), 1);
}

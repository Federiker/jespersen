#![cfg(feature = "ee")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn ewe_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Ɖevi la le agbalẽ xlẽm, eƒe home nye ₵20.", Language::Ewe).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak).split(' ').next(), Some("ɖˈevi"));
    assert_eq!(read(Notation::Ipa).split(' ').next(), Some("ˈɖevi"));
    // ₵20 is said as blaeve sidi.
    assert!(read(Notation::Ipa).contains("blaˈeve ˈsidi"), "{}", read(Notation::Ipa));
    assert_eq!("ee-GH".parse::<Language>().unwrap(), Language::Ewe);
    assert_eq!(Language::Ewe.to_string(), "ee");
    assert_eq!(phonemize("Dr. Kofi va.", Language::Ewe).unwrap().len(), 1);
}

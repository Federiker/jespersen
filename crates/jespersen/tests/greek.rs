#![cfg(feature = "el")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn greek_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Καλημέρα, το εισιτήριο κοστίζει 25 €.", Language::Greek).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak).split(',').next(), Some("kalimˈera"));
    assert_eq!(read(Notation::Ipa).split(',').next(), Some("kaliˈmeɾa"));
    assert_eq!("el-GR".parse::<Language>().unwrap(), Language::Greek);
    assert_eq!(Language::Greek.to_string(), "el");
    assert_eq!(phonemize("Ο κ. Παπαδόπουλος ήρθε.", Language::Greek).unwrap().len(), 1);
}

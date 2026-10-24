#![cfg(feature = "rw")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn kinyarwanda_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize(
                "Umwana arasoma igitabo n'umwete, gitwara 5.000 Frw.",
                Language::Kinyarwanda,
            )
            .unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak).split(' ').next(), Some("umŋwˈana"));
    assert_eq!(read(Notation::Ipa).split(' ').next(), Some("uˈmŋwana"));
    assert!(read(Notation::Espeak).contains("numŋwˈete"));
    assert_eq!("rw-RW".parse::<Language>().unwrap(), Language::Kinyarwanda);
    assert_eq!(Language::Kinyarwanda.to_string(), "rw");
    assert_eq!(phonemize("Dr. Mukamana yaje.", Language::Kinyarwanda).unwrap().len(), 1);
}

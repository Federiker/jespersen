#![cfg(feature = "rn")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn kirundi_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Umwana arasoma igitabo n'umwete, kigura 5.000 Fbu.", Language::Kirundi).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak).split(' ').next(), Some("umŋwˈana"));
    assert_eq!(read(Notation::Ipa).split(' ').next(), Some("uˈmŋwana"));
    assert!(read(Notation::Espeak).contains("numŋwˈete"));
    assert_eq!("rn-BI".parse::<Language>().unwrap(), Language::Kirundi);
    assert_eq!(Language::Kirundi.to_string(), "rn");
    assert_eq!(phonemize("Dr. Ndayishimiye yaje.", Language::Kirundi).unwrap().len(), 1);
}

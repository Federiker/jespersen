#![cfg(feature = "ha")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn hausa_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Ƙasar Najeriya babba ce, kuma farashinsa ₦200 ne.", Language::Hausa).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak).split(' ').next(), Some("kˈasaɾ"));
    assert_eq!(read(Notation::Ipa).split(' ').next(), Some("ˈkʼasaɾ"));
    // ₦200 is said as ɗari biyu naira.
    assert!(
        read(Notation::Ipa).contains("ˈɗaɾi ˈbiju ˈnaiɾa"),
        "{}",
        read(Notation::Ipa)
    );
    assert_eq!("ha-NG".parse::<Language>().unwrap(), Language::Hausa);
    assert_eq!(Language::Hausa.to_string(), "ha");
    assert_eq!(phonemize("Alh. Musa ya zo.", Language::Hausa).unwrap().len(), 1);
}

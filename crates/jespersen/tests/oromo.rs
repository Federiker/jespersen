#![cfg(feature = "om")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn oromo_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Barataan kitaaba dubbisa, gatiin isaa Br 150 dha.", Language::Oromo).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak).split(' ').next(), Some("barˈataːn"));
    assert_eq!(read(Notation::Ipa).split(' ').next(), Some("baˈrataːn"));
    assert_eq!("om-ET".parse::<Language>().unwrap(), Language::Oromo);
    assert_eq!(Language::Oromo.to_string(), "om");
    assert_eq!(phonemize("Ob. Gammachuu dhufe.", Language::Oromo).unwrap().len(), 1);
}

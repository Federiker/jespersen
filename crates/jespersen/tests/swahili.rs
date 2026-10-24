#![cfg(feature = "sw")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn swahili_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Mtoto anasoma kitabu, bei ni TSh 5.000.", Language::Swahili).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak).split(' ').next(), Some("mtˈɔtɔ"));
    assert_eq!(read(Notation::Ipa).split(' ').next(), Some("m̩ˈtɔtɔ"));
    assert_eq!("sw-TZ".parse::<Language>().unwrap(), Language::Swahili);
    assert_eq!(Language::Swahili.to_string(), "sw");
    assert_eq!(phonemize("Dkt. Amina amefika.", Language::Swahili).unwrap().len(), 1);
}

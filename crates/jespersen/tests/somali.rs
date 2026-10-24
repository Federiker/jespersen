#![cfg(feature = "so")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn somali_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize(
                "Hooyo waxay tagtay suuqa, waxayna iibsatay 3 kiilo oo bariis ah.",
                Language::Somali,
            )
            .unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak).split(' ').next(), Some("hˈoːjo"));
    assert_eq!(read(Notation::Ipa).split(' ').next(), Some("ˈhoːjo"));
    assert_eq!("so-SO".parse::<Language>().unwrap(), Language::Somali);
    assert_eq!(Language::Somali.to_string(), "so");
    assert_eq!(phonemize("Dr. Cali wuu yimid.", Language::Somali).unwrap().len(), 1);
}

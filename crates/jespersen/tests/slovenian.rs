#![cfg(feature = "sl")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn slovenian_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Otroci se igrajo v vrtu, imam 21 €.", Language::Slovenian).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("sl-SI".parse::<Language>().unwrap(), Language::Slovenian);
    assert_eq!(Language::Slovenian.to_string(), "sl");
    assert_eq!(
        phonemize("Dr. Novak je prišel. Prof. Kovač tudi.", Language::Slovenian)
            .unwrap()
            .len(),
        2
    );
}

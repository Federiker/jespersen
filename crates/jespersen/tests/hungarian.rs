#![cfg(feature = "hu")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn hungarian_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize(
                "A gyerekek az udvaron játszanak, 10 %-kal többet, mint tegnap.",
                Language::Hungarian,
            )
            .unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("hu-HU".parse::<Language>().unwrap(), Language::Hungarian);
    assert_eq!(Language::Hungarian.to_string(), "hu");
    assert_eq!(
        phonemize("Dr. Kovács megjött. Otthon van.", Language::Hungarian)
            .unwrap()
            .len(),
        2
    );
}

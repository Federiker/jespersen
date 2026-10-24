#![cfg(feature = "et")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn estonian_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Lapsed mängivad õues, 10 % rohkem kui eile.", Language::Estonian).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("et-EE".parse::<Language>().unwrap(), Language::Estonian);
    assert_eq!(Language::Estonian.to_string(), "et");
    assert_eq!(
        phonemize("Dr. Tamm tuli koju. Ta on kodus.", Language::Estonian)
            .unwrap()
            .len(),
        2
    );
}

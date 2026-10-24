#![cfg(feature = "lt")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn lithuanian_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Vaikai žaidžia kieme, 10 % daugiau nei vakar.", Language::Lithuanian).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("lt-LT".parse::<Language>().unwrap(), Language::Lithuanian);
    assert_eq!(Language::Lithuanian.to_string(), "lt");
    assert_eq!(
        phonemize("Dr. Kazlauskas atvyko. Jis yra namie.", Language::Lithuanian)
            .unwrap()
            .len(),
        2
    );
}

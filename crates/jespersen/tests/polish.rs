#![cfg(feature = "pl")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn polish_is_read_in_both_notations() {
    let read = |text, notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(&phonemize(text, Language::Polish).unwrap(), options).unwrap()
    };
    let text = "Jest dużo ludzi w sobotę, mam 22 zł i t. d.";
    println!("{}\n{}", read(text, Notation::Espeak), read(text, Notation::Ipa));
    println!("{}", read("Spotkanie 3.05.2024 o 10:30 w XX wieku.", Notation::Ipa));
    assert_eq!("pl-PL".parse::<Language>().unwrap(), Language::Polish);
    assert_eq!(Language::Polish.to_string(), "pl");
    assert_eq!(
        phonemize("Dr Nowak przyszedł. Prof. Kowalski też.", Language::Polish)
            .unwrap()
            .len(),
        2
    );
}

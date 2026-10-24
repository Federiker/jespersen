#![cfg(feature = "sq")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn albanian_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Fëmijët luajnë në oborr, 10 % më shumë se dje.", Language::Albanian).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("sq-XK".parse::<Language>().unwrap(), Language::Albanian);
    assert_eq!(Language::Albanian.to_string(), "sq");
    assert_eq!(
        phonemize("Dr. Hoxha erdhi. Është në shtëpi.", Language::Albanian)
            .unwrap()
            .len(),
        2
    );
}

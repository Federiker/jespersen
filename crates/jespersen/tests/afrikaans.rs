#![cfg(feature = "af")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn afrikaans_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize(
                "Die kinders speel op die werf, 10 % meer as gister.",
                Language::Afrikaans,
            )
            .unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("af-ZA".parse::<Language>().unwrap(), Language::Afrikaans);
    assert_eq!(Language::Afrikaans.to_string(), "af");
    assert_eq!(
        phonemize("Dr. Botha het gekom. Hy is tuis.", Language::Afrikaans)
            .unwrap()
            .len(),
        2
    );
}

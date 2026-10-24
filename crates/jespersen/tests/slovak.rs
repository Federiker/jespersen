#![cfg(feature = "sk")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn slovak_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Deti sa hrajú na ihrisku, mám 21 €.", Language::Slovak).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("sk-SK".parse::<Language>().unwrap(), Language::Slovak);
    assert_eq!(Language::Slovak.to_string(), "sk");
    assert_eq!(
        phonemize("Dr. Novák prišiel. Prof. Kováč tiež.", Language::Slovak)
            .unwrap()
            .len(),
        2
    );
}

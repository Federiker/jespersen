#![cfg(feature = "eu")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn basque_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Haurrak hondartzan daude, % 10 merkeago.", Language::Basque).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("eu-ES".parse::<Language>().unwrap(), Language::Basque);
    assert_eq!(Language::Basque.to_string(), "eu");
    assert_eq!(
        phonemize("Dr. Agirre etorri da. Etxean dago.", Language::Basque)
            .unwrap()
            .len(),
        2
    );
}

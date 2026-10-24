#![cfg(feature = "fi")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn finnish_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Kissa istuu talon katolla.", Language::Finnish).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak), "kˈissa ˈistuː tˈaloŋ kˈatolla.");
    assert_eq!(read(Notation::Ipa), "ˈkissɑ ˈistuː ˈtɑlon ˈkɑtollɑ.");
    assert_eq!("fi-FI".parse::<Language>().unwrap(), Language::Finnish);
    assert_eq!(Language::Finnish.to_string(), "fi");
    assert_eq!(Language::Finnish.notations(), [Notation::Ipa, Notation::Espeak]);
    // esim. does not end the sentence.
    assert_eq!(
        phonemize("Esim. Oulu on kaupunki.", Language::Finnish).unwrap().len(),
        1
    );
}

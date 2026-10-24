#![cfg(feature = "id")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn indonesian_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Anak-anak pergi ke sekolah, harganya Rp 5.000.", Language::Indonesian).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("id-ID".parse::<Language>().unwrap(), Language::Indonesian);
    assert_eq!(Language::Indonesian.to_string(), "id");
    assert_eq!(phonemize("Dr. Siregar datang.", Language::Indonesian).unwrap().len(), 1);
}

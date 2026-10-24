#![cfg(feature = "lv")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn latvian_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Bērni spēlē pagalmā, par 10 % vairāk nekā vakar.", Language::Latvian).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("lv-LV".parse::<Language>().unwrap(), Language::Latvian);
    assert_eq!(Language::Latvian.to_string(), "lv");
    assert_eq!(
        phonemize("Dr. Ozols ir atnācis. Viņš ir mājās.", Language::Latvian)
            .unwrap()
            .len(),
        2
    );
}

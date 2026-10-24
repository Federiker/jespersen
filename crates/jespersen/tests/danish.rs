#![cfg(feature = "da")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn danish_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Børnene leger i haven, 10 % mere end i går.", Language::Danish).unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    // NST's lexicon gives the stød, which plain IPA writes and eSpeak NG's notation does not.
    assert!(read(Notation::Ipa).contains("ˈbœɐ̯ˀnənə"));
    assert!(!read(Notation::Espeak).contains('ˀ'));
    assert_eq!("da-DK".parse::<Language>().unwrap(), Language::Danish);
    assert_eq!(Language::Danish.to_string(), "da");
    assert_eq!(
        phonemize("Dr. Hansen er kommet. Han er hjemme.", Language::Danish)
            .unwrap()
            .len(),
        2
    );
}

#![cfg(feature = "ga")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn irish_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize(
                "Tá an cat ina chodladh ar an mbord, 10 % níos fearr ná inné.",
                Language::Irish,
            )
            .unwrap(),
            options,
        )
        .unwrap()
    };
    println!("{}\n{}", read(Notation::Espeak), read(Notation::Ipa));
    assert_eq!("ga-IE".parse::<Language>().unwrap(), Language::Irish);
    assert_eq!(Language::Irish.to_string(), "ga");
    assert_eq!(
        phonemize("Bhuail an Dr. Ó Briain leis. Tá sé sásta.", Language::Irish)
            .unwrap()
            .len(),
        2
    );
}

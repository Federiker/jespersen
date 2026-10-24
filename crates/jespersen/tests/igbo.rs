#![cfg(feature = "ig")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn igbo_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Ụlọ akwụkwọ ahụ dị mma, ọ na-efu ₦200.", Language::Igbo).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(read(Notation::Espeak).split(' ').next(), Some("ˈʊlɔ"));
    assert_eq!(read(Notation::Ipa).split(' ').next(), Some("ˈʊlɔ"));
    // ₦200 is said as narị abụọ naịra.
    assert!(
        read(Notation::Ipa).contains("ˈnaɾɪ aˈbʊɔ naˈɪɾa"),
        "{}",
        read(Notation::Ipa)
    );
    assert_eq!("ig-NG".parse::<Language>().unwrap(), Language::Igbo);
    assert_eq!(Language::Igbo.to_string(), "ig");
    assert_eq!(phonemize("Maz. Okafor bịara.", Language::Igbo).unwrap().len(), 1);
}

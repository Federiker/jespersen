#![cfg(feature = "fr")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn french_is_read_in_both_notations() {
    let read = |text, notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(&phonemize(text, Language::French).unwrap(), options).unwrap()
    };
    let text = "Les enfants jouent à l'école, M. Dupont a 21 ans.";
    assert_eq!(
        read(text, Notation::Espeak),
        "le-z ɑ̃fˈɑ̃ ʒˈut a lekˈɔl, məsjˈø dypˈɔ̃ a vˈɛ̃t e œ̃n ˈɑ̃."
    );
    assert_eq!(
        read(text, Notation::Ipa),
        "lez ɑ̃ˈfɑ̃ ˈʒut a leˈkɔl, məˈsjø dyˈpɔ̃ a ˈvɛ̃t e œ̃n ˈɑ̃."
    );
    assert_eq!("fr-CA".parse::<Language>().unwrap(), Language::French);
    assert_eq!(Language::French.to_string(), "fr");
    assert_eq!(
        phonemize("M. Martin est là. Mme Dupont aussi.", Language::French)
            .unwrap()
            .len(),
        2
    );
}

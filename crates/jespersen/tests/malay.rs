#![cfg(feature = "ms")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn malay_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Budak itu pergi ke sekolah, harganya RM 5.", Language::Malay).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(
        read(Notation::Espeak),
        "bˈudaʔ ˈitu pərɡˈi kə səkˈolah, harɡˈaɲə lˈimə rˈiŋɡit."
    );
    assert_eq!(
        read(Notation::Ipa),
        "ˈbudaʔ ˈitu pərˈɡi kə səˈkolah, harˈɡaɲə ˈlimə ˈriŋɡit."
    );
    assert_eq!("ms-MY".parse::<Language>().unwrap(), Language::Malay);
    assert_eq!(Language::Malay.to_string(), "ms");
    assert_eq!(phonemize("Dr. Ahmad datang.", Language::Malay).unwrap().len(), 1);
}

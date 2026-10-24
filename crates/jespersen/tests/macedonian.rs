#![cfg(feature = "mk")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn macedonian_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Децата не знаат, имаат 25 ден.", Language::Macedonian).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(
        read(Notation::Espeak),
        "dˈɛtsata nˈɛ znaat, ˈimaat dvˈaɛsɛt i pˈɛt dˈɛnari."
    );
    assert_eq!(
        read(Notation::Ipa),
        "ˈdɛtsata ˈnɛ znaat, ˈimaat ˈdvaɛsɛt i ˈpɛt ˈdɛnari."
    );
    assert_eq!("mk-MK".parse::<Language>().unwrap(), Language::Macedonian);
    assert_eq!(Language::Macedonian.to_string(), "mk");
    assert_eq!(
        phonemize("Д-р Петровски дојде. Проф. Николов исто.", Language::Macedonian)
            .unwrap()
            .len(),
        2
    );
}

#![cfg(feature = "sr")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn serbian_is_read_alike_in_both_scripts() {
    let read = |text: &str, notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(&phonemize(text, Language::Serbian).unwrap(), options).unwrap()
    };
    let cyrillic = "Деца се играју на мору, имам 21 €.";
    let latin = "Deca se igraju na moru, imam 21 €.";
    for notation in [Notation::Espeak, Notation::Ipa] {
        assert_eq!(read(cyrillic, notation), read(latin, notation));
    }
    assert_eq!(
        read(latin, Notation::Ipa),
        "ˈdetsa ˈse ˈiɡraju ˈna ˈmoru, ˌimam ˈdvadeset ˈjedan ˈevro."
    );
    assert_eq!("sr-Latn".parse::<Language>().unwrap(), Language::Serbian);
    assert_eq!("sr-Cyrl-RS".parse::<Language>().unwrap(), Language::Serbian);
    assert_eq!(Language::Serbian.to_string(), "sr");
    assert_eq!(
        phonemize("Др. Петровић је дошао. Prof. Jovanović takođe.", Language::Serbian)
            .unwrap()
            .len(),
        2
    );
}

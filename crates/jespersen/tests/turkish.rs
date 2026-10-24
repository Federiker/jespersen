#![cfg(feature = "tr")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn turkish_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("İstanbul'a gidiyorum, %50 indirim var.", Language::Turkish).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(
        read(Notation::Espeak),
        "ɪstˈanbuɫa ɟidˈijoɾʊm, jøzdˈɛ ællˈɪ ɪndiɾˈɪm vˈar."
    );
    assert_eq!(
        read(Notation::Ipa),
        "isˈtanbula ɡiˈdijorum, jyzˈde elˈli indiˈrim ˈvar."
    );
    assert_eq!("tr-TR".parse::<Language>().unwrap(), Language::Turkish);
    assert_eq!(Language::Turkish.to_string(), "tr");
    assert_eq!(phonemize("Dr. Yılmaz geldi.", Language::Turkish).unwrap().len(), 1);
}

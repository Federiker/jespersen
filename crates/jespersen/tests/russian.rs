#![cfg(feature = "ru")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[test]
fn russian_is_read_in_both_notations() {
    let read = |notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(
            &phonemize("Кошка спит на столе, а цена выросла на 10 %.", Language::Russian).unwrap(),
            options,
        )
        .unwrap()
    };
    assert_eq!(
        read(Notation::Espeak),
        "kˈoʃka spʲˈit nə stʌɭʲˈe, a tsynˈɑ vˈyrʌsɭa nə dʲˈesʲʌtʲ prʌtsˈɛntʌf."
    );
    assert_eq!(
        read(Notation::Ipa),
        "ˈkoʂkə ˈspʲit nə stɐˈlʲe, ɐ tsɨˈna ˈvɨrəsɫə nə ˈdʲesʲɪtʲ prɐˈtsɛntəf."
    );
    assert_eq!("ru-RU".parse::<Language>().unwrap(), Language::Russian);
    assert_eq!(Language::Russian.to_string(), "ru");
    assert_eq!(
        phonemize("Пришёл проф. Иванов. Он сказал «здравствуйте».", Language::Russian)
            .unwrap()
            .len(),
        2
    );
}

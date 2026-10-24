#![cfg(feature = "pt")]

use jespersen::{Language, Notation, PortugueseVariety, WriteOptions, phonemize, write};

#[test]
fn portuguese_is_read_in_both_varieties_and_notations() {
    let read = |text, language, notation| {
        let options = WriteOptions {
            notation,
            drop_punctuation: false,
        };
        write(&phonemize(text, language).unwrap(), options).unwrap()
    };
    let portugal = Language::Portuguese(PortugueseVariety::European);
    let brazil = Language::Portuguese(PortugueseVariety::Brazilian);
    let text = "O gato está no telhado, e os amigos têm 21 livros.";
    assert_eq!(
        read(text, portugal, Notation::Espeak),
        "ʊ ɡˈatw eʃtˌa nʊ tˌeʎˈadʊ, i uz ɐmˈiɡʊʃ tˈeɪŋ vˈiŋtɨ i u\u{303}ŋ lˈivɹʊʃ."
    );
    assert_eq!(
        read(text, portugal, Notation::Ipa),
        "u ˈɡatw ɨʃˌta nu tɨˈʎadu, i uz ɐˈmiɡuʃ ˈtẽj̃ ˈvĩtɨ i u\u{303} ˈlivɾuʃ."
    );
    assert_eq!(
        read(text, brazil, Notation::Espeak),
        "ʊ ɡˈatw estˌa nʊ tˌeljˈadʊ, i ʊz ˌæmˈiɡʊs tˈeɪŋ vˈiŋtʃj i u\u{303}ŋ lˈivrʊs."
    );
    assert_eq!(
        read("A reunião é a 3/5/2024 às 10:30, custa 15 €.", portugal, Notation::Ipa),
        "ɐ ʁewniˈɐ̃w̃ ɛ ɐ ˈtɾeʒ dɨ ˈmaju dɨ ˈdojʒ ˈmil i ˈvĩtɨ i ˈkwatɾw ˌaʒ ˈdɛz i ˈtɾĩtɐ, ˈkuʃtɐ ˈkĩzɨ ˈewɾuʃ."
    );
    assert_eq!("pt-PT".parse::<Language>().unwrap(), portugal);
    assert_eq!("pt_BR".parse::<Language>().unwrap(), brazil);
    assert_eq!(portugal.to_string(), "pt");
    assert_eq!(brazil.to_string(), "pt-BR");
    assert_eq!(
        phonemize("O Dr. Silva chegou. A Sra. Costa também.", portugal)
            .unwrap()
            .len(),
        2
    );
}

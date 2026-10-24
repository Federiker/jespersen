use jespersen_phonemes::Notation;

use crate::{
    numbers::{cardinal, ordinal},
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("kedi", "keˈdi", "kedˈɪ"),
        ("evin", "eˈvin", "evˈɪn"),
        ("bakıyor", "baˈkɯjor", "bakˈɯjɔr"),
        ("gidiyorum", "ɡiˈdijorum", "ɟidˈijoɾʊm"),
        ("nasılsınız", "naˈsɯlsɯnɯz", "nasˈɯɫsɯnɯz"),
        ("iyiyim", "iˈjijim", "ijˈijɪm"),
        ("güzel", "ɡyˈzel", "ɟyzˈæl"),
        ("dağ", "ˈdaː", "dˈaː"),
        ("değil", "deˈjil", "dejˈɪl"),
        ("yağmur", "jaːˈmur", "jaːmˈʊr"),
        ("teşekkür", "teʃekˈkyr", "teʃekːˈør"),
        ("Ankara", "ˈankara", "ˈankaɾa"),
        ("İstanbul", "isˈtanbul", "ɪstˈanbʊɫ"),
        ("kitaplar", "kitapˈlar", "kitapɫˈar"),
        ("çiçek", "tʃiˈtʃek", "tʃitʃˈɛk"),
    ];
    let wrong: Vec<String> = cases
        .iter()
        .filter_map(|(word, ipa, espeak)| {
            let said = pronounce(word).unwrap();
            let (got_ipa, got_espeak) = (said.to_notation(Notation::Ipa), said.to_notation(Notation::Espeak));
            (got_ipa != *ipa || got_espeak != *espeak).then(|| format!("{word}: {got_ipa} {got_espeak}"))
        })
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "sıfır"),
        (21, "yirmi bir"),
        (100, "yüz"),
        (1250, "bin iki yüz elli"),
        (2024, "iki bin yirmi dört"),
        (3_000_000, "üç milyon"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
    assert_eq!(ordinal(1), "birinci");
    assert_eq!(ordinal(4), "dördüncü");
    assert_eq!(ordinal(10), "onuncu");
    assert_eq!(ordinal(6), "altıncı");
}

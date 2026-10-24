use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Luganda, cardinal},
    read_tokens,
    sentence::elide,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("Luganda", "luˈɡaːnda", "luɡˈaːnda"),
        ("omusajja", "omuˈsaɟɟa", "omusˈaɟɟa"),
        ("ekkubo", "ekˈkubo", "ekkˈubo"),
        ("nnyini", "ˈɲɲini", "ɲɲˈini"),
        ("enkoko", "eːˈŋkoko", "eːŋkˈoko"),
        ("ng'ombe", "ˈŋoːmbe", "ŋˈoːmbe"),
        ("ŋŋaali", "ˈŋŋaːli", "ŋŋˈaːli"),
        ("kyalo", "ˈcalo", "cˈalo"),
        ("gyendi", "ˈɟeːndi", "ɟˈeːndi"),
        ("emmere", "emˈmeɾe", "emmˈeɾe"),
        ("Kampala", "kaːˈmpala", "kaːmpˈala"),
        ("taata", "ˈtaːta", "tˈaːta"),
        ("ekkya", "ˈecca", "ˈecca"),
        ("eggya", "ˈeɟɟa", "ˈeɟɟa"),
        ("taxi", "ˈtaksi", "tˈaksi"),
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
    assert!(pronounce("ç").is_none());
    assert!(pronounce("'").is_none());
}

#[test]
fn elided_words_join_the_next() {
    assert_eq!(
        elide("n", pronounce("abantu").unwrap())
            .unwrap()
            .to_notation(Notation::Espeak),
        "nabˈaːntu"
    );
    assert_eq!(
        elide("nga", pronounce("abantu").unwrap())
            .unwrap()
            .to_notation(Notation::Espeak),
        "ŋɡaabˈaːntu"
    );
    assert!(elide("ç", pronounce("abantu").unwrap()).is_none());
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "zeero"),
        (7, "musanvu"),
        (10, "kkumi"),
        (11, "kkumi na emu"),
        (21, "amakumi abiri mu emu"),
        (60, "nkaaga"),
        (100, "kikumi"),
        (125, "kikumi mu amakumi abiri mu ttaano"),
        (600, "lukaaga"),
        (1000, "lukumi"),
        (2024, "enkumi bbiri mu amakumi abiri mu nnya"),
        (1_000_000, "akakadde kamu"),
        (3_000_000, "obukadde ssatu"),
        (2_000_000_000, "obuwumbi bbiri"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_says_signs_and_dates() {
    let locale = Luganda;
    assert_eq!(locale.ordinal(1, Gender::Alone).as_deref(), Some("okusooka"));
    assert_eq!(locale.ordinal(2, Gender::Alone).as_deref(), Some("bbiri"));
    assert_eq!(locale.cardinal(4, Gender::Alone).as_deref(), Some("nnya"));
    assert_eq!(
        locale.date(3, 5, 2024).as_deref(),
        Some("ssatu Maayi enkumi bbiri mu amakumi abiri mu nnya")
    );
    assert_eq!(locale.date(3, 13, 2024), None);
    assert_eq!(
        (locale.decimal_word(','), locale.decimal_word('.')),
        ("kkoma", "akatonnyeze")
    );
    assert_eq!(
        (locale.range(), locale.ratio(), locale.minus(), locale.plus()),
        ("okutuuka", "ku", "kuggyako", "gattako")
    );
    assert_eq!(locale.percent(), "ku kikumi");
    assert_eq!(locale.currency("UGX"), Some(("ssiringi", "ssiringi")));
    for sign in ["€", "$", "£"] {
        assert!(locale.currency(sign).is_some());
    }
    assert_eq!(locale.currency("¥"), None);
    for symbol in ["%", "&", "@", "+", "=", "°", "#", "€", "$"] {
        assert!(locale.symbol(symbol).is_some(), "{symbol}");
    }
    assert_eq!(locale.symbol("~"), None);
    for letter in ('a'..='z').chain(['ŋ']) {
        assert!(locale.letter(letter).is_some(), "{letter}");
    }
    assert_eq!(locale.letter('ß'), None);
    for sign in ['@', '.', '/', ':', '-', '_'] {
        assert!(locale.address_sign(sign).is_some(), "{sign}");
    }
    assert_eq!(locale.address_sign('!'), None);
    assert_eq!(locale.abbreviation("Dr"), Some("dokita"));
    assert!(crate::is_abbreviation("Omw"));
    assert!(!crate::is_abbreviation("omusajja"));
    assert!(locale.elides());
    assert_eq!(locale.vowels(), "aeiou");
}

#[test]
fn connectives_lose_their_stress() {
    let tokens = [
        ("Ekitabo", TokenKind::Word),
        ("kya", TokenKind::Word),
        ("omwana", TokenKind::Word),
    ];
    let said: Vec<String> = read_tokens(&tokens)
        .iter()
        .flatten()
        .flat_map(|reading| reading.words.iter())
        .map(|word| word.to_notation(Notation::Espeak))
        .collect();
    assert_eq!(said, ["ekitˈabo", "ca", "omwˈana"]);
}

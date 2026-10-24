use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Somali, cardinal},
    read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("Soomaali", "soːˈmaːli", "soːmˈaːli"),
        ("magaalo", "maˈɣaːlo", "maɣˈaːlo"),
        ("khamiis", "xaˈmiːs", "xamˈiːs"),
        ("cad", "ˈʕad", "ʔˈad"),
        ("xoog", "ˈħoːɡ", "hˈoːɡ"),
        ("dhul", "ˈɖul", "ɖˈul"),
        ("gabadh", "ˈɡaβaɖ", "ɡˈaβaɖ"),
        ("aabbe", "ˈaːbbe", "ˈaːbbe"),
        ("aabo", "ˈaːβo", "ˈaːβo"),
        ("hooyo", "ˈhoːjo", "hˈoːjo"),
        ("Muqdisho", "muqˈdiʃo", "muqdˈiʃo"),
        ("ba'an", "ˈbaʔan", "bˈaʔan"),
        ("shan", "ˈʃan", "ʃˈan"),
        ("waddan", "ˈwaddan", "wˈaddan"),
        ("jacayl", "ˈdʒaʕajl", "dʒˈaʔajl"),
        ("qof", "ˈqof", "qˈof"),
        ("dadka", "ˈdadka", "dˈadka"),
        ("fiican", "ˈfiːʕan", "fˈiːʔan"),
        ("Zanzibar", "zanˈziβar", "zanzˈiβar"),
        ("televishan", "teleˈviʃan", "televˈiʃan"),
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
fn numbers_in_words() {
    for (number, words) in [
        (0, "eber"),
        (7, "toddoba"),
        (10, "toban"),
        (11, "kow iyo toban"),
        (21, "kow iyo labaatan"),
        (45, "shan iyo afartan"),
        (90, "sagaashan"),
        (100, "boqol"),
        (105, "boqol iyo shan"),
        (125, "boqol iyo shan iyo labaatan"),
        (200, "laba boqol"),
        (1000, "kun"),
        (1990, "kun iyo sagaal boqol iyo sagaashan"),
        (2024, "laba kun iyo afar iyo labaatan"),
        (1_000_000, "malyuun"),
        (3_000_000_000, "saddex bilyan"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_says_signs_and_dates() {
    let locale = Somali;
    for (number, words) in [
        (1, "kowaad"),
        (2, "labaad"),
        (3, "saddexaad"),
        (4, "afraad"),
        (7, "toddobaad"),
        (10, "tobanaad"),
        (21, "kow iyo labaatanaad"),
    ] {
        assert_eq!(
            locale.ordinal(number, Gender::Alone).as_deref(),
            Some(words),
            "{number}"
        );
    }
    assert_eq!(locale.cardinal(4, Gender::Alone).as_deref(), Some("afar"));
    assert_eq!(
        locale.date(3, 5, 2024).as_deref(),
        Some("saddex Maajo laba kun iyo afar iyo labaatan")
    );
    assert_eq!(locale.date(3, 13, 2024), None);
    assert_eq!(
        (locale.decimal_word(','), locale.decimal_word('.')),
        ("hakad", "dhibic")
    );
    assert_eq!(
        (locale.range(), locale.ratio(), locale.minus(), locale.plus()),
        ("ilaa", "ka", "taban", "lagu daray")
    );
    assert!(locale.percent_first());
    assert_eq!(locale.percent(), "boqolkiiba");
    assert_eq!(locale.currency("Sh"), Some(("shilin", "shilin")));
    for sign in ["SOS", "$", "€"] {
        assert!(locale.currency(sign).is_some());
    }
    assert_eq!(locale.currency("¥"), None);
    for symbol in ["%", "&", "+", "=", "#", "$", "€"] {
        assert!(locale.symbol(symbol).is_some(), "{symbol}");
    }
    assert_eq!(locale.symbol("~"), None);
    for letter in 'a'..='z' {
        assert!(locale.letter(letter).is_some(), "{letter}");
    }
    assert_eq!(locale.letter('ß'), None);
    for sign in ['@', '.', '/', ':', '-', '_'] {
        assert!(locale.address_sign(sign).is_some(), "{sign}");
    }
    assert_eq!(locale.address_sign('!'), None);
    assert_eq!(locale.abbreviation("Dr"), Some("dhakhtar"));
    assert!(crate::is_abbreviation("Md"));
    assert!(!crate::is_abbreviation("hooyo"));
    assert_eq!(locale.vowels(), "aeiou");
}

#[test]
fn particles_lose_their_stress() {
    let tokens = [
        ("Aabbe", TokenKind::Word),
        ("iyo", TokenKind::Word),
        ("hooyo", TokenKind::Word),
    ];
    let read = read_tokens(&tokens);
    let said: Vec<String> = read
        .iter()
        .flatten()
        .flat_map(|reading| reading.words.iter())
        .map(|word| word.to_notation(Notation::Espeak))
        .collect();
    assert_eq!(said, ["ˈaːbbe", "ijo", "hˈoːjo"]);
}

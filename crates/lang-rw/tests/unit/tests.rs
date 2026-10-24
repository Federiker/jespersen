use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Kinyarwanda, cardinal},
    read_tokens,
    sentence::elide,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("umuntu", "uˈmuːntu", "umˈuːntu"),
        ("Rwanda", "ˈɾɡwaːnda", "ɾɡwˈaːnda"),
        ("inka", "ˈiːŋka", "ˈiːŋka"),
        ("umwana", "uˈmŋwana", "umŋwˈana"),
        ("abantu", "aˈβaːntu", "aβˈaːntu"),
        ("imbwa", "ˈiːmbɡwa", "ˈiːmbɡwa"),
        ("icyumba", "iˈcuːmba", "icˈuːmba"),
        ("ishyamba", "iˈʃcaːmba", "iʃcˈaːmba"),
        ("jye", "ˈɟe", "ɟˈe"),
        ("ijya", "ˈiɟa", "ˈiɟa"),
        ("cyane", "ˈcane", "cˈane"),
        ("amazi", "aˈmazi", "amˈazi"),
        ("twese", "ˈtkwese", "tkwˈese"),
        ("isoko", "iˈsoko", "isˈoko"),
        ("ubwenge", "uˈβɡweːŋɡe", "uβɡwˈeːŋɡe"),
        ("pfa", "ˈpfa", "pfˈa"),
        ("itsinda", "iˈtsiːnda", "itsˈiːnda"),
        ("baaba", "ˈβaːβa", "βˈaːβa"),
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
    let word = pronounce("umwana").unwrap();
    assert_eq!(
        elide("n", word.clone()).unwrap().to_notation(Notation::Espeak),
        "numŋwˈana"
    );
    assert_eq!(
        elide("nk", pronounce("uko").unwrap())
            .unwrap()
            .to_notation(Notation::Espeak),
        "ŋkˈuko"
    );
    assert_eq!(
        elide("kandi", word).unwrap().to_notation(Notation::Espeak),
        "kaːndiumŋwˈana"
    );
    assert!(elide("ç", pronounce("uko").unwrap()).is_none());
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "zeru"),
        (7, "karindwi"),
        (10, "icumi"),
        (11, "cumi na rimwe"),
        (20, "makumyabiri"),
        (45, "mirongo ine na gatanu"),
        (100, "ijana"),
        (125, "ijana na makumyabiri na gatanu"),
        (800, "magana umunani"),
        (1000, "igihumbi"),
        (1250, "igihumbi na magana abiri na mirongo itanu"),
        (2024, "ibihumbi bibiri na makumyabiri na kane"),
        (15_000, "ibihumbi cumi na gatanu"),
        (2_000_000, "miliyoni kabiri"),
        (3_000_000_000, "miliyari gatatu"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_says_signs_and_dates() {
    let locale = Kinyarwanda;
    assert_eq!(locale.ordinal(1, Gender::Alone).as_deref(), Some("mbere"));
    assert_eq!(locale.ordinal(2, Gender::Alone).as_deref(), Some("kabiri"));
    assert_eq!(locale.cardinal(4, Gender::Alone).as_deref(), Some("kane"));
    assert_eq!(
        locale.date(3, 5, 2024).as_deref(),
        Some("gatatu Gicurasi ibihumbi bibiri na makumyabiri na kane")
    );
    assert_eq!(locale.date(3, 13, 2024), None);
    assert_eq!(
        (locale.decimal_word(','), locale.decimal_word('.')),
        ("akitso", "akadomo")
    );
    assert_eq!(
        (locale.range(), locale.ratio(), locale.minus(), locale.plus()),
        ("kugeza", "kuri", "gukuramo", "wongeyeho")
    );
    assert_eq!(locale.percent(), "ku ijana");
    assert_eq!(locale.currency("Frw"), Some(("ifaranga", "amafaranga")));
    for sign in ["€", "$", "£"] {
        assert!(locale.currency(sign).is_some());
    }
    assert_eq!(locale.currency("¥"), None);
    for symbol in ["%", "&", "@", "+", "=", "°", "#", "€", "$"] {
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
    assert_eq!(locale.abbreviation("Dr"), Some("dogiteri"));
    assert!(crate::is_abbreviation("Nyak"));
    assert!(!crate::is_abbreviation("umuntu"));
    assert!(locale.elides());
    assert_eq!(locale.vowels(), "aeiou");
}

#[test]
fn connectives_lose_their_stress() {
    let tokens = [
        ("Igitabo", TokenKind::Word),
        ("cya", TokenKind::Word),
        ("umwana", TokenKind::Word),
    ];
    let said: Vec<String> = read_tokens(&tokens)
        .iter()
        .flatten()
        .flat_map(|reading| reading.words.iter())
        .map(|word| word.to_notation(Notation::Espeak))
        .collect();
    assert_eq!(said, ["iɡitˈaβo", "ca", "umŋwˈana"]);
}

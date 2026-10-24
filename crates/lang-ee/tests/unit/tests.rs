use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Ewe, cardinal},
    read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("Eʋegbe", "eˈβeɡ\u{361}be", "eβˈeɡbe"),
        ("ɖevi", "ˈɖevi", "ɖˈevi"),
        ("ƒe", "ˈɸe", "ɸˈe"),
        ("ŋkeke", "ŋ\u{329}ˈkeke", "ŋkˈeke"),
        ("ɣe", "ˈɣe", "ɣˈe"),
        ("kple", "ˈk\u{361}ple", "kplˈe"),
        ("nyɔnu", "ˈɲɔnu", "ɲˈɔnu"),
        ("dzidzɔ", "ˈdzidzɔ", "dzˈidzɔ"),
        ("atsiã", "aˈtsia\u{303}", "atsˈia\u{303}"),
        ("ɛ\u{303}", "ˈɛ\u{303}", "ˈɛ\u{303}"),
        ("hãdzidzi", "ɦa\u{303}ˈdzidzi", "ha\u{303}dzˈidzi"),
        ("Mawu", "ˈmawu", "mˈawu"),
        ("xɔ", "ˈxɔ", "xˈɔ"),
        ("ʋu", "ˈβu", "βˈu"),
        ("dɔn", "ˈdɔ\u{303}", "dˈɔ\u{303}"),
        ("atɔ\u{303}", "ˈatɔ\u{303}", "ˈatɔ\u{303}"),
        ("m̀", "ˈm\u{329}", "ˈm"),
        ("blema", "ˈblema", "blˈema"),
        ("Tɔgbi", "ˈtɔɡ\u{361}bi", "tˈɔɡbi"),
        ("rɔ", "ˈɾɔ", "ɾˈɔ"),
        ("zero", "ˈzeɾo", "zˈeɾo"),
        ("fiafitɔ", "fiaˈfitɔ", "fiafˈitɔ"),
        ("taxi", "ˈtaxi", "tˈaxi"),
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
fn numbers_in_tens() {
    for (number, words) in [
        (0, "zero"),
        (1, "ɖeka"),
        (10, "ewo"),
        (11, "wuiɖekɛ"),
        (12, "wuieve"),
        (20, "blaeve"),
        (21, "blaeve vɔ ɖeka"),
        (99, "blaasieke vɔ asieke"),
        (100, "alafa ɖeka"),
        (125, "alafa ɖeka kple blaeve vɔ atɔ̃"),
        (1000, "akpe ɖeka"),
        (2024, "akpe eve kple blaeve vɔ ene"),
        (12_000, "akpe wuieve"),
        (1_000_000, "miliɔn ɖeka"),
        (3_000_000_000, "biliɔn etɔ̃"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
    for number in (0..10_000).step_by(7) {
        let words = cardinal(number);
        assert!(
            words.split(' ').all(|word| pronounce(word).is_some()),
            "{number}: {words}"
        );
    }
}

#[test]
fn the_locale_says_signs_and_dates() {
    let locale = Ewe;
    assert_eq!(locale.ordinal(1, Gender::Alone).as_deref(), Some("gbãtɔ"));
    assert_eq!(locale.ordinal(2, Gender::Alone).as_deref(), Some("evelia"));
    assert_eq!(locale.cardinal(4, Gender::Alone).as_deref(), Some("ene"));
    assert_eq!(
        locale.date(3, 5, 2024).as_deref(),
        Some("ŋkeke etɔ̃ Damɛ ƒe akpe eve kple blaeve vɔ ene")
    );
    assert_eq!(locale.date(3, 13, 2024), None);
    assert_eq!((locale.decimal_word(','), locale.decimal_word('.')), ("kɔma", "pɔint"));
    assert_eq!(
        (locale.range(), locale.minus(), locale.plus(), locale.percent()),
        ("va ɖo", "ɖe eme", "kpe ɖe", "le alafa me")
    );
    assert_eq!(locale.currency("₵"), Some(("sidi", "sidi")));
    for sign in ["GH₵", "GHS", "CFA", "FCFA", "€", "$", "£"] {
        assert!(locale.currency(sign).is_some());
    }
    assert_eq!(locale.currency("¥"), None);
    for symbol in ["%", "&", "@", "+", "=", "#", "₵", "$"] {
        assert!(locale.symbol(symbol).is_some(), "{symbol}");
    }
    assert_eq!(locale.symbol("~"), None);
    for letter in ('a'..='z').chain(['ɖ', 'ɛ', 'ƒ', 'ɣ', 'ŋ', 'ɔ', 'ʋ']) {
        let name = locale.letter(letter).unwrap();
        assert!(pronounce(name).is_some(), "{letter}");
    }
    assert_eq!(locale.letter('ß'), None);
    for sign in ['@', '.', '/', ':', '-', '_'] {
        let words = locale.address_sign(sign).unwrap();
        assert!(words.split(' ').all(|word| pronounce(word).is_some()), "{sign}");
    }
    assert_eq!(locale.address_sign('!'), None);
    assert_eq!(locale.abbreviation("Dr"), Some("dɔkta"));
    assert!(crate::is_abbreviation("Prof"));
    assert!(!crate::is_abbreviation("ɖevi"));
    assert_eq!(locale.vowels(), "aeɛioɔu");
}

#[test]
fn function_words_lose_their_stress_whatever_their_marks() {
    let tokens = [
        ("Ɖevi", TokenKind::Word),
        ("lá", TokenKind::Word),
        ("le", TokenKind::Word),
        ("suku", TokenKind::Word),
    ];
    let said: Vec<String> = read_tokens(&tokens)
        .iter()
        .flatten()
        .flat_map(|reading| reading.words.iter())
        .map(|word| word.to_notation(Notation::Espeak))
        .collect();
    assert_eq!(said, ["ɖˈevi", "la", "le", "sˈuku"]);
}

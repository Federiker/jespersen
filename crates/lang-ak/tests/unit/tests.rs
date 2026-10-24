use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Akan, cardinal},
    read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("Akan", "ˈakan", "ˈakan"),
        ("Twi", "ˈtɕʷi", "tʃwˈi"),
        ("kyɛ", "ˈtɕɛ", "tʃˈɛ"),
        ("gyata", "ˈdʑata", "dʒˈata"),
        ("hyɛ", "ˈɕɛ", "ʃˈɛ"),
        ("dwom", "ˈdʑʷom", "dʒwˈom"),
        ("hwɛ", "ˈɕʷɛ", "ʃwˈɛ"),
        ("nkɔ", "ˈŋ\u{329}kɔ", "ˈŋkɔ"),
        ("nnipa", "n\u{329}ˈnipa", "nnˈipa"),
        ("mpɔ", "ˈm\u{329}pɔ", "ˈmpɔ"),
        ("ɔman", "ˈɔman", "ˈɔman"),
        ("Kwame", "ˈkʷame", "kwˈame"),
        ("Nyame", "ˈɲame", "ɲˈame"),
        ("anwummerɛ", "aɲʷumˈmeɾɛ", "aɲwummˈeɾɛ"),
        ("nsuo", "n\u{329}ˈsuo", "nsˈuo"),
        ("ɛdan", "ˈɛdan", "ˈɛdan"),
        ("sika", "ˈsika", "sˈika"),
        ("Yesu", "ˈjesu", "jˈesu"),
        ("jɛ", "ˈdʑɛ", "dʒˈɛ"),
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
fn numbers_in_tens() {
    for (number, words) in [
        (0, "hwee"),
        (1, "baako"),
        (10, "du"),
        (11, "dubaako"),
        (12, "dumienu"),
        (20, "aduonu"),
        (21, "aduonu baako"),
        (100, "ɔha"),
        (125, "ɔha ne aduonu anum"),
        (200, "ahaanu"),
        (1000, "apem"),
        (2024, "mpem abien ne aduonu anan"),
        (12_000, "mpem dumienu"),
        (1_000_000, "ɔpepem"),
        (3_000_000_000, "ɔpepepem abiɛsa"),
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
    let locale = Akan;
    assert_eq!(locale.ordinal(1, Gender::Alone).as_deref(), Some("kan"));
    assert_eq!(locale.ordinal(2, Gender::Alone).as_deref(), Some("a ɛtɔ so abien"));
    assert_eq!(locale.cardinal(4, Gender::Alone).as_deref(), Some("anan"));
    assert_eq!(
        locale.date(3, 5, 2024).as_deref(),
        Some("Kɔtɔnimma da abiɛsa afe mpem abien ne aduonu anan")
    );
    assert_eq!(locale.date(3, 13, 2024), None);
    assert_eq!((locale.decimal_word(','), locale.decimal_word('.')), ("kɔma", "pɔint"));
    assert_eq!(
        (locale.range(), locale.minus(), locale.plus(), locale.percent()),
        ("kosi", "yi fi", "ka ho", "ɔha mu")
    );
    assert_eq!(locale.currency("₵"), Some(("sidi", "sidi")));
    for sign in ["GH₵", "GHS", "€", "$", "£"] {
        assert!(locale.currency(sign).is_some());
    }
    assert_eq!(locale.currency("¥"), None);
    for symbol in ["%", "&", "@", "+", "=", "#", "₵", "$"] {
        assert!(locale.symbol(symbol).is_some(), "{symbol}");
    }
    assert_eq!(locale.symbol("~"), None);
    for letter in ('a'..='z').chain(['ɛ', 'ɔ']) {
        let name = locale.letter(letter).unwrap();
        assert!(pronounce(name).is_some(), "{letter}");
    }
    assert_eq!(locale.letter('ß'), None);
    for sign in ['@', '.', '/', ':', '-', '_'] {
        let words = locale.address_sign(sign).unwrap();
        assert!(words.split(' ').all(|word| pronounce(word).is_some()), "{sign}");
    }
    assert_eq!(locale.address_sign('!'), None);
    assert_eq!(locale.abbreviation("Owu"), Some("owura"));
    assert!(crate::is_abbreviation("Dr"));
    assert!(!crate::is_abbreviation("sika"));
    assert_eq!(locale.vowels(), "aeɛioɔu");
}

#[test]
fn function_words_lose_their_stress_whatever_their_marks() {
    let tokens = [
        ("Abofra", TokenKind::Word),
        ("no", TokenKind::Word),
        ("wɔ", TokenKind::Word),
        ("fie", TokenKind::Word),
    ];
    let said: Vec<String> = read_tokens(&tokens)
        .iter()
        .flatten()
        .flat_map(|reading| reading.words.iter())
        .map(|word| word.to_notation(Notation::Espeak))
        .collect();
    assert_eq!(said, ["abˈofɾa", "no", "wɔ", "fˈie"]);
}

use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Oromo, cardinal},
    read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("Oromoo", "oˈromoː", "orˈomoː"),
        ("barataa", "baˈrataː", "barˈataː"),
        ("dhugaa", "ˈɗuɡaː", "dˈuɡaː"),
        ("haadha", "ˈhaːɗa", "hˈaːda"),
        ("qopphaa'uu", "kʼopʼˈpʼaːʔuː", "koppˈaːʔuː"),
        ("xiqqaa", "ˈtʼikʼkʼaː", "tˈikkaː"),
        ("caffee", "ˈtʃʼaffeː", "tʃˈaffeː"),
        ("nyaata", "ˈɲaːta", "ɲˈaːta"),
        ("shan", "ˈʃan", "ʃˈan"),
        ("ja'a", "ˈdʒaʔa", "dʒˈaʔa"),
        ("bishaan", "ˈbiʃaːn", "bˈiʃaːn"),
        ("Finfinnee", "finˈfinneː", "finfˈinneː"),
        ("beekumsa", "beːˈkumsa", "beːkˈumsa"),
        ("barachuu", "baˈratʃuː", "barˈatʃuː"),
        ("tsaatsee", "ˈtsʼaːtsʼeː", "tsˈaːtseː"),
        ("televizhinii", "televiˈʒiniː", "televiʒˈiniː"),
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
        (0, "duwwaa"),
        (6, "ja'a"),
        (10, "kudhan"),
        (11, "kudha tokko"),
        (20, "digdama"),
        (21, "digdamii tokko"),
        (45, "afurtamii shan"),
        (100, "dhibba tokko"),
        (125, "dhibba tokko fi digdamii shan"),
        (300, "dhibba sadii"),
        (1990, "kuma tokko fi dhibba sagal fi sagaltama"),
        (2024, "kuma lama fi digdamii afur"),
        (1_000_000, "miliyoona tokko"),
        (3_000_000_000, "biiliyoona sadii"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_says_signs_and_dates() {
    let locale = Oromo;
    for (number, words) in [
        (1, "tokkoffaa"),
        (2, "lammaffaa"),
        (3, "sadaffaa"),
        (10, "kurnaffaa"),
        (20, "digdamaffaa"),
        (0, "duwwaaffaa"),
    ] {
        assert_eq!(
            locale.ordinal(number, Gender::Alone).as_deref(),
            Some(words),
            "{number}"
        );
    }
    assert_eq!(locale.cardinal(4, Gender::Alone).as_deref(), Some("afur"));
    assert_eq!(
        locale.date(3, 5, 2024).as_deref(),
        Some("sadii Caamsa kuma lama fi digdamii afur")
    );
    assert_eq!(locale.date(3, 13, 2024), None);
    assert_eq!(
        (locale.decimal_word(','), locale.decimal_word('.')),
        ("koomaa", "tuqaa")
    );
    assert_eq!(
        (locale.range(), locale.ratio(), locale.minus(), locale.plus()),
        ("hanga", "hanga", "hir'isuu", "ida'uu")
    );
    assert!(locale.percent_first());
    assert_eq!(locale.percent(), "dhibbantaa");
    assert_eq!(locale.currency("Br"), Some(("birrii", "birrii")));
    for sign in ["ETB", "$", "€"] {
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
    assert_eq!(locale.abbreviation("Ob"), Some("obbo"));
    assert!(crate::is_abbreviation("Dr"));
    assert!(!crate::is_abbreviation("haadha"));
    assert_eq!(locale.vowels(), "aeiou");
}

#[test]
fn particles_lose_their_stress() {
    let tokens = [
        ("Abbaa", TokenKind::Word),
        ("fi", TokenKind::Word),
        ("haadha", TokenKind::Word),
    ];
    let read = read_tokens(&tokens);
    let said: Vec<String> = read
        .iter()
        .flatten()
        .flat_map(|reading| reading.words.iter())
        .map(|word| word.to_notation(Notation::Espeak))
        .collect();
    assert_eq!(said, ["ˈabbaː", "fi", "hˈaːda"]);
}

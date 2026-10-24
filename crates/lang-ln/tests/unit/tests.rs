use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Lingala, cardinal},
    read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("mbóka", "ˈmboka", "mbˈoka"),
        ("ndako", "ˈndako", "ndˈako"),
        ("Nzámbe", "ˈnzambe", "nzˈambe"),
        ("mokɔlɔ", "moˈkɔlɔ", "mokˈɔlɔ"),
        ("lɛlɔ́", "ˈlɛlɔ", "lˈɛlɔ"),
        ("nkoko", "ˈŋkoko", "ŋkˈoko"),
        ("nyama", "ˈɲama", "ɲˈama"),
        ("ngai", "ˈŋɡai", "ŋɡˈai"),
        ("mwana", "ˈmwana", "mwˈana"),
        ("libóta", "liˈbota", "libˈota"),
        ("lèlò", "ˈlɛlɔ", "lˈɛlɔ"),
        ("naino", "naˈino", "naˈino"),
        ("kaa", "ˈkaː", "kˈaː"),
        ("shamba", "ˈʃamba", "ʃˈamba"),
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
    // A tone mark written as a combining accent is set aside too.
    assert_eq!(
        pronounce("le\u{301}lo\u{301}").unwrap().to_notation(Notation::Espeak),
        "lˈelo"
    );
    assert!(pronounce("ç").is_none());
    assert!(pronounce("'").is_none());
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "zero"),
        (1, "mɔkɔ"),
        (10, "zomi"),
        (11, "zomi na mɔkɔ"),
        (20, "ntuku mibale"),
        (45, "ntuku minei na mitano"),
        (100, "nkama"),
        (125, "nkama na ntuku mibale na mitano"),
        (300, "nkama misato"),
        (1000, "nkoto"),
        (2024, "nkoto mibale na ntuku mibale na minei"),
        (1_000_000, "milio"),
        (3_000_000_000, "miliari misato"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_says_signs_and_dates() {
    let locale = Lingala;
    assert_eq!(locale.ordinal(1, Gender::Alone).as_deref(), Some("yambo"));
    assert_eq!(locale.ordinal(2, Gender::Alone).as_deref(), Some("mibale"));
    assert_eq!(locale.cardinal(4, Gender::Alone).as_deref(), Some("minei"));
    assert_eq!(
        locale.date(3, 5, 2024).as_deref(),
        Some("misato sanza ya mitano nkoto mibale na ntuku mibale na minei")
    );
    assert_eq!(
        locale.date(1, 1, 2000).as_deref(),
        Some("mɔkɔ sanza ya yambo nkoto mibale")
    );
    assert_eq!(locale.date(3, 13, 2024), None);
    assert_eq!(locale.date(3, 0, 2024), None);
    assert_eq!((locale.decimal_word(','), locale.decimal_word('.')), ("koma", "pwɛ"));
    assert_eq!(
        (locale.range(), locale.ratio(), locale.minus(), locale.plus()),
        ("tii", "na", "longola", "bakisa")
    );
    assert_eq!(locale.percent(), "na nkama");
    assert_eq!(locale.currency("FC"), Some(("falanga", "falanga")));
    for sign in ["€", "$", "£"] {
        assert!(locale.currency(sign).is_some());
    }
    assert_eq!(locale.currency("¥"), None);
    for symbol in ["%", "&", "@", "+", "=", "°", "#", "€", "$"] {
        assert!(locale.symbol(symbol).is_some(), "{symbol}");
    }
    assert_eq!(locale.symbol("~"), None);
    for letter in ('a'..='z').chain(['ɛ', 'ɔ']) {
        assert!(locale.letter(letter).is_some(), "{letter}");
    }
    assert_eq!(locale.letter('ß'), None);
    for sign in ['@', '.', '/', ':', '-', '_'] {
        assert!(locale.address_sign(sign).is_some(), "{sign}");
    }
    assert_eq!(locale.address_sign('!'), None);
    assert_eq!(locale.abbreviation("Dr"), Some("dokotolo"));
    assert!(crate::is_abbreviation("Prof"));
    assert!(!crate::is_abbreviation("mbóka"));
    assert_eq!(locale.vowels(), "aeɛioɔu");
}

#[test]
fn connectives_lose_their_stress() {
    let tokens = [
        ("Ndako", TokenKind::Word),
        ("ya", TokenKind::Word),
        ("mwana", TokenKind::Word),
    ];
    let said: Vec<String> = read_tokens(&tokens)
        .iter()
        .flatten()
        .flat_map(|reading| reading.words.iter())
        .map(|word| word.to_notation(Notation::Espeak))
        .collect();
    assert_eq!(said, ["ndˈako", "ja", "mwˈana"]);
}

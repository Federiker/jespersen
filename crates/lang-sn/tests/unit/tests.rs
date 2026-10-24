use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Shona, cardinal},
    read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("baba", "ˈɓaɓa", "bˈaba"),
        ("mbudzi", "ˈmbudzi", "mbˈudzi"),
        ("bhuku", "ˈb̤uku", "bˈuku"),
        ("dhadha", "ˈd̤ad̤a", "dˈada"),
        ("dura", "ˈɗuɾa", "dˈuɾa"),
        ("mhuri", "ˈm̤uɾi", "mˈuɾi"),
        ("vanhu", "ˈʋan̤u", "ʋˈanu"),
        ("vhiki", "ˈviki", "vˈiki"),
        ("svika", "ˈs͎ika", "swˈika"),
        ("zvakanaka", "z͎akaˈnaka", "zwakanˈaka"),
        ("tsvuku", "ˈts͎uku", "tswˈuku"),
        ("dzvinyu", "ˈdz͎iɲu", "dzwˈiɲu"),
        ("chingwa", "ˈtʃiŋɡwa", "tʃˈiŋɡwa"),
        ("nguva", "ˈŋɡuʋa", "ŋɡˈuʋa"),
        ("ng'ombe", "ˈŋombe", "ŋˈombe"),
        ("zhizha", "ˈʒiʒa", "ʒˈiʒa"),
        ("hapana", "ɦaˈpana", "hapˈana"),
        ("pfungwa", "ˈpfuŋɡwa", "pfˈuŋɡwa"),
        ("bvudzi", "ˈbvudzi", "bvˈudzi"),
        ("shiri", "ˈʃiɾi", "ʃˈiɾi"),
        ("nkuku", "ˈŋkuku", "ŋkˈuku"),
        ("jongwe", "ˈdʒoŋɡwe", "dʒˈoŋɡwe"),
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
fn numbers_in_words() {
    for (number, words) in [
        (0, "ziro"),
        (1, "poshi"),
        (4, "ina"),
        (10, "gumi"),
        (11, "gumi nerimwe"),
        (14, "gumi nena"),
        (20, "makumi maviri"),
        (45, "makumi mana neshanu"),
        (100, "zana"),
        (125, "zana nemakumi maviri neshanu"),
        (300, "mazana matatu"),
        (1000, "churu"),
        (2024, "zviuru zviviri nemakumi maviri nena"),
        (15_000, "zviuru gumi neshanu"),
        (1_000_000, "miriyoni"),
        (3_000_000, "mamiriyoni tatu"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_says_signs_and_dates() {
    let locale = Shona;
    assert_eq!(locale.ordinal(1, Gender::Alone).as_deref(), Some("kutanga"));
    assert_eq!(locale.ordinal(2, Gender::Alone).as_deref(), Some("piri"));
    assert_eq!(locale.cardinal(3, Gender::Alone).as_deref(), Some("tatu"));
    assert_eq!(
        locale.date(3, 5, 2024).as_deref(),
        Some("tatu Chivabvu zviuru zviviri nemakumi maviri nena")
    );
    assert_eq!(locale.date(3, 13, 2024), None);
    assert_eq!((locale.decimal_word(','), locale.decimal_word('.')), ("koma", "poindi"));
    assert_eq!(
        (locale.range(), locale.ratio(), locale.minus(), locale.plus()),
        ("kusvika", "ku", "bvisa", "wedzera")
    );
    assert_eq!(locale.percent(), "muzana");
    assert_eq!(locale.currency("$"), Some(("dhora", "madhora")));
    for sign in ["€", "£", "ZiG"] {
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
    assert_eq!(locale.abbreviation("Dr"), Some("chiremba"));
    assert!(crate::is_abbreviation("Prof"));
    assert!(!crate::is_abbreviation("baba"));
    assert_eq!(locale.vowels(), "aeiou");
}

#[test]
fn connectives_lose_their_stress() {
    let tokens = [
        ("Bhuku", TokenKind::Word),
        ("ra", TokenKind::Word),
        ("mwana", TokenKind::Word),
    ];
    let said: Vec<String> = read_tokens(&tokens)
        .iter()
        .flatten()
        .flat_map(|reading| reading.words.iter())
        .map(|word| word.to_notation(Notation::Espeak))
        .collect();
    assert_eq!(said, ["bˈuku", "ɾa", "mwˈana"]);
}

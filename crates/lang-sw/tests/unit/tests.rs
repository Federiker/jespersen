use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Swahili, cardinal},
    read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("kitabu", "kiˈtabu", "kitˈabu"),
        ("habari", "haˈbari", "habˈari"),
        ("mtu", "ˈm̩tu", "ˈmtu"),
        ("nchi", "ˈn̩tʃi", "ˈntʃi"),
        ("mji", "ˈm̩dʒi", "ˈmdʒi"),
        ("mzee", "m̩ˈzɛɛ", "mzˈɛɛ"),
        ("mwanamke", "mwanaˈm̩kɛ", "mwanaˈmkɛ"),
        ("mbwa", "ˈm̩bwa", "ˈmbwa"),
        ("mvua", "m̩ˈvua", "mvˈua"),
        ("chumvi", "ˈtʃumvi", "tʃˈumvi"),
        ("zungumza", "zuˈŋɡumza", "zuŋɡˈumza"),
        ("nzi", "ˈn̩zi", "ˈnzi"),
        ("mbuzi", "ˈmbuzi", "mbˈuzi"),
        ("tembo", "ˈtɛmbɔ", "tˈɛmbɔ"),
        ("ngoma", "ˈŋɡɔma", "ŋɡˈɔma"),
        ("ng'ombe", "ˈŋɔmbɛ", "ŋˈɔmbɛ"),
        ("nyumba", "ˈɲumba", "ɲˈumba"),
        ("chakula", "tʃaˈkula", "tʃakˈula"),
        ("shule", "ˈʃulɛ", "ʃˈulɛ"),
        ("dhahabu", "ðaˈhabu", "ðahˈabu"),
        ("thelathini", "θɛlaˈθini", "θɛlaθˈini"),
        ("ghali", "ˈɣali", "ɣˈali"),
        ("kheri", "ˈxɛri", "xˈɛri"),
        ("saa", "ˈsaa", "sˈaa"),
        ("kwenda", "ˈkwɛnda", "kwˈɛnda"),
        ("Afrika", "aˈfrika", "afrˈika"),
        ("daktari", "dakˈtari", "daktˈari"),
        ("jambo", "ˈdʒambɔ", "dʒˈambɔ"),
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
        (0, "sifuri"),
        (7, "saba"),
        (10, "kumi"),
        (11, "kumi na moja"),
        (20, "ishirini"),
        (45, "arobaini na tano"),
        (100, "mia moja"),
        (105, "mia moja na tano"),
        (120, "mia moja na ishirini"),
        (125, "mia moja ishirini na tano"),
        (1250, "elfu moja mia mbili na hamsini"),
        (2024, "elfu mbili ishirini na nne"),
        (300_000, "elfu mia tatu"),
        (1_000_000, "milioni moja"),
        (3_000_000_000, "bilioni tatu"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_says_signs_and_dates() {
    let locale = Swahili;
    assert_eq!(locale.ordinal(1, Gender::Alone).as_deref(), Some("kwanza"));
    assert_eq!(locale.ordinal(2, Gender::Alone).as_deref(), Some("pili"));
    assert_eq!(locale.ordinal(3, Gender::Alone).as_deref(), Some("tatu"));
    assert_eq!(locale.cardinal(4, Gender::Alone).as_deref(), Some("nne"));
    assert_eq!(
        locale.date(3, 5, 2024).as_deref(),
        Some("tatu Mei elfu mbili ishirini na nne")
    );
    assert_eq!(locale.date(3, 13, 2024), None);
    assert_eq!((locale.decimal_word(','), locale.decimal_word('.')), ("koma", "nukta"));
    assert_eq!(
        (locale.range(), locale.ratio(), locale.minus(), locale.plus()),
        ("hadi", "kwa", "hasi", "jumlisha")
    );
    assert!(locale.percent_first());
    assert_eq!(locale.percent(), "asilimia");
    assert_eq!(locale.currency("TSh"), Some(("shilingi", "shilingi")));
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
    assert_eq!(locale.abbreviation("Dkt"), Some("daktari"));
    assert!(crate::is_abbreviation("Bw"));
    assert!(!crate::is_abbreviation("kitabu"));
    assert_eq!(locale.vowels(), "aeiou");
}

#[test]
fn connectives_lose_their_stress() {
    let tokens = [
        ("Kitabu", TokenKind::Word),
        ("cha", TokenKind::Word),
        ("mtoto", TokenKind::Word),
    ];
    let read = read_tokens(&tokens);
    let said: Vec<String> = read
        .iter()
        .flatten()
        .flat_map(|reading| reading.words.iter())
        .map(|word| word.to_notation(Notation::Espeak))
        .collect();
    assert_eq!(said, ["kitˈabu", "tʃa", "mtˈɔtɔ"]);
}

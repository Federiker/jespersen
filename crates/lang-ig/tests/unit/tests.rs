use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Igbo, cardinal},
    read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("ụlọ", "ˈʊlɔ", "ˈʊlɔ"),
        // The same word with its dots as combining marks.
        ("u\u{323}lo\u{323}", "ˈʊlɔ", "ˈʊlɔ"),
        ("Igbo", "ˈiɡ\u{361}bo", "ˈiɡbo"),
        ("ndị", "ˈn\u{329}dɪ", "ˈndɪ"),
        ("mmiri", "m\u{329}ˈmiɾi", "mmˈiɾi"),
        ("nke", "ˈn\u{329}ke", "ˈnke"),
        ("n'ụlọ", "n\u{329}ˈʊlɔ", "nˈʊlɔ"),
        ("nwa", "ˈŋʷa", "ŋwˈa"),
        ("nyere", "ˈɲeɾe", "ɲˈeɾe"),
        ("akwụkwọ", "aˈkʷʊkʷɔ", "akwˈʊkwɔ"),
        ("ọkụkọ", "ɔˈkʊkɔ", "ɔkˈʊkɔ"),
        ("chi", "ˈtʃi", "tʃˈi"),
        ("ụgbọ", "ˈʊɡ\u{361}bɔ", "ˈʊɡbɔ"),
        ("mkpụrụ", "m\u{329}ˈk\u{361}pʊɾʊ", "mkpˈʊɾʊ"),
        ("ogwu", "ˈoɡʷu", "ˈoɡwu"),
        ("ihe", "ˈiɦe", "ˈihe"),
        ("ṅụọ", "ˈŋʊɔ", "ŋˈʊɔ"),
        ("gheghe", "ˈɣeɣe", "ɣˈeɣe"),
        ("ashịrịkọ", "aʃɪˈɾɪkɔ", "aʃɪɾˈɪkɔ"),
        ("jụọ", "ˈdʒʊɔ", "dʒˈʊɔ"),
        ("Ṅ", "ˈŋ\u{329}", "ˈŋ"),
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
        (0, "efu"),
        (1, "otu"),
        (9, "itoolu"),
        (10, "iri"),
        (11, "iri na otu"),
        (20, "iri abụọ"),
        (25, "iri abụọ na ise"),
        (100, "otu narị"),
        (105, "otu narị na ise"),
        (250, "narị abụọ na iri ise"),
        (1000, "otu puku"),
        (2024, "puku abụọ na iri abụọ na anọ"),
        (12_000, "puku iri na abụọ"),
        (1_000_000, "otu nde"),
        (3_000_000_000, "ijeri atọ"),
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
    let locale = Igbo;
    assert_eq!(locale.ordinal(1, Gender::Alone).as_deref(), Some("nke mbụ"));
    assert_eq!(locale.ordinal(2, Gender::Alone).as_deref(), Some("nke abụọ"));
    assert_eq!(locale.cardinal(4, Gender::Alone).as_deref(), Some("anọ"));
    assert_eq!(
        locale.date(3, 5, 2024).as_deref(),
        Some("ụbọchị atọ n'ọnwa ise afọ puku abụọ na iri abụọ na anọ")
    );
    assert_eq!(locale.date(3, 13, 2024), None);
    assert_eq!((locale.decimal_word(','), locale.decimal_word('.')), ("kọma", "ntụpọ"));
    assert_eq!(
        (locale.range(), locale.minus(), locale.plus(), locale.percent()),
        ("ruo", "wepụ", "gbakwunye", "pasentị")
    );
    assert_eq!(locale.currency("₦"), Some(("naịra", "naịra")));
    for sign in ["NGN", "€", "$", "£"] {
        assert!(locale.currency(sign).is_some());
    }
    assert_eq!(locale.currency("¥"), None);
    for symbol in ["%", "&", "@", "+", "=", "#", "₦", "$"] {
        assert!(locale.symbol(symbol).is_some(), "{symbol}");
    }
    assert_eq!(locale.symbol("~"), None);
    for letter in ('a'..='z').chain(['ị', 'ọ', 'ụ', 'ṅ']) {
        let name = locale.letter(letter).unwrap();
        assert!(pronounce(name).is_some(), "{letter}");
    }
    assert_eq!(locale.letter('ß'), None);
    for sign in ['@', '.', '/', ':', '-', '_'] {
        let words = locale.address_sign(sign).unwrap();
        assert!(words.split(' ').all(|word| pronounce(word).is_some()), "{sign}");
    }
    assert_eq!(locale.address_sign('!'), None);
    assert_eq!(locale.abbreviation("Maz"), Some("mazị"));
    assert!(crate::is_abbreviation("Dr"));
    assert!(!crate::is_abbreviation("ụlọ"));
    assert_eq!(locale.vowels(), "aeiouịọụ");
}

#[test]
fn function_words_lose_their_stress_whatever_their_marks() {
    let tokens = [
        ("Ọ", TokenKind::Word),
        ("bụ̀", TokenKind::Word),
        ("akwụkwọ", TokenKind::Word),
        ("ya", TokenKind::Word),
    ];
    let said: Vec<String> = read_tokens(&tokens)
        .iter()
        .flatten()
        .flat_map(|reading| reading.words.iter())
        .map(|word| word.to_notation(Notation::Espeak))
        .collect();
    assert_eq!(said, ["ɔ", "bʊ", "akwˈʊkwɔ", "ja"]);
}

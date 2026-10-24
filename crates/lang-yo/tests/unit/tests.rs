use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Yoruba, cardinal},
    read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("ọmọ", "ˈɔmɔ", "ˈɔmɔ"),
        // The same word with its dots as combining marks.
        ("o\u{323}mo\u{323}", "ˈɔmɔ", "ˈɔmɔ"),
        ("Yorùbá", "joˈɾuba", "joɾˈuba"),
        ("ẹ̀gbọ́n", "ˈɛɡ\u{361}bɔ\u{303}", "ˈɛɡbɔ\u{303}"),
        ("ìlú", "ˈilu", "ˈilu"),
        ("ṣé", "ˈʃe", "ʃˈe"),
        ("ńlá", "ˈn\u{329}la", "ˈnla"),
        ("m̀bá", "ˈm\u{329}ba", "ˈmba"),
        ("pupa", "ˈk\u{361}puk\u{361}pa", "kpˈukpa"),
        ("jẹun", "ˈɟɛu\u{303}", "ɟˈɛu\u{303}"),
        ("Ọlọ́run", "ɔˈlɔɾu\u{303}", "ɔlˈɔɾu\u{303}"),
        ("gbogbo", "ˈɡ\u{361}boɡ\u{361}bo", "ɡbˈoɡbo"),
        ("orúkọ", "oˈɾukɔ", "oɾˈukɔ"),
        ("ẹran", "ˈɛɾa\u{303}", "ˈɛɾa\u{303}"),
        ("ọ̀nà", "ˈɔna", "ˈɔna"),
        ("Nàìjíríà", "naiɟiˈɾia", "naiɟiɾˈia"),
        ("hẹn", "ˈhɛ\u{303}", "hˈɛ\u{303}"),
        ("wọn", "ˈwɔ\u{303}", "wˈɔ\u{303}"),
        ("ìwé", "ˈiwe", "ˈiwe"),
        ("dókítà", "doˈkita", "dokˈita"),
        ("ẹ̀ẹ́dẹ́gbẹ̀ta", "ɛɛdɛˈɡ\u{361}bɛta", "ɛɛdɛɡbˈɛta"),
        ("ojú-ọ̀nà", "oɟuˈɔna", "oɟuˈɔna"),
        ("taxi", "ˈtaksi", "tˈaksi"),
        ("zoo", "ˈzoo", "zˈoo"),
        ("vcq", "", ""),
    ];
    let wrong: Vec<String> = cases
        .iter()
        .filter(|(word, _, _)| *word != "vcq")
        .filter_map(|(word, ipa, espeak)| {
            let said = pronounce(word).unwrap();
            let (got_ipa, got_espeak) = (said.to_notation(Notation::Ipa), said.to_notation(Notation::Espeak));
            (got_ipa != *ipa || got_espeak != *espeak).then(|| format!("{word}: {got_ipa} {got_espeak}"))
        })
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
    assert!(pronounce("vcq").is_none());
    assert!(pronounce("ç").is_none());
    assert!(pronounce("'").is_none());
}

#[test]
fn numbers_in_twenties_and_hundreds() {
    for (number, words) in [
        (0, "òdo"),
        (1, "ọ̀kan"),
        (10, "ẹ̀wá"),
        (11, "ọ̀kanlá"),
        (14, "ẹ̀rìnlá"),
        (15, "àrùndínlogún"),
        (19, "ọ̀kandínlogún"),
        (20, "ogún"),
        (21, "ọ̀kanlélogún"),
        (25, "àrùndínlọgbọ̀n"),
        (30, "ọgbọ̀n"),
        (45, "àrùndínlàádọ́ta"),
        (50, "àádọ́ta"),
        (100, "ọgọ́rùn"),
        (105, "àrùndínlàádọ́fà"),
        (199, "ọ̀kandínligba"),
        (200, "igba"),
        (250, "igba ó lé àádọ́ta"),
        (300, "ọ̀ọ́dúnrún"),
        (1000, "ẹgbẹ̀rún"),
        (1999, "ẹgbẹ̀rún ó lé ẹ̀ẹ́dẹ́gbẹ̀rún ó lé ọ̀kandínlọgọ́rùn"),
        (2024, "ẹgbẹ̀rún méjì ó lé ẹ̀rìnlélogún"),
        (12_000, "ẹgbẹ̀rún èjìlá"),
        (1_000_000, "mílíọ̀nù"),
        (3_500_000, "mílíọ̀nù mẹ́ta ó lé ẹgbẹ̀rún ẹ̀ẹ́dẹ́gbẹ̀ta"),
        (2_000_000_000, "bílíọ̀nù méjì"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
    // Every number's words can be said.
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
    let locale = Yoruba;
    assert_eq!(locale.ordinal(1, Gender::Alone).as_deref(), Some("kínní"));
    assert_eq!(locale.ordinal(10, Gender::Alone).as_deref(), Some("kẹwàá"));
    assert_eq!(locale.ordinal(11, Gender::Alone), None);
    assert_eq!(locale.ordinal(0, Gender::Alone), None);
    assert_eq!(locale.cardinal(4, Gender::Alone).as_deref(), Some("ẹ̀rin"));
    assert_eq!(
        locale.date(3, 5, 2024).as_deref(),
        Some("ọjọ́ ẹ̀ta oṣù Èbìbì ọdún ẹgbẹ̀rún méjì ó lé ẹ̀rìnlélogún")
    );
    assert_eq!(locale.date(3, 13, 2024), None);
    assert_eq!((locale.decimal_word(','), locale.decimal_word('.')), ("kọ́mà", "pọ́ìntì"));
    assert_eq!(
        (locale.range(), locale.ratio(), locale.minus(), locale.plus()),
        ("sí", "sí", "àyọkúrò", "àfikún")
    );
    assert_eq!(locale.percent(), "nínú ọgọ́rùn");
    assert_eq!(locale.currency("₦"), Some(("náírà", "náírà")));
    for sign in ["NGN", "€", "$", "£"] {
        assert!(locale.currency(sign).is_some());
    }
    assert_eq!(locale.currency("¥"), None);
    for symbol in ["%", "&", "@", "+", "=", "#", "₦", "$"] {
        assert!(locale.symbol(symbol).is_some(), "{symbol}");
    }
    assert_eq!(locale.symbol("~"), None);
    for letter in ('a'..='z').chain(['ẹ', 'ọ', 'ṣ']) {
        let name = locale.letter(letter).unwrap();
        assert!(pronounce(name).is_some(), "{letter}");
    }
    assert_eq!(locale.letter('ß'), None);
    for sign in ['@', '.', '/', ':', '-', '_'] {
        assert!(locale.address_sign(sign).is_some(), "{sign}");
    }
    assert_eq!(locale.address_sign('!'), None);
    assert_eq!(locale.abbreviation("Ọ̀gb"), Some("ọ̀gbẹ́ni"));
    assert_eq!(locale.abbreviation("Dr"), Some("dókítà"));
    assert!(crate::is_abbreviation("Prof"));
    assert!(!crate::is_abbreviation("ọmọ"));
    assert_eq!(locale.vowels(), "aeiouẹọ");
}

#[test]
fn function_words_lose_their_stress_whatever_their_marks() {
    let tokens = [
        ("Wọ́n", TokenKind::Word),
        ("fẹ́", TokenKind::Word),
        ("ọmọ", TokenKind::Word),
        ("náà", TokenKind::Word),
    ];
    let said: Vec<String> = read_tokens(&tokens)
        .iter()
        .flatten()
        .flat_map(|reading| reading.words.iter())
        .map(|word| word.to_notation(Notation::Espeak))
        .collect();
    assert_eq!(said, ["wɔ\u{303}", "fˈɛ", "ˈɔmɔ", "nˈaa"]);
}

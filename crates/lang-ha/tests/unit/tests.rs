use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Hausa, cardinal},
    read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("Hausa", "ˈhausa", "hˈaʊsa"),
        ("ɓera", "ˈɓeɾa", "bˈeɾa"),
        ("ɗaki", "ˈɗaki", "dˈaki"),
        ("ƙasa", "ˈkʼasa", "kˈasa"),
        ("tsuntsu", "ˈsʼunsʼu", "sˈunsu"),
        ("\u{2BC}ya\u{2BC}ya", "ˈʔʲaʔʲa", "jˈaja"),
        ("ƴaƴa", "ˈʔʲaʔʲa", "jˈaja"),
        ("sa'a", "ˈsaʔa", "sˈaʔa"),
        ("dalla", "ˈdalːa", "dˈalːa"),
        ("shanu", "ˈʃanu", "ʃˈanu"),
        ("gwamna", "ˈɡʷamna", "ɡwˈamna"),
        ("kyau", "ˈkʲau", "kjˈaʊ"),
        ("ƙwarai", "ˈkʼʷaɾai", "kwˈaɾaɪ"),
        ("ƙyalle", "ˈkʼʲalːe", "kjˈalːe"),
        ("gyara", "ˈɡʲaɾa", "ɡjˈaɾa"),
        ("fyaɗe", "ˈɸʲaɗe", "fjˈade"),
        ("ma'aikata", "maʔaiˈkata", "maʔaɪkˈata"),
        ("cikin", "ˈtʃikin", "tʃˈikin"),
        ("kàasuwā", "kaaˈsuwa", "kaasˈuwa"),
        ("aiki", "ˈaiki", "ˈaɪki"),
        ("Najeriya", "nadʒeˈɾija", "nadʒeɾˈija"),
        ("fari", "ˈɸaɾi", "fˈaɾi"),
        ("yi-yi", "ˈjiji", "jˈiji"),
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
    assert!(pronounce("pv").is_none());
    assert!(pronounce("ç").is_none());
    assert!(pronounce("'").is_none());
}

#[test]
fn numbers_in_tens() {
    for (number, words) in [
        (0, "sifiri"),
        (1, "ɗaya"),
        (11, "goma sha ɗaya"),
        (20, "ashirin"),
        (25, "ashirin da biyar"),
        (40, "arbaʼin"),
        (100, "ɗari"),
        (125, "ɗari da ashirin da biyar"),
        (200, "ɗari biyu"),
        (1000, "dubu"),
        (2024, "dubu biyu da ashirin da huɗu"),
        (12_000, "dubu goma sha biyu"),
        (1_000_000, "miliyan"),
        (3_000_000_000, "biliyan uku"),
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
    let locale = Hausa;
    assert_eq!(locale.ordinal(1, Gender::Alone).as_deref(), Some("na farko"));
    assert_eq!(locale.ordinal(2, Gender::Alone).as_deref(), Some("na biyu"));
    assert_eq!(locale.cardinal(4, Gender::Alone).as_deref(), Some("huɗu"));
    assert_eq!(
        locale.date(3, 5, 2024).as_deref(),
        Some("uku ga watan Mayu dubu biyu da ashirin da huɗu")
    );
    assert_eq!(locale.date(3, 13, 2024), None);
    assert_eq!((locale.decimal_word(','), locale.decimal_word('.')), ("waƙafi", "digo"));
    assert_eq!(
        (locale.range(), locale.minus(), locale.plus(), locale.percent()),
        ("zuwa", "ragi", "ƙari", "cikin ɗari")
    );
    assert_eq!(locale.currency("₦"), Some(("naira", "naira")));
    for sign in ["NGN", "CFA", "FCFA", "€", "$", "£"] {
        assert!(locale.currency(sign).is_some());
    }
    assert_eq!(locale.currency("¥"), None);
    for symbol in ["%", "&", "@", "+", "=", "#", "₦", "$"] {
        assert!(locale.symbol(symbol).is_some(), "{symbol}");
    }
    assert_eq!(locale.symbol("~"), None);
    for letter in ('a'..='z').chain(['ɓ', 'ɗ', 'ƙ', 'ƴ']) {
        let name = locale.letter(letter).unwrap();
        assert!(pronounce(name).is_some(), "{letter}");
    }
    assert_eq!(locale.letter('ß'), None);
    for sign in ['@', '.', '/', ':', '-', '_'] {
        let words = locale.address_sign(sign).unwrap();
        assert!(words.split(' ').all(|word| pronounce(word).is_some()), "{sign}");
    }
    assert_eq!(locale.address_sign('!'), None);
    assert_eq!(locale.abbreviation("Alh"), Some("alhaji"));
    assert!(crate::is_abbreviation("Mal"));
    assert!(!crate::is_abbreviation("gida"));
    assert_eq!(locale.vowels(), "aeiou");
}

#[test]
fn function_words_lose_their_stress_whatever_their_marks() {
    let tokens = [
        ("Yā", TokenKind::Word),
        ("tafi", TokenKind::Word),
        ("kasuwa", TokenKind::Word),
        ("da", TokenKind::Word),
        ("safe", TokenKind::Word),
    ];
    let said: Vec<String> = read_tokens(&tokens)
        .iter()
        .flatten()
        .flat_map(|reading| reading.words.iter())
        .map(|word| word.to_notation(Notation::Espeak))
        .collect();
    assert_eq!(said, ["ja", "tˈafi", "kasˈuwa", "da", "sˈafe"]);
}

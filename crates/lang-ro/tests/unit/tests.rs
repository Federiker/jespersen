use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Romanian, is_abbreviation},
    numbers::cardinal,
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("casă", "ˈkasə", "kˈasə"),
        ("școală", "ˈʃko̯alə", "ʃkˈɔalə"),
        ("seară", "ˈse̯aɾə", "sˈeaɾə"),
        ("câine", "ˈkɨjne", "kˈyɪne"),
        ("copiii", "koˈpiji", "kopˈiji"),
        ("lupi", "ˈlupʲ", "lˈupʲʲ"),
        ("ochi", "ˈokʲ", "ˈokʲ"),
        ("dealuri", "de̯aˈlurʲ", "dealˈurʲ"),
        ("chiar", "ˈkjar", "kjˈar"),
        ("ghiveci", "ɡiˈvetʃʲ", "ɡivˈetʃʲ"),
        ("ciorbă", "ˈtʃorbə", "tʃˈorbə"),
        ("două", "ˈdowə", "dˈouə"),
        ("plouat", "ploˈwat", "ploʊˈat"),
        ("ziua", "ˈziwa", "zˈiwa"),
        ("exemplu", "eɡˈzemplu", "eɡzˈemplu"),
        ("lângă", "ˈlɨŋɡə", "lˈɨŋɡə"),
        ("încă", "ˈɨnkə", "ˈɨnkə"),
        ("totul", "ˈtotul", "tˈotul"),
        ("împăratului", "ɨmpəˈɾatuluj", "ˌɨmpəɾˈatuluɪ"),
        ("biroul", "biˈɾowl", "biɾˈoʊl"),
        ("cafea", "kaˈfe̯a", "kafˈea"),
        ("curtea", "ˈkurte̯a", "kˈurtea"),
        ("apoi", "aˈpoj", "apˈoj"),
        ("tramvai", "tramˈvaj", "tramvˈaɪ"),
        ("făcu", "fəˈku", "fəkˈu"),
        ("făcù", "fəˈku", "fəkˈu"),
        ("știu", "ˈʃtiw", "ʃtˈiʊ"),
        ("fie", "ˈfie", "fˈie"),
        ("aceea", "aˈtʃeja", "atʃˈeɪa"),
        ("râul", "ˈrɨwl", "rˈyʊl"),
        ("mari", "ˈmaɾʲ", "mˈaɾʲʲ"),
        ("românía", "romɨˈnia", "rˌomɨnˈia"),
        ("așá", "aˈʃa", "aʃˈa"),
        ("Victoriei", "viktoˈɾiej", "vˌiktoɾˈieɪ"),
        ("doisprezece", "ˈdojsprezetʃe", "dˈoɪsprezˌetʃe"),
        ("universitatea", "universiˈtate̯a", "ˌunivˌersitˈatea"),
        ("mergem", "ˈmerdʒem", "mˈerdʒem"),
        ("erou", "eˈɾow", "eɾˈoʊ"),
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
    assert!(pronounce("ß").is_none());
    assert!(pronounce("-").is_none());
    assert_eq!(pronounce("ș").unwrap().to_notation(Notation::Espeak), "ʃ");
}

#[test]
fn function_words_lose_their_stress() {
    let read = |text: &str| -> Vec<String> {
        let tokens: Vec<(&str, TokenKind)> = text.split(' ').map(|token| (token, TokenKind::Word)).collect();
        read_tokens(&tokens)
            .into_iter()
            .flatten()
            .flat_map(|reading| reading.words)
            .map(|said| said.to_notation(Notation::Espeak))
            .collect()
    };
    assert_eq!(read("el este acasă"), ["jel", "jˌeste", "akˈasə"]);
    assert_eq!(read("dar așa e"), ["dar", "aʃˈa", "je"]);
    assert_eq!(read("B"), ["bˈe"]);
}

#[test]
fn numbers_in_words() {
    for (number, feminine, words) in [
        (0, false, "zero"),
        (1, false, "unu"),
        (1, true, "una"),
        (2, true, "două"),
        (12, false, "doisprezece"),
        (12, true, "douăsprezece"),
        (21, false, "douăzeci și unu"),
        (100, false, "o sută"),
        (200, false, "două sute"),
        (1000, false, "o mie"),
        (2024, false, "două mii douăzeci și patru"),
        (20_000, false, "douăzeci de mii"),
        (1_000_000, false, "un milion"),
        (3_000_000, false, "trei milioane"),
    ] {
        assert_eq!(cardinal(number, feminine), words, "{number}");
    }
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(
        Romanian.date(3, 5, 2024).unwrap(),
        "trei mai două mii douăzeci și patru"
    );
    assert_eq!(
        Romanian.date(1, 12, 2024).unwrap(),
        "întâi decembrie două mii douăzeci și patru"
    );
    assert_eq!(Romanian.date(1, 13, 2024), None);
    assert_eq!(Romanian.time(10, 30).unwrap(), "zece și treizeci");
    assert_eq!(Romanian.time(10, 0).unwrap(), "ora zece");
    assert_eq!(Romanian.cardinal(2, Gender::Feminine).unwrap(), "două");
    assert_eq!(Romanian.currency("€"), Some(("euro", "euro")));
    assert_eq!(Romanian.currency("$"), Some(("dolar", "dolari")));
    assert_eq!(Romanian.currency("£"), Some(("liră", "lire")));
    assert_eq!(Romanian.currency("RON"), Some(("leu", "lei")));
    assert_eq!(Romanian.currency("¥"), None);
    let words = [
        Romanian.decimal_word(','),
        Romanian.decimal_word('.'),
        Romanian.range(),
        Romanian.ratio(),
        Romanian.minus(),
        Romanian.plus(),
        Romanian.percent(),
    ];
    assert_eq!(
        words,
        ["virgulă", "punct", "până la", "supra", "minus", "plus", "la sută"]
    );
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Romanian.symbol(symbol)).count(), 9);
    let letters: String = "aăâbcdefghiîjklmnopqrsștțuvwxyz"
        .chars()
        .filter_map(|letter| Romanian.letter(letter))
        .collect();
    assert!(
        letters.starts_with("aăîbecedeeefgehașiî") && letters.ends_with("icsi greczet"),
        "{letters}"
    );
    assert_eq!(Romanian.letter('1'), None);
    assert_eq!(
        "@./:-_!".chars().filter_map(|sign| Romanian.address_sign(sign)).count(),
        6
    );
    assert!(is_abbreviation("Dr") && is_abbreviation("nr") && !is_abbreviation("casa"));
    assert_eq!(Romanian.abbreviation("str"), Some("strada"));
    assert_eq!(Romanian.gender_of("liră"), Gender::Feminine);
    assert_eq!(Romanian.gender_of("dolar"), Gender::Masculine);
    assert_eq!(Romanian.vowels(), "aăâeiîou");
}

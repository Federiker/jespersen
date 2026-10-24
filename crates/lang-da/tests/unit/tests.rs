use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Danish, is_abbreviation},
    nst,
    numbers::cardinal,
    phonemes::{K, L, N, R, R_VOCALIC, S, T, V, legal_onset},
    sampa::{Symbol, symbols},
    sentence::read_tokens,
    spelling::pronounce,
    words::respelling,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("hus", "ˈhus", "hˈʔus"),
        ("skole", "ˈskoːlə", "skˈʔolə"),
        ("gade", "ˈɡaðə", "ɡˈaðə"),
        ("tak", "ˈtɑk", "tˈ?ɑk"),
        ("bog", "ˈbɒw", "bˈɒw"),
        ("far", "ˈfɑ", "fˈɑ"),
        ("begynde", "beˈɡønə", "bʔeɡˈʔœnə"),
        ("nation", "naˈɕoːn", "naʃˈʔon"),
        ("universitet", "ˌuniˌʋɛɐ̯siˈteːt", "ˌʔuniʋˌεɐ̯sitˈʔet"),
        ("studere", "stuˈðeːɐ", "stuðˈʔeʌ"),
        ("dag", "ˈdaj", "dˈaj"),
        ("tyve", "ˈtyʋə", "tˈʔyʋə"),
        ("konge", "ˈkʌŋə", "kˈʔʌŋə"),
        ("paa", "ˈpɔː", "pˈɒ"),
        ("vand", "ˈʋan", "ʋˈan"),
        ("pige", "ˈpiə", "pˈʔiə"),
        ("hjerte", "ˈjɛɐ̯tə", "jˈεɐ̯tə"),
        ("barn", "ˈbɑn", "bˈɑn"),
        ("bord", "ˈboːɐ̯", "bˈʔoɐ̯"),
        ("kort", "ˈkɒːt", "kˈɒɒt"),
        ("borg", "ˈbɒːw", "bˈɒɒw"),
        ("ikke", "ˈekə", "ˈʔekə"),
        ("der", "ˈdɛɐ̯", "dˈεɐ̯"),
        ("mere", "ˈmeːɐ", "mˈʔeʌ"),
        ("vej", "ˈʋɑj", "ʋˈ?ɑj"),
        ("øje", "ˈʌjə", "ˈʔʌjə"),
        ("over", "ˈɒwɐ", "ˈɒwʔʌ"),
        ("navn", "ˈnɑwn", "nˈ?ɑwn"),
        ("haven", "ˈhaʋən", "hˈaʋən"),
        ("kære", "ˈkɛːɐ", "kˈɛʔʌ"),
        ("mindre", "ˈmenʁɐ", "mˈʔenʁʔʌ"),
        ("bagerne", "ˈbajɐnə", "bˈajʔʌnə"),
        ("arbejder", "ˈɑbɑjˌdɐ", "ˈɑb?ɑjdˌʔʌ"),
        ("tusind", "ˈtusən", "tˈʔusən"),
        ("meget", "ˈmeːjəð", "mˈʔejəð"),
        ("dejligt", "ˈdɑjlit", "dˈ?ɑjlit"),
        ("femten", "ˈfɛmdən", "fˈεmdən"),
        ("stille", "ˈstelə", "stˈʔelə"),
        ("vilde", "ˈʋilə", "ʋˈʔilə"),
        ("smil", "ˈsmil", "smˈʔil"),
        ("min", "ˈmin", "mˈʔin"),
        ("fisk", "ˈfesk", "fˈʔesk"),
        ("luft", "ˈlɔft", "lˈɒft"),
        ("bjerget", "ˈbjɛɐ̯wəð", "bjˈεɐ̯wəð"),
        ("selv", "ˈsɛl", "sˈεl"),
        ("hvid", "ˈʋið", "ʋˈʔið"),
        ("sjæl", "ˈɕɛl", "ʃˈεl"),
        ("chef", "ˈɕɛf", "ʃˈεf"),
        ("mand", "ˈman", "mˈan"),
        ("café", "kaˈfeː", "kafˈʔe"),
        ("enormität", "ˈeːnɒːɐ̯ˌmitɛt", "ˈʔenɒɒɐ̯mˌʔitεt"),
        ("o'", "ˈʌ", "ˈʔʌ"),
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
    assert!(pronounce("ł").is_none());
    assert!(pronounce("-").is_none());
}

#[test]
fn words_from_nst() {
    let cases = [
        ("hund", "ˈhunˀ", "hˈʔun"),
        ("hus", "ˈhuːˀs", "hˈʔus"),
        ("efter", "ˈɛfdɐ", "ˈεftʔʌ"),
        ("ikke", "ˈeɡə", "ˈʔekə"),
        ("lidt", "ˈled", "lˈʔet"),
        ("præsten", "ˈpʁɛsdən", "pʁˈεsdən"),
        ("brød", "ˈbʁøðˀ", "bɐ̯ˈʔœð"),
        ("banegård", "ˈbaːnəˌɡɒːˀ", "bˈanəɡˌɒ"),
        ("universitet", "ˌuniˌʋɛɐ̯siˈteːˀd", "ˌʔuniʋˌεɐ̯sitˈʔet"),
        ("så", "ˈsɒ", "sˈɒ"),
        ("godt", "ˈɡʌd", "ɡˈʔʌt"),
        ("kunde", "ˈkɔnə", "kˈɒnə"),
        ("sig", "ˈsɑj", "sˈ?ɑj"),
        ("man", "ˈman", "mˈan"),
    ];
    let wrong: Vec<String> = cases
        .iter()
        .filter_map(|(word, ipa, espeak)| {
            let said = nst::pronounce(word).unwrap();
            let (got_ipa, got_espeak) = (said.to_notation(Notation::Ipa), said.to_notation(Notation::Espeak));
            (got_ipa != *ipa || got_espeak != *espeak).then(|| format!("{word}: {got_ipa} {got_espeak}"))
        })
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
    assert!(nst::pronounce("xyzzy").is_none());
    assert_eq!(symbols("\"a#"), None);
    assert_eq!(
        symbols("s'a:?"),
        Some(vec![
            Symbol::Sound("s'"),
            Symbol::Sound("a"),
            Symbol::Long,
            Symbol::Stod
        ])
    );
}

#[test]
fn the_word_list_respells_what_the_rules_miss() {
    assert_eq!(respelling("det"), Some("de"));
    assert_eq!(respelling("og"), Some("o'"));
    assert_eq!(respelling("hus"), None);
}

/// A sentence's words in eSpeak NG's notation, read together.
fn read(text: &str) -> Vec<String> {
    let tokens: Vec<(&str, TokenKind)> = text
        .split(' ')
        .map(|token| {
            let kind = if token.chars().all(|c| c.is_ascii_digit()) {
                TokenKind::Number
            } else {
                TokenKind::Word
            };
            (token, kind)
        })
        .collect();
    read_tokens(&tokens)
        .into_iter()
        .flatten()
        .flat_map(|reading| reading.words)
        .map(|said| said.to_notation(Notation::Espeak))
        .collect()
}

#[test]
fn function_words_lose_their_stress() {
    assert_eq!(read("hun og jeg"), ["hʔun", "ʌ", "jɑj"]);
    assert_eq!(read("det er i en sin"), ["de", "ɛɐ̯", "i", "ʔen", "sin"]);
    assert_eq!(read("han havde huset"), ["han", "hˌaðə", "hˈʔusəð"]);
    assert_eq!(read("og"), ["ˈʔʌ"]);
    assert_eq!(read("21"), ["ˈʔenʌtˈʔyʋə"]);
    assert_eq!(read("200"), ["tˈʔohˈʔunʁʔʌðə"]);
    assert_eq!(read("B"), ["bˈʔe"]);
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "nul"),
        (1, "en"),
        (13, "tretten"),
        (21, "en-og-tyve"),
        (50, "halvtreds"),
        (100, "et hundrede"),
        (101, "et hundrede og en"),
        (300, "tre-hundrede"),
        (1000, "et tusind"),
        (2024, "to tusind og fire-og-tyve"),
        (1_000_000, "en million"),
        (3_000_000, "tre millioner"),
        (1_000_000_000, "en milliard"),
        (2_000_000_000, "to milliarder"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(Danish.date(3, 5, 2024).unwrap(), "tre maj to tusind og fire-og-tyve");
    assert_eq!(Danish.date(1, 13, 2024), None);
    assert_eq!(Danish.time(10, 30).unwrap(), "ti tredive");
    assert_eq!(Danish.time(1, 0).unwrap(), "klokken en");
    assert_eq!(Danish.cardinal(1, Gender::Feminine).unwrap(), "en");
    assert_eq!(Danish.currency("€"), Some(("euro", "euro")));
    assert_eq!(Danish.currency("$"), Some(("dollar", "dollar")));
    assert_eq!(Danish.currency("£"), Some(("pund", "pund")));
    assert_eq!(Danish.currency("kr"), Some(("krone", "kroner")));
    assert_eq!(Danish.currency("¥"), None);
    let words = [
        Danish.decimal_word(','),
        Danish.decimal_word('.'),
        Danish.range(),
        Danish.ratio(),
        Danish.minus(),
        Danish.plus(),
        Danish.percent(),
    ];
    assert_eq!(
        words,
        ["komma", "punktum", "til", "delt med", "minus", "plus", "procent"]
    );
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Danish.symbol(symbol)).count(), 9);
    let letters: String = "abcdefghijklmnopqrstuvwxyzæøå"
        .chars()
        .filter_map(|letter| Danish.letter(letter))
        .collect();
    assert!(
        letters.starts_with("abesedeeefgehåijodkå") && letters.ends_with("eksysetæøå"),
        "{letters}"
    );
    assert_eq!(Danish.letter('1'), None);
    assert_eq!(
        "@./:-_!".chars().filter_map(|sign| Danish.address_sign(sign)).count(),
        6
    );
    assert!(is_abbreviation("Dr") && is_abbreviation("osv") && !is_abbreviation("hus"));
    assert_eq!(Danish.abbreviation("ca"), Some("cirka"));
    assert_eq!(Danish.vowels(), "aeiouyæøå");
}

#[test]
fn syllables_start_as_danish_allows() {
    assert!(legal_onset(&[T]) && legal_onset(&[S, T, R]) && legal_onset(&[K, V]) && legal_onset(&[K, L]));
    assert!(legal_onset(&[T, R_VOCALIC]));
    assert!(!legal_onset(&[N, T]) && !legal_onset(&[L, N, T, R]));
}

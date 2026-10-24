use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Norwegian, is_abbreviation},
    numbers::cardinal,
    phonemes::{K, L, N, R, S, T, V, legal_onset},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("hus", "ˈhʉːs", "hˈʉːs"),
        ("skole", "ˈskuːlə", "skˈuːla"),
        ("gate", "ˈɡɑːtə", "ɡˈata"),
        ("snakke", "ˈsnɑkːə", "snˈɑkːa"),
        ("bok", "ˈbuːk", "bˈuːk"),
        ("far", "ˈfɑːr", "fˈar"),
        ("begynne", "bəˈjʏnːə", "bəjˈynna"),
        ("nasjon", "ˈnɑʃuːn", "nˈɑʃuːn"),
        ("universitet", "ˈʉːniːˌværʃiːtət", "ˈʉːniːvˌarʃiːtət"),
        ("studere", "ˈstʉːdeːrə", "stˈʉːdeːra"),
        ("dag", "ˈdɑːɡ", "dˈaɡ"),
        ("tjue", "ˈçʉːə", "ʂˈʉːa"),
        ("hundre", "ˈhʉnːrə", "hˈʉnnra"),
        ("konge", "ˈkɔŋə", "kˈɔŋa"),
        ("år", "ˈoːr", "ˈoːr"),
        ("vann", "ˈvɑn", "vˈɑn"),
        ("gutt", "ˈɡʉtː", "ɡˈʉtː"),
        ("jente", "ˈjɛntə", "jˈɛnta"),
        ("kjøkken", "ˈçœkːən", "xˈœkːən"),
        ("kvinne", "ˈkvɪnːə", "kvˈɪnna"),
        ("sju", "ˈʃʉː", "ʃˈʉː"),
        ("sjø", "ˈʃøː", "ʃˈøː"),
        ("ski", "ˈʃiː", "ʃˈiː"),
        ("gi", "ˈjiː", "jˈiː"),
        ("hjem", "ˈjɛm", "jˈɛm"),
        ("kjære", "ˈçæːrə", "xˈaːra"),
        ("hjerte", "ˈjærtə", "jˈarta"),
        ("barn", "ˈbɑrn", "bˈɑrn"),
        ("bord", "ˈbuːr", "bˈuːr"),
        ("fors", "ˈfɔʃ", "fˈɔʃ"),
        ("norsk", "ˈnɔrʃk", "nˈɔrʃk"),
        ("land", "ˈlɑnː", "lˈɑnn"),
        ("vilde", "ˈvɪlːə", "vˈɪlla"),
        ("takk", "ˈtɑkː", "tˈɑkː"),
        ("kirke", "ˈçɪrkə", "xˈɪrka"),
        ("gjennom", "ˈjɛnːɔm", "jˈɛnnɔm"),
        ("sommer", "ˈsɔmːər", "sˈɔmmər"),
        ("her", "ˈhæːr", "hˈɛːr"),
        ("hei", "ˈhæɪ", "hˈaɪ"),
        ("øy", "ˈøʏ", "ˈøy"),
        ("sau", "ˈsæʉ", "sˈaʊ"),
        ("hva", "ˈvɑː", "vˈɑ"),
        ("jeg", "ˈjæɪ", "jˈɛːɪ"),
        ("da", "ˈdɑː", "dˈɑ"),
        ("ganske", "ˈɡɑnskə", "ɡˈɑnska"),
        ("million", "ˈmɪlːiːˌuːn", "mˈɪlliːˌuːn"),
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
    assert_eq!(read("jeg har en bil"), ["jˌɛːɪ", "har", "eːn", "bˈiːl"]);
    assert_eq!(read("de og det"), ["diː", "oː", "deː"]);
    assert_eq!(read("21"), ["ʂˈʉːaˈeːn"]);
    assert_eq!(read("200"), ["tˈuːhˈʉnnra"]);
    assert_eq!(read("B"), ["bˈeː"]);
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "null"),
        (1, "en"),
        (13, "tretten"),
        (21, "tjue-en"),
        (80, "åtti"),
        (100, "ett hundre"),
        (101, "ett hundre og en"),
        (300, "tre-hundre"),
        (1000, "ett tusen"),
        (2024, "to tusen og tjue-fire"),
        (1_000_000, "en million"),
        (3_000_000, "tre millioner"),
        (2_000_000_000, "to milliarder"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(Norwegian.date(3, 5, 2024).unwrap(), "tre mai to tusen og tjue-fire");
    assert_eq!(Norwegian.date(1, 13, 2024), None);
    assert_eq!(Norwegian.time(10, 30).unwrap(), "ti tretti");
    assert_eq!(Norwegian.time(1, 0).unwrap(), "klokken en");
    assert_eq!(Norwegian.cardinal(1, Gender::Feminine).unwrap(), "en");
    assert_eq!(Norwegian.currency("€"), Some(("euro", "euro")));
    assert_eq!(Norwegian.currency("kr"), Some(("krone", "kroner")));
    assert_eq!(Norwegian.currency("¥"), None);
    let words = [
        Norwegian.decimal_word(','),
        Norwegian.decimal_word('.'),
        Norwegian.range(),
        Norwegian.ratio(),
        Norwegian.minus(),
        Norwegian.plus(),
        Norwegian.percent(),
    ];
    assert_eq!(words, ["komma", "punkt", "til", "delt på", "minus", "plus", "prosent"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Norwegian.symbol(symbol)).count(), 9);
    let letters: String = "abcdefghijklmnopqrstuvwxyzæøå"
        .chars()
        .filter_map(|letter| Norwegian.letter(letter))
        .collect();
    assert!(
        letters.starts_with("abecedeeeffgehåijekå") && letters.ends_with("exysettæøå"),
        "{letters}"
    );
    assert_eq!(Norwegian.letter('1'), None);
    assert_eq!(
        "@./:-_!"
            .chars()
            .filter_map(|sign| Norwegian.address_sign(sign))
            .count(),
        6
    );
    assert!(is_abbreviation("Dr") && is_abbreviation("osv") && !is_abbreviation("hus"));
    assert_eq!(Norwegian.abbreviation("ca"), Some("cirka"));
    assert_eq!(Norwegian.vowels(), "aeiouyæøå");
}

#[test]
fn syllables_start_as_norwegian_allows() {
    assert!(legal_onset(&[T]) && legal_onset(&[S, T, R]) && legal_onset(&[K, V]) && legal_onset(&[K, L]));
    assert!(!legal_onset(&[N, T]) && !legal_onset(&[L, N, T, R]));
}

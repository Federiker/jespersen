use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Welsh, is_abbreviation},
    numbers::cardinal,
    phonemes::{D, K, L, N, R, S, T, W, legal_onset},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("tŷ", "ˈtɨː", "tˈɨː"),
        ("tai", "ˈtaɪ", "tˈaɪ"),
        ("ysgol", "ˈəsɡɔl", "ˈʌsɡɔl"),
        ("stryd", "ˈstrɨːd", "strˈɨːd"),
        ("siarad", "ˈʃarad", "ʃˈarad"),
        ("llyfr", "ˈɬɨvr", "ɬˈøvr"),
        ("tad", "ˈtɑːd", "tˈɑːd"),
        ("mam", "ˈmam", "mˈam"),
        ("dechrau", "ˈdeːχraɨ", "dˈeːçraɨ"),
        ("cenedl", "ˈkɛnɛdl", "kˈɛnɛdl"),
        ("prifysgol", "prɪˈvəsɡɔl", "prɨvˈʌsɡɔl"),
        ("astudio", "aˈstɨːdjɔ", "astˈɨːdjɔ"),
        ("dydd", "ˈdɨːð", "dˈɨːð"),
        ("dyddiau", "ˈdəðjaɨ", "dˈʌðjaɨ"),
        ("ugain", "ˈɨːɡaɪn", "ˈɨːɡaɪn"),
        ("cant", "ˈkant", "kˈant"),
        ("brenin", "ˈbrɛnɪn", "brˈɛnɨn"),
        ("blwyddyn", "ˈbluɨðɨn", "blˈuɨðøn"),
        ("mab", "ˈmɑːb", "mˈɑːb"),
        ("dŵr", "ˈduːr", "dˈuːr"),
        ("bachgen", "ˈbɑːχɡɛn", "bˈɑːxɡɛn"),
        ("merch", "ˈmɛrχ", "mˈɛrx"),
        ("da", "ˈdɑː", "dˈɑː"),
        ("dweud", "ˈdwəɨd", "dwˈəɨd"),
        ("newydd", "ˈnɛʊɨð", "nˈeʊøð"),
        ("cegin", "ˈkeːɡɪn", "kˈeːɡɨn"),
        ("bara", "ˈbara", "bˈara"),
        ("gwraig", "ˈɡwraɪɡ", "ɡwrˈaɪɡ"),
        ("mwy", "ˈmuɨ", "mˈuɨ"),
        ("llawer", "ˈɬaʊɛr", "ɬˈaʊɛr"),
        ("ddim", "ˈðɪm", "ðˈɨm"),
        ("un", "ˈɨːn", "ˈɨːn"),
        ("pump", "ˈpɨmp", "pˈømp"),
        ("chwech", "ˈχweːχ", "xwˈeːç"),
        ("saith", "ˈsaɪθ", "sˈaɪθ"),
        ("wyth", "ˈuɨθ", "ˈuɨθ"),
        ("naw", "ˈnaʊ", "nˈaʊ"),
        ("mil", "ˈmiːl", "mˈiːl"),
        ("miliwn", "ˈmiːlɪʊn", "mˈiːlɪun"),
        ("Cymru", "ˈkəmrɨ", "kˈʌmrø"),
        ("rhaid", "ˈr̥aɪd", "hrˈaɪd"),
        ("ffordd", "ˈfɔrð", "fˈɔrð"),
        ("bwrdd", "ˈbʊrð", "bˈʊrð"),
        ("eglwys", "ˈɛɡluɨs", "ˈɛɡluɨs"),
        ("coch", "ˈkoːχ", "kˈoːx"),
        ("diolch", "ˈdjɔlχ", "djˈɔlx"),
        ("heddiw", "ˈheːðɪʊ", "hˈeːðɪu"),
        ("nghymru", "ˈŋhəmrɨ", "ŋhˈʌmrø"),
        ("hwn", "ˈhʊn", "hˈʊn"),
        ("hefyd", "ˈheːvɨd", "hˈeːvød"),
        ("athro", "ˈɑːθrɔ", "ˈɑːθrɔ"),
        ("bryn", "ˈbrɨn", "brˈøn"),
        ("cyffrous", "ˈkəfrɔɨs", "kˈʌfrɔɨs"),
        ("siwgr", "ˈʃʊɡr", "ʃˈʊɡr"),
        ("Cymráeg", "kəmˈrɑɨɡ", "kʌmrˈɑɨɡ"),
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
    assert_eq!(read("mae'r gath yn y tŷ"), ["mɑɨr", "ɡˈɑːθ", "ən", "ə", "tˈɨː"]);
    assert_eq!(read("fy mam a i"), ["vø", "mˈam", "a", "ɨ"]);
    assert_eq!(read("Cymraeg"), ["kʌmrˈɑɨɡ"]);
    assert_eq!(read("21"), ["dˈaɨðˌeːɡˈɨːn"]);
    assert_eq!(read("200"), ["dˈaɨɡˌant"]);
    assert_eq!(read("B"), ["bˈiː"]);
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "dim"),
        (1, "un"),
        (10, "deg"),
        (13, "un-deg-tri"),
        (21, "dau-ddeg-un"),
        (50, "pum-deg"),
        (100, "cant"),
        (101, "cant un"),
        (300, "tri-chant"),
        (1000, "mil"),
        (2024, "dau mil dau-ddeg-pedwar"),
        (1_000_000, "un miliwn"),
        (2_000_000_000, "dau biliwn"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(Welsh.date(3, 5, 2024).unwrap(), "tri Mai dau mil dau-ddeg-pedwar");
    assert_eq!(Welsh.date(1, 13, 2024), None);
    assert_eq!(Welsh.time(10, 30).unwrap(), "deg tri-deg");
    assert_eq!(Welsh.time(1, 0).unwrap(), "un o'r gloch");
    assert_eq!(Welsh.cardinal(1, Gender::Feminine).unwrap(), "un");
    assert_eq!(Welsh.currency("€"), Some(("ewro", "ewro")));
    assert_eq!(Welsh.currency("£"), Some(("punt", "punt")));
    assert_eq!(Welsh.currency("¥"), None);
    let words = [
        Welsh.decimal_word(','),
        Welsh.decimal_word('.'),
        Welsh.range(),
        Welsh.ratio(),
        Welsh.minus(),
        Welsh.plus(),
        Welsh.percent(),
    ];
    assert_eq!(words, ["coma", "pwynt", "i", "rhannu â", "minws", "plws", "y cant"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Welsh.symbol(symbol)).count(), 9);
    let letters: String = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .filter_map(|letter| Welsh.letter(letter))
        .collect();
    assert!(
        letters.starts_with("abiecdieefegaetsijece") && letters.ends_with("wecsysed"),
        "{letters}"
    );
    assert_eq!(Welsh.letter('1'), None);
    assert_eq!("@./:-_!".chars().filter_map(|sign| Welsh.address_sign(sign)).count(), 6);
    assert!(is_abbreviation("Dr") && is_abbreviation("parch") && !is_abbreviation("tŷ"));
    assert_eq!(Welsh.abbreviation("dr"), Some("doctor"));
    assert_eq!(Welsh.vowels(), "aeiouwy");
}

#[test]
fn syllables_start_as_welsh_allows() {
    assert!(legal_onset(&[T]) && legal_onset(&[S, T, R]) && legal_onset(&[D, W]) && legal_onset(&[K, L]));
    assert!(!legal_onset(&[N, T]) && !legal_onset(&[L, N, T, R]));
}

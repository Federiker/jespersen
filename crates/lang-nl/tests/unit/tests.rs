use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Dutch, is_abbreviation},
    numbers::cardinal,
    phonemes::{J, K, L, N, R, S, T, W, X, legal_onset},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("huis", "ˈhœys", "hˈœys"),
        ("school", "ˈsxoːl", "sxˈoːl"),
        ("straat", "ˈstraːt", "strˈaːt"),
        ("spreken", "ˈspreːkən", "sprˈeːkən"),
        ("boek", "ˈbuk", "bˈuk"),
        ("vader", "ˈvaːdər", "vˈaːdər"),
        ("beginnen", "bəˈɣɪnən", "bəɣˈɪnən"),
        ("verstaan", "vərˈstaːn", "vərstˈaːn"),
        ("natie", "ˈnaːtsi", "nˈaːtsi"),
        ("universiteit", "ynivɛrsiˈtɛit", "ˌynivˌɛrsitˈɛɪt"),
        ("studeren", "styˈdeːrən", "stydˈɪːrən"),
        ("bakkerij", "bɑkəˈrɛi", "bˌɑkərˈɛɪ"),
        ("filosofie", "filoːzoːˈfi", "fˌiloːzoːfˈi"),
        ("dag", "ˈdɑx", "dˈɑx"),
        ("dagen", "ˈdaːɣən", "dˈaːɣən"),
        ("twintig", "ˈtʋɪntəx", "tʋˈɪntəx"),
        ("honderd", "ˈhɔndərt", "hˈɔndərt"),
        ("koning", "ˈkoːnɪŋ", "kˈoːnɪŋ"),
        ("weinig", "ˈʋɛinəx", "ʋˈɛɪnəx"),
        ("jaar", "ˈjaːr", "jˈaːr"),
        ("zoon", "ˈzoːn", "zˈoːn"),
        ("water", "ˈʋaːtər", "ʋˈaːtər"),
        ("tijd", "ˈtɛit", "tˈɛɪt"),
        ("jongen", "ˈjɔŋən", "jˈɔŋən"),
        ("meisje", "ˈmɛisjə", "mˈɛɪsjə"),
        ("goed", "ˈɣut", "ɣˈut"),
        ("zeggen", "ˈzɛɣən", "zˈɛɣən"),
        ("nieuw", "ˈniu̯", "nˈiw"),
        ("keuken", "ˈkøːkən", "kˈøːkən"),
        ("auto", "ˈʌutoː", "ˈʌʊtoː"),
        ("vrouw", "ˈvrʌu", "vrˈʌʊ"),
        ("meer", "ˈmeːr", "mˈɪːr"),
        ("begeerte", "bəˈɣeːrtə", "bəɣˈeːrtə"),
        ("mensch", "ˈmɛns", "mˈɛns"),
        ("tusschen", "ˈtʏsən", "tˈɵsən"),
        ("verschijnen", "vərˈsxɛinən", "vərsxˈɛɪnən"),
        ("vriendelijk", "ˈvrindələk", "vrˈindələk"),
        ("procent", "proːˈsɛnt", "proːsˈɛnt"),
        ("station", "staːˈʃɔn", "staːʃˈɔn"),
        ("kantoor", "kɑnˈtoːr", "kɑntˈoːr"),
        ("functie", "ˈfʏnksi", "fˈɵnksi"),
        ("jeugd", "ˈjøːxt", "jˈøːxt"),
        ("Amsterdam", "ˈɑmstərˌdɑm", "ˈɑmstərdˌɑm"),
        ("vergadering", "vərˈɣaːdəˌrɪŋ", "vərɣˈaːdərˌɪŋ"),
        ("mooi", "ˈmoːj", "mˈoːj"),
        ("fraai", "ˈfraːj", "frˈaːj"),
        ("moeite", "ˈmujtə", "mˈujtə"),
        ("leeuw", "ˈleːu̯", "lˈeːw"),
        ("ruw", "ˈryu̯", "rˈyw"),
        ("lachen", "ˈlɑxən", "lˈɑxən"),
        ("idee", "iˈdeː", "idˈeː"),
        ("duizènd", "ˈdœyzɛnt", "dˈœyzɛnt"),
        ("café", "kaːˈfeː", "kaːfˈeː"),
        ("waaróm", "ʋaːˈrɔm", "ʋaːrˈɔm"),
        ("moê", "ˈmu", "mˈu"),
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
fn function_words_and_sounds_across_them() {
    assert_eq!(read("de kat en het dak"), ["də", "kˈɑt", "ɛn", "hə", "tˈɑk"]);
    assert_eq!(read("niet de deur"), ["nˌi", "tə", "dˈøːr"]);
    assert_eq!(read("er is een huis"), ["ər", "ɪs", "ən", "hˈœys"]);
    assert_eq!(read("waarom"), ["ʋaːrˈɔm"]);
    assert_eq!(read("21"), ["ˈeːnɛntʋˌɪntəx"]);
    assert_eq!(read("200"), ["tʋˈeːhˌɔndərt"]);
    assert_eq!(read("B"), ["bˈeː"]);
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "nul"),
        (1, "een"),
        (13, "dertien"),
        (21, "een-en-twintig"),
        (22, "twee-ën-twintig"),
        (80, "tachtig"),
        (100, "honderd"),
        (101, "honderd een"),
        (300, "drie-honderd"),
        (1000, "duizend"),
        (2024, "twee duizend vier-en-twintig"),
        (1_000_000, "een miljoen"),
        (3_000_000, "drie miljoen"),
        (2_000_000_000, "twee miljard"),
        (1_000_000_000_000, "een biljoen"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(Dutch.date(3, 5, 2024).unwrap(), "drie mei twee duizend vier-en-twintig");
    assert_eq!(Dutch.date(1, 13, 2024), None);
    assert_eq!(Dutch.time(10, 30).unwrap(), "tien uur dertig");
    assert_eq!(Dutch.time(1, 0).unwrap(), "een uur");
    assert_eq!(Dutch.cardinal(1, Gender::Feminine).unwrap(), "een");
    assert_eq!(Dutch.currency("€"), Some(("euro", "euro")));
    assert_eq!(Dutch.currency("£"), Some(("pond", "pond")));
    assert_eq!(Dutch.currency("¥"), None);
    let words = [
        Dutch.decimal_word(','),
        Dutch.decimal_word('.'),
        Dutch.range(),
        Dutch.ratio(),
        Dutch.minus(),
        Dutch.plus(),
        Dutch.percent(),
    ];
    assert_eq!(words, ["komma", "punt", "tot", "op", "min", "plus", "procent"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Dutch.symbol(symbol)).count(), 9);
    let letters: String = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .filter_map(|letter| Dutch.letter(letter))
        .collect();
    assert!(
        letters.starts_with("aabeeceedeeeeefgeehaiejeeka") && letters.ends_with("iksijzet"),
        "{letters}"
    );
    assert_eq!(Dutch.letter('1'), None);
    assert_eq!("@./:-_!".chars().filter_map(|sign| Dutch.address_sign(sign)).count(), 6);
    assert!(is_abbreviation("Dr") && is_abbreviation("bijv") && !is_abbreviation("huis"));
    assert_eq!(Dutch.abbreviation("enz"), Some("enzovoort"));
    assert_eq!(Dutch.vowels(), "aeiouyéëïö");
}

#[test]
fn syllables_start_as_dutch_allows() {
    assert!(legal_onset(&[T]) && legal_onset(&[S, T, R]) && legal_onset(&[S, X]) && legal_onset(&[T, W]));
    assert!(legal_onset(&[K, L]) && legal_onset(&[S, J]) && !legal_onset(&[N, T]) && !legal_onset(&[L, N, T, R]));
}

use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Albanian, is_abbreviation},
    numbers::cardinal,
    phonemes::{B, K, N, R, SH, T, legal_onset},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("shtëpi", "ˈʃtəpi", "ʃtˈʌpi"),
        ("shtëpia", "ʃtəˈpia", "ʃtəpˈiɑ"),
        ("mirë", "ˈmiɾə", "mˈirə"),
        ("faleminderit", "falɛminˈdɛɾit", "fˌaɫemindˈɛrɪt"),
        ("shqip", "ˈʃcip", "ʃtɕˈip"),
        ("qytet", "ˈcytɛt", "tɕˈytɛt"),
        ("qyteti", "cyˈtɛti", "tɕytˈɛti"),
        ("gjuha", "ˈɟuha", "dʑˈuhɑ"),
        ("dhe", "ˈðɛ", "ðˈɛ"),
        ("një", "ˈɲə", "ɲˈʌ"),
        ("dhjetë", "ˈðjɛtə", "ðjˈetə"),
        ("njëzet", "ˈɲəzɛt", "ɲˈʌzɛt"),
        ("është", "ˈəʃtə", "ˈʌʃtə"),
        ("çfarë", "ˈtʃfaɾə", "tʃfˈarə"),
        ("xhami", "ˈdʒami", "dʒˈami"),
        ("zhurmë", "ˈʒuɾmə", "ʒˈurmə"),
        ("rrugë", "əˈruɡə", "ərˈuɡə"),
        ("rri", "ˈəri", "ˈəri"),
        ("lule", "ˈlulɛ", "ɫˈuɫe"),
        ("thikë", "ˈθikə", "θˈikə"),
        ("dhoma", "ˈðɔma", "ðˈɔmɑ"),
        ("nata", "ˈnata", "nˈatɑ"),
        ("mëngjes", "mənˈɟɛs", "məndʑˈɛs"),
        ("fëmijë", "fəˈmijə", "fəmˈijə"),
        ("fëmijët", "fəˈmijət", "fəmˈijət"),
        ("shkollë", "ˈʃkɔɫə", "ʃkˈɔllə"),
        ("babai", "baˈbai", "babˈai"),
        ("vajza", "ˈvajza", "vˈaɪzɑ"),
        ("qen", "ˈcɛn", "tɕˈen"),
        ("mal", "ˈmal", "mˈal"),
        ("mali", "ˈmali", "mˈaɫi"),
        ("malet", "ˈmalɛt", "mˈaɫɛt"),
        ("libër", "ˈlibəɾ", "ɫˈibər"),
        ("telefon", "tɛlɛˈfɔn", "tˌɛɫefˈɔn"),
        ("kompjuter", "kɔmpjuˈtɛɾ", "kˌɔmpjutˈɛr"),
        ("universitet", "univɛɾˈsitɛt", "ˌunivɛrsˈitɛt"),
        ("bibliotekë", "bibliɔˈtɛkə", "bˌibɫiɔtˈɛkə"),
        ("fshat", "ˈfʃat", "fʃˈat"),
        ("muaj", "ˈmwaj", "mwˈɑɪ"),
        ("grua", "ˈɡɾwa", "ɡrwˈɑ"),
        ("shkruaj", "ˈʃkɾwaj", "ʃkrwˈɑɪ"),
        ("fle", "ˈflɛ", "fɫˈe"),
        ("lehtë", "ˈlɛhtə", "ɫˈehtə"),
        ("dëgjoj", "ˈdəɟɔj", "dˈʌdʑɔɪ"),
        ("lexoj", "ˈlɛdzɔj", "ɫˈedzɔɪ"),
        ("xixë", "ˈdzidzə", "dzˈidzə"),
        ("futboll", "futˈbɔl", "futbˈɔl"),
        ("problem", "pɾɔˈblɛm", "prɔbɫˈem"),
        ("fshatar", "fʃaˈtaɾ", "fʃatˈɑr"),
        ("harro", "ˈharɔ", "hˈɑrɔ"),
        ("zjarrit", "ˈzjarit", "zjˈɑrɪt"),
        ("majë", "ˈmajə", "mˈaɪə"),
        ("dielli", "ˈdjɛɫi", "djˈelli"),
        ("qielli", "ˈcjɛɫi", "tɕjˈelli"),
        ("njerëz", "ˈɲɛɾəz", "ɲˈerəz"),
        ("shpesh", "ˈʃpɛʃ", "ʃpˈeʃ"),
        ("freskëta", "ˈfɾɛskəta", "frˈɛskətɑ"),
        ("kënaqësi", "kəˈnacəsi", "kənˈatɕəsi"),
        ("kodrave", "ˈkɔdɾavɛ", "kˈɔdravɛ"),
        ("rritur", "əˈrituɾ", "ərˈitur"),
        ("Durrësit", "duˈrəsit", "durˈʌsɪt"),
        ("kënga", "ˈkənɡa", "kˈʌnɡɑ"),
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
    assert_eq!(read("në shtëpi të madhe"), ["nə", "ʃtˈʌpi", "tə", "mˈaðɛ"]);
    assert_eq!(read("ne jemi mirë"), ["nˈɛ", "jˈemi", "mˈirə"]);
    assert_eq!(read("janë mirë"), ["jˌanə", "mˈirə"]);
    assert_eq!(read("21"), ["ɲˈʌzɛtɛɲˈʌ"]);
    assert_eq!(read("15"), ["pˈɛsəmbˌəðjetə"]);
    assert_eq!(read("B"), ["bˈʌ"]);
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "zero"),
        (1, "një"),
        (12, "dy-mbëdhjetë"),
        (21, "njëzet-e-një"),
        (40, "dyzet"),
        (100, "njëqind"),
        (101, "njëqind e një"),
        (300, "treqind"),
        (1000, "një mijë"),
        (2024, "dy mijë e njëzet-e-katër"),
        (1999, "një mijë e nëntëqind e nëntëdhjetë-e-nëntë"),
        (1_000_000, "një milion"),
        (3_000_000, "tre milionë"),
        (2_000_000_000, "dy miliardë"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(Albanian.date(3, 5, 2024).unwrap(), "tre maj dy mijë e njëzet-e-katër");
    assert_eq!(Albanian.date(1, 13, 2024), None);
    assert_eq!(Albanian.time(10, 30).unwrap(), "dhjetë e tridhjetë");
    assert_eq!(Albanian.time(10, 0).unwrap(), "ora dhjetë");
    assert_eq!(Albanian.cardinal(2, Gender::Alone).unwrap(), "dy");
    assert_eq!(Albanian.currency("€"), Some(("euro", "euro")));
    assert_eq!(Albanian.currency("ALL"), Some(("lek", "lekë")));
    assert_eq!(Albanian.currency("¥"), None);
    let words = [
        Albanian.decimal_word(','),
        Albanian.decimal_word('.'),
        Albanian.range(),
        Albanian.ratio(),
        Albanian.minus(),
        Albanian.plus(),
        Albanian.percent(),
    ];
    assert_eq!(words, ["presje", "pikë", "deri", "me", "minus", "plus", "përqind"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Albanian.symbol(symbol)).count(), 9);
    let letters: String = "abcçdeëfghijklmnopqrstuvwxyz"
        .chars()
        .filter_map(|letter| Albanian.letter(letter))
        .collect();
    assert!(
        letters.starts_with("abëcëçëdëeëfëgëhëijë") && letters.ends_with("xëyzë"),
        "{letters}"
    );
    assert_eq!(Albanian.letter('1'), None);
    assert_eq!(
        "@./:-_!".chars().filter_map(|sign| Albanian.address_sign(sign)).count(),
        6
    );
    assert!(is_abbreviation("Dr") && is_abbreviation("nr") && !is_abbreviation("shtëpi"));
    assert_eq!(Albanian.abbreviation("etj"), Some("etjerë"));
    assert_eq!(Albanian.vowels(), "aeëiouy");
}

#[test]
fn syllables_start_as_albanian_allows() {
    assert!(legal_onset(&[T]) && legal_onset(&[B, R]) && legal_onset(&[SH, K]) && legal_onset(&[SH, T, R]));
    assert!(!legal_onset(&[N, T]) && !legal_onset(&[K, T]) && !legal_onset(&[N, T, R, R]));
}

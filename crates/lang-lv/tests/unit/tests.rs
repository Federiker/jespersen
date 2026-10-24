use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Latvian, is_abbreviation},
    numbers::cardinal,
    phonemes::{K, L, N, R, S, T, V, legal_onset},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("māja", "ˈmaːja", "mˈaːja"),
        ("latvietis", "ˈlatviɛtis", "lˈatʋietis"),
        ("skaists", "ˈskai̯sts", "skˈaists"),
        ("ābols", "ˈaːbuɔls", "ˈaːbuols"),
        ("meitene", "ˈmei̯tene", "mˈeitene"),
        ("zēns", "ˈzæːns", "zˈæːns"),
        ("bērni", "ˈbæːrni", "bˈæːrni"),
        ("ķēķis", "ˈceːcis", "cˈeːcis"),
        ("ģimene", "ˈɟimene", "ɟˈimene"),
        ("ļoti", "ˈʎuɔti", "ʎˈuoti"),
        ("ņemt", "ˈɲemt", "ɲˈemt"),
        ("četri", "ˈtʃetri", "t͡ʃˈetri"),
        ("žurnāls", "ˈʒurnaːls", "ʒˈurnaːls"),
        ("dzīvot", "ˈdziːvuɔt", "dzˈiːʋuot"),
        ("džungļi", "ˈdʒuŋɡʎi", "dʐˈuŋɡʎi"),
        ("viens", "ˈviɛns", "ʋˈiens"),
        ("tūkstotis", "ˈtuːkstuɔtis", "tˈuːkstuotis"),
        ("pilsēta", "ˈpilsæːta", "pˈilsæːta"),
        ("vēstule", "ˈvæːstule", "ʋˈæːstule"),
        ("zelts", "ˈzælts", "zˈælts"),
        ("cena", "ˈtsæna", "t͡sˈæna"),
        ("meža", "ˈmeʒa", "mˈeʒa"),
        ("skola", "ˈskuɔla", "skˈuola"),
        ("telefons", "ˈtelefɔːns", "tˈelefoːns"),
        ("hokejs", "ˈxɔkejs", "hˈokejs"),
        ("bibliotēka", "ˈbibliɔteːka", "bˈiblioteːka"),
        ("birojs", "ˈbirɔjs", "bˈirojs"),
        ("teātris", "ˈteaːtris", "tˈeaːtris"),
        ("olimpiāde", "ˈɔlimpiaːde", "ˈolimpiaːde"),
        ("kalna", "ˈkalna", "kˈallna"),
        ("mēnesis", "ˈmeːnesis", "mˈeːnesis"),
        ("xilofons", "ˈksilɔːfɔːns", "ksˈiloːfoːns"),
        ("òpera", "ˈɔpera", "ˈopera"),
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
    assert_eq!(read("Rīga ir pilsēta"), ["rˈiːɡa", "ir", "pˈilsæːta"]);
    assert_eq!(read("viņa bija mājās"), ["ʋˌiɲa", "bˌija", "mˈaːjaːs"]);
    assert_eq!(read("opera"), ["ˈopera"]);
    assert_eq!(read("21"), ["dˈiʋdesmit", "ʋˈiens"]);
    assert_eq!(read("B"), ["bˈeː"]);
}

#[test]
fn numbers_in_words() {
    for (number, feminine, words) in [
        (0, false, "nulle"),
        (1, false, "viens"),
        (1, true, "viena"),
        (2, true, "divas"),
        (10, false, "desmit"),
        (12, false, "divpadsmit"),
        (21, false, "divdesmit viens"),
        (24, true, "divdesmit četras"),
        (40, false, "četrdesmit"),
        (100, false, "simts"),
        (101, false, "simts viens"),
        (200, false, "divi simti"),
        (1000, false, "tūkstotis"),
        (2024, false, "divi tūkstoši divdesmit četri"),
        (1_000_000, false, "miljons"),
        (3_000_000, false, "trīs miljoni"),
        (2_000_000_000, false, "divi miljardi"),
    ] {
        assert_eq!(cardinal(number, feminine), words, "{number}");
    }
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(
        Latvian.date(3, 5, 2024).unwrap(),
        "trīs maijs divi tūkstoši divdesmit četri"
    );
    assert_eq!(Latvian.date(1, 13, 2024), None);
    assert_eq!(Latvian.time(10, 30).unwrap(), "desmit trīsdesmit");
    assert_eq!(Latvian.time(10, 0).unwrap(), "pulksten desmit");
    assert_eq!(Latvian.cardinal(2, Gender::Feminine).unwrap(), "divas");
    assert_eq!(Latvian.cardinal(2, Gender::Alone).unwrap(), "divi");
    assert_eq!(Latvian.currency("€"), Some(("eiro", "eiro")));
    assert_eq!(Latvian.currency("$"), Some(("dolārs", "dolāri")));
    assert_eq!(Latvian.currency("¥"), None);
    let words = [
        Latvian.decimal_word(','),
        Latvian.decimal_word('.'),
        Latvian.range(),
        Latvian.ratio(),
        Latvian.minus(),
        Latvian.plus(),
        Latvian.percent(),
    ];
    assert_eq!(words, ["komats", "punkts", "līdz", "pret", "mīnus", "plus", "procenti"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Latvian.symbol(symbol)).count(), 9);
    let letters: String = "aābcčdeēfgģhiījkķlļmnņopqrsštuūvwxyzž"
        .chars()
        .filter_map(|letter| Latvian.letter(letter))
        .collect();
    assert!(
        letters.starts_with("agarais ābēcēčēdē") && letters.ends_with("iksigrekzēžē"),
        "{letters}"
    );
    assert_eq!(Latvian.letter('1'), None);
    assert_eq!(
        "@./:-_!".chars().filter_map(|sign| Latvian.address_sign(sign)).count(),
        6
    );
    assert!(is_abbreviation("Dr") && is_abbreviation("utt") && !is_abbreviation("māja"));
    assert_eq!(Latvian.abbreviation("piem"), Some("piemēram"));
    assert_eq!(Latvian.gender_of("grāmatas"), Gender::Feminine);
    assert_eq!(Latvian.gender_of("dolāri"), Gender::Masculine);
    assert_eq!(Latvian.vowels(), "aāeēiīouū");
}

#[test]
fn syllables_start_as_latvian_allows() {
    assert!(legal_onset(&[T]) && legal_onset(&[S, T, R]) && legal_onset(&[S, V]) && legal_onset(&[K, L]));
    assert!(!legal_onset(&[N, T]) && !legal_onset(&[L, N, T, R]));
}

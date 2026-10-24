use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Lithuanian, is_abbreviation},
    numbers::{cardinal, form},
    phonemes::{K, K_SOFT, L, N, R, S, SH, T, T_SOFT, V, hard, is_soft, legal_onset, soft},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("namas", "ˈnɐmɐs", "nˈamas"),
        ("tėvas", "ˈtʲeːʋɐs", "tʲˈeevas"),
        ("motina", "moːˈtʲɪnɐ", "moːtʲˈɪna"),
        ("mokykla", "moːˈkʲiːkɫɐ", "moːkʲˈiːkl̩a"),
        ("knyga", "ˈknʲiːɡɐ", "knʲˈiːɡa"),
        ("žmogus", "ˈʒmoːɡʊs", "ʒmˈoːɡus"),
        ("vanduo", "ˈʋɐnduɔ", "vˈanduo"),
        ("šuo", "ˈʃuɔ", "ʃˈuo"),
        ("duona", "ˈduɔnɐ", "dˈuona"),
        ("miestas", "ˈmʲiɛstɐs", "mʲˈiestas"),
        ("gatvė", "ˈɡɐtʲʋʲeː", "ɡˈatʲvʲee"),
        ("kelias", "ˈkʲɛlʲɛs", "kʲˈeɭes"),
        ("lietuva", "lʲiɛˈtʊʋɐ", "ɭietˈuva"),
        ("lietuvių", "lʲiɛˈtʊʋʲuː", "ɭietˈuvʲuː"),
        ("kalba", "ˈkɐɫbɐ", "kˈal̩ba"),
        ("diena", "ˈdʲiɛnɐ", "dʲˈiena"),
        ("naktis", "ˈnɐktʲɪs", "nˈaktʲis"),
        ("rytas", "ˈrʲiːtɐs", "rʲˈiːtas"),
        ("vakaras", "ʋɐˈkɐrɐs", "vakˈaras"),
        ("didelis", "dʲɪˈdʲɛlʲɪs", "dʲidʲˈeɭis"),
        ("dabar", "ˈdɐbɐr", "dˈabar"),
        ("eiti", "ɛˈɪtʲɪ", "eˈɪtʲi"),
        ("eina", "ɛˈɪnɐ", "eˈɪna"),
        ("kalbėti", "kɐlʲˈbʲeːtʲɪ", "kaɭbʲˈeetʲi"),
        ("myliu", "ˈmʲiːlʲʊ", "mʲˈiːɭʲu"),
        ("čia", "ˈtʃʲɛ", "tɕˈe"),
        ("šiandien", "ˈɕɛnʲdʲiɛn", "ɕˈenʲdʲien"),
        ("penkì", "pʲɛnʲˈkʲɪ", "pʲenʲkʲˈɪ"),
        ("tū́kstantis", "ˈtuːkstɐnʲtʲɪs", "tˈuːkstanʲtʲis"),
        ("universitetas", "ʊnʲɪʋʲɛrsʲɪˈtʲɛtɐs", "unʲivʲerʂitʲˈetas"),
        ("česnakas", "tʃʲɛˈsnɐkɐs", "tɕesnˈakas"),
        ("džiaugsmas", "dʒʲɛˈʊksmɐs", "dʑeˈuksmas"),
        ("dzūkas", "ˈdzuːkɐs", "dzˈuːkas"),
        ("choras", "ˈxoːrɐs", "xˈoːras"),
        ("ąžuolas", "aːˈʒuɔɫɐs", "aːʒˈuol̩as"),
        ("žąsis", "ˈʒaːsʲɪs", "ʒˈaːʂis"),
        ("kęsti", "ˈkʲæːsʲtʲɪ", "kʲˈeɑʂtʲi"),
        ("šiltas", "ˈɕɪɫtɐs", "ɕˈɪl̩tas"),
        ("labas", "ˈɫɐbɐs", "l̩ˈabas"),
        ("ačiū", "ˈɐtʃʲuː", "ˈatɕʲuː"),
        ("kiaulė", "kʲɛˈʊlʲeː", "kʲeˈuɭee"),
        ("šiukšlės", "ˈɕʊkɕlʲeːs", "ɕʲˈukɕɭees"),
        ("kelionė", "kʲɛˈlʲoːnʲeː", "kʲeɭʲˈoːnʲee"),
        ("biblioteka", "bʲɪblʲoːˈtʲɛkɐ", "bʲibɭʲoːtʲˈeka"),
        ("sniegas", "ˈsʲnʲiɛɡɐs", "ʂnʲˈieɡas"),
        ("ankstyvas", "ɐnkˈsʲtʲiːʋɐs", "ankʂtʲˈiːvas"),
        ("svečias", "ˈsʲʋʲɛtʃʲɛs", "ʂvʲˈetɕes"),
        ("daug", "ˈdɐuk", "dˈauk"),
        ("kad", "ˈkɐt", "kˈat"),
        ("mėgsta", "ˈmʲeːkstɐ", "mʲˈeeksta"),
        ("futbolas", "fʊdˈboːɫɐs", "fudbˈoːl̩as"),
        ("užsienis", "ʊˈsʲiɛnʲɪs", "uʂˈienʲis"),
        ("važiuosime", "ʋɐʑuɔˈsʲɪmʲɛ", "vaʒʲuoʂˈɪmʲe"),
        ("žmonių", "ˈʒmoːnʲuː", "ʒmˈoːnʲʲuː"),
        ("vaisius", "ˈʋɐisʲʊs", "vˈaiʂʲus"),
        ("euras", "ˈɛurɐs", "ˈɛuras"),
        ("konferencija", "koːnʲfʲɛrʲɛnʲˈtsʲɪjɐ", "koːnʲfʲerʲenʲtsʲˈɪja"),
        ("tūkstančiai", "tuːkˈstɐntʃʲɛi", "tuːkstˈantɕei"),
        ("lėktuvas", "lʲeːkˈtʊʋɐs", "ɭeektˈuvas"),
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
fn function_words_and_numbers_in_a_sentence() {
    assert_eq!(read("ir jis eina"), ["ir", "jˈɪs", "eˈɪna"]);
    assert_eq!(read("21"), ["dvʲˈɪdʲeɕimtvʲˈienas"]);
    assert_eq!(read("2024"), ["dˈu", "tˈuːkstantɕei", "dvʲˈɪdʲeɕimtkʲeturʲˈɪ"]);
    assert_eq!(read("penki"), ["pʲenʲkʲˈɪ"]);
    assert_eq!(read("B"), ["bʲˈee"]);
}

#[test]
fn numbers_in_words() {
    for (number, feminine, words) in [
        (0, false, "nulis"),
        (1, false, "vienas"),
        (1, true, "viena"),
        (2, true, "dvi"),
        (4, true, "keturios"),
        (12, false, "dvylika"),
        (21, false, "dvidešimt-vienas"),
        (30, false, "trisdešimt"),
        (100, false, "šimtas"),
        (101, false, "šimtas vienas"),
        (300, false, "trys-šimtai"),
        (1000, false, "tūkstantis"),
        (2024, false, "du tūkstančiai dvidešimt-keturi"),
        (10_000, false, "dešimt tūkstančių"),
        (21_000, false, "dvidešimt-vienas tūkstantis"),
        (1_000_000, false, "milijonas"),
        (3_000_000, false, "trys milijonai"),
        (2_000_000_000, false, "du milijardai"),
    ] {
        assert_eq!(cardinal(number, feminine), words, "{number}");
    }
    assert_eq!(
        [form(1), form(2), form(10), form(11), form(21), form(25)],
        [0, 1, 2, 2, 0, 1]
    );
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(
        Lithuanian.date(3, 5, 2024).unwrap(),
        "du tūkstančiai dvidešimt-keturi gegužės trys"
    );
    assert_eq!(Lithuanian.date(1, 13, 2024), None);
    assert_eq!(Lithuanian.time(10, 30).unwrap(), "dešimt trisdešimt");
    assert_eq!(Lithuanian.time(10, 0).unwrap(), "dešimt");
    assert_eq!(Lithuanian.cardinal(2, Gender::Feminine).unwrap(), "dvi");
    assert_eq!(Lithuanian.currency("€"), Some(("euras", "eurai")));
    assert_eq!(Lithuanian.currency_name("€", Some(1)), Some("euras"));
    assert_eq!(Lithuanian.currency_name("€", Some(3)), Some("eurai"));
    assert_eq!(Lithuanian.currency_name("$", Some(15)), Some("dolerių"));
    assert_eq!(Lithuanian.currency_name("£", None), Some("svarų"));
    assert_eq!(Lithuanian.currency("¥"), None);
    let words = [
        Lithuanian.decimal_word(','),
        Lithuanian.decimal_word('.'),
        Lithuanian.range(),
        Lithuanian.ratio(),
        Lithuanian.minus(),
        Lithuanian.plus(),
        Lithuanian.percent(),
    ];
    assert_eq!(words, ["kablelis", "taškas", "iki", "iš", "minus", "plius", "procentų"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Lithuanian.symbol(symbol)).count(), 9);
    let letters: String = "aąbcčdeęėfghiįyjklmnopqrsštuųūvwxzž"
        .chars()
        .filter_map(|letter| Lithuanian.letter(letter))
        .collect();
    assert!(
        letters.starts_with("aa nosinėbėcėčėdė") && letters.ends_with("ikszėžė"),
        "{letters}"
    );
    assert_eq!(Lithuanian.letter('1'), None);
    assert_eq!(
        "@./:-_!"
            .chars()
            .filter_map(|sign| Lithuanian.address_sign(sign))
            .count(),
        6
    );
    assert!(is_abbreviation("Dr") && is_abbreviation("pvz") && !is_abbreviation("namas"));
    assert_eq!(Lithuanian.abbreviation("nr"), Some("numeris"));
    assert_eq!(Lithuanian.gender_of("knyga"), Gender::Feminine);
    assert_eq!(Lithuanian.gender_of("euras"), Gender::Masculine);
    assert_eq!(Lithuanian.vowels(), "aąeęėiįyouųū");
}

#[test]
fn soft_consonants_and_syllables() {
    assert_eq!(soft(K), K_SOFT);
    assert_eq!(hard(T_SOFT), T);
    assert!(is_soft(T_SOFT) && !is_soft(T));
    assert!(legal_onset(&[T]) && legal_onset(&[S, T, R]) && legal_onset(&[SH, V]) && legal_onset(&[K, L]));
    assert!(!legal_onset(&[N, T]) && !legal_onset(&[L, N, T, R]));
}

use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};

use crate::{
    Variety,
    locale::{Portuguese, is_abbreviation},
    numbers::{cardinal, ordinal},
    sentence::say,
};

#[test]
fn words_in_both_varieties() {
    // The word, then European plain IPA and eSpeak NG notation, then Brazilian eSpeak NG notation.
    let cases = [
        ("casa", "ˈkazɐ", "kˈazɐ", "kˈazæ"),
        ("leite", "ˈlejtɨ", "lˈeɪtɨ", "lˈeɪtʃy"),
        ("cidade", "siˈdadɨ", "sˌidˈadɨ", "sˌidˈadʒy"),
        ("porta", "ˈpɔɾtɐ", "pˈɔɾətɐ", "pˈɔɾətæ"),
        ("também", "tɐ̃ˈbẽj̃", "tɐ̃mbˈeɪŋ", "tɐ̃mbˈeɪŋ"),
        ("mão", "ˈmɐ̃w̃", "mˈɐ̃ʊ̃", "mˈɐ̃ʊ̃"),
        ("mãe", "ˈmɐ̃j̃", "mˈɐ̃j", "mˈɐ̃y"),
        ("põe", "ˈpo\u{303}j̃", "pˈo\u{303}j", "pˈo\u{303}j"),
        ("falamos", "fɐˈlɐmuʃ", "fˌɐlˈɐ̃mʊʃ", "fˌalˈɐ̃mʊs"),
        ("comer", "kuˈmeɾ", "kumˈeɹ", "komˈer"),
        ("Brasil", "bɾɐˈzil", "bɹɐzˈil", "brazˈiʊ"),
        ("sal", "ˈsal", "sˈɑl", "sˈaʊ"),
        ("filho", "ˈfiʎu", "fˈiʎʊ", "fˈiljʊ"),
        ("rainha", "ʁɐˈiɲɐ", "ʁˌɐˈiɲɐ", "xˌaˈiɲæ"),
        ("exame", "ɨˈzɐmɨ", "ˌezˈɐ̃mɨ", "ˌezˈɐ̃my"),
        ("táxi", "ˈtaksi", "tˈaksi", "tˈaksi"),
        ("água", "ˈaɡwɐ", "ˈaɡwɐ", "ˈaɡwæ"),
        ("mesmo", "ˈmeʒmu", "mˈeʒmʊ", "mˈezmʊ"),
        ("escola", "ˈʃkɔlɐ", "ʃkˈɔlɐ", "ˌeskˈɔlæ"),
        ("pátio", "ˈpatju", "pˈatjʊ", "pˈatʃjʊ"),
        ("amigos", "ɐˈmiɡuʃ", "ɐmˈiɡʊʃ", "ˌæmˈiɡʊs"),
        ("universidade", "univɨɾsiˈdadɨ", "ˌunivˌeɾəsidˈadɨ", "ˌunivˌeɾəsidˈadʒy"),
        ("muito", "ˈmwĩtu", "mwˈiŋtʊ", "mwˈiŋtʊ"),
        ("depois", "dɨˈpojʃ", "dɨpˈoɪʃ", "depˈoɪs"),
        ("punhado", "pu\u{303}ˈɲadu", "pˌu\u{303}ɲˈadʊ", "pˌu\u{303}ɲˈadʊ"),
        // Function words alone keep their stress; a sentence takes it away.
        ("de", "ˈdɨ", "dˈɨ", "dʒˈy"),
        ("os", "ˈuʃ", "ˈuʃ", "ˈʊs"),
        ("ser", "ˈsɨɾ", "sˈɨɹ", "sˈer"),
        ("e", "ˈi", "ˈi", "ˈi"),
        ("com", "ˈkõ", "kˈom", "kˈoŋ"),
        ("pègadas", "pɨˈɡadɐʃ", "pˌeɡˈadɐʃ", "pˌeɡˈadæs"),
    ];
    let wrong: Vec<String> = cases
        .iter()
        .filter_map(|(word, ipa, european, brazilian)| {
            let portugal = say(word, Variety::European).unwrap();
            let brazil = say(word, Variety::Brazilian).unwrap();
            let got = (
                portugal.to_notation(Notation::Ipa),
                portugal.to_notation(Notation::Espeak),
                brazil.to_notation(Notation::Espeak),
            );
            (got != ((*ipa).to_owned(), (*european).to_owned(), (*brazilian).to_owned()))
                .then(|| format!("{word}: {} {} {}", got.0, got.1, got.2))
        })
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
    assert!(say("ŋ", Variety::European).is_none());
}

#[test]
fn numbers_in_words() {
    for (number, feminine, variety, words) in [
        (0, false, Variety::European, "zero"),
        (16, false, Variety::European, "dezasseis"),
        (16, false, Variety::Brazilian, "dezesseis"),
        (21, false, Variety::European, "vinte e um"),
        (21, true, Variety::European, "vinte e uma"),
        (100, false, Variety::European, "cem"),
        (101, false, Variety::European, "cento e um"),
        (200, true, Variety::European, "duzentas"),
        (345, false, Variety::European, "trezentos e quarenta e cinco"),
        (1000, false, Variety::European, "mil"),
        (1300, false, Variety::European, "mil e trezentos"),
        (1999, false, Variety::European, "mil novecentos e noventa e nove"),
        (2024, false, Variety::European, "dois mil e vinte e quatro"),
        (2_000_000, false, Variety::European, "dois milhões"),
        (1_000_000_000, false, Variety::European, "mil milhões"),
        (1_000_000_000, false, Variety::Brazilian, "um bilhão"),
    ] {
        assert_eq!(cardinal(number, feminine, variety), words, "{number}");
    }
    assert_eq!(ordinal(21, true).unwrap(), "vigésima primeira");
    assert_eq!(ordinal(3, false).unwrap(), "terceiro");
    assert_eq!(ordinal(100, false), None);
}

#[test]
fn the_locale_reads_signs_times_dates_and_centuries() {
    let portugal = Portuguese {
        variety: Variety::European,
    };
    let brazil = Portuguese {
        variety: Variety::Brazilian,
    };
    assert_eq!(portugal.time(10, 30).unwrap(), "dez e trinta");
    assert_eq!(portugal.time(1, 0).unwrap(), "uma hora");
    assert_eq!(portugal.time(8, 0).unwrap(), "oito horas");
    assert_eq!(
        portugal.date(1, 5, 2024).unwrap(),
        "um de maio de dois mil e vinte e quatro"
    );
    assert_eq!(
        brazil.date(1, 5, 2024).unwrap(),
        "primeiro de maio de dois mil e vinte e quatro"
    );
    assert_eq!(portugal.date(1, 13, 2024), None);
    assert_eq!(portugal.ordinal(2, Gender::Feminine).unwrap(), "segunda");
    assert_eq!(portugal.cardinal(2, portugal.gender_of("casas")).unwrap(), "duas");
    assert_eq!(portugal.gender_of("dias"), Gender::Masculine);
    assert_eq!(portugal.gender_of("lições"), Gender::Feminine);
    assert_eq!(portugal.gender_of("livros"), Gender::Masculine);
    assert_eq!(portugal.roman(20, Some("século"), None).unwrap(), "vinte");
    assert_eq!(portugal.roman(6, Some("João"), None).unwrap(), "sexto");
    assert_eq!(portugal.roman(14, Some("Luís"), None).unwrap(), "catorze");
    assert_eq!(portugal.roman(5, Some("o"), None), None);
    assert_eq!(portugal.roman(5, None, None), None);
    let words = [
        portugal.decimal_word(','),
        portugal.decimal_word('.'),
        portugal.range(),
        portugal.ratio(),
        portugal.minus(),
        portugal.plus(),
        portugal.percent(),
    ];
    assert_eq!(words, ["vírgula", "ponto", "a", "sobre", "menos", "mais", "por cento"]);
    for sign in ["€", "$", "R$", "£"] {
        assert!(portugal.currency(sign).is_some(), "{sign}");
    }
    assert_eq!(portugal.currency("¥"), None);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| portugal.symbol(symbol)).count(), 9);
    let letters: String = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .filter_map(|letter| portugal.letter(letter))
        .collect();
    assert!(
        letters.starts_with("ábêcêdêéefeguê") && letters.ends_with("xisípsilonzê"),
        "{letters}"
    );
    assert_eq!(brazil.letter('g'), Some("gê"));
    assert_eq!(brazil.letter('k'), Some("cá"));
    assert_eq!(portugal.letter('1'), None);
    assert_eq!(
        "@./:-_!".chars().filter_map(|sign| portugal.address_sign(sign)).count(),
        6
    );
    assert!(is_abbreviation("Dr") && is_abbreviation("etc") && !is_abbreviation("n") && !is_abbreviation("casa"));
    assert_eq!(portugal.abbreviation("Av"), Some("avenida"));
    assert_eq!(portugal.abbreviation("n"), None);
    assert!(portugal.vowels().contains('ã'));
}

use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Slovenian, is_abbreviation},
    numbers::{Agreement, cardinal, day},
    sentence::{read_tokens, say_word},
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("hiša", "ˈxiːʃa", "xˈiːʃa"),
        ("knjiga", "ˈkɲiːɡa", "kɲˈiːɡa"),
        ("dober", "ˈdoːbər", "dˈoːbər"),
        ("vrt", "ˈʋərt", "ʋˈərt"),
        ("trgu", "ˈtərɡu", "tˈərɡu"),
        ("volk", "ˈʋoːu̯k", "ʋˈoːwk"),
        ("lev", "ˈleːu̯", "lˈeːw"),
        ("vse", "uˈseː", "usˈeː"),
        ("vrata", "ˈʋraːta", "ʋrˈaːta"),
        ("okolje", "ɔˈkoːljɛ", "ɔkˈoːljɛ"),
        ("lehko", "ˈleːxkɔ", "lˈeːhkɔ"),
        ("svojih", "ˈsʋoːjix", "sʋˈoːjih"),
        ("polh", "ˈpoːu̯x", "pˈoːwx"),
        ("enkrat", "ˈeːŋkrat", "ˈeːŋkrat"),
        ("zjutraj", "zjuˈtraːj", "zjutrˈaːj"),
        ("mislil", "miˈsliːu̯", "mislˈiːw"),
        ("deset", "dɛˈseːt", "dɛsˈeːt"),
        ("žogo", "ˈʒɔːɡɔ", "ʒˈɔːɡɔ"),
        ("še", "ˈʃɛ", "ʃˈɛ"),
        ("sem", "səm", "səm"),
        ("učitelj", "uˈtʃiːtɛʎ", "utʃˈiːtɛʎ"),
    ];
    let wrong: Vec<String> = cases
        .iter()
        .filter_map(|(word, ipa, espeak)| {
            let said = say_word(word).unwrap();
            let (got_ipa, got_espeak) = (said.to_notation(Notation::Ipa), said.to_notation(Notation::Espeak));
            (got_ipa != *ipa || got_espeak != *espeak).then(|| format!("{word}: {got_ipa} {got_espeak}"))
        })
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
    assert!(say_word("ß").is_none());
}

/// The words of a sentence (tokens split at spaces) in eSpeak NG's notation.
fn sentence(text: &str) -> String {
    let tokens: Vec<(&str, TokenKind)> = text.split(' ').map(|token| (token, TokenKind::Word)).collect();
    read_tokens(&tokens)
        .into_iter()
        .flatten()
        .flat_map(|reading| reading.words)
        .map(|said| said.to_notation(Notation::Espeak))
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn prepositions_voicing_and_function_words() {
    // The prepositions of one consonant join the next word and leave it its stress.
    assert_eq!(sentence("v redu"), "urˈeːdu");
    assert_eq!(sentence("z žogo"), "zʒˈɔːɡɔ");
    assert_eq!(sentence("brat dela"), "brˈaːd dˈeːla");
    assert_eq!(sentence("knjig in"), "kɲˈiːk ˈiːn");
    assert_eq!(sentence("je bila doma"), "jɛ bˌila dˈoːma");
    assert_eq!(sentence("R"), "ˈeːr");
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "nič"),
        (1, "ena"),
        (15, "petnajst"),
        (20, "dvajset"),
        (21, "ênaindvajset"),
        (24, "štíriindvajset"),
        (200, "dvesto"),
        (1000, "tisoč"),
        (2024, "dva tisoč štíriindvajset"),
        (2_000_000, "dva milijona"),
        (5_000_000, "pet milijonov"),
    ] {
        assert_eq!(cardinal(number, Agreement::Counting), words, "{number}");
    }
    assert_eq!(cardinal(1, Agreement::Masculine), "en");
    assert_eq!(cardinal(2, Agreement::Feminine), "dve");
    assert_eq!(day(3).unwrap(), "tretjega");
    assert_eq!(day(21).unwrap(), "ênaindvajsetega");
    assert_eq!(day(32), None);
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(
        Slovenian.date(3, 5, 2024).unwrap(),
        "tretjega maja dva tisoč štíriindvajset"
    );
    assert_eq!(Slovenian.date(1, 13, 2024), None);
    assert_eq!(Slovenian.cardinal(1, Slovenian.gender_of("knjiga")).unwrap(), "ena");
    assert_eq!(Slovenian.cardinal(1, Gender::Masculine).unwrap(), "en");
    for (count, name) in [
        (Some(1), "evro"),
        (Some(2), "evra"),
        (Some(3), "evri"),
        (Some(5), "evrov"),
        (None, "evrov"),
    ] {
        assert_eq!(Slovenian.currency_name("€", count), Some(name), "{count:?}");
    }
    assert_eq!(Slovenian.currency("$"), Some(("dolar", "dolarjev")));
    assert_eq!(Slovenian.currency("£"), Some(("funt", "funtov")));
    assert_eq!(Slovenian.currency("¥"), None);
    let words = [
        Slovenian.decimal_word(','),
        Slovenian.decimal_word('.'),
        Slovenian.range(),
        Slovenian.ratio(),
        Slovenian.minus(),
        Slovenian.plus(),
        Slovenian.percent(),
    ];
    assert_eq!(words, ["cela", "pika", "do", "skozi", "minus", "plus", "odstotkov"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Slovenian.symbol(symbol)).count(), 9);
    let letters: String = "abcčdefghijklmnopqrsštuvwxyzž"
        .chars()
        .filter_map(|letter| Slovenian.letter(letter))
        .collect();
    assert!(
        letters.starts_with("abecečede") && letters.ends_with("ipsilonzeže"),
        "{letters}"
    );
    assert_eq!(Slovenian.letter('1'), None);
    assert_eq!(
        "@./:-_!"
            .chars()
            .filter_map(|sign| Slovenian.address_sign(sign))
            .count(),
        6
    );
    assert!(is_abbreviation("Npr") && is_abbreviation("itd") && !is_abbreviation("hiša"));
    assert_eq!(Slovenian.abbreviation("št"), Some("številka"));
    assert_eq!(Slovenian.vowels(), "aeiou");
}

use jespersen_phonemes::Notation;
use jespersen_spoken::Locale;
use jespersen_text::TokenKind;

use crate::{
    cyrillic::to_latin,
    locale::{Serbian, is_abbreviation},
    numbers::{Agreement, cardinal, day},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn cyrillic_turns_into_latin() {
    assert_eq!(to_latin("Љубав и њега"), "ljubav i njega");
    assert_eq!(to_latin("џеп, ђак, ћуд"), "džep, đak, ćud");
    assert_eq!(to_latin("Beograd"), "beograd");
}

#[test]
fn words_from_their_spelling_in_both_scripts() {
    let cases = [
        ("мачка", "ˈmatʃka", "mˈatʃkæ"),
        ("mačka", "ˈmatʃka", "mˈatʃkæ"),
        ("кућа", "ˈkutɕa", "kˈutɕæ"),
        ("лепо", "ˈlepo", "lˈɛpo"),
        ("њега", "ˈɲeɡa", "ɲˈeɡæ"),
        ("џеп", "ˈdʒep", "dʒˈɛp"),
        ("ђак", "ˈdʑak", "dʑˈak"),
        ("смрт", "smr̩t", "smrt"),
        ("Србија", "sr̩ˈbija", "srbˈijæ"),
        ("љубав", "ˈʎubav", "ʎˈubæv"),
        ("hiljada", "ˈxiʎada", "xˈiʎædæ"),
        ("добар", "ˈdobar", "dˈobɑr"),
        ("крову", "ˈkrovu", "kɾˈovʊ"),
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
fn voicing_and_function_words_in_both_scripts() {
    assert_eq!(sentence("put do mora"), "pˈud dˈo mˈoræ");
    assert_eq!(sentence("све је у реду"), "svˈɛ je ʊ ɾˈɛdʊ");
    assert_eq!(sentence("sve je u redu"), "svˈɛ je ʊ ɾˈɛdʊ");
    assert_eq!(sentence("били смо тамо"), "bˌɪlɪ smo tˈamo");
    assert_eq!(sentence("R"), "ˈɛr");
    assert_eq!(sentence("Љ"), "ʎˈe");
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "nula"),
        (21, "dvadeset jedan"),
        (200, "dvesta"),
        (345, "trista četrdeset pet"),
        (1000, "hiljadu"),
        (2024, "dve hiljade dvadeset četiri"),
        (5000, "pet hiljada"),
        (2_000_000, "dva miliona"),
        (3_000_000_000, "tri milijarde"),
    ] {
        assert_eq!(cardinal(number, Agreement::Masculine), words, "{number}");
    }
    assert_eq!(cardinal(22, Agreement::Feminine), "dvadeset dve");
    assert_eq!(day(3).unwrap(), "trećeg");
    assert_eq!(day(21).unwrap(), "dvadeset prvog");
    assert_eq!(day(32), None);
}

#[test]
fn the_locale_reads_signs_times_dates_and_money() {
    assert_eq!(
        Serbian.date(3, 5, 2024).unwrap(),
        "trećeg maja dve hiljade dvadeset četiri"
    );
    assert_eq!(Serbian.date(1, 13, 2024), None);
    assert_eq!(Serbian.time(10, 30).unwrap(), "deset i trideset");
    assert_eq!(Serbian.time(8, 0).unwrap(), "osam sati");
    assert_eq!(Serbian.cardinal(1, Serbian.gender_of("књига")).unwrap(), "jedna");
    assert_eq!(Serbian.cardinal(1, Serbian.gender_of("grad")).unwrap(), "jedan");
    for (count, name) in [
        (Some(1), "dinar"),
        (Some(3), "dinara"),
        (Some(21), "dinar"),
        (None, "dinara"),
    ] {
        assert_eq!(Serbian.currency_name("din", count), Some(name), "{count:?}");
    }
    assert_eq!(Serbian.currency("€"), Some(("evro", "evra")));
    assert_eq!(Serbian.currency("¥"), None);
    assert_eq!(Serbian.currency_name("¥", Some(1)), None);
    let words = [
        Serbian.decimal_word(','),
        Serbian.decimal_word('.'),
        Serbian.range(),
        Serbian.ratio(),
        Serbian.minus(),
        Serbian.plus(),
        Serbian.percent(),
    ];
    assert_eq!(words, ["zapeta", "tačka", "do", "kroz", "minus", "plus", "odsto"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Serbian.symbol(symbol)).count(), 9);
    let latin: String = "abcčćdđefghijklmnopqrsštuvwxyzž"
        .chars()
        .filter_map(|letter| Serbian.letter(letter))
        .collect();
    assert!(
        latin.starts_with("abecečećedeđe") && latin.ends_with("ipsilonzeže"),
        "{latin}"
    );
    let cyrillic: String = "абвгдђежзијклљмнњопрстћуфхцчџш"
        .chars()
        .filter_map(|letter| Serbian.letter(letter))
        .collect();
    assert!(
        cyrillic.starts_with("abevegedeđe") && cyrillic.ends_with("cečedžeše"),
        "{cyrillic}"
    );
    assert_eq!(Serbian.letter('1'), None);
    assert_eq!(
        "@./:-_!".chars().filter_map(|sign| Serbian.address_sign(sign)).count(),
        6
    );
    assert!(is_abbreviation("Npr") && is_abbreviation("итд") && !is_abbreviation("kuća"));
    assert_eq!(Serbian.abbreviation("бр"), Some("broj"));
    assert_eq!(Serbian.vowels(), "aeiouаеиоу");
}

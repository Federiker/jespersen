use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Croatian, is_abbreviation},
    numbers::{Agreement, cardinal, day},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("mačka", "ˈmatʃka", "mˈatʃkæ"),
        ("kuća", "ˈkutɕa", "kˈutɕæ"),
        ("žena", "ˈʒena", "ʒˈɛnæ"),
        ("lijepo", "ˈlijepo", "lˈijepo"),
        ("njega", "ˈɲeɡa", "ɲˈeɡæ"),
        ("džep", "ˈdʒep", "dʒˈɛp"),
        ("đak", "ˈdʑak", "dʑˈak"),
        ("smrt", "smr̩t", "smrt"),
        ("trgu", "tr̩ˈɡu", "trɡˈu"),
        ("vrh", "vr̩x", "vrh"),
        ("Hrvatska", "xr̩ˈvatska", "xrvˈatskæ"),
        ("kuha", "ˈkuxa", "kˈuhæ"),
        ("dobar", "ˈdobar", "dˈobɑr"),
        ("igraju", "ˈiɡraju", "ˈiɡɾɑjʊ"),
        ("igralištu", "ˈiɡraliʃtu", "ˈiɡɾælˌɪʃtʊ"),
        ("novčaniku", "ˈnovtʃaniku", "nˈoftʃænˌɪkʊ"),
        ("dvanaest", "ˈdvanaest", "dvˈanaɪst"),
        ("krovu", "ˈkrovu", "kɾˈovʊ"),
        ("banka", "ˈbaŋka", "bˈaŋkæ"),
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
fn voicing_and_function_words() {
    assert_eq!(sentence("put do mora"), "pˈud dˈo mˈoræ");
    assert_eq!(sentence("od povrća"), "ˈot pˈovrtɕæ");
    assert_eq!(sentence("sve je u redu"), "svˈɛ je ʊ ɾˈɛdʊ");
    assert_eq!(sentence("bili smo tamo"), "bˌɪlɪ smo tˈamo");
    assert_eq!(sentence("R"), "ˈɛr");
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "nula"),
        (15, "petnaest"),
        (21, "dvadeset jedan"),
        (200, "dvjesto"),
        (345, "tristo četrdeset pet"),
        (1000, "tisuću"),
        (2024, "dvije tisuće dvadeset četiri"),
        (5000, "pet tisuća"),
        (2_000_000, "dva milijuna"),
        (3_000_000_000, "tri milijarde"),
    ] {
        assert_eq!(cardinal(number, Agreement::Masculine), words, "{number}");
    }
    assert_eq!(cardinal(1, Agreement::Feminine), "jedna");
    assert_eq!(cardinal(22, Agreement::Feminine), "dvadeset dvije");
    assert_eq!(day(3).unwrap(), "trećeg");
    assert_eq!(day(1).unwrap(), "prvog");
    assert_eq!(day(20).unwrap(), "dvadesetog");
    assert_eq!(day(21).unwrap(), "dvadeset prvog");
    assert_eq!(day(32), None);
}

#[test]
fn the_locale_reads_signs_times_dates_and_money() {
    assert_eq!(
        Croatian.date(3, 5, 2024).unwrap(),
        "trećeg svibnja dvije tisuće dvadeset četiri"
    );
    assert_eq!(Croatian.date(1, 13, 2024), None);
    assert_eq!(Croatian.time(10, 30).unwrap(), "deset i trideset");
    assert_eq!(Croatian.time(8, 0).unwrap(), "osam sati");
    assert_eq!(Croatian.cardinal(1, Croatian.gender_of("knjiga")).unwrap(), "jedna");
    assert_eq!(Croatian.cardinal(1, Gender::Masculine).unwrap(), "jedan");
    for (count, name) in [
        (Some(1), "euro"),
        (Some(3), "eura"),
        (Some(21), "euro"),
        (Some(12), "eura"),
        (None, "eura"),
    ] {
        assert_eq!(Croatian.currency_name("€", count), Some(name), "{count:?}");
    }
    assert_eq!(Croatian.currency_name("kn", Some(3)), Some("kune"));
    assert_eq!(Croatian.currency("$"), Some(("dolar", "dolara")));
    assert_eq!(Croatian.currency("£"), Some(("funta", "funti")));
    assert_eq!(Croatian.currency("¥"), None);
    let words = [
        Croatian.decimal_word(','),
        Croatian.decimal_word('.'),
        Croatian.range(),
        Croatian.ratio(),
        Croatian.minus(),
        Croatian.plus(),
        Croatian.percent(),
    ];
    assert_eq!(words, ["zarez", "točka", "do", "kroz", "minus", "plus", "posto"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Croatian.symbol(symbol)).count(), 9);
    let letters: String = "abcčćdđefghijklmnopqrsštuvwxyzž"
        .chars()
        .filter_map(|letter| Croatian.letter(letter))
        .collect();
    assert!(
        letters.starts_with("abecečećedeđe") && letters.ends_with("ipsilonzeže"),
        "{letters}"
    );
    assert_eq!(Croatian.letter('1'), None);
    assert_eq!(
        "@./:-_!".chars().filter_map(|sign| Croatian.address_sign(sign)).count(),
        6
    );
    assert!(is_abbreviation("Npr") && is_abbreviation("itd") && !is_abbreviation("kuća"));
    assert_eq!(Croatian.abbreviation("br"), Some("broj"));
    assert_eq!(Croatian.vowels(), "aeiou");
}

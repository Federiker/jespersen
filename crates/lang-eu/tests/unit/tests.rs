use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Basque, is_abbreviation},
    numbers::cardinal,
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("etxe", "ˈetʃe", "ˈetʃe"),
        ("zuhaitz", "s̻uˈai̯ts̻", "s̻uˈaɪts̻"),
        ("itsaso", "iˈts̺as̺o", "its̺ˈas̺ˌo"),
        ("xake", "ˈʃake", "ʃˈake"),
        ("gizon", "ɡiˈs̻on", "ɡis̻ˈon"),
        ("ttipi", "ˈcipi", "cˈipi"),
        ("llabur", "ʎaˈbuɾ", "ʎabˈuɾ"),
        ("herri", "ˈeri", "ˈeri"),
        ("ilargi", "iˈʎaɾɡi", "iʎˈaɾɡˌi"),
        ("baino", "ˈbai̯ɲo", "bˈaɪɲo"),
        ("katua", "kaˈtua", "katˈuˌa"),
        ("anaia", "aˈnai̯a", "anˈaɪˌa"),
        ("jende", "ˈjende", "ɟˈende"),
        ("jan", "ˈjan", "jˈan"),
        ("dira", "diˈɾa", "diɾˈa"),
        ("euskara", "eu̯s̺ˈkaɾa", "eʊs̺kˈaɾˌa"),
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

#[test]
fn soft_consonants_and_lighter_words() {
    let tokens: Vec<(&str, TokenKind)> = "ondo dago eta begira dago"
        .split(' ')
        .map(|token| (token, TokenKind::Word))
        .collect();
    let written: Vec<String> = read_tokens(&tokens)
        .into_iter()
        .flatten()
        .flat_map(|reading| reading.words)
        .map(|said| said.to_notation(Notation::Espeak))
        .collect();
    assert_eq!(written, ["ˈondo", "ðˈaɣo", "ˌeta", "βeɣˈiɾˌa", "ðˈaɣo"]);
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "zero"),
        (11, "hamaika"),
        (20, "hogei"),
        (21, "hogeita bat"),
        (40, "berrogei"),
        (55, "berrogeita hamabost"),
        (99, "laurogeita hemeretzi"),
        (100, "ehun"),
        (101, "ehun eta bat"),
        (200, "berrehun"),
        (1000, "mila"),
        (2024, "bi mila eta hogeita lau"),
        (1999, "mila bederatziehun eta laurogeita hemeretzi"),
        (1_000_000, "milioi bat"),
        (3_000_000, "hiru milioi"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(
        Basque.date(3, 5, 2024).unwrap(),
        "bi mila eta hogeita lauko maiatzaren hirua"
    );
    assert_eq!(Basque.date(1, 13, 2024), None);
    assert_eq!(Basque.cardinal(1, Gender::Alone).unwrap(), "bat");
    assert!(Basque.percent_first());
    assert_eq!(Basque.currency("€"), Some(("euro", "euro")));
    assert_eq!(Basque.currency("$"), Some(("dolar", "dolar")));
    assert_eq!(Basque.currency("£"), Some(("libera", "libera")));
    assert_eq!(Basque.currency("¥"), None);
    let words = [
        Basque.decimal_word(','),
        Basque.decimal_word('.'),
        Basque.range(),
        Basque.ratio(),
        Basque.minus(),
        Basque.plus(),
        Basque.percent(),
    ];
    assert_eq!(words, ["koma", "puntu", "eta", "zati", "minus", "plus", "ehuneko"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Basque.symbol(symbol)).count(), 9);
    let letters: String = "abcdefghijklmnñopqrstuvwxyz"
        .chars()
        .filter_map(|letter| Basque.letter(letter))
        .collect();
    assert!(
        letters.starts_with("abezedeeefegehatxe") && letters.ends_with("ixai grekoazeta"),
        "{letters}"
    );
    assert_eq!(Basque.letter('1'), None);
    assert_eq!(
        "@./:-_!".chars().filter_map(|sign| Basque.address_sign(sign)).count(),
        6
    );
    assert!(is_abbreviation("Dr") && is_abbreviation("etab") && !is_abbreviation("k") && !is_abbreviation("etxe"));
    assert_eq!(Basque.abbreviation("adib"), Some("adibidez"));
    assert_eq!(Basque.abbreviation("k"), None);
    assert_eq!(Basque.vowels(), "aeiou");
}

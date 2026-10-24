use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Estonian, is_abbreviation},
    numbers::cardinal,
    phonemes::{K, LONG, OFFGLIDES, S, T, legal_onset},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("maja", "ˈmaja", "mˈajɑ"),
        ("maa", "ˈmaːː", "mˈaaː"),
        ("kala", "ˈkala", "kˈalɑ"),
        ("kalla", "ˈkalːa", "kˈalla"),
        ("kaala", "ˈkaːla", "kˈaːla"),
        ("saab", "ˈsaːːb̥", "sˈaaːb"),
        ("saada", "ˈsaːd̥a", "sˈaːda"),
        ("linn", "ˈlinː", "lˈinn"),
        ("õun", "ˈɤu̯n", "ˈɵun"),
        ("õde", "ˈɤd̥e", "ˈɵdɛ"),
        ("käsi", "ˈkæsʲi", "kˈæs^ɪ"),
        ("möödas", "ˈmøːd̥as", "mˈøːdas"),
        ("üks", "ˈyks", "ˈyks"),
        ("seitse", "ˈsei̯tse", "sˈeitse"),
        ("kaheksa", "ˈkaheksa", "kˈahɛksa"),
        ("kümme", "ˈkymːe", "kˈymme"),
        ("tuhat", "ˈtuhat", "tˈuhɑt"),
        ("tere", "ˈtere", "tˈerɛ"),
        ("aitäh", "ˈai̯tːæh", "ˈaitːæh"),
        ("kool", "ˈkoːːl", "kˈooːl"),
        ("koolid", "ˈkoːlʲid̥", "kˈoːʎid"),
        ("palju", "ˈpalʲju", "pˈaʎju"),
        ("päike", "ˈpæi̯kːe", "pˈæikːe"),
        ("õhtu", "ˈɤhtu", "ˈɵhtu"),
        ("kirik", "ˈkirik", "kˈirɪk"),
        ("lugema", "ˈluɡ̊ema", "lˈuɡɛma"),
        ("kirjutama", "ˈkirjuˌtama", "kˈirjutˌama"),
        ("raamatukogu", "ˈraːmaˌtukoɡ̊u", "rˈaːmatˌukoɡu"),
        ("hiir", "ˈhiːːr", "hˈiiːr"),
        ("jõgi", "ˈjɤɡ̊i", "jˈɵɡɪ"),
        ("hea", "ˈhea̯ː", "hˈeaː"),
        ("kassi", "ˈkasʲːi", "kˈas^s^i"),
        ("padi", "ˈpad̥ʲi", "pˈad^ɪ"),
        ("lõpp", "ˈlɤpː", "lˈɵpː"),
        ("õppima", "ˈɤpːima", "ˈɵpːima"),
        ("šokolaad", "ʃokoˈlaːd̥", "ʃˌokolˈaːd"),
        ("universiteet", "unʲiversiˈteːt", "ˌuɲɪvˌersitˈeːt"),
        ("Eesti", "ˈeːsʲti", "ˈeːs^ti"),
        ("poisid", "ˈpoi̯sid̥", "pˈoisid"),
        ("saata", "ˈsaːtːa", "sˈaːtːa"),
        ("kaupa", "ˈkau̯pːa", "kˈaupːa"),
        ("siia", "ˈsiːja", "sˈiːja"),
        ("uue", "ˈuːwe", "ˈuːwe"),
        ("meie", "ˈmei̯je", "mˈeije"),
        ("kass", "ˈkai̯sʲː", "kˈais^s^"),
        ("küll", "ˈkyi̯lʲː", "kˈyiʎʎ"),
        ("all", "ˈalː", "ˈall"),
        ("tundi", "ˈtunʲd̥i", "tˈuɲdi"),
        ("ütles", "ˈytːles", "ˈytːles"),
        ("sööks", "ˈsøːks", "sˈøːks"),
        ("ning", "ˈniŋɡ̊", "nˈiŋɡ"),
        ("koju", "ˈkoju", "kˈojʊ"),
        ("kõvaks", "ˈkɤvaks", "kˈɵvaks"),
        ("maailm", "ˈmaːilm", "mˈaːilm"),
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
    // The older spelling's w is v.
    assert_eq!(pronounce("wiis").unwrap().to_notation(Notation::Espeak), "vˈiiːs");
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
    assert_eq!(read("ma ei tea"), ["ma", "ˌeiː", "tˈeaː"]);
    assert_eq!(read("see on hea"), ["sˌeeː", "on", "hˈeaː"]);
    assert_eq!(read("21"), ["kˈakskymmend", "ˈyks"]);
    assert_eq!(read("B"), ["bˈeeː"]);
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "null"),
        (1, "üks"),
        (10, "kümme"),
        (12, "kaksteist"),
        (20, "kakskümmend"),
        (21, "kakskümmend üks"),
        (100, "sada"),
        (101, "sada üks"),
        (300, "kolmsada"),
        (1000, "tuhat"),
        (2024, "kaks tuhat kakskümmend neli"),
        (1_000_000, "miljon"),
        (3_000_000, "kolm miljonit"),
        (2_000_000_000, "kaks miljardit"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(
        Estonian.date(3, 5, 2024).unwrap(),
        "kolm mai kaks tuhat kakskümmend neli"
    );
    assert_eq!(Estonian.date(1, 13, 2024), None);
    assert_eq!(Estonian.time(10, 30).unwrap(), "kümme kolmkümmend");
    assert_eq!(Estonian.time(10, 0).unwrap(), "kell kümme");
    assert_eq!(Estonian.cardinal(2, Gender::Alone).unwrap(), "kaks");
    assert_eq!(Estonian.currency("€"), Some(("euro", "eurot")));
    assert_eq!(Estonian.currency("£"), Some(("nael", "naela")));
    assert_eq!(Estonian.currency("¥"), None);
    let words = [
        Estonian.decimal_word(','),
        Estonian.decimal_word('.'),
        Estonian.range(),
        Estonian.ratio(),
        Estonian.minus(),
        Estonian.plus(),
        Estonian.percent(),
    ];
    assert_eq!(
        words,
        ["koma", "punkt", "kuni", "jagatud", "miinus", "pluss", "protsenti"]
    );
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Estonian.symbol(symbol)).count(), 9);
    let letters: String = "abcdefghijklmnopqrsšzžtuvwõäöüxy"
        .chars()
        .filter_map(|letter| Estonian.letter(letter))
        .collect();
    assert!(
        letters.starts_with("aabeetseedeeeeeffgeehaaiijott") && letters.ends_with("üüiksigrek"),
        "{letters}"
    );
    assert_eq!(Estonian.letter('1'), None);
    assert_eq!(
        "@./:-_!".chars().filter_map(|sign| Estonian.address_sign(sign)).count(),
        6
    );
    assert!(is_abbreviation("Dr") && is_abbreviation("jne") && !is_abbreviation("maja"));
    assert_eq!(Estonian.abbreviation("nt"), Some("näiteks"));
    assert_eq!(Estonian.vowels(), "aeiouõäöüy");
}

#[test]
fn syllables_start_as_estonian_allows() {
    let glide = OFFGLIDES[2].1[0];
    assert!(legal_onset(&[T]) && !legal_onset(&[K, S]) && !legal_onset(&[LONG]) && !legal_onset(&[glide]));
}

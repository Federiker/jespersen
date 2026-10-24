use jespersen_phonemes::Notation;

use crate::{
    numbers::{Case, cardinal, ordinal},
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("kot", "ˈkɔt", "kˈɔt"),
        ("dziękuję", "dʑɛŋˈkujɛ", "dʑɛŋkˈujɛ"),
        ("wszystko", "ˈfʃɨst.kɔ", "fʃˈɨstkɔ"),
        ("chleb", "ˈxlɛp", "xlˈɛp"),
        ("ząb", "ˈzɔmp", "zˈɔmp"),
        ("mąż", "ˈmɔ̃ʃ", "mˈɔ̃ʃ"),
        ("się", "ˈɕɛ", "ɕˈɛ"),
        ("pięknie", "ˈpjɛŋ.kɲɛ", "pʲˈɛŋkɲʲɛ"),
        ("biuro", "ˈbju.rɔ", "bjˈurɔ"),
        ("lubi", "ˈlu.bi", "lˈubʲi"),
        ("świeże", "ˈɕfjɛ.ʒɛ", "ɕfʲˈɛʒɛ"),
        ("chrząszcz", "ˈxʃɔ̃ʃtʃ", "xʃˈɔ̃ʃtʃ"),
        ("trwała", "ˈtrfa.wa", "trfˈawa"),
        ("sześćdziesiąt", "ʃɛˈʑdʑɛ.ɕɔnt", "ʃɛʑdʑˈɛɕɔnt"),
        ("uniwersytet", "u.ɲi.vɛrˈsɨ.tɛt", "ˌuɲivɛrsˈɨtɛt"),
        ("śpiewaliśmy", "ɕpjɛˈva.li.ɕmɨ", "ɕpʲɛvˈaliɕmˌɨ"),
        ("matematyka", "ma.tɛˈma.tɨ.ka", "mˌatɛmˈatɨkˌa"),
        ("nauczyciel", "na.uˈtʃɨ.tɕɛl", "nˌautʃˈɨtɕɛl"),
        ("Europa", "ɛwˈrɔ.pa", "ɛwrˈɔpa"),
        ("Gdańsku", "ˈɡdaɲ.sku", "ɡdˈaɲsku"),
        ("piętnaście", "pjɛˈtnaɕ.tɕɛ", "pʲɛtnˈaɕtɕɛ"),
        ("części", "ˈtʃɛ̃ɕ.tɕi", "tʃˈɛɲɕtɕi"),
        ("jakąś", "ˈja.kɔ̃ɕ", "jˈakɔ̃ɕ"),
        ("drogi", "ˈdrɔ.ɡi", "drˈɔɡʲi"),
        ("ziemie", "ˈʑɛ.mjɛ", "ʑˈɛmjɛ"),
        ("niniejszéj", "ɲiˈɲɛj.ʃɛj", "ɲiɲʲˈɛjʃɛj"),
        ("w", "v", "w"),
    ];
    let wrong: Vec<String> = cases
        .iter()
        .filter_map(|(word, ipa, espeak)| {
            let said = pronounce(word).unwrap();
            let ipa = ipa.replace('.', "");
            let (got_ipa, got_espeak) = (said.to_notation(Notation::Ipa), said.to_notation(Notation::Espeak));
            (got_ipa != ipa || got_espeak != *espeak).then(|| format!("{word}: {got_ipa} {got_espeak}"))
        })
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
    assert!(pronounce("ç").is_none());
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "zero"),
        (15, "piętnaście"),
        (21, "dwadzieścia jeden"),
        (345, "trzysta czterdzieści pięć"),
        (1000, "tysiąc"),
        (2024, "dwa tysiące dwadzieścia cztery"),
        (5000, "pięć tysięcy"),
        (21_000, "dwadzieścia jeden tysięcy"),
        (2_000_000, "dwa miliony"),
    ] {
        assert_eq!(cardinal(number, false), words, "{number}");
    }
    assert_eq!(cardinal(1, true), "jedna");
    for (number, case, words) in [
        (3, Case::Genitive, "trzeciego"),
        (2024, Case::Genitive, "dwa tysiące dwudziestego czwartego"),
        (1918, Case::Genitive, "tysiąc dziewięćset osiemnastego"),
        (2000, Case::Genitive, "dwutysięcznego"),
        (1900, Case::Genitive, "tysiąc dziewięćsetnego"),
        (20, Case::Nominative, "dwudziesty"),
        (2, Case::Nominative, "drugi"),
        (20, Case::Locative, "dwudziestym"),
        (10, Case::Feminine, "dziesiąta"),
        (3, Case::Feminine, "trzecia"),
    ] {
        assert_eq!(ordinal(number, case).unwrap(), words, "{number}");
    }
    assert_eq!(ordinal(0, Case::Genitive), None);
}

#[test]
fn the_locale_reads_signs_times_money_and_centuries() {
    use jespersen_spoken::{Gender, Locale};

    use crate::locale::{Polish, is_abbreviation};

    assert_eq!(Polish.time(10, 30).unwrap(), "dziesiąta trzydzieści");
    assert_eq!(Polish.time(8, 5).unwrap(), "ósma zero pięć");
    assert_eq!(Polish.time(0, 0).unwrap(), "zero");
    assert_eq!(
        Polish.date(11, 11, 1918).unwrap(),
        "jedenastego listopada tysiąc dziewięćset osiemnastego"
    );
    assert_eq!(Polish.date(1, 13, 2000), None);
    assert_eq!(Polish.cardinal(1, Polish.gender_of("osoba")).unwrap(), "jedna");
    assert_eq!(Polish.gender_of("dom"), Gender::Masculine);
    for (count, name) in [
        (Some(1), "złoty"),
        (Some(22), "złote"),
        (Some(12), "złotych"),
        (None, "złotych"),
    ] {
        assert_eq!(Polish.currency_name("zł", count), Some(name), "{count:?}");
    }
    assert_eq!(Polish.currency("$"), Some(("dolar", "dolarów")));
    assert_eq!(Polish.currency_name("€", Some(5)), Some("euro"));
    assert_eq!(Polish.currency_name("£", Some(3)), Some("funty"));
    assert_eq!(Polish.currency_name("gr", Some(5)), Some("groszy"));
    assert_eq!(Polish.currency("¥"), None);
    assert_eq!(Polish.roman(20, None, Some("wiek")).unwrap(), "dwudziesty");
    assert_eq!(Polish.roman(20, Some("w"), Some("wieku")).unwrap(), "dwudziestym");
    assert_eq!(Polish.roman(19, Some("od"), Some("wieku")).unwrap(), "dziewiętnastego");
    assert_eq!(Polish.roman(2, Some("Paweł"), None).unwrap(), "drugi");
    assert_eq!(Polish.roman(2, Some("i"), None), None);
    let words = [
        Polish.decimal_word(','),
        Polish.decimal_word('.'),
        Polish.range(),
        Polish.ratio(),
        Polish.minus(),
        Polish.plus(),
        Polish.percent(),
    ];
    assert_eq!(
        words,
        ["przecinek", "kropka", "do", "przez", "minus", "plus", "procent"]
    );
    let symbols: Vec<Option<&str>> = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"]
        .iter()
        .map(|symbol| Polish.symbol(symbol))
        .collect();
    assert_eq!(symbols.iter().flatten().count(), 9);
    let letters: String = "aąbcćdeęfghijklłmnńoópqrsśtuvwxyzźż"
        .chars()
        .filter_map(|letter| Polish.letter(letter))
        .collect();
    assert!(
        letters.starts_with("aąbececiede") && letters.ends_with("zetzietżet"),
        "{letters}"
    );
    assert_eq!(Polish.letter('1'), None);
    let signs: Vec<Option<&str>> = "@./:-_!".chars().map(|sign| Polish.address_sign(sign)).collect();
    assert_eq!(signs.iter().flatten().count(), 6);
    assert!(is_abbreviation("Np") && is_abbreviation("itd") && !is_abbreviation("dom"));
    assert_eq!(Polish.abbreviation("ul"), Some("ulica"));
    assert_eq!(Polish.vowels(), "aąeęéioóuy");
}

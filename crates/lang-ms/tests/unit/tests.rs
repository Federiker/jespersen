use jespersen_phonemes::Notation;
use jespersen_spoken::Locale;
use jespersen_text::TokenKind;

use crate::{
    locale::{Malay, cardinal, is_abbreviation},
    read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("budak", "ˈbudaʔ", "bˈudaʔ"),
        ("tidak", "ˈtidaʔ", "tˈidaʔ"),
        ("saya", "ˈsajə", "sˈajə"),
        ("ya", "ˈja", "jˈa"),
        ("besar", "bəˈsar", "bəsˈar"),
        ("meja", "ˈmedʒə", "mˈedʒə"),
        ("kereta", "kəˈretə", "kərˈetə"),
        ("pantai", "ˈpantai̯", "pˈantaɪ"),
        ("pulau", "ˈpulau̯", "pˈulaʊ"),
        ("main", "ˈmain", "mˈain"),
        ("nyanyi", "ˈɲaɲi", "ɲˈaɲi"),
        ("syarikat", "ʃaˈrikat", "ʃarˈikat"),
        ("khabar", "ˈxabar", "xˈabar"),
        ("maghrib", "ˈmaɣrib", "mˈaɣrib"),
        ("rumah", "ˈrumah", "rˈumah"),
        ("malaysia", "malajˈsiə", "mˌalajsˈiə"),
        ("kerajaan", "kəraˈdʒaan", "kəradʒˈaan"),
        ("bangun", "ˈbaŋun", "bˈaŋun"),
        ("teksi", "ˈteksi", "tˈeksi"),
        ("xilofon", "ksiˈlofon", "ksilˈofon"),
        ("quran", "ˈkuran", "kˈuran"),
        ("zaman", "ˈzaman", "zˈaman"),
        ("vila", "ˈvilə", "vˈilə"),
        ("wang", "ˈwaŋ", "wˈaŋ"),
        ("cawan", "ˈtʃawan", "tʃˈawan"),
        ("fikir", "ˈfikir", "fˈikir"),
        ("amboi", "ˈamboi̯", "ˈambɔɪ"),
        ("survei", "ˈsurvei̯", "sˈurveɪ"),
        ("jam'iyah", "dʒaˈmijah", "dʒamˈijah"),
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
    assert!(pronounce("ç").is_none());
    assert!(pronounce("'").is_none());
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "sifar"),
        (8, "lapan"),
        (11, "sebelas"),
        (18, "lapan belas"),
        (21, "dua puluh satu"),
        (100, "seratus"),
        (1250, "seribu dua ratus lima puluh"),
        (2024, "dua ribu dua puluh empat"),
        (1_000_000, "sejuta"),
        (3_000_000_000, "tiga bilion"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_says_signs_and_dates() {
    let gender = jespersen_spoken::Gender::Masculine;
    assert_eq!(Malay.ordinal(1, gender).as_deref(), Some("pertama"));
    assert_eq!(Malay.ordinal(3, gender).as_deref(), Some("ketiga"));
    assert_eq!(
        Malay.date(31, 8, 1957).as_deref(),
        Some("tiga puluh satu Ogos seribu sembilan ratus lima puluh tujuh")
    );
    assert_eq!(Malay.date(1, 13, 2000), None);
    assert_eq!(Malay.decimal_word('.'), "perpuluhan");
    assert_eq!(Malay.decimal_word(','), "koma");
    assert_eq!(Malay.currency("RM"), Some(("ringgit", "ringgit")));
    assert_eq!(Malay.currency("¥"), None);
    assert_eq!(Malay.symbol("%"), Some("peratus"));
    assert_eq!(Malay.symbol("§"), None);
    assert_eq!(Malay.letter('W'), Some("dabelyu"));
    assert_eq!(Malay.letter('ß'), None);
    assert_eq!(Malay.address_sign('/'), Some("garis condong"));
    assert_eq!(Malay.address_sign('!'), None);
    assert!(is_abbreviation("Dr"));
    assert!(!is_abbreviation("rumah"));
    let signs = [
        Malay.range(),
        Malay.ratio(),
        Malay.minus(),
        Malay.plus(),
        Malay.percent(),
        Malay.vowels(),
    ];
    assert_eq!(signs, ["hingga", "per", "negatif", "positif", "peratus", "aeioué"]);
}

#[test]
fn sentences_lose_the_stress_of_function_words() {
    let tokens = [
        ("Saya", TokenKind::Word),
        ("pergi", TokenKind::Word),
        ("ke", TokenKind::Word),
        ("sekolah", TokenKind::Word),
        (".", TokenKind::Punctuation),
    ];
    let read = read_tokens(&tokens);
    let said: Vec<String> = read
        .iter()
        .flatten()
        .flat_map(|reading| reading.words.iter().map(|said| said.to_notation(Notation::Espeak)))
        .collect();
    assert_eq!(said, ["sˈajə", "pərɡˈi", "kə", "səkˈolah"]);
}

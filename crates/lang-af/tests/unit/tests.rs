use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Afrikaans, is_abbreviation},
    numbers::cardinal,
    phonemes::{K, L, N, R, S, T, W, legal_onset},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("huis", "ˈɦœys", "hˈøys"),
        ("skool", "ˈskuəl", "skˈʊəl"),
        ("straat", "ˈstrɑːt", "strˈɑːt"),
        ("boek", "ˈbuk", "bˈuk"),
        ("vader", "ˈfɑːdər", "fˈɑːdər"),
        ("moeder", "ˈmudər", "mˈudər"),
        ("begin", "bəˈχən", "bəxˈən"),
        ("verstaan", "fərˈstɑːn", "fərstˈɑːn"),
        ("nasie", "ˈnɑːsi", "nˈɑːsi"),
        ("universiteit", "ynivərsiˈtəit", "ynivərsitˈɛɪt"),
        ("studeer", "styˈdiər", "stydˈiər"),
        ("dag", "ˈdaχ", "dˈɐx"),
        ("twintig", "ˈtwəntəχ", "twˈəntəx"),
        ("honderd", "ˈɦɔnərt", "hˈɔnərt"),
        ("koning", "ˈkuənəŋ", "kˈʊənəŋ"),
        ("jaar", "ˈjɑːr", "jˈɑːr"),
        ("seun", "ˈsɪøn", "sˈøən"),
        ("water", "ˈvɑːtər", "vˈɑːtər"),
        ("tyd", "ˈtəit", "tˈɛɪt"),
        ("meisie", "ˈməisi", "mˈɛɪsi"),
        ("goed", "ˈχut", "xˈut"),
        ("sê", "ˈsɛː", "sˈeː"),
        ("nuut", "ˈnyt", "nˈyt"),
        ("brood", "ˈbruət", "brˈʊət"),
        ("motor", "ˈmuətər", "mˈʊətər"),
        ("vrou", "ˈfrœu", "frˈəʊ"),
        ("meer", "ˈmiər", "mˈiər"),
        ("baie", "ˈbaiə", "bˈaɪə"),
        ("ek", "ˈæk", "ˈæk"),
        ("elf", "ˈælf", "ˈælf"),
        ("wêreld", "ˈvɛːrəlt", "vˈæːrəlt"),
        ("môre", "ˈmɔːrə", "mˈɔːrə"),
        ("brûe", "ˈbrœːə", "brˈœːə"),
        ("liefde", "ˈlifdə", "lˈifdə"),
        ("geskiedenis", "χəˈskidənəs", "xəskˈidənəs"),
        ("opgewonde", "ˈɔpχəˌvɔndə", "ˈɔpxəvˌɔndə"),
        ("sjokolade", "ʃuəkuəˈlɑːdə", "ʃʊəkʊəlˈɑːdə"),
        ("dink", "ˈdəŋk", "dˈəŋk"),
        ("lank", "ˈlaŋk", "lˈɐŋk"),
        ("mense", "\u{2c8}m\u{25b}\u{303}ns\u{259}", "m\u{2c8}e\u{303}ns\u{259}"),
        ("kans", "ˈkɑ̃ːns", "kˈɑ̃ns"),
        ("kinders", "ˈkəndərs", "kˈəndərs"),
        ("onder", "ˈɔnər", "ˈɔnər"),
        ("vier", "ˈfiːr", "fˈiːr"),
        ("diere", "ˈdiːrə", "dˈiːrə"),
        ("snoer", "ˈsnur", "snˈuːr"),
        ("voël", "ˈfuəl", "fˈuːəl"),
        ("mooi", "ˈmoːi", "mˈʊɪ"),
        ("waai", "ˈvaːi", "vˈɑɪ"),
        ("goeie", "ˈχuiə", "xˈuɪə"),
        ("berge", "ˈbærɡə", "bˈærɡə"),
        ("warm", "ˈvarəm", "vˈɐrəm"),
        ("filosofie", "filuəsuəˈfi", "filʊəsʊəfˈi"),
        ("Botha", "ˈbuəta", "bˈʊətɐ"),
        ("hij", "ˈɦəi", "hˈəj"),
        ("tijd", "ˈtəit", "tˈɛɪt"),
        ("twee", "ˈtwiə", "twˈiə"),
        ("antwoord", "ˈantvuərt", "ˈɐntvʊərt"),
        ("euro", "ˈɪøru", "ˈøəru"),
        ("auto", "ˈœutu", "ˈəʊtu"),
        ("dae", "ˈdɑː", "dˈɑː"),
        ("gereën", "χəˈriən", "xərˈiən"),
        ("weg", "ˈvæχ", "vˈæx"),
        ("ver", "ˈfɛːr", "fˈæːr"),
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
    assert!(pronounce("ł").is_none());
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
    assert_eq!(read("die kat en die hond"), ["di", "kˈɐt", "ɛn", "di", "hˈɔnt"]);
    assert_eq!(read("sy sê te veel"), ["sɛɪ", "sˈeː", "tə", "fˈiəl"]);
    assert_eq!(read("'n huis"), ["ə", "hˈøys"]);
    assert_eq!(read("kombuis"), ["kɔmbˈøys"]);
    assert_eq!(read("21"), ["ˈiənɛntwˈəntəx"]);
    assert_eq!(read("101"), ["ˈiənhˈɔnərtɛn", "ˈiən"]);
    assert_eq!(read("B"), ["bˈiə"]);
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "nul"),
        (1, "een"),
        (13, "dertien"),
        (21, "een-en-twintig"),
        (80, "tagtig"),
        (100, "een-honderd"),
        (101, "een-honderd-en een"),
        (300, "drie-honderd"),
        (1000, "een duisend"),
        (2024, "twee duisend vier-en-twintig"),
        (1_000_000, "een miljoen"),
        (3_000_000, "drie miljoen"),
        (2_000_000_000, "twee miljard"),
        (1_000_000_000_000, "een biljoen"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(
        Afrikaans.date(3, 5, 2024).unwrap(),
        "drie Mei twee duisend vier-en-twintig"
    );
    assert_eq!(Afrikaans.date(1, 13, 2024), None);
    assert_eq!(Afrikaans.time(10, 30).unwrap(), "tien uur dertig");
    assert_eq!(Afrikaans.time(1, 0).unwrap(), "een uur");
    assert_eq!(Afrikaans.cardinal(1, Gender::Feminine).unwrap(), "een");
    assert_eq!(Afrikaans.currency("€"), Some(("euro", "euro")));
    assert_eq!(Afrikaans.currency("R"), Some(("rand", "rand")));
    assert_eq!(Afrikaans.currency("¥"), None);
    let words = [
        Afrikaans.decimal_word(','),
        Afrikaans.decimal_word('.'),
        Afrikaans.range(),
        Afrikaans.ratio(),
        Afrikaans.minus(),
        Afrikaans.plus(),
        Afrikaans.percent(),
    ];
    assert_eq!(words, ["komma", "punt", "tot", "op", "minus", "plus", "persent"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Afrikaans.symbol(symbol)).count(), 9);
    let letters: String = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .filter_map(|letter| Afrikaans.letter(letter))
        .collect();
    assert!(
        letters.starts_with("aabeeseedeeeeefgeehaijotka") && letters.ends_with("iksyzet"),
        "{letters}"
    );
    assert_eq!(Afrikaans.letter('1'), None);
    assert_eq!(
        "@./:-_!"
            .chars()
            .filter_map(|sign| Afrikaans.address_sign(sign))
            .count(),
        6
    );
    assert!(is_abbreviation("Dr") && is_abbreviation("bv") && !is_abbreviation("huis"));
    assert_eq!(Afrikaans.abbreviation("ens"), Some("ensovoorts"));
    assert_eq!(Afrikaans.vowels(), "aeiouyêôûëï");
}

#[test]
fn syllables_start_as_afrikaans_allows() {
    assert!(legal_onset(&[T]) && legal_onset(&[S, T, R]) && legal_onset(&[K, W]) && legal_onset(&[K, L]));
    assert!(!legal_onset(&[N, T]) && !legal_onset(&[L, N, T, R]));
}

use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Hungarian, is_abbreviation},
    numbers::cardinal,
    phonemes::{B, K, N, R, S, T, legal_onset},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("ház", "ˈhaːz", "hˈaːz"),
        ("házak", "ˈhaːzɒk", "hˈaːzɑk"),
        ("magyar", "ˈmɒɟɒr", "mˈɑɟɑr"),
        ("szép", "ˈseːp", "sˈeːp"),
        ("köszönöm", "ˈkøsønøm", "kˈøsønøm"),
        ("kert", "ˈkɛrt", "kˈɛrt"),
        ("lány", "ˈlaːɲ", "lˈaːɲ"),
        ("fiú", "ˈfiuː", "fˈiuː"),
        ("gyerek", "ˈɟɛrɛk", "ɟˈɛrɛk"),
        ("tyúk", "ˈcuːk", "cˈuːk"),
        ("hely", "ˈhɛj", "hˈɛj"),
        ("zsák", "ˈʒaːk", "ʒˈaːk"),
        ("csak", "ˈtʃɒk", "tʃˈɑk"),
        ("madzag", "ˈmɒdzːɒɡ", "mˈɑdzːɑɡ"),
        ("brindzsa", "ˈbrindʒɒ", "brˈindʒɑ"),
        ("város", "ˈvaːroʃ", "vˈaːroʃ"),
        ("öröm", "ˈørøm", "ˈørøm"),
        ("őr", "ˈøːr", "ˈøːr"),
        ("üveg", "ˈyvɛɡ", "ˈyvɛɡ"),
        ("űr", "ˈyːr", "ˈyːr"),
        ("kettő", "ˈkɛtːøː", "kˈɛtːøː"),
        ("három", "ˈhaːrom", "hˈaːrom"),
        ("nyolc", "ˈɲolts", "ɲˈolts"),
        ("kilenc", "ˈkilɛnts", "kˈilɛnts"),
        ("húsz", "ˈhuːs", "hˈuːs"),
        ("ezer", "ˈɛzɛr", "ˈɛzɛr"),
        ("millió", "ˈmilːioː", "mˈillioː"),
        ("megint", "ˈmɛɡint", "mˈɛɡint"),
        ("szeretlek", "ˈsɛrɛtlɛk", "sˈɛrɛtlɛk"),
        ("huszonegy", "ˈhusonɛɟ", "hˈusonɛɟ"),
        ("kétszáz", "ˈkeːtsaːz", "kˈeːtsaːz"),
        ("Budapest", "ˈbudɒpɛʃt", "bˈudɑpɛʃt"),
        ("olló", "ˈolːoː", "ˈolloː"),
        ("meggy", "ˈmɛɟː", "mˈɛɟː"),
        ("itt", "ˈitː", "ˈitː"),
        ("látja", "ˈlaːcːɒ", "lˈaːcːɑ"),
        ("adja", "ˈɒɟːɒ", "ˈɑɟːɑ"),
        ("mondja", "ˈmonɟɒ", "mˈonɟɑ"),
        ("hallja", "ˈhɒjːɒ", "hˈɑjjɑ"),
        ("asszony", "ˈɒsːoɲ", "ˈɑssoɲ"),
        ("reggel", "ˈrɛɡːɛl", "rˈɛɡːɛl"),
        ("hosszú", "ˈhosːuː", "hˈossuː"),
        ("mennyi", "ˈmɛɲːi", "mˈɛɲɲi"),
        ("orr", "ˈor", "ˈor"),
        ("hattyú", "ˈhɒcːuː", "hˈɑcːuː"),
        ("meccs", "ˈmɛtʃː", "mˈɛtʃː"),
        ("egészség", "ˈɛɡeːʃeːɡ", "ˈɛɡeːʃeːɡ"),
        ("azt", "ˈɒst", "ˈɑst"),
        ("kérdezte", "ˈkeːrdɛstɛ", "kˈeːrdɛstɛ"),
        ("könyvtár", "ˈkøɲvtaːr", "kˈøɲvtaːr"),
        ("leszállt", "ˈlɛsaːlt", "lˈɛsaːlt"),
        ("utca", "ˈutsːɒ", "ˈutsːɑ"),
        ("mondta", "ˈmontːɒ", "mˈontːɑ"),
        ("xilofon", "ˈksilofon", "ksˈilofon"),
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
fn articles_are_lighter() {
    assert_eq!(read("a ház és az udvar"), ["ˌɑ", "hˈaːz", "ˌeːʃ", "ˌɑz", "ˈudvɑr"]);
    assert_eq!(read("21"), ["hˈusonɛɟ"]);
    assert_eq!(read("B"), ["bˈeː"]);
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "nulla"),
        (2, "kettő"),
        (12, "tizenkettő"),
        (21, "huszonegy"),
        (40, "negyven"),
        (100, "száz"),
        (101, "száz egy"),
        (200, "kétszáz"),
        (900, "kilencszáz"),
        (1000, "ezer"),
        (2024, "kétezer huszonnégy"),
        (1999, "ezer kilencszáz kilencvenkilenc"),
        (1_000_000, "egymillió"),
        (3_000_000, "hárommillió"),
        (2_000_000_000, "kétmilliárd"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(Hungarian.date(3, 5, 2024).unwrap(), "kétezer huszonnégy május három");
    assert_eq!(Hungarian.date(1, 13, 2024), None);
    assert_eq!(Hungarian.time(10, 30).unwrap(), "tíz harminc");
    assert_eq!(Hungarian.time(10, 0).unwrap(), "tíz óra");
    assert_eq!(Hungarian.cardinal(2, Gender::Alone).unwrap(), "kettő");
    assert_eq!(Hungarian.currency("€"), Some(("euró", "euró")));
    assert_eq!(Hungarian.currency("Ft"), Some(("forint", "forint")));
    assert_eq!(Hungarian.currency("¥"), None);
    let words = [
        Hungarian.decimal_word(','),
        Hungarian.decimal_word('.'),
        Hungarian.range(),
        Hungarian.ratio(),
        Hungarian.minus(),
        Hungarian.plus(),
        Hungarian.percent(),
    ];
    assert_eq!(words, ["egész", "pont", "tól", "per", "mínusz", "plusz", "százalék"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Hungarian.symbol(symbol)).count(), 9);
    let letters: String = "aábcdeéfghiíjklmnoóöőpqrstuúüűvwxyz"
        .chars()
        .filter_map(|letter| Hungarian.letter(letter))
        .collect();
    assert!(
        letters.starts_with("aábécédéeéefgéhá") && letters.ends_with("ikszipszilonzé"),
        "{letters}"
    );
    assert_eq!(Hungarian.letter('1'), None);
    assert_eq!(
        "@./:-_!"
            .chars()
            .filter_map(|sign| Hungarian.address_sign(sign))
            .count(),
        6
    );
    assert!(is_abbreviation("Dr") && is_abbreviation("stb") && !is_abbreviation("ház"));
    assert_eq!(Hungarian.abbreviation("pl"), Some("például"));
    assert_eq!(Hungarian.vowels(), "aáeéiíoóöőuúüű");
}

#[test]
fn syllables_start_as_hungarian_allows() {
    assert!(legal_onset(&[T]) && legal_onset(&[B, R]) && !legal_onset(&[S, T]) && !legal_onset(&[N, K]));
}

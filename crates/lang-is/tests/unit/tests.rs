use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Icelandic, is_abbreviation},
    numbers::{Form, cardinal, day_ordinal},
    phonemes::{J, K, L, N, R, S, T, TH, V, legal_onset},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("hús", "ˈhuːs", "hˈuːs"),
        ("tala", "ˈtʰaːla", "tˈaːla"),
        ("móðir", "ˈmouːðɪr", "mˈoʊːðɪr"),
        ("þjóð", "ˈθjouːð", "θjˈoʊːð"),
        ("dagar", "ˈtaːɣar", "dˈaːɣar"),
        ("konungur", "ˈkʰɔːnuŋkʏr", "kˈɔːnuŋɡyr"),
        ("vatn", "ˈvaʰtn\u{325}", "ʋˈahdhn#"),
        ("stelpa", "ˈstɛlpa", "sdˈɛlba"),
        ("nýr", "ˈniːr", "nˈiːr"),
        ("bíll", "ˈpitl\u{325}", "bˈitl#"),
        ("mikið", "ˈmɪːcɪð", "mˈɪːɟɪð"),
        ("þrír", "ˈθriːr", "θərˈiːr"),
        ("sex", "ˈsɛks", "sˈɛks"),
        ("níu", "ˈniːʏ", "nˈiːy"),
        ("tólf", "ˈtʰoul\u{325}f", "tˈoʊl#f"),
        ("hvað", "ˈkʰvaːð", "kʋˈaːð"),
        ("barn", "ˈpartn\u{325}", "bˈardn#"),
        ("gegnum", "ˈcɛknʏm", "ɟˈɛɡnym"),
        ("vetur", "ˈvɛːtʏr", "ʋˈɛːdyr"),
        ("eyja", "ˈeiːja", "ˈeɪːja"),
        ("epli", "ˈɛplɪ", "ˈɛblɪ"),
        ("skip", "ˈscɪːp", "sɟˈɪːb"),
        ("sitja", "ˈsɪːtja", "sˈɪːdja"),
        ("sagði", "ˈsaɣðɪ", "sˈaɣðɪ"),
        ("hlaupa", "ˈl\u{325}œyːpa", "l#ˈøyːba"),
        ("jökull", "ˈjœːkʏtl\u{325}", "jˈœːɡytl#"),
        ("gefa", "ˈcɛːva", "ɟˈɛːʋa"),
        ("takk", "ˈtʰaʰk", "tˈahɡ"),
        ("sól", "ˈsouːl\u{325}", "sˈoʊːl#"),
        ("bréf", "ˈprjɛːf", "bərjˈɛːf"),
        ("tafla", "ˈtʰapla", "tˈabla"),
        ("vitlaus", "ˈvɪʰtlœys", "ʋˈɪhdløys"),
        ("kalt", "ˈkʰal\u{325}t", "kˈal#d"),
        ("spurning", "ˈspʏrtn\u{325}iŋk", "sbˈyrdn#iŋɡ"),
        ("landið", "ˈlantɪð", "lˈandɪð"),
        ("sögur", "ˈsœːɣʏr", "sˈœːɣyr"),
        ("ekki", "ˈɛʰcɪ", "ˈɛhɟɪ"),
        ("fara", "ˈfaːra", "fˈaːɾra"),
        ("kennari", "ˈcʰɛnːarɪ", "cˈɛnnaɾrɪ"),
        ("segir", "ˈseiːjɪr", "sˈeɪːjɪr"),
        ("bogi", "ˈpɔijɪ", "bˈɔɪjɪ"),
        ("fersk", "ˈfɛr\u{325}sk", "fˈɛrr#sɡ"),
        ("margt", "ˈmarxt", "mˈarxd"),
        ("þess", "ˈθɛsː", "θˈɛss"),
        ("einn", "ˈeitn", "ˈeɪdn"),
        ("lampi", "ˈlam\u{325}pɪ", "lˈam#bɪ"),
        ("banki", "ˈpauɲ\u{30a}cɪ", "bˈaʊɲ#ɟɪ"),
        ("lengi", "ˈleiɲcɪ", "lˈeɪɲɟɪ"),
        ("kirkja", "ˈcʰɪr\u{325}ca", "cˈɪrr#ca"),
        ("hjálp", "ˈçaul\u{325}p", "çˈaʊl#b"),
        ("hratt", "ˈr\u{325}aʰt", "hr#ˈahd"),
        ("hnífur", "ˈn\u{325}iːvʏr", "hn#ˈiːʋyr"),
        ("kvöld", "ˈkvœlt", "ɡʋˈœld"),
        ("gjöf", "ˈcœːf", "ɟˈœːf"),
        ("mamma", "ˈmamːa", "mˈamma"),
        ("Unalāshka", "ˈʏːnalashka", "ˈyːnalashɡa"),
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
    assert_eq!(read("ég er heima"), ["jɛːx", "ɛːr", "hˈeɪːma"]);
    assert_eq!(read("þetta er ekki hús"), ["θˌɛhda", "ɛːr", "ˌɛhɟɪ", "hˈuːs"]);
    assert_eq!(read("21"), ["tˈyhdyɣˌyɔːɣˈeɪdn"]);
    assert_eq!(read("40"), ["fjˈœːɾrytˌiy"]);
    assert_eq!(read("101"), ["ˈeɪhd", "hˈyndəraðɔːɣ", "ˈeɪdn"]);
    assert_eq!(read("B"), ["bjˈɛː"]);
    assert_eq!(read("W"), ["tʋˈœːʋaldʋaf"]);
}

#[test]
fn numbers_in_words() {
    for (number, form, words) in [
        (0, Form::Masculine, "núll"),
        (1, Form::Masculine, "einn"),
        (1, Form::Feminine, "ein"),
        (2, Form::Neuter, "tvö"),
        (13, Form::Masculine, "þrettán"),
        (21, Form::Masculine, "tuttugu-og-einn"),
        (40, Form::Masculine, "fjöru-tíu"),
        (93, Form::Feminine, "níu-tíu-og-þrjár"),
        (100, Form::Masculine, "eitt hundrað"),
        (101, Form::Masculine, "eitt hundrað-og einn"),
        (125, Form::Masculine, "eitt hundrað tuttugu-og-fimm"),
        (300, Form::Masculine, "þrjú hundruð"),
        (1000, Form::Masculine, "eitt þúsund"),
        (1875, Form::Neuter, "eitt þúsund átta hundruð sjö-tíu-og-fimm"),
        (2024, Form::Neuter, "tvö þúsund tuttugu-og-fjögur"),
        (1_000_000, Form::Masculine, "ein milljón"),
        (3_000_000, Form::Masculine, "þrjár milljónir"),
    ] {
        assert_eq!(cardinal(number, form), words, "{number}");
    }
    assert_eq!(day_ordinal(1).unwrap(), "fyrsti");
    assert_eq!(day_ordinal(23).unwrap(), "tuttugasti og þriðji");
    assert_eq!(day_ordinal(30).unwrap(), "þrítugasti");
    assert_eq!(day_ordinal(31).unwrap(), "þrítugasti og fyrsti");
    assert_eq!(day_ordinal(32), None);
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(
        Icelandic.date(3, 5, 2024).unwrap(),
        "þriðji maí tvö þúsund tuttugu-og-fjögur"
    );
    assert_eq!(Icelandic.date(1, 13, 2024), None);
    assert_eq!(Icelandic.date(0, 5, 2024), None);
    assert_eq!(Icelandic.time(10, 30).unwrap(), "tíu þrjá-tíu");
    assert_eq!(Icelandic.time(1, 0).unwrap(), "klukkan eitt");
    assert_eq!(Icelandic.cardinal(2, Gender::Alone).unwrap(), "tveir");
    assert_eq!(Icelandic.cardinal(2, Gender::Feminine).unwrap(), "tvær");
    assert_eq!(Icelandic.currency("€"), Some(("evra", "evrur")));
    assert_eq!(Icelandic.currency("$"), Some(("dollari", "dollarar")));
    assert_eq!(Icelandic.currency("£"), Some(("pund", "pund")));
    assert_eq!(Icelandic.currency("kr"), Some(("króna", "krónur")));
    assert_eq!(Icelandic.currency("¥"), None);
    assert_eq!(Icelandic.gender_of("krónur"), Gender::Feminine);
    assert_eq!(Icelandic.gender_of("dollarar"), Gender::Masculine);
    let words = [
        Icelandic.decimal_word(','),
        Icelandic.decimal_word('.'),
        Icelandic.range(),
        Icelandic.ratio(),
        Icelandic.minus(),
        Icelandic.plus(),
        Icelandic.percent(),
    ];
    assert_eq!(words, ["komma", "punktur", "til", "á móti", "mínus", "plús", "prósent"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Icelandic.symbol(symbol)).count(), 9);
    let letters: Vec<&str> = "aábcdðeéfghiíjklmnoópqrstuúvwxyýzþæö"
        .chars()
        .filter_map(|letter| Icelandic.letter(letter))
        .collect();
    assert_eq!(letters.len(), 36);
    assert!(
        letters
            .iter()
            .all(|name| name.split(' ').all(|word| pronounce(word).is_some()))
    );
    assert_eq!(Icelandic.letter('1'), None);
    assert_eq!(
        "@./:-_!"
            .chars()
            .filter_map(|sign| Icelandic.address_sign(sign))
            .count(),
        6
    );
    assert!(is_abbreviation("Dr") && is_abbreviation("bls") && !is_abbreviation("hús"));
    assert_eq!(Icelandic.abbreviation("kl"), Some("klukkan"));
    assert_eq!(Icelandic.vowels(), "aáeéiíoóuúyýæö");
}

#[test]
fn syllables_start_as_icelandic_allows() {
    assert!(legal_onset(&[T]) && legal_onset(&[T, R]) && legal_onset(&[TH, J]) && legal_onset(&[K, V]));
    assert!(legal_onset(&[S, T]) && legal_onset(&[S, T, R]) && legal_onset(&[N, J]));
    assert!(!legal_onset(&[N, T]) && !legal_onset(&[L, S]) && !legal_onset(&[S, T, L]));
    assert!(!legal_onset(&[S, K, R, J]) && !legal_onset(&[R, L]));
}

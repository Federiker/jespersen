use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{German, is_abbreviation},
    numbers::{Form, cardinal},
    phonemes::{J, K, L, N, R, SH, T, TS, V, legal_onset},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("Haus", "ˈhaʊ̯s", "hˈaʊs"),
        ("Häuser", "ˈhɔʏ̯zɐ", "hˈɔøzɜ"),
        ("Schule", "ˈʃuːlə", "ʃˈuːlə"),
        ("Straße", "ˈʃtʁaːsə", "ʃtɾˈɑːsə"),
        ("sprechen", "ˈʃpʁɛçən", "ʃpɾˈɛçən"),
        ("ach", "ˈax", "ˈax"),
        ("möchte", "ˈmœçtə", "mˈœçtə"),
        ("Kinder", "ˈkɪndɐ", "kˈɪndɜ"),
        ("Vater", "ˈfaːtɐ", "fˈɑːtɜ"),
        ("Mutter", "ˈmʊtɐ", "mˈʊtɜ"),
        ("beginnen", "bəˈɡɪnən", "bəɡˈɪnən"),
        ("verstehen", "fɛɐ̯ˈʃteːən", "fɛɾʃtˈeːən"),
        ("Geburtstag", "ɡəˈbʊɐ̯tstaːk", "ɡəbˈʊɐtstɑːk"),
        ("Nation", "naˈtsjoːn", "natsjˈoːn"),
        ("Universität", "uːniːvɛɐ̯ziːˈtɛːt", "ˌuːniːvˌɛɾziːtˈɛːt"),
        ("studieren", "ʃtuːˈdiːʁən", "ʃtuːdˈiːrən"),
        ("Bäckerei", "bɛkeːˈʁaɪ̯", "bˌɛkeːrˈaɪ"),
        ("Philosophie", "fiːloːzoːˈfiː", "fˌiːloːzoːfˈiː"),
        ("Tag", "ˈtaːk", "tˈɑːk"),
        ("Tage", "ˈtaːɡə", "tˈɑːɡə"),
        ("und", "ˈʊnt", "ˈʊnt"),
        ("zwanzig", "ˈtsvantsɪç", "tsvˈantsɪç"),
        ("König", "ˈkøːnɪç", "kˈøːnɪç"),
        ("ruhigen", "ˈʁuːɪɡən", "rˈuːɪɡən"),
        ("Jahr", "ˈjaːɐ̯", "jˈɑːɾ"),
        ("sehen", "ˈzeːən", "zˈeːən"),
        ("Sohn", "ˈzoːn", "zˈoːn"),
        ("Stadt", "ˈʃtat", "ʃtˈat"),
        ("Quelle", "ˈkvɛlə", "kvˈɛlə"),
        ("Vogel", "ˈfoːɡəl", "fˈoːɡəl"),
        ("Wasser", "ˈvasɐ", "vˈasɜ"),
        ("Zeit", "ˈtsaɪ̯t", "tsˈaɪt"),
        ("Junge", "ˈjʊŋə", "jˈʊŋə"),
        ("Garten", "ˈɡaɐ̯tən", "ɡˈaɾtən"),
        ("Arbeit", "ˈaɐ̯baɪ̯t", "ˈaɾbaɪt"),
        ("Freund", "ˈfʁɔʏ̯nt", "frˈɔønt"),
        ("sechs", "ˈzɛks", "zˈɛks"),
        ("endlich", "ˈɛntlɪç", "ˈɛntlɪç"),
        ("gleich", "ˈɡlaɪ̯ç", "ɡlˈaɪç"),
        ("übrig", "ˈyːbʁɪç", "ˈyːbɾɪç"),
        ("Absicht", "ˈapˌzɪçt", "ˈapzˌɪçt"),
        ("aufstehen", "ˈaʊ̯fˌʃteːən", "ˈaʊfʃtˌeːən"),
        ("Vorzimmer", "ˈfoːɐ̯ˌtsɪmɐ", "fˈoːɾtsˌɪmɜ"),
        ("daran", "daːˈʁan", "dɑːrˈan"),
        ("davon", "daːˈfɔn", "dɑːfˈɔn"),
        ("Familie", "faˈmiːlɪə", "famˈiːlɪə"),
        ("langsam", "ˈlaŋzaːm", "lˈaŋzɑːm"),
        ("lebt", "ˈleːpt", "lˈeːpt"),
        ("Dschungel", "ˈdʒʊŋəl", "dʒˈʊŋəl"),
        ("Patient", "paˈtsjɛnt", "patsjˈɛnt"),
        ("Chor", "ˈkoːɐ̯", "kˈoːɾ"),
        ("wurde", "ˈvʊɐ̯də", "vˈʊɐdə"),
        ("müssen", "ˈmʏsən", "mˈʏsən"),
        ("fünfzehn", "ˈfʏnftseːn", "fˈʏnftseːn"),
        ("Qualität", "kvaliːˈtɛːt", "kvˌaliːtˈɛːt"),
        ("theáter", "teːˈaːtɐ", "teːˈɑːtɜ"),
        ("gében", "ˈɡeːbən", "ɡˈeːbən"),
        ("Beinchen", "ˈbaɪ̯nçən", "bˈaɪnçən"),
        ("Photo", "ˈfoːtoː", "fˈoːtoː"),
        ("früh", "ˈfʁyː", "frˈyː"),
        ("deutsch", "ˈdɔʏ̯tʃ", "dˈɔøtʃ"),
        ("Pflaume", "ˈpflaʊ̯mə", "pflˈaʊmə"),
        ("Axt", "ˈakst", "ˈakst"),
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
    assert_eq!(read("der Mann und die Frau"), ["dɛɾ", "mˈan", "ʊnt", "diː", "frˈaʊ"]);
    assert_eq!(read("er hat es gesehen"), ["ɛɾ", "hat", "ɛs", "ɡəzˈeːən"]);
    assert_eq!(read("eine Erde"), ["ˌaɪnə", "ˈeːɾdə"]);
    assert_eq!(read("Theater"), ["teːˈɑːtɜ"]);
    assert_eq!(read("21"), ["ˈaɪn", "ʊnttsvˈantsɪç"]);
    assert_eq!(read("101"), ["ˈaɪnhˈʊndɜt", "ˈaɪns"]);
    assert_eq!(read("B"), ["bˈeː"]);
}

#[test]
fn numbers_in_words() {
    for (number, form, words) in [
        (0, Form::Alone, "null"),
        (1, Form::Alone, "eins"),
        (1, Form::Before, "ein"),
        (1, Form::Feminine, "eine"),
        (12, Form::Alone, "zwölf"),
        (17, Form::Alone, "siebzehn"),
        (21, Form::Alone, "ein- und-zwanzig"),
        (24, Form::Alone, "vier und-zwanzig"),
        (30, Form::Alone, "dreißig"),
        (100, Form::Alone, "ein-hundert"),
        (300, Form::Alone, "drei-hundert"),
        (1000, Form::Alone, "ein- tausend"),
        (2024, Form::Alone, "zwei tausend vier und-zwanzig"),
        (1_000_000, Form::Alone, "eine Million"),
        (3_000_000, Form::Alone, "drei Millionen"),
        (2_000_000_000, Form::Alone, "zwei Milliarden"),
        (1_000_000_000_000, Form::Alone, "eine Billion"),
    ] {
        assert_eq!(cardinal(number, form), words, "{number}");
    }
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(
        German.date(3, 5, 2024).unwrap(),
        "dritter Mai zwei tausend vier und-zwanzig"
    );
    assert_eq!(German.date(32, 5, 2024), None);
    assert_eq!(German.date(1, 13, 2024), None);
    assert_eq!(German.time(10, 30).unwrap(), "zehn Uhr dreißig");
    assert_eq!(German.time(1, 0).unwrap(), "ein Uhr");
    assert_eq!(German.cardinal(1, Gender::Feminine).unwrap(), "eine");
    assert_eq!(German.cardinal(1, Gender::Masculine).unwrap(), "ein");
    assert_eq!(German.currency("€"), Some(("Euro", "Euro")));
    assert_eq!(German.currency("CHF"), Some(("Franken", "Franken")));
    assert_eq!(German.currency("¥"), None);
    let words = [
        German.decimal_word(','),
        German.decimal_word('.'),
        German.range(),
        German.ratio(),
        German.minus(),
        German.plus(),
        German.percent(),
    ];
    assert_eq!(words, ["Komma", "Punkt", "bis", "durch", "minus", "plus", "Prozent"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§", "¶"];
    assert_eq!(symbols.iter().filter_map(|symbol| German.symbol(symbol)).count(), 10);
    let letters: String = "aäbcdefghijklmnoöpqrsßtuüvwxyz"
        .chars()
        .filter_map(|letter| German.letter(letter))
        .collect();
    assert!(
        letters.starts_with("aäbecedeeefgehaijottka") && letters.ends_with("ixypsilonzett"),
        "{letters}"
    );
    assert_eq!(German.letter('1'), None);
    assert_eq!(
        "@./:-_!".chars().filter_map(|sign| German.address_sign(sign)).count(),
        6
    );
    assert!(is_abbreviation("Dr") && is_abbreviation("usw") && !is_abbreviation("Haus"));
    assert_eq!(German.abbreviation("bzw"), Some("beziehungsweise"));
    assert_eq!(German.gender_of("Stunde"), Gender::Feminine);
    assert_eq!(German.gender_of("Euro"), Gender::Masculine);
    assert_eq!(German.vowels(), "aäeioöuüy");
}

#[test]
fn syllables_start_as_german_allows() {
    assert!(legal_onset(&[T]) && legal_onset(&[SH, T, R]) && legal_onset(&[K, V]) && legal_onset(&[TS, J]));
    assert!(legal_onset(&[SH, L]) && !legal_onset(&[N, T]) && !legal_onset(&[L, N, T, R]));
}

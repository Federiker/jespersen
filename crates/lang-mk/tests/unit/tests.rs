use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Macedonian, is_abbreviation},
    numbers::{Agreement, cardinal, day},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("вода", "ˈvɔda", "vˈɔda"),
        ("планина", "ˈpɫanina", "plˈanina"),
        ("учителот", "uˈtʃitɛɫɔt", "utʃˈitɛlɔt"),
        ("Македонија", "makɛˈdɔnija", "makɛdˈɔnija"),
        ("телефон", "ˈtɛlɛfɔn", "tˈɛlɛfɔn"),
        ("град", "ˈɡrat", "ɡrˈat"),
        ("нож", "ˈnɔʃ", "nˈɔʃ"),
        ("љубов", "ˈʎubɔf", "ʎˈubɔf"),
        ("сватба", "ˈsvadba", "svˈadba"),
        ("прст", "ˈpr̩st", "pˈrst"),
        ("'рѓа", "ˈr̩ɟa", "ˈrɟa"),
        ("банка", "ˈbaŋka", "bˈaŋka"),
        ("ѕвезда", "ˈdzvɛzda", "dzvˈɛzda"),
        ("ќе", "ˈcɛ", "cˈɛ"),
        ("њива", "ˈɲiva", "ɲˈiva"),
        ("џеб", "ˈdʒɛp", "dʒˈɛp"),
        ("леб", "ˈlɛp", "lˈɛp"),
        ("хотел", "ˈxɔtɛɫ", "xˈɔtɛl"),
        ("цреша", "ˈtsrɛʃa", "tsrˈɛʃa"),
        ("чаша", "ˈtʃaʃa", "tʃˈaʃa"),
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
    assert!(pronounce("'").is_none());
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
fn clitics_join_the_accentual_word() {
    assert_eq!(sentence("не знам"), "nˈɛ znam");
    assert_eq!(sentence("јас не знам"), "jˈas nˈɛ znam");
    assert_eq!(sentence("не разбирам"), "nɛ rˈazbiram");
    assert_eq!(sentence("ќе дојдам"), "cˈɛ dɔjdam");
    assert_eq!(sentence("не ми се спие"), "nɛ mi sˈɛ spiɛ");
    assert_eq!(sentence("тој живее во Скопје"), "tˈɔj ʒˈivɛɛ vɔ skˈɔpjɛ");
    assert_eq!(sentence("дај ми го"), "dˈaj mi ɡɔ");
    assert_eq!(sentence("го"), "ɡˈɔ");
    assert_eq!(sentence("Б"), "bˈɛ");
    assert_eq!(sentence("д-р Петровски"), "dˈɔktɔr pˈɛtrɔfski");
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "нула"),
        (15, "петнаесет"),
        (25, "дваесет и пет"),
        (105, "сто и пет"),
        (120, "сто и дваесет"),
        (1000, "илјада"),
        (1975, "илјада деветстотини седумдесет и пет"),
        (2024, "две илјади дваесет и четири"),
        (25_000, "дваесет и пет илјади"),
        (2_000_000, "два милиони"),
        (1_000_000_000, "милијарда"),
    ] {
        assert_eq!(cardinal(number, Agreement::Masculine), words, "{number}");
    }
    assert_eq!(cardinal(2, Agreement::Feminine), "две");
    assert_eq!(cardinal(21, Agreement::Feminine), "дваесет и една");
    assert_eq!(day(3).unwrap(), "трети");
    assert_eq!(day(20).unwrap(), "дваесетти");
    assert_eq!(day(21).unwrap(), "дваесет и први");
    assert_eq!(day(32), None);
}

#[test]
fn the_locale_reads_signs_times_dates_and_money() {
    assert_eq!(
        Macedonian.date(3, 5, 2024).unwrap(),
        "трети мај две илјади дваесет и четири"
    );
    assert_eq!(Macedonian.date(1, 13, 2024), None);
    assert_eq!(Macedonian.time(10, 30).unwrap(), "десет и триесет");
    assert_eq!(Macedonian.time(8, 0).unwrap(), "осум часот");
    assert_eq!(Macedonian.cardinal(1, Macedonian.gender_of("книга")).unwrap(), "една");
    assert_eq!(Macedonian.cardinal(1, Macedonian.gender_of("град")).unwrap(), "еден");
    assert_eq!(Macedonian.cardinal(2, Gender::Alone).unwrap(), "два");
    assert_eq!(Macedonian.currency("ден"), Some(("денар", "денари")));
    assert_eq!(Macedonian.currency("€"), Some(("евро", "евра")));
    assert_eq!(Macedonian.currency("¥"), None);
    let words = [
        Macedonian.decimal_word(','),
        Macedonian.decimal_word('.'),
        Macedonian.range(),
        Macedonian.ratio(),
        Macedonian.minus(),
        Macedonian.plus(),
        Macedonian.percent(),
    ];
    assert_eq!(words, ["запирка", "точка", "до", "на", "минус", "плус", "проценти"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Macedonian.symbol(symbol)).count(), 9);
    let cyrillic = "абвгдѓежзѕијклљмнњопрстќуфхцчџш";
    assert_eq!(
        cyrillic.chars().filter_map(|letter| Macedonian.letter(letter)).count(),
        31
    );
    let latin = "abcdefghijklmnopqrstuvwxyz";
    assert_eq!(latin.chars().filter_map(|letter| Macedonian.letter(letter)).count(), 26);
    assert_eq!(Macedonian.letter('1'), None);
    assert_eq!(
        "@./:-_!"
            .chars()
            .filter_map(|sign| Macedonian.address_sign(sign))
            .count(),
        6
    );
    assert!(is_abbreviation("Проф") && is_abbreviation("итн") && !is_abbreviation("куќа"));
    assert_eq!(Macedonian.abbreviation("бр"), Some("број"));
    assert_eq!(Macedonian.vowels(), "аеиоу");
}

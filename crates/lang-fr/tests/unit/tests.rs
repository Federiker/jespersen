use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{French, is_abbreviation},
    numbers::{cardinal, ordinal},
    sentence::{read_tokens, say},
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("chat", "ˈʃa", "ʃˈa"),
        ("petit", "pəˈti", "pətˈi"),
        ("fils", "ˈfis", "fˈis"),
        ("huit", "ˈɥit", "yˈit"),
        ("vingt", "ˈvɛ̃", "vˈɛ̃"),
        ("femme", "ˈfam", "fˈam"),
        ("ville", "ˈvil", "vˈil"),
        ("famille", "faˈmij", "famˈij"),
        ("travail", "tʁaˈvaj", "tʁavˈaj"),
        ("oeil", "ˈœj", "ˈœj"),
        ("pays", "pɛˈi", "pɛˈi"),
        ("album", "alˈbɔm", "albˈɔm"),
        ("bonne", "ˈbɔn", "bˈɔn"),
        ("ennui", "ɑ̃ˈnɥi", "ɑ̃nyˈi"),
        ("hiver", "iˈvɛʁ", "ivˈɛʁ"),
        ("samedi", "samˈdi", "samdˈi"),
        ("portefeuille", "pɔʁtəˈfœj", "pɔʁtəfˈœj"),
        ("parlent", "ˈpaʁl", "pˈaʁl"),
        ("souvent", "suˈvɑ̃", "suvˈɑ̃"),
        ("ensemble", "ɑ̃ˈsɑ̃bl", "ɑ̃sˈɑ̃bl"),
        ("nation", "naˈsjɔ̃", "nasjˈɔ̃"),
        ("question", "kɛsˈtjɔ̃", "kɛstjˈɔ̃"),
        ("bibliothèque", "biblioˈtɛk", "bibliotˈɛk"),
        ("âne", "ˈɑn", "ˈaːn"),
        ("Europe", "øˈʁɔp", "øʁˈɔp"),
        ("heure", "ˈœʁ", "ˈœʁ"),
        ("heureuse", "øˈʁøz", "øʁˈøz"),
        ("chrétien", "kʁeˈtjɛ̃", "kʁetjˈɛ̃"),
        ("vient", "ˈvjɛ̃", "vjˈɛ̃"),
        ("les", "ˈle", "lˈe-"),
        ("examen", "ɛɡzaˈmɛ̃", "ɛɡzamˈɛ̃"),
    ];
    let wrong: Vec<String> = cases
        .iter()
        .filter_map(|(word, ipa, espeak)| {
            let said = say(word).unwrap();
            let (got_ipa, got_espeak) = (said.to_notation(Notation::Ipa), said.to_notation(Notation::Espeak));
            (got_ipa != *ipa || got_espeak != *espeak).then(|| format!("{word}: {got_ipa} {got_espeak}"))
        })
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
    assert!(say("ŋ").is_none());
}

/// The words of a sentence (tokens split at spaces, punctuation marked) in eSpeak NG's notation.
fn sentence(text: &str) -> String {
    let tokens: Vec<(&str, TokenKind)> = text
        .split(' ')
        .map(|token| {
            let kind = match token.chars().next() {
                Some(c) if c.is_ascii_digit() => TokenKind::Number,
                Some(c) if c.is_alphabetic() => TokenKind::Word,
                _ => TokenKind::Punctuation,
            };
            (token, kind)
        })
        .collect();
    read_tokens(&tokens)
        .into_iter()
        .flatten()
        .flat_map(|reading| reading.words)
        .map(|said| said.to_notation(Notation::Espeak))
        .filter(|written| !written.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn liaison_elision_and_function_words() {
    assert_eq!(sentence("les amis"), "le-z amˈi");
    assert_eq!(sentence("les héros"), "le- eʁˈo");
    assert_eq!(sentence("un homme"), "œ̃n ˈɔm");
    assert_eq!(sentence("petit ami"), "pətˈit amˈi");
    assert_eq!(sentence("le chat est là"), "lə- ʃˈa ɛ lˈa");
    assert_eq!(sentence("trop ému"), "tʁop emˈy");
    assert_eq!(sentence("et il vient"), "e il vjˈɛ̃");
    assert_eq!(sentence("six enfants"), "sˈiz ɑ̃fˈɑ̃");
    assert_eq!(sentence("six , dix"), "sˈis dˈis");
    assert_eq!(sentence("l'école"), "lekˈɔl");
    assert_eq!(sentence("jusqu'à Lyon"), "ʒyska ljˈɔ̃");
    assert_eq!(sentence("aujourd'hui"), "oʒuʁdyˈi");
    assert_eq!(sentence("toujours là"), "tuʒˌuʁ lˈa");
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "zéro"),
        (17, "dix-sept"),
        (21, "vinte et un"),
        (22, "vinte-deux"),
        (71, "soixante et onze"),
        (75, "soixante-quinze"),
        (80, "quatre-vingts"),
        (91, "quatre-vingt-onze"),
        (100, "cent"),
        (101, "cent un"),
        (200, "deux cents"),
        (201, "deux cent un"),
        (1000, "mille"),
        (2024, "deux mille vinte-quatre"),
        (200_000, "deux cent mille"),
        (1_000_000, "un million"),
        (3_000_000_000, "trois milliards"),
    ] {
        assert_eq!(cardinal(number, false), words, "{number}");
    }
    assert_eq!(cardinal(21, true), "vinte et une");
    assert_eq!(ordinal(1, false).unwrap(), "premier");
    assert_eq!(ordinal(1, true).unwrap(), "première");
    assert_eq!(ordinal(5, false).unwrap(), "cinquième");
    assert_eq!(ordinal(9, false).unwrap(), "neuvième");
    assert_eq!(ordinal(4, false).unwrap(), "quatrième");
    assert_eq!(ordinal(80, false).unwrap(), "quatre-vingtième");
    assert_eq!(ordinal(200, false).unwrap(), "deux centième");
    assert_eq!(ordinal(0, false), None);
}

#[test]
fn the_locale_reads_signs_times_dates_and_kings() {
    assert_eq!(French.time(10, 30).unwrap(), "dix heures trente");
    assert_eq!(French.time(1, 0).unwrap(), "une heure");
    assert_eq!(French.date(1, 5, 2024).unwrap(), "premier mai deux mille vinte-quatre");
    assert_eq!(French.date(3, 5, 2024).unwrap(), "trois mai deux mille vinte-quatre");
    assert_eq!(French.date(1, 13, 2024), None);
    assert_eq!(French.cardinal(1, French.gender_of("heures")).unwrap(), "une");
    assert_eq!(French.gender_of("livres"), Gender::Masculine);
    assert_eq!(French.ordinal(1, Gender::Feminine).unwrap(), "première");
    assert_eq!(French.roman(14, Some("Louis"), None).unwrap(), "quatorze");
    assert_eq!(French.roman(1, Some("François"), None).unwrap(), "premier");
    assert_eq!(French.roman(5, Some("le"), None), None);
    let words = [
        French.decimal_word(','),
        French.decimal_word('.'),
        French.range(),
        French.ratio(),
        French.minus(),
        French.plus(),
        French.percent(),
    ];
    assert_eq!(words, ["virgule", "point", "à", "sur", "moins", "plus", "pour cent"]);
    for sign in ["€", "$", "£", "CHF"] {
        assert!(French.currency(sign).is_some(), "{sign}");
    }
    assert_eq!(French.currency("¥"), None);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| French.symbol(symbol)).count(), 9);
    let letters: String = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .filter_map(|letter| French.letter(letter))
        .collect();
    assert!(
        letters.starts_with("abécédéeuèffegéache") && letters.ends_with("ixeigreczède"),
        "{letters}"
    );
    assert_eq!(French.letter('1'), None);
    assert_eq!(
        "@./:-_!".chars().filter_map(|sign| French.address_sign(sign)).count(),
        6
    );
    assert!(is_abbreviation("M") && is_abbreviation("Mme") && is_abbreviation("etc") && !is_abbreviation("m"));
    assert_eq!(French.abbreviation("Av"), Some("avenue"));
    assert!(French.elides());
    assert!(French.vowels().contains('é'));
    assert_eq!(French.ordinal_signs().len(), 5);
}

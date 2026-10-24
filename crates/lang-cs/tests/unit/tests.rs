use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Czech, is_abbreviation},
    numbers::{Agreement, Case, cardinal, ordinal},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("děti", "ˈɟɛcɪ", "ɟˈeci"),
        ("mě", "ˈmɲɛ", "mɲˈe"),
        ("pět", "ˈpjɛt", "pjˈet"),
        ("tři", "ˈtr̝̊ɪ", "tr̝̊ˈi"),
        ("hřib", "ˈɦr̝ɪp", "hr̝ˈip"),
        ("Bůh", "ˈbuːx", "bˈuːx"),
        ("kůň", "ˈkuːɲ", "kˈuːɲ"),
        ("krk", "ˈkr̩k", "kˈr̩k"),
        ("vlk", "ˈvl̩k", "vˈl̩k"),
        ("bratr", "ˈbratr̩", "brˈatr̩"),
        ("všechno", "ˈfʃɛxno", "fʃˈexno"),
        ("kdo", "ˈɡdo", "kdˈo"),
        ("shoda", "ˈzɦoda", "shˈoda"),
        ("maminka", "ˈmamɪŋka", "mˈamiŋka"),
        ("odejdeš", "ˈodɛjdɛʃ", "ˈodeɪdeʃ"),
        ("historie", "ˈɦɪstorɪjɛ", "hˈistorˌije"),
        ("mrkev", "ˈmr̩kɛf", "mˈr̩kev"),
        ("kancelář", "ˈkantsɛlaːr̝̊", "kˈantselaːr̝"),
        ("republika", "ˈrɛpublɪka", "rˈepublˌika"),
        ("Sulla", "ˈsula", "sˈula"),
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

/// The words of a sentence (tokens split at spaces, punctuation marked) in `notation`.
fn sentence(text: &str, notation: Notation) -> String {
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
        .map(|said| said.to_notation(notation))
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn prepositions_voicing_and_function_words() {
    // A preposition joins a word of up to three syllables, and takes its stress.
    assert_eq!(sentence("na stole je kniha", Notation::Espeak), "nˈastole je kɲˈiha");
    assert_eq!(sentence("na zahradě", Notation::Espeak), "nˈazahrˌaɟe");
    assert_eq!(sentence("na konferenci", Notation::Espeak), "na kˈonferˌentsi");
    // A preposition of one consonant: plain IPA voices it by the next word, eSpeak NG keeps it.
    assert_eq!(sentence("v pokoji", Notation::Ipa), "f ˈpokojɪ");
    assert_eq!(sentence("v pokoji", Notation::Espeak), "v pˈokoji");
    assert_eq!(sentence("z domu", Notation::Ipa), "z ˈdomu");
    assert_eq!(sentence("z domu", Notation::Espeak), "s dˈomu");
    assert_eq!(sentence("s bratrem", Notation::Ipa), "z ˈbratrɛm");
    assert_eq!(sentence("k domu", Notation::Ipa), "ɡ ˈdomu");
    // Voicing across words.
    assert_eq!(sentence("než odejdeš", Notation::Espeak), "nˈeʒ ˈodeɪdeʃ");
    assert_eq!(sentence("než odejdeš", Notation::Ipa), "ˈnɛʃ ˈodɛjdɛʃ");
    assert_eq!(sentence("kost bolí", Notation::Espeak), "kˈozd bˈoliː");
    assert_eq!(sentence("tak dobře", Notation::Espeak), "tˈak dˈobr̝e");
    assert_eq!(sentence("hrad stojí", Notation::Espeak), "hrˈat stˈojiː");
    // Lighter and unstressed words, and a letter on its own.
    assert_eq!(sentence("byli tam", Notation::Espeak), "bˌili tˈam");
    assert_eq!(sentence("R", Notation::Espeak), "ˈer");
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "nula"),
        (1, "jedna"),
        (15, "patnáct"),
        (21, "dvacet jedna"),
        (200, "dvě stě"),
        (345, "tři sta čtyřicet pět"),
        (1000, "tisíc"),
        (2024, "dva tisíce dvacet čtyři"),
        (5000, "pět tisíc"),
        (2_000_000, "dva miliony"),
        (2_000_000_000, "dvě miliardy"),
    ] {
        assert_eq!(cardinal(number, Agreement::Counting), words, "{number}");
    }
    assert_eq!(cardinal(1, Agreement::Masculine), "jeden");
    assert_eq!(cardinal(22, Agreement::Feminine), "dvacet dvě");
    assert_eq!(ordinal(3, Case::Genitive).unwrap(), "třetího");
    assert_eq!(ordinal(21, Case::Genitive).unwrap(), "dvacátého prvního");
    assert_eq!(ordinal(20, Case::Neuter).unwrap(), "dvacáté");
    assert_eq!(ordinal(4, Case::Masculine).unwrap(), "čtvrtý");
    assert_eq!(ordinal(100, Case::Masculine), None);
}

#[test]
fn the_locale_reads_signs_dates_money_and_centuries() {
    assert_eq!(
        Czech.date(3, 5, 2024).unwrap(),
        "třetího května dva tisíce dvacet čtyři"
    );
    assert_eq!(Czech.date(1, 13, 2024), None);
    assert_eq!(Czech.cardinal(1, Czech.gender_of("koruna")).unwrap(), "jedna");
    assert_eq!(Czech.cardinal(1, Czech.gender_of("dům")).unwrap(), "jeden");
    assert_eq!(Czech.cardinal(1, Gender::Alone).unwrap(), "jedna");
    for (count, name) in [
        (Some(1), "koruna"),
        (Some(3), "koruny"),
        (Some(5), "korun"),
        (Some(21), "korun"),
        (None, "korun"),
    ] {
        assert_eq!(Czech.currency_name("Kč", count), Some(name), "{count:?}");
    }
    assert_eq!(Czech.currency("$"), Some(("dolar", "dolarů")));
    assert_eq!(Czech.currency("€"), Some(("euro", "eur")));
    assert_eq!(Czech.currency("£"), Some(("libra", "liber")));
    assert_eq!(Czech.currency("¥"), None);
    assert_eq!(Czech.roman(20, None, Some("století")).unwrap(), "dvacáté");
    assert_eq!(Czech.roman(4, Some("Karel"), None).unwrap(), "čtvrtý");
    assert_eq!(Czech.roman(4, Some("a"), None), None);
    let words = [
        Czech.decimal_word(','),
        Czech.decimal_word('.'),
        Czech.range(),
        Czech.ratio(),
        Czech.minus(),
        Czech.plus(),
        Czech.percent(),
    ];
    assert_eq!(words, ["celá", "tečka", "až", "lomeno", "minus", "plus", "procent"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Czech.symbol(symbol)).count(), 9);
    let letters: String = "aábcčdďeéfghiíjklmnňoópqrřsštťuúvwxzž"
        .chars()
        .filter_map(|letter| Czech.letter(letter))
        .collect();
    assert!(
        letters.starts_with("aábécéčédéďéédlouhé é") && letters.ends_with("zetžet"),
        "{letters}"
    );
    assert_eq!(Czech.letter('1'), None);
    assert_eq!("@./:-_!".chars().filter_map(|sign| Czech.address_sign(sign)).count(), 6);
    assert!(is_abbreviation("Např") && is_abbreviation("atd") && !is_abbreviation("dům"));
    assert_eq!(Czech.abbreviation("ul"), Some("ulice"));
    assert!(Czech.vowels().contains('ů'));
}

use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Slovak, is_abbreviation},
    numbers::{Agreement, Case, cardinal, ordinal},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("deti", "ˈɟeci", "dʲˈetʲi"),
        ("ľudia", "ˈʎuɟi̯a", "ʎˈudʲia"),
        ("päť", "ˈpæc", "pˈetʲ"),
        ("môj", "ˈmu̯oj", "mˈuoj"),
        ("pôjdeme", "ˈpu̯ojɟeme", "pˈuojdʲˌeme"),
        ("piesok", "ˈpi̯esok", "piˈesok"),
        ("vĺča", "ˈvl̩ːtʃa", "vˈl̩ːtʃa"),
        ("hŕba", "ˈɦr̩ːba", "hˈr̩ːba"),
        ("chlieb", "ˈxli̯ep", "xliˈep"),
        ("dážď", "ˈdaːʃc", "dˈaːʃtʲ"),
        ("domov", "ˈdomou̯", "dˈomow"),
        ("pravda", "ˈprau̯da", "prˈawda"),
        ("hlavná", "ˈɦlavnaː", "hlˈavnaː"),
        ("ľahko", "ˈʎaxko", "ʎˈahko"),
        ("červené", "ˈtʃerveneː", "tʃˈerveneː"),
        ("strednej", "ˈstrednej", "strˈednej"),
        ("tento", "ˈtento", "tˈento"),
        ("polievku", "ˈpoli̯eu̯ku", "pˈoliˌewku"),
        ("lietadlo", "ˈli̯etadlo", "liˈetadlo"),
        ("stretnutie", "ˈstretnuci̯e", "strˈetnutʲie"),
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
    assert_eq!(sentence("na stole je kniha", Notation::Espeak), "nˈastole je kɲˈiha");
    assert_eq!(sentence("na večeru", Notation::Espeak), "nˈavetʃˌeru");
    assert_eq!(sentence("na konferencii", Notation::Espeak), "na kˈonferˌentsii");
    assert_eq!(sentence("do lesa", Notation::Espeak), "dˈo lˈesa");
    // A preposition of one consonant: plain IPA voices it by the next word, eSpeak NG keeps it.
    assert_eq!(sentence("v meste", Notation::Ipa), "v ˈmesce");
    assert_eq!(sentence("v pokoji", Notation::Ipa), "f ˈpokoji");
    assert_eq!(sentence("v pokoji", Notation::Espeak), "v pˈokoji");
    assert_eq!(sentence("z poľa", Notation::Ipa), "s ˈpoʎa");
    assert_eq!(sentence("s bratom", Notation::Ipa), "z ˈbratom");
    assert_eq!(sentence("k domu", Notation::Ipa), "ɡ ˈdomu");
    // Voicing across words: before a vowel or a sonorant too in plain IPA.
    assert_eq!(sentence("otec ide", Notation::Ipa), "ˈocedz ˈiɟe");
    assert_eq!(sentence("otec ide", Notation::Espeak), "ˈotʲets ˈidʲe");
    assert_eq!(sentence("dub rastie", Notation::Espeak), "dˈub rˈastʲie");
    assert_eq!(sentence("keď odídeš", Notation::Espeak), "kˈedʲ ˈodʲiːdʲeʃ");
    assert_eq!(sentence("ako sa máš", Notation::Espeak), "ˌako sa mˈaːʃ");
    assert_eq!(sentence("R", Notation::Espeak), "ˈer");
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "nula"),
        (1, "jeden"),
        (15, "pätnásť"),
        (21, "dvadsať jeden"),
        (200, "dvesto"),
        (345, "tristo štyridsať päť"),
        (1000, "tisíc"),
        (2024, "dve tisíc dvadsať štyri"),
        (2_000_000, "dva milióny"),
        (5_000_000_000, "päť miliárd"),
    ] {
        assert_eq!(cardinal(number, Agreement::Counting), words, "{number}");
    }
    assert_eq!(cardinal(1, Agreement::Feminine), "jedna");
    assert_eq!(cardinal(2, Agreement::Masculine), "dva");
    assert_eq!(ordinal(3, Case::Genitive).unwrap(), "tretieho");
    assert_eq!(ordinal(5, Case::Genitive).unwrap(), "piateho");
    assert_eq!(ordinal(21, Case::Genitive).unwrap(), "dvadsiateho prvého");
    assert_eq!(ordinal(20, Case::Neuter).unwrap(), "dvadsiate");
    assert_eq!(ordinal(4, Case::Masculine).unwrap(), "štvrtý");
    assert_eq!(ordinal(3, Case::Neuter).unwrap(), "tretie");
    assert_eq!(ordinal(100, Case::Masculine), None);
}

#[test]
fn the_locale_reads_signs_dates_money_and_centuries() {
    assert_eq!(
        Slovak.date(3, 5, 2024).unwrap(),
        "tretieho mája dve tisíc dvadsať štyri"
    );
    assert_eq!(Slovak.date(1, 13, 2024), None);
    assert_eq!(Slovak.cardinal(1, Slovak.gender_of("koruna")).unwrap(), "jedna");
    assert_eq!(Slovak.cardinal(1, Gender::Masculine).unwrap(), "jeden");
    for (count, name) in [
        (Some(1), "euro"),
        (Some(3), "eurá"),
        (Some(5), "eur"),
        (Some(22), "eur"),
        (None, "eur"),
    ] {
        assert_eq!(Slovak.currency_name("€", count), Some(name), "{count:?}");
    }
    assert_eq!(Slovak.currency("$"), Some(("dolár", "dolárov")));
    assert_eq!(Slovak.currency("Kč"), Some(("koruna", "korún")));
    assert_eq!(Slovak.currency("£"), Some(("libra", "libier")));
    assert_eq!(Slovak.currency("¥"), None);
    assert_eq!(Slovak.roman(20, None, Some("storočie")).unwrap(), "dvadsiate");
    assert_eq!(Slovak.roman(4, Some("Karol"), None).unwrap(), "štvrtý");
    assert_eq!(Slovak.roman(4, Some("a"), None), None);
    let words = [
        Slovak.decimal_word(','),
        Slovak.decimal_word('.'),
        Slovak.range(),
        Slovak.ratio(),
        Slovak.minus(),
        Slovak.plus(),
        Slovak.percent(),
    ];
    assert_eq!(words, ["celá", "bodka", "až", "lomeno", "minus", "plus", "percent"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Slovak.symbol(symbol)).count(), 9);
    let letters: String = "aäbcčdďefghijklĺľmnňoôpqrŕsštťuvwxyzž"
        .chars()
        .filter_map(|letter| Slovak.letter(letter))
        .collect();
    assert!(
        letters.starts_with("áširoké ebécéčédéďé") && letters.ends_with("ypsilonzetžet"),
        "{letters}"
    );
    assert_eq!(Slovak.letter('1'), None);
    assert_eq!(
        "@./:-_!".chars().filter_map(|sign| Slovak.address_sign(sign)).count(),
        6
    );
    assert!(is_abbreviation("Napr") && is_abbreviation("atď") && !is_abbreviation("dom"));
    assert_eq!(Slovak.abbreviation("ul"), Some("ulica"));
    assert!(Slovak.vowels().contains('ô'));
}

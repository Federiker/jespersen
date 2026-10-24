use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    letters::letters,
    locale::{Greek, cardinal},
    read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("καλημέρα", "kaliˈmeɾa", "kalimˈera"),
        ("ευχαριστώ", "efxaɾiˈsto", "efxaristˈo"),
        ("παιδιά", "peˈðʝa", "peðʝˈa"),
        ("γιατί", "ʝaˈti", "ʝatˈi"),
        ("και", "ˈce", "cˈe"),
        ("χέρι", "ˈçeɾi", "çˈeri"),
        ("γάλα", "ˈɣala", "ɣˈala"),
        ("γέρος", "ˈʝeɾos", "ʝˈeros"),
        ("άγγελος", "ˈaŋɟelos", "ˈaŋɟelos"),
        ("μπαμπάς", "bamˈbas", "bambˈas"),
        ("ντομάτα", "doˈmata", "domˈata"),
        ("γκρεμός", "ɡɾeˈmos", "ɡremˈos"),
        ("αυτός", "afˈtos", "aftˈos"),
        ("εύκολος", "ˈefkolos", "ˈefkolos"),
        ("Εύα", "ˈeva", "ˈeva"),
        ("πιάνο", "ˈpçano", "pçˈano"),
        ("ελιά", "eˈʎa", "eʎˈa"),
        ("χρόνια", "ˈxɾoɲa", "xrˈoɲa"),
        ("μια", "ˈmɲa", "mɲˈa"),
        ("αύριο", "ˈavɾio", "ˈavrio"),
        ("κύριος", "ˈciɾios", "cˈirios"),
        ("Κυριακή", "ciɾʝaˈci", "cirʝacˈi"),
        ("ποιος", "ˈpços", "pçˈos"),
        ("σβήνω", "ˈzvino", "zvˈino"),
        ("κόσμος", "ˈkozmos", "kˈozmos"),
        ("τσάι", "ˈtsai", "tsˈai"),
        ("τζάκι", "ˈdzaci", "dzˈaci"),
        ("Πειραιάς", "piɾeˈas", "pireˈas"),
        ("οκτώ", "oˈkto", "oktˈo"),
        ("ξένος", "ˈksenos", "ksˈenos"),
        ("ψωμί", "psoˈmi", "psomˈi"),
        ("θάλασσα", "ˈθalasa", "θˈalasa"),
        ("δώδεκα", "ˈðoðeka", "ðˈoðeka"),
        ("κοροϊδεύω", "koɾoiˈðevo", "koroiðˈevo"),
        ("ΑΘΗΝΑ", "aˈθina", "aθˈina"),
        ("ἄνθρωπος", "ˈanθɾopos", "ˈanθropos"),
        ("τὸ", "ˈto", "tˈo"),
        ("ᾠδή", "oˈði", "oðˈi"),
        ("σ'", "s", "s"),
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
fn polytonic_and_combining_marks_fold() {
    let plain = |word: &str| -> Vec<(char, bool, bool)> {
        letters(word)
            .iter()
            .map(|letter| (letter.base, letter.accent, letter.diaeresis))
            .collect()
    };
    assert_eq!(plain("ἄ"), [('α', true, false)]);
    assert_eq!(plain("ὁ"), [('ο', false, false)]);
    assert_eq!(plain("ῆ"), [('η', true, false)]);
    assert_eq!(plain("ᾳ"), [('α', false, false)]);
    assert_eq!(plain("ῥ"), [('ρ', false, false)]);
    assert_eq!(plain("ΐ"), [('ι', true, true)]);
    assert_eq!(plain("ῧ"), [('υ', true, true)]);
    assert_eq!(plain("Ὲ"), [('ε', true, false)]);
    assert_eq!(plain("Ὸ"), [('ο', true, false)]);
    assert_eq!(plain("ῶ"), [('ω', true, false)]);
    assert_eq!(plain("ᾼ"), [('α', false, false)]);
    assert_eq!(plain("ὺ"), [('υ', true, false)]);
    assert_eq!(plain("ᾑ"), [('η', false, false)]);
    assert_eq!(plain("ε\u{301}"), [('ε', true, false)]);
    assert_eq!(plain("ι\u{308}"), [('ι', false, true)]);
    assert_eq!(plain("\u{313}α"), [('α', false, false)]);
    assert_eq!(plain("ς"), [('σ', false, false)]);
}

#[test]
fn numbers_agree_with_what_they_count() {
    for (number, gender, words) in [
        (0, Gender::Alone, "μηδέν"),
        (1, Gender::Alone, "ένα"),
        (1, Gender::Masculine, "ένας"),
        (1, Gender::Feminine, "μία"),
        (3, Gender::Feminine, "τρεις"),
        (14, Gender::Masculine, "δεκατέσσερις"),
        (13, Gender::Alone, "δεκατρία"),
        (21, Gender::Feminine, "είκοσι μία"),
        (100, Gender::Alone, "εκατό"),
        (120, Gender::Alone, "εκατόν είκοσι"),
        (200, Gender::Masculine, "διακόσιοι"),
        (300, Gender::Feminine, "τριακόσιες"),
        (1000, Gender::Alone, "χίλια"),
        (1000, Gender::Feminine, "χίλιες"),
        (2024, Gender::Alone, "δύο χιλιάδες είκοσι τέσσερα"),
        (3400, Gender::Alone, "τρεις χιλιάδες τετρακόσια"),
        (200_000, Gender::Alone, "διακόσιες χιλιάδες"),
        (1_000_000, Gender::Alone, "ένα εκατομμύριο"),
        (2_500_000, Gender::Alone, "δύο εκατομμύρια πεντακόσιες χιλιάδες"),
        (1_000_000_000, Gender::Alone, "ένα δισεκατομμύριο"),
        (3_000_000_000, Gender::Alone, "τρία δισεκατομμύρια"),
    ] {
        assert_eq!(cardinal(number, gender), words, "{number} {gender:?}");
    }
}

#[test]
fn the_locale_says_signs_dates_and_genders() {
    let locale = Greek;
    assert_eq!(
        locale.date(3, 5, 2024).as_deref(),
        Some("τρεις Μαΐου δύο χιλιάδες είκοσι τέσσερα")
    );
    assert_eq!(
        locale.date(1, 1, 2000).as_deref(),
        Some("πρώτη Ιανουαρίου δύο χιλιάδες")
    );
    assert_eq!(locale.date(1, 13, 2000), None);
    assert_eq!(locale.cardinal(4, Gender::Feminine).as_deref(), Some("τέσσερις"));
    for (word, gender) in [
        ("άνθρωπος", Gender::Masculine),
        ("μαθητές", Gender::Feminine),
        ("ώρα", Gender::Feminine),
        ("παιδιά", Gender::Alone),
        ("βιβλίο", Gender::Alone),
        ("μαθήματα", Gender::Alone),
        ("ευρώ", Gender::Alone),
        ("φοιτητές", Gender::Feminine),
        ("άνδρες", Gender::Feminine),
    ] {
        assert_eq!(locale.gender_of(word), gender, "{word}");
    }
    assert_eq!(
        (locale.decimal_word(','), locale.decimal_word('.')),
        ("κόμμα", "τελεία")
    );
    assert_eq!(
        (
            locale.range(),
            locale.ratio(),
            locale.minus(),
            locale.plus(),
            locale.percent()
        ),
        ("έως", "προς", "μείον", "συν", "τοις εκατό")
    );
    for sign in ["€", "$", "£"] {
        assert!(locale.currency(sign).is_some());
    }
    assert_eq!(locale.currency("¥"), None);
    for symbol in ["%", "&", "@", "+", "=", "°", "#", "€", "$"] {
        assert!(locale.symbol(symbol).is_some(), "{symbol}");
    }
    assert_eq!(locale.symbol("~"), None);
    for letter in ('a'..='z').chain('α'..='ω').filter(|letter| *letter != 'ς') {
        assert!(locale.letter(letter).is_some(), "{letter}");
    }
    assert_eq!(locale.letter('Ω'), Some("ωμέγα"));
    assert_eq!(locale.letter('ß'), None);
    for sign in ['@', '.', '/', ':', '-', '_'] {
        assert!(locale.address_sign(sign).is_some(), "{sign}");
    }
    assert_eq!(locale.address_sign('!'), None);
    assert_eq!(locale.abbreviation("Π.χ"), Some("παραδείγματος χάριν"));
    assert!(crate::is_abbreviation("δηλ"));
    assert!(!crate::is_abbreviation("σπίτι"));
    assert!(locale.vowels().contains('ά'));
}

#[test]
fn articles_and_particles_lose_their_stress() {
    let tokens = [
        ("Το", TokenKind::Word),
        ("παιδί", TokenKind::Word),
        ("θα", TokenKind::Word),
        ("έρθει", TokenKind::Word),
    ];
    let said: Vec<String> = read_tokens(&tokens)
        .iter()
        .flatten()
        .flat_map(|reading| reading.words.iter())
        .map(|word| word.to_notation(Notation::Espeak))
        .collect();
    assert_eq!(said, ["to", "peðˈi", "θa", "ˈerθi"]);
}

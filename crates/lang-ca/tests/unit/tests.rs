use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Catalan, is_abbreviation},
    numbers::{Form, cardinal},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("casa", "ˈkazə", "kˈazə"),
        ("porta", "ˈpɔrtə", "pˈɔrtə"),
        ("cafè", "kəˈfɛ", "kəfˈɛ"),
        ("però", "pəˈɾɔ", "pəɾˈɔ"),
        ("menjar", "mənˈʒa", "mənʒˈa"),
        ("plaça", "ˈplasə", "plˈasə"),
        ("quatre", "ˈkwatɾə", "kwˈatɾə"),
        ("guerra", "ˈɡɛrə", "ɡˈɛrə"),
        ("llibre", "ˈʎibɾə", "ʎˈibɾə"),
        ("fill", "ˈfiʎ", "fˈiʎ"),
        ("cotxe", "ˈkɔtʃə", "kˈɔtʃə"),
        ("feixuc", "fəˈʃuk", "fəʃˈuk"),
        ("feina", "ˈfɛjnə", "fˈɛjnə"),
        ("xocolata", "ʃukuˈlatə", "ʃukulˈatə"),
        ("metge", "ˈmedʒə", "mˈedʒə"),
        ("vaig", "ˈbatʃ", "bˈatʃ"),
        ("mig", "ˈmitʃ", "mˈitʃ"),
        ("any", "ˈaɲ", "ˈaɲ"),
        ("anys", "ˈaɲʃ", "ˈaɲʃ"),
        ("paraula", "pəˈɾawlə", "pəɾˈawlə"),
        ("rossa", "ˈrosə", "rˈosə"),
        ("taula", "ˈtawlə", "tˈawlə"),
        ("moment", "muˈmen", "mumˈen"),
        ("molt", "ˈmɔl", "mˈɔl"),
        ("camp", "ˈkam", "kˈam"),
        ("banc", "ˈbaŋ", "bˈaŋ"),
        ("sang", "ˈsaŋ", "sˈaŋ"),
        ("exemple", "əɡˈzemplə", "əɡzˈemplə"),
        ("explicar", "əkspliˈka", "əksplikˈa"),
        ("taxi", "ˈtaksi", "tˈaksi"),
        ("col·legi", "kulˈlɛʒi", "kullˈɛʒi"),
        ("setmana", "səmˈmanə", "səmmˈanə"),
        ("aigua", "ˈajɡwə", "ˈajɡwə"),
        ("pingüí", "piŋˈɡwi", "piŋɡwˈi"),
        ("raïm", "rəˈim", "rəˈim"),
        ("noia", "ˈnɔjə", "nˈɔjə"),
        ("veure", "ˈbɛwɾə", "bˈɛwɾə"),
        ("ciutat", "siwˈtat", "siwtˈat"),
        ("havia", "əˈbiə", "əbˈiə"),
        ("escola", "əsˈkɔlə", "əskˈɔlə"),
        ("famosa", "fəˈmozə", "fəmˈozə"),
        ("solament", "ˈsɔləˈmen", "sˈɔləmˈen"),
        ("generalment", "ʒənəˈɾalˈmen", "ʒənəɾˈalmˈen"),
        ("carrer", "kəˈre", "kərˈe"),
        ("ésser", "ˈesər", "ˈesər"),
        ("regular", "rəɡuˈlar", "rəɡulˈar"),
        ("mar", "ˈmar", "mˈar"),
        ("poble", "ˈpɔbblə", "pˈɔbblə"),
        ("oblidar", "ubliˈda", "uβlidˈa"),
        ("cinquanta", "siŋˈkwantə", "siŋkwˈantə"),
        ("consciència", "kunsiˈɛnsiə", "kunsiˈɛnsiə"),
        ("excessos", "əkˈsesus", "əksˈesus"),
        ("angoixa", "əŋˈɡoʃə", "əŋɡˈoʃə"),
        ("satisfactori", "sətisfəkˈtɔɾi", "sətisfəktˈɔɾi"),
        ("setze", "ˈsɛdzə", "sˈɛdzə"),
        ("qüestió", "kwəstiˈo", "kwəstiˈo"),
        ("biblioteca", "bibliuˈtɛkə", "biβliutˈɛkə"),
        ("adjectiu", "ədʒəkˈtiw", "ədʒəktˈiw"),
        ("absolut", "əpsuˈlut", "əpsulˈut"),
        ("heura", "ˈɛwɾə", "ˈɛwɾə"),
        ("ahir", "əˈi", "əˈi"),
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
    assert!(pronounce("h").is_none());
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
fn sounds_change_across_words() {
    assert_eq!(read("la boca"), ["lə", "βˈokə"]);
    assert_eq!(read("els gats i els homes"), ["əlz", "ɣˈadz", "j", "əlz", "ˈɔməs"]);
    assert_eq!(read("en fusta"), ["əɱ", "fˈustə"]);
    assert_eq!(read("que el veu"), ["k", "əl", "βˈɛw"]);
    assert_eq!(read("amb ells i amb tu"), ["əmb", "ˈeʎz", "j", "əm", "tˈu"]);
    assert_eq!(read("cinc anys i cinc dies"), ["sˈiŋk", "ˈaɲʒ", "i", "sˈiŋ", "dˈiəs"]);
    assert_eq!(read("camp obert"), ["kˈamp", "uβˈɛrt"]);
    assert_eq!(read("vaig anar"), ["bˈadʒ", "ənˈa"]);
    assert_eq!(read("un pot de vi"), ["um", "pˈɔd", "də", "βˈi"]);
    assert_eq!(read("era un home"), ["ˈeɾə", "wn", "ˈɔmə"]);
    assert_eq!(read("la mar endins"), ["lə", "mˈaɾ", "əndˈins"]);
    assert_eq!(read("tots junts"), ["tˈodʒ", "ʒˈuns"]);
    assert_eq!(read("un llop"), ["uɲ", "ʎˈop"]);
}

#[test]
fn elisions_and_pronouns_join_their_word() {
    assert_eq!(read("l'home d'aigua"), ["lˈɔmə", "ðˈajɣwə"]);
    assert_eq!(read("posa'l"), ["pˈozəl"]);
    assert_eq!(read("menjar-se'l"), ["mənʒˈarsəl"]);
    assert_eq!(read("anar-hi"), ["ənˈaɾi"]);
    assert_eq!(read("vint-i-un"), ["bˌintiˈun"]);
    assert_eq!(read("Barcelona-Girona"), ["bərsəlˈonə", "ʒiɾˈonə"]);
    assert_eq!(read("B"), ["bˈe"]);
    assert_eq!(read("W"), ["bəðˈɔbblə"]);
}

#[test]
fn numbers_in_words() {
    for (number, form, words) in [
        (0, Form::Alone, "zero"),
        (1, Form::Alone, "u"),
        (1, Form::Masculine, "un"),
        (1, Form::Feminine, "una"),
        (2, Form::Feminine, "dues"),
        (17, Form::Alone, "disset"),
        (21, Form::Alone, "vint-i-u"),
        (22, Form::Feminine, "vint-i-dues"),
        (31, Form::Masculine, "trenta-un"),
        (40, Form::Alone, "quaranta"),
        (100, Form::Alone, "cent"),
        (101, Form::Alone, "cent u"),
        (200, Form::Alone, "dos-cents"),
        (300, Form::Feminine, "tres-centes"),
        (1000, Form::Alone, "mil"),
        (2024, Form::Alone, "dos mil vint-i-quatre"),
        (2000, Form::Feminine, "dues mil"),
        (21_000, Form::Alone, "vint-i-un mil"),
        (1_000_000, Form::Alone, "un milió"),
        (3_000_000, Form::Alone, "tres milions"),
        (2_000_000_000_000, Form::Alone, "dos bilions"),
    ] {
        assert_eq!(cardinal(number, form), words, "{number}");
    }
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(
        Catalan.date(3, 5, 2024).unwrap(),
        "tres de maig de dos mil vint-i-quatre"
    );
    assert_eq!(Catalan.date(1, 4, 2024).unwrap(), "u d'abril de dos mil vint-i-quatre");
    assert_eq!(Catalan.date(1, 13, 2024), None);
    assert_eq!(Catalan.time(10, 30).unwrap(), "les deu i trenta");
    assert_eq!(Catalan.time(1, 0).unwrap(), "la una");
    assert_eq!(Catalan.cardinal(2, Gender::Feminine).unwrap(), "dues");
    assert_eq!(Catalan.currency("€"), Some(("euro", "euros")));
    assert_eq!(Catalan.currency("$"), Some(("dòlar", "dòlars")));
    assert_eq!(Catalan.currency("£"), Some(("lliura", "lliures")));
    assert_eq!(Catalan.currency("¥"), None);
    let words = [
        Catalan.decimal_word(','),
        Catalan.decimal_word('.'),
        Catalan.range(),
        Catalan.ratio(),
        Catalan.minus(),
        Catalan.plus(),
        Catalan.percent(),
    ];
    assert_eq!(words, ["coma", "punt", "a", "entre", "menys", "més", "per cent"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Catalan.symbol(symbol)).count(), 9);
    let letters: String = "abcçdefghijklmnopqrstuvwxyz"
        .chars()
        .filter_map(|letter| Catalan.letter(letter))
        .collect();
    assert!(
        letters.starts_with("abecece trencadadeeefagehacijota") && letters.ends_with("icsi gregazeta"),
        "{letters}"
    );
    assert_eq!(Catalan.letter('1'), None);
    assert_eq!(
        "@./:-_!".chars().filter_map(|sign| Catalan.address_sign(sign)).count(),
        6
    );
    assert!(is_abbreviation("Sr") && is_abbreviation("núm") && !is_abbreviation("casa"));
    assert_eq!(Catalan.abbreviation("pàg"), Some("pàgina"));
    assert_eq!(Catalan.gender_of("lliura"), Gender::Feminine);
    assert_eq!(Catalan.gender_of("llibres"), Gender::Masculine);
    assert!(Catalan.elides());
    assert_eq!(Catalan.vowels(), "aàeèéiíïoòóuúü");
}

#[test]
fn syllables_start_as_catalan_allows() {
    use crate::phonemes::{B, J, K, L, P, R_TAP, S, T, W, legal_onset};
    assert!(legal_onset(&[P]) && legal_onset(&[B, R_TAP]) && legal_onset(&[K, W]) && legal_onset(&[P, L, W]));
    assert!(!legal_onset(&[S, T]) && !legal_onset(&[J, W]) && !legal_onset(&[S, T, R_TAP, W]));
}

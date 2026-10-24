use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Irish, is_abbreviation},
    numbers::{cardinal, is_number_word},
    phonemes::{K, L, N, R, S, SH, T, legal_onset},
    sentence::read_tokens,
    spelling::{pronounce, pronounce_bare},
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        // Broad and slender consonants, and eSpeak NG's back and front a.
        ("fear", "ˈfar", "fˈar"),
        ("mac", "ˈmak", "mˈAk"),
        ("teach", "ˈtʲax", "cˈAx"),
        ("tír", "ˈtʲiːɾʲ", "cˈiːr"),
        ("triall", "ˈtʲɾʲiəl", "tʃrˈiəl"),
        ("idir", "ˈɪdʲɪɾʲ", "ˈɪdʒɪr"),
        ("déanamh", "ˈdʲeːnəv", "dʲˈeːnɐv"),
        ("de", "ˈdʲɛ", "dˈɛ"),
        ("deoch", "ˈdʲɔx", "dʒˈɔx"),
        ("leis", "ˈʎɛʃ", "lˈɛʃ"),
        ("liom", "ˈʎɔm", "lʲˈɔm"),
        ("eile", "ˈɛʎə", "ˈɛlə"),
        ("bhaile", "ˈvaʎə", "vˈAlʲə"),
        ("sin", "ˈʃɪɲ", "ʃˈɪn"),
        ("fiú", "ˈfʲuː", "fʲˈuː"),
        ("fios", "ˈfɪs", "fˈɪs"),
        ("rang", "ˈɾaŋ", "ɹˈAŋ"),
        ("glan", "ˈɡlan", "ɡlˈAn"),
        ("fáilte", "ˈfɑːi̯ʎtʲə", "fˈɑːi̯lʲtʃə"),
        // The vowel groups and the síneadh fada.
        ("ceann", "ˈcaun", "kʲˈaʊn"),
        ("labhair", "ˈlauəɾʲ", "lˈaʊər"),
        ("domhan", "ˈd̪auən", "d̪ˈaʊən"),
        ("neamh", "ˈɲav", "nʲˈav"),
        ("tadhg", "ˈt̪aiɡ", "t̪ˈaɪɡ"),
        ("caint", "ˈkaintʲ", "kˈaɪntʃ"),
        ("bord", "ˈboːrd̪", "bˈoːrd̪"),
        ("aon", "ˈeːən", "ˈeːən"),
        ("caol", "kəˈeːəl", "kəˈeːəl"),
        ("gaol", "ˈɡweːəl", "ɡwˈeːəl"),
        ("caoi", "kəˈiː", "kəˈiː"),
        ("ceol", "ˈcoːl", "kʲˈoːl"),
        ("toil", "ˈt̪ɔʎ", "t̪ˈɔlʲ"),
        ("soir", "ˈsɛɾʲ", "sˈɛr"),
        ("cuid", "kəˈɪdʲ", "kəˈɪdʒ"),
        ("duit", "ˈd̪ɪtʲ", "d̪ˈɪc"),
        ("amháin", "əˈwɑːi̯ɲ", "əwˈɑːi̯n"),
        ("dóibh", "ˈd̪oːi̯v", "d̪ˈoːi̯v"),
        ("tháinig", "ˈhɑːɲɪɟ", "hˈɑːnʲɪɡ"),
        ("cúige", "ˈkuːi̯ɟə", "kˈuːi̯ɡʲə"),
        ("uaidh", "ˈui̯ɟ", "ˈui̯ɡ"),
        ("bhriain", "ˈvɾʲiːi̯ɲ", "vrˈiːi̯n"),
        ("nua", "ˈnuːə", "nˈuːə"),
        ("níor", "ˈɲiːər", "nʲˈiːər"),
        ("beó", "bəˈoː", "bəˈoː"),
        // Lenition and eclipsis.
        ("bhfuil", "ˈwɪʎ", "wˈɪlʲ"),
        ("mheán", "ˈvʲɑːn", "vʲˈɑːn"),
        ("thighearna", "ˈçɪɣərnə", "hʲˈɪʁɐrnə"),
        ("ghiolla", "ˈɣjɔlə", "ʁjˈɔlə"),
        ("dhuine", "ɣəˈɪɲə", "ɣəˈɪnʲə"),
        ("mbaile", "ˈmaʎə", "mˈAlʲə"),
        ("gcathair", "ˈɡahəɾʲ", "ɡˈAhər"),
        ("bpáiste", "ˈbɑːi̯ʃtʲə", "bˈɑːi̯ʃtʃə"),
        ("dtí", "ˈdʲiː", "dʒˈiː"),
        ("ngarraí", "ˈŋariː", "ŋˈariː"),
        ("tsaol", "t̪əˈeːəl", "t̪əˈeːəl"),
        ("t-am", "ˈt̪aum", "t̪ˈaʊm"),
        ("n-aon", "ˈneːən", "nˈeːən"),
        ("d'ól", "ˈd̪oːl", "dˈoːl"),
        // The endings, the epenthetic ə and the unstressed vowels.
        ("bhíodh", "ˈviːəx", "vˈiːəχ"),
        ("cuireadh", "kəˈɪɾʲə", "kəˈɪrɐ"),
        ("murchadh", "ˈmʊrxəx", "mˈʊrxɐχ"),
        ("codladh", "ˈkɔlə", "kˈɔlɐ"),
        ("rádh", "ˈɾɑː", "ɹˈɑː"),
        ("déarfadh", "ˈdʲeːrhəx", "dʲˈeːrhɐχ"),
        ("orm", "ˈɔɾəm", "ˈɔɹəm"),
        ("arm", "ˈaɾəm", "ˈAɹəm"),
        ("gnó", "ɡəˈnoː", "ɡənˈoː"),
        ("rinne", "ˈɾɪnjə", "ɹˈɪnjə"),
        ("linn", "ˈʎɪŋ", "lʲˈɪŋ"),
        ("againn", "ˈaɡɪŋ", "ˈAɡɪŋ"),
        ("obair", "ˈɔbəɾʲ", "ˈɔbɐr"),
        ("athair", "ˈahəɾʲ", "ˈAhər"),
        ("turais", "ˈt̪ʊɾɪʃ", "t̪ˈʊɹɪʃ"),
        ("measaim", "ˈmasəm", "mˈasəm"),
        ("dubh", "ˈd̪ʊvv", "d̪ˈʊvv"),
        ("maith", "ˈma", "mˈA"),
        ("ríghthe", "ˈɾiːɣçə", "ɹˈiːʁhʲə"),
        ("croídhe", "ˈkɾiːjə", "kɹˈiːjə"),
        ("oídhche", "ˈiːdʲhçə", "ˈiːdʒhçʲə"),
        // The stress: a long second syllable after a short first, -ís, and an á after it.
        ("foláir", "fəˈlɑːi̯ɾʲ", "fɐlˈɑːi̯r"),
        ("airís", "əˈɾʲiːʃ", "ɐrˈiːʃ"),
        ("bhídís", "ˈviːˈdʲiːʃ", "vˈiːdʒˈiːʃ"),
        ("amhrán", "ˈauˌɾɑːn", "ˈaʊɹˌɑːn"),
        ("hataí", "ˈhat̪iː", "hˈAt̪iː"),
        ("aˈmach", "əˈmax", "əmˈAx"),
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
    assert!(pronounce("bh").is_none());
    assert!(pronounce("x1").is_none());
    assert_eq!(pronounce_bare("bh").unwrap().to_notation(Notation::Espeak), "v");
}

/// A sentence's words in eSpeak NG's notation, read together.
fn read(text: &str) -> Vec<String> {
    read_in(text, Notation::Espeak)
}

/// A sentence's words in `notation`, read together.
fn read_in(text: &str, notation: Notation) -> Vec<String> {
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
        .map(|said| said.to_notation(notation))
        .collect()
}

#[test]
fn words_as_munster_says_them() {
    // ag keeps its g before a vowel and a pause only; eSpeak NG's notation always writes it.
    assert_eq!(read_in("ag teacht", Notation::Ipa), ["ɛ", "ˈtʲaxt̪"]);
    assert_eq!(read("ag teacht"), ["ɛɡ", "cˈAxt̪"]);
    assert_eq!(read_in("ag ól", Notation::Ipa), ["ɛɡ", "ˈoːl"]);
    assert_eq!(read_in("bhí sé ag", Notation::Ipa), ["ˈviː", "ˈʃeː", "ɛɡ"]);
    // ar has a broad r, air a slender one.
    assert_eq!(read_in("ar scoil", Notation::Ipa), ["ˈɛr", "ˈskɔʎ"]);
    assert_eq!(read_in("air", Notation::Ipa), ["ˈɛɾʲ"]);
    // The forms of ag are stressed on their second syllable.
    assert_eq!(read_in("duine acu", Notation::Ipa), ["ˈd̪ɪɲə", "əˈkʊ"]);
    assert_eq!(read_in("aige aici", Notation::Ipa), ["əˈɟɛ", "əˈcɪ"]);
    assert_eq!(read_in("amuigh", Notation::Ipa), ["əˈmuː"]);
}

#[test]
fn function_words_lose_their_stress() {
    assert_eq!(read("tá an fear sa teach"), ["t̪ˈɑː", "ən", "fˈar", "sə", "cˈAx"]);
    assert_eq!(read("do mo chara"), ["dɐ", "mɐ", "xˈAɹə"]);
    assert_eq!(read("seo agus sin"), ["ʃʊ", "ˌAɡəs", "ʃˈɪn"]);
    assert_eq!(read("ar an mbus"), ["ˈɛɹ", "ən", "mˈʊs"]);
    assert_eq!(read("fuar agus te"), ["fˈuəɹ", "ˌAɡəs", "cˈɛ"]);
    assert_eq!(read("nuair a"), ["nˈuər", "ə"]);
    assert_eq!(read("is é raibh"), ["ɪs", "ˈeː", "ɹˈɛv"]);
    assert_eq!(read("an t-am"), ["ən", "t̪ˈaʊm"]);
    assert_eq!(read("d’ aois"), ["d̪", "ˈiːʃ"]);
    assert_eq!(read("B"), ["bˈeː"]);
    assert_eq!(read("21"), ["fˈɪçʲəəheːən"]);
    assert_eq!(read("12"), ["d̪ˈoːjeːɡ"]);
    assert_eq!(read("200"), ["ɣˈɑːçʲeːd̪"]);
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "náid"),
        (1, "a haon"),
        (10, "a deich"),
        (11, "aon-déag"),
        (12, "dó-dhéag"),
        (17, "seacht-déag"),
        (20, "fiche"),
        (21, "fiche-a-haon"),
        (99, "nócha-a-naoi"),
        (100, "céad"),
        (101, "céad a haon"),
        (200, "dhá-chéad"),
        (700, "seacht-gcéad"),
        (1000, "míle"),
        (1916, "míle naoi-gcéad sé-déag"),
        (2024, "dhá-mhíle fiche-a-ceathair"),
        (7000, "seacht-míle"),
        (20_000, "fiche míle"),
        (1_000_000, "milliún"),
        (3_000_000, "trí-mhilliún"),
        (2_000_000_000, "dhá-bhilliún"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
    assert!(is_number_word("haon") && is_number_word("gcéad") && is_number_word("a") && !is_number_word("fear"));
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(
        Irish.date(3, 5, 2024).unwrap(),
        "a trí Bealtaine dhá-mhíle fiche-a-ceathair"
    );
    assert_eq!(Irish.date(1, 13, 2024), None);
    assert_eq!(Irish.time(10, 30).unwrap(), "a deich tríocha");
    assert_eq!(Irish.time(1, 0).unwrap(), "a haon a chlog");
    assert_eq!(Irish.cardinal(1, Gender::Feminine).unwrap(), "a haon");
    assert_eq!(Irish.currency("€"), Some(("euro", "cent")));
    assert_eq!(Irish.currency("£"), Some(("punt", "pingin")));
    assert_eq!(Irish.currency("$"), Some(("dollar", "cent")));
    assert_eq!(Irish.currency("¥"), None);
    let words = [
        Irish.decimal_word(','),
        Irish.range(),
        Irish.ratio(),
        Irish.minus(),
        Irish.plus(),
        Irish.percent(),
    ];
    assert_eq!(words, ["pointe", "go", "roinnte ar", "lúide", "móide", "faoin gcéad"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Irish.symbol(symbol)).count(), 9);
    let letters: Vec<&str> = "abcdefghijklmnopqrstuvwxyzé"
        .chars()
        .filter_map(|letter| Irish.letter(letter))
        .collect();
    assert_eq!(letters.len(), 27);
    assert_eq!((letters[0], letters[7], letters[26]), ("á", "héis", "é"));
    assert_eq!(Irish.letter('1'), None);
    assert_eq!("@./:-_!".chars().filter_map(|sign| Irish.address_sign(sign)).count(), 6);
    assert!(is_abbreviation("Dr") && is_abbreviation("srl") && !is_abbreviation("teach"));
    assert_eq!(Irish.abbreviation("uimh"), Some("uimhir"));
    assert_eq!(Irish.vowels(), "aeiouáéíóú");
}

#[test]
fn syllables_start_as_irish_allows() {
    assert!(legal_onset(&[T]) && legal_onset(&[K, R]) && legal_onset(&[S, T]) && legal_onset(&[SH, K]));
    assert!(!legal_onset(&[N, T]) && !legal_onset(&[L, N]) && !legal_onset(&[S, T, R]));
}

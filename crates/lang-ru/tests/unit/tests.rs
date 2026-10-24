use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Russian, is_abbreviation},
    numbers::{Agreement, Case, cardinal, form, ordinal},
    phonemes::{J, J_SIGN, K, L, M, N, R, S, T, legal_onset},
    sentence::read_tokens,
    spelling::pronounce,
    stress::stressed,
    words::{entries, lookup, text},
};

/// `word` with its stress, in plain IPA and eSpeak NG's notation.
fn said(word: &str) -> (String, String) {
    let stressed = stressed(word);
    let pronunciation = pronounce(&stressed.letters, stressed.stress).unwrap();
    (
        pronunciation.to_notation(Notation::Ipa),
        pronunciation.to_notation(Notation::Espeak),
    )
}

#[test]
fn words_from_their_spelling() {
    let cases = [
        // The unstressed vowels reduced by their distance from the stress.
        ("молоко", "məɫɐˈko", "mʌɭʌkˈo"),
        ("хорошо", "xərɐˈʂo", "xʌrʌʃˈo"),
        ("город", "ˈɡorət", "ɡˈorʌt"),
        ("часы", "tɕɪˈsɨ", "tʃʲasˈy"),
        ("книга", "ˈknʲiɡə", "knʲˈiɡa"),
        ("время", "ˈvrʲemʲə", "vrʲˈemʲʌ"),
        ("тысяча", "ˈtɨsʲɪtɕə", "tˈysʲʌtʃʲʌ"),
        ("дети", "ˈdʲetʲɪ", "dʲˈetʲɪ"),
        ("цена", "tsɨˈna", "tsynˈɑ"),
        ("шесть", "ˈʂɛstʲ", "ʃˈɛstʲ"),
        ("энергия", "ɪˈnʲerɡʲɪjə", "ɛnʲˈerɡʲija"),
        ("их", "ˈix", "ˈɪx"),
        // j and the vowel: я and ю as one vowel in eSpeak NG's notation, е and ё after j.
        ("язык", "jɪˈzɨk", "jazˈyk"),
        ("поют", "pɐˈjut", "pʌˈjut"),
        ("новая", "ˈnovəjə", "nˈovʌja"),
        ("красное", "ˈkrasnəjɪ", "krˈɑsnʌjɪ"),
        ("читаешь", "tɕɪˈtajɪʂ", "tʃʲitˈɑjiʃ"),
        ("если", "ˈjeslʲɪ", "jˈesɭʲɪ"),
        ("его", "jɪˈvo", "jɪvˈo"),
        ("моё", "mɐˈjo", "mʌjˈɵ"),
        ("ёлка", "ˈjoɫkə", "jˈɵɭka"),
        ("объём", "ɐˈbjom", "ʌbjˈɵm"),
        ("семья", "sʲɪˈmʲja", "sʲimʲjˈja"),
        ("счастье", "ˈɕːasʲtʲjɪ", "ɕˈɑsʲtʲjjɪ"),
        // Soft consonants, and ь after н, л, р and в in eSpeak NG's notation.
        ("день", "ˈdʲenʲ", "dʲˈen"),
        ("только", "ˈtolʲkə", "tˈoɭkʌ"),
        ("роль", "ˈrolʲ", "rˈoɭʲ"),
        ("борьба", "bɐrʲˈba", "bʌrɪ^bˈɑ"),
        ("церковь", "ˈtsɛrkəfʲ", "tsˈɛrkʌf"),
        ("работница", "rɐˈbotʲnʲɪtsə", "rabˈotʲnʲitsa"),
        ("Россия", "rɐˈsʲsʲijə", "rʌsʲsʲˈija"),
        ("отсюда", "ɐˈtʲsʲudə", "ʌtʲsʲˈu\"da"),
        // Voicing, and the groups said shorter than written.
        ("хлеб", "ˈxlʲep", "xɭʲˈep"),
        ("вокзал", "vɐˈɡzaɫ", "vʌɡzˈɑɭ"),
        ("сделать", "ˈzʲdʲeɫətʲ", "ʑdʲˈeɭʌtʲ"),
        ("здесь", "ˈzʲdʲesʲ", "ʑdʲˈesʲ"),
        ("жизнь", "ˈʐɨzʲnʲ", "ʒˈyʑn"),
        ("лёгкий", "ˈlʲoxkʲɪj", "ɭʲˈɵxkʲij"),
        ("сегодня", "sʲɪˈvodnʲə", "sʲivˈodnʲʌ"),
        ("красного", "ˈkrasnəvə", "krˈɑsnʌvʌ"),
        ("много", "ˈmnoɡə", "mnˈoɡʌ"),
        ("солнце", "ˈsontsɨ", "sˈoɭntsy"),
        ("здравствуйте", "ˈzdrastvʊjtʲɪ", "zdrˈɑstvujtʲi"),
        ("поздно", "ˈpoznə", "pˈoznʌ"),
        ("учится", "ˈutɕɪtsə", "ˈutʃʲitsʌ"),
        ("учиться", "ʊˈtɕitsə", "utʃʲˈitsʌ"),
        ("мужчина", "mʊˈɕːinə", "muɕˈina"),
        ("детский", "ˈdʲetskʲɪj", "dʲˈetskʲij"),
        ("пятитысячного", "pʲɪtʲɪˈtɨsʲɪtɕnəvə", "pʲʌtʲitˈysʲʌtʃʲnʌvʌ"),
    ];
    for (word, ipa, espeak) in cases {
        assert_eq!(said(word), (ipa.to_owned(), espeak.to_owned()), "{word}");
    }
    assert!(pronounce("abc", Some(0)).is_none());
    assert!(pronounce("", None).is_none());
}

#[test]
fn the_stress_comes_from_the_text_the_list_or_the_rules() {
    let cases = [
        // Written: an acute accent or ё.
        ("за́мок", 0),
        ("ещё", 1),
        // Listed forms, ё put back where the text leaves it out, respellings.
        ("руки", 0),
        ("еще", 1),
        ("что", 0),
        // Stems: endings that keep the stress or take it.
        ("книгами", 0),
        ("столом", 1),
        ("говорят", 2),
        ("идет", 1),
        // Prefixes, one or two, вы- taking the stress.
        ("рассказать", 2),
        ("предоставить", 2),
        ("вызовет", 0),
        ("выступил", 0),
        ("выросла", 0),
        // Endings whose stress is predictable.
        ("промышленность", 1),
        ("государственный", 2),
        ("учительница", 1),
        ("отправляется", 2),
        ("станция", 0),
        ("организация", 3),
        ("исторический", 2),
        ("человеческого", 2),
        ("получаемое", 2),
        ("социализм", 3),
        ("забывала", 2),
        ("воспрещено", 3),
        ("сопутствовала", 1),
        ("достигнуть", 1),
        // The last vowel but one.
        ("собака", 1),
    ];
    for (word, stress) in cases {
        assert_eq!(stressed(word).stress, Some(stress), "{word}");
    }
    assert_eq!(stressed("еще").letters, "ещё");
    assert_eq!(stressed("вызовет").letters, "вызовет");
    assert_eq!(lookup("что").unwrap().letters, "што");
    assert_eq!(stressed("вкл").stress, None);
}

#[test]
fn the_word_list_is_well_formed() {
    let count = entries(text()).unwrap();
    assert!(count > 1000, "{count}");
    assert_eq!(entries("молоко́\nмолоко"), Err(2));
    assert_eq!(entries("стол+x"), Err(1));
    assert_eq!(entries("слово\tсло́во"), Err(1));
    assert_eq!(entries("сло́во\tсло́в-n"), Err(1));
    assert_eq!(entries("сло́во\tсло́во\tеще"), Err(1));
    assert_eq!(entries("# comment\n\nсло́во"), Ok(1));
    let mut seen = std::collections::HashSet::new();
    for line in text().lines().filter(|line| !line.starts_with('#') && !line.is_empty()) {
        let word: String = line
            .split('\t')
            .next()
            .unwrap()
            .chars()
            .filter(|&c| c != '\u{301}')
            .collect();
        assert!(seen.insert(word.clone()), "listed twice: {word}");
    }
}

fn read(text: &str) -> Vec<String> {
    read_in(text, Notation::Espeak)
}

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
fn function_words_lose_their_stress() {
    assert_eq!(
        read("я не знаю что делать"),
        ["ˈja", "nʲɪ", "znˈɑju", "ʃto", "dʲˈeɭʌtʲ"]
    );
    assert_eq!(read("на столе"), ["nə", "stʌɭʲˈe"]);
    assert_eq!(read("для нас"), ["dɭʲɑ", "nˈɑs"]);
    assert_eq!(read("было тихо"), ["bˌyɭʌ", "tʲˈixʌ"]);
    assert_eq!(read("Н Крупская"), ["ˈɛn", "krˈupskʌja"]);
    assert_eq!(read("й"), ["ɪkrˈɑtkʌjɪ"]);
    assert_eq!(read("Ц.К"), ["tsˈɛkˈɑ"]);
    assert_eq!(read("Ц.7"), Vec::<String>::new());
}

#[test]
fn voicing_across_words() {
    assert_eq!(read("в саду"), ["f", "sadˈu"]);
    assert_eq!(read("в доме"), ["v", "dˈomʲi"]);
    assert_eq!(read("домов за"), ["dʌmˈov", "za"]);
    assert_eq!(read_in("из дома", Notation::Ipa), ["ɪz", "ˈdomə"]);
    assert_eq!(read("из дома"), ["ɪs", "dˈoma"]);
    assert_eq!(read_in("с братом", Notation::Ipa), ["z", "ˈbratəm"]);
}

#[test]
fn numbers_in_words() {
    assert_eq!(cardinal(0, Agreement::Masculine), "ноль");
    assert_eq!(cardinal(21, Agreement::Masculine), "двадцать один");
    assert_eq!(cardinal(22, Agreement::Feminine), "двадцать две");
    assert_eq!(cardinal(1000, Agreement::Masculine), "тысяча");
    assert_eq!(cardinal(2000, Agreement::Masculine), "две тысячи");
    assert_eq!(cardinal(5321, Agreement::Masculine), "пять тысяч триста двадцать один");
    assert_eq!(cardinal(11_000, Agreement::Masculine), "одиннадцать тысяч");
    assert_eq!(cardinal(1_000_000, Agreement::Masculine), "один миллион");
    assert_eq!(cardinal(3_000_000_000, Agreement::Masculine), "три миллиарда");
    assert_eq!(form(1, ["a", "b", "c"]), "a");
    assert_eq!(form(3, ["a", "b", "c"]), "b");
    assert_eq!(form(12, ["a", "b", "c"]), "c");
    assert_eq!(form(25, ["a", "b", "c"]), "c");
    assert_eq!(ordinal(3, Case::Neuter).unwrap(), "третье");
    assert_eq!(ordinal(3, Case::Prepositional).unwrap(), "третьем");
    assert_eq!(ordinal(2, Case::Masculine).unwrap(), "второй");
    assert_eq!(ordinal(20, Case::Prepositional).unwrap(), "двадцатом");
    assert_eq!(ordinal(24, Case::Genitive).unwrap(), "двадцать четвёртого");
    assert_eq!(ordinal(1915, Case::Genitive).unwrap(), "тысяча девятьсот пятнадцатого");
    assert_eq!(ordinal(2024, Case::Genitive).unwrap(), "две тысячи двадцать четвёртого");
    assert_eq!(ordinal(1900, Case::Genitive).unwrap(), "тысяча девятисотого");
    assert_eq!(ordinal(2000, Case::Genitive).unwrap(), "двухтысячного");
    assert_eq!(ordinal(5000, Case::Genitive).unwrap(), "пятитысячного");
    assert_eq!(ordinal(40, Case::Masculine).unwrap(), "сороковой");
    assert_eq!(ordinal(0, Case::Masculine), None);
    assert_eq!(ordinal(10_000, Case::Masculine), None);
    assert_eq!(ordinal(2_000_000, Case::Masculine), None);
    assert_eq!(read("2 дня"), ["dvˈɑ", "dnʲˈɑ"]);
    assert_eq!(read("2 книги"), ["dvʲˈe", "knʲˈiɡʲɪ"]);
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    let ru = Russian;
    assert_eq!(ru.cardinal(1, Gender::Feminine).unwrap(), "одна");
    assert_eq!(ru.cardinal(1, Gender::Alone).unwrap(), "один");
    assert_eq!(
        ru.date(3, 5, 2024).unwrap(),
        "третье мая две тысячи двадцать четвёртого года"
    );
    assert_eq!(ru.date(1, 13, 2024), None);
    assert_eq!(ru.decimal_word(','), "запятая");
    assert_eq!(ru.decimal_word('.'), "точка");
    assert_eq!(
        (ru.range(), ru.ratio(), ru.minus(), ru.plus(), ru.percent()),
        ("до", "дробь", "минус", "плюс", "процентов")
    );
    assert_eq!(ru.currency("₽"), Some(("рубль", "рублей")));
    assert_eq!(ru.currency("¥"), None);
    assert_eq!(ru.currency_name("$", Some(2)), Some("доллара"));
    assert_eq!(ru.currency_name("₽", Some(21)), Some("рубль"));
    assert_eq!(ru.currency_name("€", None), Some("евро"));
    assert_eq!(ru.currency_name("£", Some(5)), Some("фунтов"));
    assert_eq!(ru.symbol("@"), Some("собака"));
    assert_eq!(ru.symbol("~"), None);
    assert_eq!(ru.letter('Щ'), Some("ща"));
    assert_eq!(ru.letter('w'), Some("дубль-вэ"));
    assert_eq!(ru.letter('7'), None);
    assert_eq!(ru.abbreviation("т.е"), Some("то есть"));
    assert!(is_abbreviation("тов"));
    assert!(!is_abbreviation("кот"));
    assert_eq!(ru.address_sign('@'), Some("собака"));
    assert_eq!(ru.address_sign('#'), None);
    assert_eq!(ru.gender_of("книги"), Gender::Feminine);
    assert_eq!(ru.gender_of("дня"), Gender::Masculine);
    assert_eq!(ru.gender_of("рублей"), Gender::Masculine);
    assert_eq!(ru.roman(20, None, Some("веке")).unwrap(), "двадцатом");
    assert_eq!(ru.roman(20, None, Some("века")).unwrap(), "двадцатого");
    assert_eq!(ru.roman(19, None, Some("век")).unwrap(), "девятнадцатый");
    assert_eq!(ru.roman(20, None, Some("столетие")).unwrap(), "двадцатое");
    assert_eq!(ru.roman(1, Some("Пётр"), None).unwrap(), "первый");
    assert_eq!(ru.roman(5, Some("и"), Some("и")), None);
    assert_eq!(ru.vowels(), "аеёиоуыэюя");
}

#[test]
fn syllables_start_as_russian_allows() {
    assert!(legal_onset(&[S, T, R]));
    assert!(legal_onset(&[M, N]));
    assert!(legal_onset(&[K, J]));
    assert!(legal_onset(&[T, J_SIGN, J]));
    assert!(!legal_onset(&[R, T]));
    assert!(!legal_onset(&[J, T]));
    assert!(!legal_onset(&[J_SIGN, T]));
    assert!(!legal_onset(&[S, T, R, L, N]));
    assert!(legal_onset(&[N]));
    assert!(legal_onset(&[L]));
}

use jespersen_phonemes::Notation;
use jespersen_text::{TokenKind, sentences};

use crate::{is_abbreviation, numbers::cardinal, read_tokens, spelling::pronounce};

fn read(text: &str, notation: Notation) -> String {
    let mut out = Vec::new();
    for sentence in sentences(text, is_abbreviation) {
        let tokens: Vec<(&str, TokenKind)> = sentence.tokens.iter().map(|token| (token.text, token.kind)).collect();
        for reading in read_tokens(&tokens).into_iter().flatten() {
            out.extend(reading.words.iter().map(|word| word.to_notation(notation)));
        }
    }
    out.join(" ")
}

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("kissa", "ˈkissɑ", "kˈissa"),
        ("istuu", "ˈistuː", "ˈistuː"),
        ("kenkä", "ˈkeŋkæ", "kˈeŋkæ"),
        ("kengät", "ˈkeŋŋæt", "kˈeŋŋæt"),
        ("aurinko", "ˈɑuriŋko", "ˈauriŋko"),
        ("yö", "ˈyø", "ˈyø"),
        ("tie", "ˈtie", "tˈie"),
        ("leikki", "ˈleikki", "lˈeikːi"),
        ("tuhansia", "ˈtuhɑnsiɑ", "tˈuhansˌia"),
        ("jyväskylä", "ˈjyʋæskylæ", "jˈyvæskˌylæ"),
        ("omenaa", "ˈomenɑː", "ˈomenaː"),
        ("zoo", "ˈtsoː", "tsˈoː"),
        ("šakki", "ˈʃɑkki", "ʃˈakːi"),
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
}

#[test]
fn numbers_and_running_speech() {
    for (number, words) in [
        (0, "nolla"),
        (10, "kymmenen"),
        (11, "yksitoista"),
        (21, "kaksikymmentäyksi"),
        (100, "sata"),
        (1250, "tuhat kaksisataa viisikymmentä"),
        (2024, "kaksi tuhatta kaksikymmentäneljä"),
        (1_000_000, "miljoona"),
        (3_000_000_000, "kolme miljardia"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
    assert_eq!(
        read("Kissa istuu talon katolla.", Notation::Espeak),
        "kˈissa ˈistuː tˈaloŋ kˈatolla"
    );
    assert_eq!(read("Suomi on kaunis.", Notation::Espeak), "sˈuomɪ oŋ kˈaunɪs");
    assert_eq!(
        read("Se maksaa 5 € ja 50 %, esim. 3.5.2024.", Notation::Ipa),
        "se ˈmɑksɑː ˈʋiːsi ˈeuroɑ jɑ ˈʋiːsikymmentæ ˈprosenttiɑ ˈesimerkiksi ˈkolmɑs ˈtoukokuːtɑ ˈkɑksi ˈtuhɑttɑ ˈkɑksikymmentæneljæ"
    );
}

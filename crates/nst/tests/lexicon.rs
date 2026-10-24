//! Which of NST's entries each language keeps, and the table written from them.

use jespersen_nst::{Language, entries, extract, latin1};

/// An NST line of 51 fields: the word, part of speech, source (field 5), garbage mark (7),
/// transcription (11) and sets (29).
fn line(word: &str, pos: &str, source: &str, garbage: &str, sampa: &str, sets: &str) -> String {
    let mut fields = vec![""; 51];
    fields[0] = word;
    fields[1] = pos;
    fields[5] = source;
    fields[7] = garbage;
    fields[11] = sampa;
    fields[29] = sets;
    fields.join(";")
}

#[test]
fn languages_are_named_by_their_codes() {
    assert_eq!("da".parse(), Ok(Language::Danish));
    assert_eq!("sv".parse(), Ok(Language::Swedish));
    assert_eq!("nb".parse(), Ok(Language::Bokmal));
    assert!("no".parse::<Language>().unwrap_err().contains("da, sv or nb"));
}

#[test]
fn latin1_reads_each_byte_as_its_character() {
    assert_eq!(latin1(b"r\xf8d \xa4"), "rød ¤");
}

#[test]
fn garbage_phrases_and_empty_transcriptions_are_left_out() {
    let text = [
        line("hund", "NN|SIN", "LEX", "", "\"hun?", ""),
        line("xyz", "NN", "LEX", "GARB", "\"Eks", ""),
        line("New_York", "PM", "LEX", "", "\"nju:_\"jQ:g", ""),
        line("tom", "JJ", "LEX", "", "", ""),
        line("Hansen", "PM", "LEX", "", "\"han¤s@n", ""),
    ]
    .join("\n");
    let kept: Vec<_> = entries(&text, Language::Danish).unwrap().into_iter().collect();
    assert_eq!(
        kept,
        [
            ("Hansen".to_owned(), "PM".to_owned(), "\"hans@n".to_owned()),
            ("hund".to_owned(), "NN".to_owned(), "\"hun?".to_owned()),
        ]
    );
}

#[test]
fn swedish_keeps_only_what_nst_transcribed_outside_bought_lexicons() {
    let text = [
        line("och", "KN", "LEX", "", "\"Ok", "dictatraining_se"),
        line("hus", "NN", "LEX", "", "\"h}:s", "enter_se"),
        line("husen", "NN", "INFL", "", "\"h}:$s@n", "inflector"),
        line("Lund", "PM", "LEX", "", "\"lund", "telia_placenames_se"),
        line("bilen", "NN", "LEX|INFL", "", "\"bi:$l@n", "dictatraining_se|inflector"),
        line("talet", "NN", "LEX", "", "\"\"tA:$l@t", "inflector|spd_se"),
    ]
    .join("\n");
    let words: Vec<String> = entries(&text, Language::Swedish)
        .unwrap()
        .into_iter()
        .map(|(word, _, _)| word)
        .collect();
    assert_eq!(words, ["och", "talet"]);
}

#[test]
fn bokmal_leaves_out_norkompleks_and_onomastica() {
    let text = [
        line("og", "KN", "LEX", "", "\"O:", "ref_dic"),
        line("bil", "NN", "LEX", "", "\"bi:l", "baseform_lex_no|inflector_no"),
        line("Kari", "PM", "LEX", "", "\"\"kA:$rI", "namelex_no"),
        line("Statoil", "PM", "LEX", "", "\"stA:$tOjl", "company_names_no"),
    ]
    .join("\n");
    let words: Vec<String> = entries(&text, Language::Bokmal)
        .unwrap()
        .into_iter()
        .map(|(word, _, _)| word)
        .collect();
    assert_eq!(words, ["og"]);
}

#[test]
fn a_line_of_the_wrong_shape_is_an_error() {
    let error = entries("og;KN", Language::Bokmal).unwrap_err();
    assert_eq!(error.to_string(), "line 1: 2 fields, not 51");
}

#[test]
fn too_few_entries_mean_the_wrong_file() {
    let text = line("og", "KN", "LEX", "", "\"O:", "ref_dic");
    let error = extract(&text, Language::Bokmal).unwrap_err();
    assert_eq!(error.to_string(), "only 1 entries kept: is this nor030224NST.pron?");
}

#[test]
fn the_table_has_its_header_then_one_entry_a_line() {
    let text: Vec<String> = (0..30_000)
        .map(|n| line(&format!("ord{n:05}"), "NN", "LEX", "", "\"u:rd", "dictatraining_se"))
        .collect();
    let table = extract(&text.join("\n"), Language::Swedish).unwrap();
    let mut lines = table.lines();
    assert_eq!(
        lines.next(),
        Some("# Swedish words, their part of speech and pronunciation from NST's lexicon (Nordisk")
    );
    assert!(table.contains("`mise run data:nst:sv -- path/to/swe030224NST.pron`.\n"));
    assert!(table.contains("\nord00000\tNN\t\"u:rd\n"));
    assert_eq!(table.lines().filter(|line| !line.starts_with('#')).count(), 30_000);
}

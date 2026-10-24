use jespersen_text::TokenKind::{Number, Punctuation, Symbol, Word};
use jespersen_text::{Script, Sentence, Token, TokenKind, sentences};

fn no_abbreviations(_: &str) -> bool {
    false
}

fn english_titles(word: &str) -> bool {
    matches!(word, "Mr" | "Mrs" | "Dr")
}

fn texts<'a>(sentences: &[Sentence<'a>]) -> Vec<&'a str> {
    sentences.iter().map(|sentence| sentence.text).collect()
}

fn token_texts<'a>(tokens: &[Token<'a>]) -> Vec<&'a str> {
    tokens.iter().map(|token| token.text).collect()
}

#[test]
fn english_sentences_split_on_terminal_punctuation() {
    let text = "It rained. Did it? Yes! It did.";
    assert_eq!(
        texts(&sentences(text, no_abbreviations)),
        ["It rained.", "Did it?", "Yes!", "It did."]
    );
}

#[test]
fn a_lower_case_word_after_a_full_stop_does_not_start_a_sentence() {
    let text = "Bring fruit, e.g. apples. Then go.";
    assert_eq!(
        texts(&sentences(text, no_abbreviations)),
        ["Bring fruit, e.g. apples.", "Then go."]
    );
}

#[test]
fn abbreviations_from_the_language_join_sentences() {
    let text = "Mr. Smith met Dr. Jones. They talked.";
    assert_eq!(
        texts(&sentences(text, no_abbreviations)),
        ["Mr.", "Smith met Dr.", "Jones.", "They talked."]
    );
    let joined = sentences(text, english_titles);
    assert_eq!(texts(&joined), ["Mr. Smith met Dr. Jones.", "They talked."]);
    assert_eq!(joined[0].span, 0..24);
    assert_eq!(joined[0].tokens.len(), 8);
}

#[test]
fn closing_quotes_stay_with_their_sentence() {
    let text = "He said “hello.” Then he left.";
    assert_eq!(
        texts(&sentences(text, no_abbreviations)),
        ["He said “hello.”", "Then he left."]
    );
}

#[test]
fn chinese_sentences_end_at_the_full_stop_without_spaces() {
    let text = "他说：“你好。”下雨了。真的吗？";
    assert_eq!(
        texts(&sentences(text, no_abbreviations)),
        ["他说：“你好。”", "下雨了。", "真的吗？"]
    );
}

#[test]
fn tokens_have_kinds_spans_and_scripts() {
    let text = "Don't pay $3.50 — it's 10% off!";
    let found = sentences(text, no_abbreviations);
    let tokens = &found[0].tokens;
    assert_eq!(
        token_texts(tokens),
        ["Don't", "pay", "$", "3.50", "—", "it's", "10", "%", "off", "!"]
    );
    let kinds: Vec<TokenKind> = tokens.iter().map(|token| token.kind).collect();
    assert_eq!(
        kinds,
        [
            Word,
            Word,
            Symbol,
            Number,
            Punctuation,
            Word,
            Number,
            Symbol,
            Word,
            Punctuation
        ]
    );
    for token in tokens {
        assert_eq!(&text[token.span.clone()], token.text);
    }
    assert_eq!(tokens[0].script, Script::Latin);
    assert_eq!(tokens[3].script, Script::Common);
}

#[test]
fn mixed_chinese_and_english_keeps_each_script() {
    let text = "我用iPhone 15拍照。";
    let tokens = &sentences(text, no_abbreviations)[0].tokens;
    let scripts: Vec<(&str, Script)> = tokens.iter().map(|token| (token.text, token.script)).collect();
    assert_eq!(
        scripts,
        [
            ("我", Script::Han),
            ("用", Script::Han),
            ("iPhone", Script::Latin),
            ("15", Script::Common),
            ("拍", Script::Han),
            ("照", Script::Han),
            ("。", Script::Common),
        ]
    );
    assert_eq!(tokens[2].kind, TokenKind::Word);
    assert_eq!(tokens[6].kind, TokenKind::Punctuation);
}

#[test]
fn numbers_keep_their_inner_punctuation_and_suffixes() {
    let tokens = &sentences("1,000 people in the 1990s, the 3rd time", no_abbreviations)[0].tokens;
    assert_eq!(
        token_texts(tokens),
        ["1,000", "people", "in", "the", "1990s", ",", "the", "3rd", "time"]
    );
    assert_eq!(tokens[4].kind, TokenKind::Number);
    assert_eq!(tokens[7].kind, TokenKind::Number);
}

#[test]
fn scripts_of_characters() {
    assert_eq!(Script::of('é'), Script::Latin);
    assert_eq!(Script::of('Ａ'), Script::Latin, "fullwidth Latin");
    assert_eq!(Script::of('中'), Script::Han);
    assert_eq!(Script::of('〇'), Script::Han);
    assert_eq!(Script::of('𠀀'), Script::Han, "extension B");
    assert_eq!(Script::of('ж'), Script::Other);
    assert_eq!(Script::of('あ'), Script::Other);
    assert_eq!(Script::of('7'), Script::Common);
    assert_eq!(Script::of_text("--"), Script::Common);
}

#[test]
fn symbols_that_are_read_as_words_are_not_punctuation() {
    let tokens = &sentences("5‰ & 6′ @ ＋", no_abbreviations)[0].tokens;
    let kinds: Vec<(&str, TokenKind)> = tokens.iter().map(|token| (token.text, token.kind)).collect();
    assert!(kinds.contains(&("‰", TokenKind::Symbol)));
    assert!(kinds.contains(&("′", TokenKind::Symbol)));
    assert!(kinds.contains(&("&", TokenKind::Symbol)));
    assert!(kinds.contains(&("＋", TokenKind::Symbol)));
}

#[test]
fn empty_and_blank_text_has_no_sentences() {
    assert!(sentences("", no_abbreviations).is_empty());
    assert!(sentences("  \n\t ", no_abbreviations).is_empty());
}

#[test]
fn an_abbreviation_at_the_very_end_joins_nothing() {
    let found = sentences("I met Mr.", english_titles);
    assert_eq!(texts(&found), ["I met Mr."]);
}

#[test]
fn numbers_joined_by_colons_slashes_or_hyphens_stay_one_token() {
    let tokens = &sentences(
        "At 10:30 on 12/05/2024, in 1990-1995 or 555-12-34; 3 - 4 and C/D",
        no_abbreviations,
    )[0]
    .tokens;
    let numbers: Vec<&str> = tokens
        .iter()
        .filter(|token| token.kind == TokenKind::Number)
        .map(|token| token.text)
        .collect();
    assert_eq!(numbers, ["10:30", "12/05/2024", "1990-1995", "555-12-34", "3", "4"]);
    let time = tokens.iter().find(|token| token.text == "10:30").unwrap();
    assert_eq!(time.span, 3..8);
    assert!(token_texts(tokens).contains(&"/"), "C/D: letters stay apart");
}

#[test]
fn hyphenated_words_stay_one_token() {
    let tokens = &sentences("A well-known state-of-the-art plan - or not", no_abbreviations)[0].tokens;
    assert_eq!(
        token_texts(tokens),
        ["A", "well-known", "state-of-the-art", "plan", "-", "or", "not"]
    );
    assert_eq!(tokens[2].kind, TokenKind::Word);
    assert_eq!(tokens[2].span, 13..29);
}

#[test]
fn a_single_line_break_inside_a_sentence_does_not_end_it() {
    let text = "I have seldom heard him\nmention her. She left.\n\nA new\nparagraph here.\nAnd this.";
    assert_eq!(
        texts(&sentences(text, no_abbreviations)),
        [
            "I have seldom heard him\nmention her.",
            "She left.",
            "A new\nparagraph here.",
            "And this."
        ]
    );
    let blank = "No stop at the end\n\nof a paragraph";
    assert_eq!(
        texts(&sentences(blank, no_abbreviations)).len(),
        2,
        "a blank line still ends it"
    );
}

#[test]
fn a_sign_before_a_number_belongs_to_it() {
    let tokens = &sentences("It was -5 or +3, not 7-2 or 8 - 1.", no_abbreviations)[0].tokens;
    let numbers: Vec<&str> = tokens
        .iter()
        .filter(|token| token.kind == TokenKind::Number)
        .map(|token| token.text)
        .collect();
    assert_eq!(numbers, ["-5", "+3", "7-2", "8", "1"]);
}

#[test]
fn web_and_e_mail_addresses_are_one_token() {
    let text = "Write to john.smith@example.co.uk, or see https://www.example.com/page?q=1. Also www.jespersen.org!";
    let tokens: Vec<&str> = sentences(text, no_abbreviations)
        .iter()
        .flat_map(|s| token_texts(&s.tokens))
        .collect();
    assert!(tokens.contains(&"john.smith@example.co.uk"));
    assert!(tokens.contains(&"https://www.example.com/page?q=1"));
    assert!(tokens.contains(&"www.jespersen.org"));
    assert!(tokens.contains(&","), "the comma after an address is punctuation");
    let plain = &sentences("Mail me @ home or at 5:30.", no_abbreviations)[0].tokens;
    assert!(token_texts(plain).contains(&"@"), "@ alone is no address");
}

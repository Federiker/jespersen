//! Numbers, money and symbols read as English words (the words, before their phonemes).

use jespersen_lang_en::{Variety, read_tokens};
use jespersen_phonemes::Notation;
use jespersen_text::TokenKind::{self, Number, Punctuation, Symbol, Word};

/// Each token's words in plain IPA, joined by spaces; `?` for a token that cannot be read.
#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn read(tokens: &[(&str, TokenKind)]) -> Vec<String> {
    read_tokens(tokens, Variety::American)
        .unwrap()
        .into_iter()
        .map(|reading| match reading {
            Some(reading) => reading
                .words
                .iter()
                .map(|word| word.to_notation(Notation::Ipa))
                .collect::<Vec<_>>()
                .join(" "),
            None => "?".to_owned(),
        })
        .collect()
}

/// The plain IPA of words read one by one, for comparison.
fn words(text: &str) -> String {
    // Each word read on its own, as a sentence of one word.
    text.split(' ')
        .map(|word| read(&[(word, Word)]).remove(0))
        .collect::<Vec<_>>()
        .join(" ")
}

fn number(text: &str) -> String {
    read(&[(text, Number)]).remove(0)
}

#[test]
fn cardinals() {
    assert_eq!(number("0"), words("zero"));
    assert_eq!(number("7"), words("seven"));
    assert_eq!(number("42"), words("forty two"));
    assert_eq!(number("105"), words("one hundred five"));
    assert_eq!(number("1,000"), words("one thousand"));
    assert_eq!(number("2,300,015"), words("two million three hundred thousand fifteen"));
    assert_eq!(number("007"), words("zero zero seven"));
}

#[test]
fn years_decades_ordinals_and_decimals() {
    assert_eq!(number("1999"), words("nineteen ninety nine"));
    assert_eq!(number("1905"), words("nineteen oh five"));
    assert_eq!(number("1900"), words("nineteen hundred"));
    assert_eq!(number("2005"), words("two thousand five"));
    assert_eq!(number("2024"), words("twenty twenty four"));
    assert_eq!(number("1066"), words("one thousand sixty six"));
    assert_eq!(number("1990s"), words("nineteen nineties"));
    assert_eq!(number("80s"), words("eighties"));
    assert_eq!(number("1st"), words("first"));
    assert_eq!(number("22nd"), words("twenty second"));
    assert_eq!(number("103rd"), words("one hundred third"));
    assert_eq!(number("12th"), words("twelfth"));
    assert_eq!(number("40th"), words("fortieth"));
    assert_eq!(number("3.14"), words("three point one four"));
}

#[test]
fn money_reads_the_amount_then_the_currency() {
    assert_eq!(read(&[("$", Symbol), ("5", Number)]), ["", &words("five dollars")]);
    assert_eq!(read(&[("$", Symbol), ("1", Number)]), ["", &words("one dollar")]);
    assert_eq!(
        read(&[("$", Symbol), ("3.50", Number)]),
        ["", &words("three dollars and fifty cents")]
    );
    assert_eq!(read(&[("$", Symbol), ("0.01", Number)]), ["", &words("one cent")]);
    assert_eq!(
        read(&[("£", Symbol), ("2.99", Number)]),
        ["", &words("two pounds and ninety nine pence")]
    );
    assert_eq!(
        read(&[("€", Symbol), ("1.5", Number)]),
        ["", &words("one point five euros")]
    );
    assert_eq!(
        read(&[("¥", Symbol), ("1,000", Number)]),
        ["", &words("one thousand yen")]
    );
}

#[test]
fn symbols_by_name() {
    assert_eq!(read(&[("10", Number), ("%", Symbol)]), [words("ten"), words("percent")]);
    assert_eq!(read(&[("&", Symbol)]), [words("and")]);
    assert_eq!(read(&[("§", Symbol)]), ["?"]);
    assert_eq!(read(&[("$", Symbol)]), ["?"], "a currency sign without an amount");
}

#[test]
fn malformed_numbers_cannot_be_read() {
    for text in ["3.", "4x", "99999999999999999999999", "1.2.3", "1--2", "a:b"] {
        assert_eq!(number(text), "?", "{text}");
    }
}

#[test]
fn punctuation_is_silent_and_separates_runs() {
    let read = read(&[("in", Word), (",", Punctuation), ("the", Word), ("end", Word)]);
    assert_eq!(read[1], "");
    assert_eq!(read[2], "ði", "the before a vowel, in its own run");
}

#[test]
fn times_dates_ranges_fractions_and_digit_groups() {
    assert_eq!(number("10:30"), words("ten thirty"));
    assert_eq!(number("9:05"), words("nine oh five"));
    assert_eq!(number("10:00"), words("ten o'clock"));
    assert_eq!(number("14:00"), words("fourteen hundred"));
    assert_eq!(number("12/05/2024"), words("December fifth twenty twenty four"));
    assert_eq!(
        number("25/12/1999"),
        words("December twenty fifth nineteen ninety nine"),
        "only one order works"
    );
    assert_eq!(number("7/4/76"), words("July fourth seventy six"));
    assert_eq!(number("1/2"), words("one half"));
    assert_eq!(number("3/4"), words("three quarters"));
    assert_eq!(number("2/3"), words("two thirds"));
    // "to" is read in running text: unstressed.
    let range = format!("{} tə {}", words("nineteen ninety"), words("nineteen ninety five"));
    assert_eq!(number("1990-1995"), range);
    assert_eq!(number("555-12-34"), words("five five five one two three four"));
    assert_eq!(number("2,50"), words("two fifty"));
    assert_eq!(
        number("13/45/2000"),
        words("thirteen slash forty five slash two thousand"),
        "no such date"
    );
    assert_eq!(
        number("25:99"),
        words("twenty five ninety nine"),
        "no such time: groups"
    );
    assert_eq!(read(&[("C", Word), ("/", Symbol), ("D", Word)])[1], words("slash"));
}

#[test]
fn roman_numerals_and_abbreviations() {
    let word = |text: &str| read(&[(text, Word)]).remove(0);
    assert_eq!(word("IV"), words("four"));
    assert_eq!(word("XIV"), words("fourteen"));
    assert_eq!(word("XL"), words("forty"));
    assert_eq!(word("II"), words("two"));
    assert_ne!(word("IIII"), words("four"), "not the usual form: read as a word");
    assert_eq!(
        read(&[("e.g", Word)])[0],
        format!("{} {}", words("for"), words("example")).replace("ˈfɔɹ", "fɚ")
    );
    assert_eq!(read(&[("No", Word), ("5", Number)]), [words("number"), words("five")]);
    assert_eq!(read(&[("No", Word), ("way", Word)])[0], words("no"));
}

#[test]
fn currency_codes_after_an_amount() {
    assert_eq!(
        read(&[("15", Number), ("EUR", Word)]),
        [words("fifteen euros"), String::new()]
    );
    assert_eq!(
        read(&[("1", Number), ("USD", Word)]),
        [words("one dollar"), String::new()]
    );
}

#[test]
fn footnote_marks_and_section_breaks_are_silent() {
    assert_eq!(read(&[("*", Symbol), ("*", Symbol), ("*", Symbol)]), ["", "", ""]);
    assert_eq!(read(&[("word", Word), ("†", Symbol)])[1], "");
}

#[test]
fn units_titles_addresses_and_dates_in_context() {
    let ipa = |tokens: &[(&str, jespersen_text::TokenKind)]| {
        read(tokens).join(" ").split_whitespace().collect::<Vec<_>>().join(" ")
    };
    assert_eq!(ipa(&[("5", Number), ("km", Word)]), words("five kilometres"));
    assert_eq!(ipa(&[("1", Number), ("kg", Word)]), words("one kilogram"));
    assert_eq!(ipa(&[("20km", Number)]), words("twenty kilometres"));
    assert_eq!(ipa(&[("-5", Number)]), words("minus five"));
    assert_eq!(ipa(&[("+3", Number)]), words("plus three"));
    assert_eq!(
        ipa(&[("20", Number), ("°", Symbol), ("C", Word)]),
        words("twenty degrees Celsius")
    );
    assert_eq!(
        ipa(&[("12", Number), ("km", Word), ("/", Symbol), ("h", Word)])
            .split(' ')
            .count(),
        4,
        "twelve kilometres per hour"
    );
    assert_eq!(read(&[("Dr", Word), ("Jones", Word)])[0], words("doctor"));
    assert_eq!(read(&[("Main", Word), ("Dr", Word)])[1], words("drive"));
    assert_eq!(read(&[("St", Word), ("Peter", Word)])[0], words("saint"));
    assert_eq!(read(&[("Acme", Word), ("Inc", Word)])[1], words("incorporated"));
    assert_eq!(
        read(&[("Jan", Word), ("5", Number)]),
        [words("January"), words("fifth")]
    );
    assert_eq!(
        read(&[("5", Number), ("January", Word)])[0].split(' ').count(),
        3,
        "the fifth of"
    );
    assert_eq!(
        read(&[("Jan", Word)])[0],
        ipa(&[("Jan", Word)]),
        "a name without a day stays a name"
    );
    let address = read(&[("john.smith@example.co.uk", Word)]).remove(0);
    assert!(address.contains(&words("smith")) && address.contains(&words("example")));
    assert_eq!(
        address.split(' ').count(),
        11,
        "john dot smith at example dot c o dot u k"
    );
    let url = read(&[("https://www.example.com/page", Word)]).remove(0);
    assert!(url.starts_with(&words("W")), "www spelled, the scheme silent: {url}");
}

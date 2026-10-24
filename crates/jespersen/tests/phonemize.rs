use jespersen::{
    BathVowel, Error, Language, LexiconError, Notation, Script, TokenKind, WriteOptions, phonemize, write,
};

fn espeak() -> WriteOptions {
    WriteOptions {
        notation: Notation::Espeak,
        drop_punctuation: false,
    }
}

#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn read(text: &str, options: WriteOptions) -> String {
    write(&phonemize(text, Language::AmericanEnglish).unwrap(), options).unwrap()
}

#[test]
fn punctuation_keeps_its_place_and_spacing() {
    assert_eq!(read("Hello, world.", espeak()), "həlˈoʊ, wˈɜːld.");
    assert_eq!(read("Hello, world.", WriteOptions::default()), "həˈloʊ, ˈwɝld.");
    assert_eq!(read("“Water!” she said.", espeak()), "“wˈɔːɾɚ!” ʃiː sˈɛd.");
}

#[test]
fn punctuation_can_be_dropped_like_espeak_does() {
    let options = WriteOptions {
        drop_punctuation: true,
        ..espeak()
    };
    assert_eq!(read("Hello, world. It rained!", options), "həlˈoʊ wˈɜːld ɪt ɹˈeɪnd");
    assert_eq!(read("...", options), "");
}

#[test]
fn sentences_and_tokens_keep_their_spans() {
    let text = "Mr. Smith arrived. He sat.";
    let sentences = phonemize(text, Language::AmericanEnglish).unwrap();
    assert_eq!(sentences.len(), 2, "Mr. does not end a sentence");
    assert_eq!(sentences[0].text, "Mr. Smith arrived.");
    assert_eq!(sentences[1].span, 19..26);
    let smith = &sentences[0].tokens[1];
    assert_eq!(
        (smith.text, smith.kind, smith.script),
        ("Smith", TokenKind::Word, Script::Latin)
    );
    assert_eq!(smith.words.as_ref().map(Vec::len), Some(1));
    let stop = sentences[0].tokens.last().unwrap();
    assert_eq!((stop.kind, &stop.words), (TokenKind::Punctuation, &Some(vec![])));
}

#[test]
fn an_abbreviations_full_stop_belongs_to_the_word() {
    assert_eq!(read("Mr. Smith's water.", espeak()), "mˈɪstɚ smˈɪθs wˈɔːɾɚ.");
    let sentences = phonemize("Ask Mr. Smith.", Language::AmericanEnglish).unwrap();
    let mister = &sentences[0].tokens[1];
    assert_eq!((mister.text, mister.span.clone()), ("Mr.", 4..7));
    // At the end of the sentence, the stop is the sentence's too.
    assert_eq!(read("I met Mr.", espeak()), "aɪ mˈɛt mˈɪstɚ.");
}

#[test]
fn what_cannot_be_pronounced_yet_is_reported_once_in_order() {
    let sentences = phonemize("Zxqv paid § to жук.", Language::AmericanEnglish).unwrap();
    let error = write(&sentences, espeak()).unwrap_err();
    assert_eq!(error, Error::Unpronounced(vec!["§".into(), "жук".into()]));
    assert!(error.to_string().contains("cannot pronounce yet: §, жук"));
}

#[test]
fn empty_text_writes_nothing() {
    assert_eq!(read("", espeak()), "");
    assert_eq!(read("   ", espeak()), "");
}

#[test]
fn language_tags() {
    assert_eq!("en-US".parse::<Language>(), Ok(Language::AmericanEnglish));
    assert_eq!("EN_us".parse::<Language>(), Ok(Language::AmericanEnglish));
    assert_eq!(Language::AmericanEnglish.to_string(), "en-US");
    assert_eq!(
        "en-gb".parse::<Language>(),
        Ok(Language::BritishEnglish(BathVowel::ByNotation))
    );
    assert_eq!(Language::BritishEnglish(BathVowel::Broad).to_string(), "en-GB");
    let unsupported = "tlh".parse::<Language>().unwrap_err();
    assert_eq!(unsupported, Error::UnsupportedLanguage("tlh".into()));
    assert_eq!(
        unsupported.to_string(),
        "jespersen does not read the language \"tlh\" yet"
    );
}

#[test]
fn lexicon_errors_keep_their_source() {
    let error = Error::from(LexiconError::Damaged);
    assert!(std::error::Error::source(&error).is_some());
    assert_eq!(error.to_string(), LexiconError::Damaged.to_string());
    assert!(std::error::Error::source(&Error::Unpronounced(vec![])).is_none());
}

#[test]
fn joined_words_lose_their_space_only_in_espeak_notation() {
    assert_eq!(read("Look in the box.", espeak()), "lˈʊk ɪnðə bˈɑːks.");
    assert_eq!(read("Look in the box.", WriteOptions::default()), "ˈlʊk ɪn ðə ˈbɑks.");
    // Punctuation between them keeps them apart.
    assert_eq!(read("Look in, the box.", espeak()), "lˈʊk ɪn, ðə bˈɑːks.");
}

#[test]
fn numbers_money_and_symbols_are_read_as_words() {
    let ipa = WriteOptions::default();
    assert_eq!(read("It cost $3.50.", ipa), "ɪt ˈkɑst ˈθɹi ˈdɑlɚz ənd ˈfɪfti ˈsɛnts.");
    assert_eq!(
        read("In 1999, 10% left.", ipa),
        "ɪn ˈnaɪntin ˈnaɪnti ˈnaɪn, ˈtɛn pɚˈsɛnt ˈlɛft."
    );
    assert_eq!(read("The 3rd one.", ipa), "ðə ˈθɝd ˈwʌn.");
    let sentences = phonemize("Pay $5 now.", Language::AmericanEnglish).unwrap();
    let dollar = &sentences[0].tokens[1];
    assert_eq!(
        (dollar.text, dollar.words.as_ref().map(Vec::len)),
        ("$", Some(0)),
        "read with its amount"
    );
    assert_eq!(
        sentences[0].tokens[2].words.as_ref().map(Vec::len),
        Some(2),
        "five dollars"
    );
}

#[test]
fn hyphenated_words_are_joined_in_espeak_notation_and_hyphenated_in_ipa() {
    assert_eq!(read("A well-known plan.", espeak()), "ɐ wˈɛlnˈoʊn plˈæn.");
    assert_eq!(
        read("A well-known plan.", WriteOptions::default()),
        "ə ˈwɛl-ˈnoʊn ˈplæn."
    );
    // Read as running text: a flap across the parts, "the" before a vowel.
    assert_eq!(read("state-of-the-art", espeak()), "stˈeɪɾʌvðɪˈɑːɹt");
    // A single letter among the parts is the letter.
    assert_eq!(read("A-B", espeak()), "ˈeɪbˈiː");
}

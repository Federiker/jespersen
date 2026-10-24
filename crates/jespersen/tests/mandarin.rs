#![cfg(feature = "zh")]

use jespersen::{Error, Language, Notation, Script, TokenKind, ToneSandhi, WriteOptions, phonemize, write};

fn options(notation: Notation) -> WriteOptions {
    WriteOptions {
        notation,
        drop_punctuation: false,
    }
}

#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn read(text: &str, sandhi: ToneSandhi, notation: Notation) -> String {
    write(&phonemize(text, Language::Mandarin(sandhi)).unwrap(), options(notation)).unwrap()
}

#[test]
fn chinese_is_read_in_misakis_notation_with_latin_punctuation() {
    assert_eq!(
        read("银行里的钱不够，他只好去问问。", ToneSandhi::Written, Notation::Misaki),
        "i↗nxa↗ŋ li↓ tɤ ʨʰjɛ↗n pu↗ kou↘, tʰa→ ꭧɨ↓xau↓ ʨʰy↘ wə↘n wən."
    );
    assert_eq!(
        read("她说：“我明天不来了。”", ToneSandhi::Written, Notation::Misaki),
        "tʰa→ ʂwo→: “wo↓ mi↗ŋtʰjɛ→n pu↘ lai↗ lɤ.”"
    );
    assert_eq!(
        read("《红楼梦》（小说）", ToneSandhi::Written, Notation::Misaki),
        "“xʊ↗ŋ lou↗ mə↘ŋ” (ɕjau↓ʂwo→)"
    );
}

#[test]
fn plain_ipa_has_tone_letters_and_spoken_tones() {
    assert_eq!(read("你好！", ToneSandhi::Spoken, Notation::Ipa), "ni˧˥ xau˨˩˦!");
    assert_eq!(read("你好！", ToneSandhi::Written, Notation::Ipa), "ni˨˩˦ xau˨˩˦!");
}

#[test]
fn a_chinese_word_is_one_token() {
    let sentences = phonemize("我去银行。", Language::Mandarin(ToneSandhi::Spoken)).unwrap();
    let tokens: Vec<(&str, TokenKind, Script)> = sentences[0]
        .tokens
        .iter()
        .map(|token| (token.text, token.kind, token.script))
        .collect();
    assert_eq!(
        tokens,
        [
            ("我", TokenKind::Word, Script::Han),
            ("去", TokenKind::Word, Script::Han),
            ("银行", TokenKind::Word, Script::Han),
            ("。", TokenKind::Punctuation, Script::Common),
        ]
    );
    assert_eq!(sentences[0].tokens[2].span, 6..12);
    assert_eq!(sentences[0].language, Language::Mandarin(ToneSandhi::Spoken));
    // 50% is one token, said 百分之五十.
    let sentences = phonemize("50%的人", Language::Mandarin(ToneSandhi::Written)).unwrap();
    assert_eq!(sentences[0].tokens[0].text, "50%");
}

#[cfg(feature = "en")]
#[test]
fn english_inside_chinese_is_read_as_english() {
    assert_eq!(
        read("我用Python写代码。", ToneSandhi::Written, Notation::Misaki),
        "wo↓ jʊ↘ŋ pˈaɪθɑːn ɕje↓ tai↘ma↓."
    );
    assert_eq!(
        read("我用Python写代码。", ToneSandhi::Written, Notation::Ipa),
        "wo˨˩˦ jʊŋ˥˩ ˈpaɪθɑn ɕje˨˩˦ tai˥˩ma˨˩˦."
    );
}

#[test]
fn each_language_is_written_in_its_own_notations() {
    let mandarin = phonemize("你好", Language::Mandarin(ToneSandhi::Spoken)).unwrap();
    assert_eq!(
        write(&mandarin, options(Notation::Espeak)),
        Err(Error::UnsupportedNotation {
            language: "zh",
            notation: Notation::Espeak
        })
    );
    assert_eq!(
        Error::UnsupportedNotation {
            language: "zh",
            notation: Notation::Espeak
        }
        .to_string(),
        "jespersen does not write zh in the Espeak notation"
    );
    assert_eq!(
        Language::Mandarin(ToneSandhi::Spoken).notations(),
        [Notation::Ipa, Notation::Misaki, Notation::Pinyin, Notation::Zhuyin]
    );
    #[cfg(feature = "en")]
    {
        let english = phonemize("hello", Language::AmericanEnglish).unwrap();
        assert!(write(&english, options(Notation::Pinyin)).is_err());
    }
}

#[test]
fn mandarin_tags_are_read() {
    for tag in ["zh", "cmn", "zh-CN", "zh_Hans", "zh-Hant", "zh-TW"] {
        assert_eq!(
            tag.parse::<Language>().unwrap(),
            Language::Mandarin(ToneSandhi::Spoken),
            "{tag}"
        );
    }
    assert_eq!(Language::Mandarin(ToneSandhi::Written).to_string(), "zh");
    assert!(
        !"zh"
            .parse::<Language>()
            .unwrap()
            .notations()
            .contains(&Notation::Espeak)
    );
}

#[test]
fn what_cannot_be_read_is_reported() {
    let sentences = phonemize("你好§", Language::Mandarin(ToneSandhi::Spoken)).unwrap();
    assert_eq!(
        write(&sentences, options(Notation::Ipa)),
        Err(Error::Unpronounced(vec!["§".to_owned()]))
    );
}

#[test]
fn pinyin_and_zhuyin_spell_the_words() {
    assert_eq!(
        read("哪儿有银行？我们去西安。", ToneSandhi::Spoken, Notation::Pinyin),
        "nǎr yǒu yínháng? wǒmen qù xī'ān."
    );
    assert_eq!(
        read("哪儿有银行？", ToneSandhi::Written, Notation::Zhuyin),
        "ㄋㄚˇㄦ ㄧㄡˇ ㄧㄣˊㄏㄤˊ?"
    );
}

#[cfg(feature = "en")]
#[test]
fn english_inside_pinyin_is_written_in_plain_ipa() {
    assert_eq!(
        read("我用Python。", ToneSandhi::Spoken, Notation::Pinyin),
        "wǒ yòng ˈpaɪθɑn."
    );
}

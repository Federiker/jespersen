use jespersen_phonemes::{Notation, Pronunciation};
use jespersen_text::{TokenKind, sentences};

use crate::{
    ToneSandhi,
    numbers::{cardinal, decimal},
    phonemes::syllable,
    pinyin::{Initial, Pinyin, Rime},
    read_tokens,
};

/// Reads `text` as the tokeniser splits it and writes each reading's words, joined by spaces;
/// `?` for a token Mandarin does not read, and nothing for one covered by the token before.
fn said(text: &str, sandhi: ToneSandhi, notation: Notation) -> Vec<String> {
    let mut out = Vec::new();
    for sentence in sentences(text, |_| false) {
        let tokens: Vec<(&str, TokenKind)> = sentence.tokens.iter().map(|token| (token.text, token.kind)).collect();
        let readings = read_tokens(&tokens, sandhi).unwrap();
        let mut covered = 0;
        for reading in readings {
            match reading {
                Some(reading) => {
                    covered = reading.tokens - 1;
                    out.push(
                        reading
                            .words
                            .iter()
                            .map(|word: &Pronunciation| word.to_notation(notation))
                            .collect::<Vec<_>>()
                            .join(" "),
                    );
                }
                None if covered > 0 => covered -= 1,
                None => out.push("?".to_owned()),
            }
        }
    }
    out
}

/// The words of `text` in misaki's notation, tones as written, as one line.
fn misaki(text: &str) -> String {
    said(text, ToneSandhi::Written, Notation::Misaki)
        .into_iter()
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// The words of `text` in plain IPA, tones as spoken, as one line.
fn spoken(text: &str) -> String {
    said(text, ToneSandhi::Spoken, Notation::Ipa)
        .into_iter()
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn pinyin_spellings_are_taken_apart() {
    let parse = |text| Pinyin::parse(text).unwrap();
    assert_eq!(parse("xiu1").rime, Rime::Iou);
    assert_eq!(parse("gui4").rime, Rime::Uei);
    assert_eq!(parse("lun2").rime, Rime::Uen);
    assert_eq!(parse("ju3").rime, Rime::V);
    assert_eq!(parse("quan2").rime, Rime::Van);
    assert_eq!(parse("yun2").rime, Rime::Vn);
    assert_eq!(parse("you3").rime, Rime::Iou);
    assert_eq!(parse("wei4").rime, Rime::Uei);
    assert_eq!(parse("wen2").rime, Rime::Uen);
    assert_eq!(parse("yi1").rime, Rime::I);
    assert_eq!(parse("ying2").rime, Rime::Ing);
    assert_eq!(parse("wu3").rime, Rime::U);
    assert_eq!(parse("yue4").rime, Rime::Ve);
    assert_eq!(parse("ya1").rime, Rime::Ia);
    assert_eq!(parse("wang2").rime, Rime::Uang);
    assert_eq!(parse("nue4").rime, Rime::Ve);
    assert_eq!(parse("lve4").rime, Rime::Ve);
    assert_eq!(parse("lü4").rime, Rime::V);
    assert_eq!(parse("zhi1").rime, Rime::Apical);
    assert_eq!(parse("ri4").rime, Rime::Apical);
    assert_eq!(parse("er2").rime, Rime::Er);
    assert_eq!(parse("ng2").rime, Rime::Ng);
    assert_eq!(parse("ê2").rime, Rime::Eh);
    assert_eq!(parse("hm5").initial, Initial::H);
    assert_eq!(parse("hng5").rime, Rime::Ng);
    assert_eq!(parse("le").tone, 5);
    assert_eq!(parse("zhuang4").initial, Initial::Zh);
    assert!(Pinyin::parse("xyz1").is_none());
    assert!(Pinyin::parse("").is_none());
}

#[test]
fn every_syllable_of_the_pinyin_table_is_read() {
    let table = include_str!("../../../../data/upstream/kfcd-pinyin/pinyinbiao");
    for line in table.lines() {
        let syllable = line.split('\t').next().unwrap();
        assert!(Pinyin::parse(syllable).is_some(), "{syllable}");
    }
}

#[test]
fn syllables_are_written_as_misaki_and_ipa_write_them() {
    let both = |pinyin: &str| {
        let syllable = syllable(Pinyin::parse(pinyin).unwrap());
        let word = Pronunciation {
            syllables: vec![syllable],
        };
        (word.to_notation(Notation::Misaki), word.to_notation(Notation::Ipa))
    };
    for (pinyin, misaki, ipa) in [
        ("zhang4", "ꭧa↘ŋ", "ʈʂaŋ˥˩"),
        ("chi1", "ꭧʰɨ→", "ʈʂʰɻ̩˥˥"),
        ("ri4", "ɻɨ↘", "ʐɻ̩˥˩"),
        ("si1", "sɨ→", "sɹ̩˥˥"),
        ("ci2", "ʦʰɨ↗", "tsʰɹ̩˧˥"),
        ("er2", "ɚ↗", "ɚ˧˥"),
        ("yuan2", "ɥɛ↗n", "ɥɛn˧˥"),
        ("jiong3", "ʨjʊ↓ŋ", "tɕjʊŋ˨˩˦"),
        ("xiu1", "ɕjou→", "ɕjou˥˥"),
        ("gui4", "kwei↘", "kwei˥˩"),
        ("dun1", "twə→n", "twən˥˥"),
        ("bo2", "pwo↗", "pwo˧˥"),
        ("o1", "ɔ→", "ɔ˥˥"),
        ("lve4", "lɥe↘", "lɥe˥˩"),
        ("yun2", "y↗n", "yn˧˥"),
        ("qu4", "ʨʰy↘", "tɕʰy˥˩"),
        ("ying1", "i→ŋ", "iŋ˥˥"),
        ("weng1", "wə→ŋ", "wəŋ˥˥"),
        ("ma5", "ma", "ma"),
        ("ng2", "ŋ↗", "ŋ̍˧˥"),
        ("m2", "m↗", "m̩˧˥"),
        ("n2", "n↗", "n̩˧˥"),
        ("hm5", "xm", "xm̩"),
        ("ê2", "ɛ↗", "ɛ˧˥"),
        ("yo1", "jɔ→", "jɔ˥˥"),
        ("yai2", "jai↗", "jai˧˥"),
    ] {
        assert_eq!(both(pinyin), (misaki.to_owned(), ipa.to_owned()), "{pinyin}");
    }
}

#[test]
fn numbers_are_written_out_as_said() {
    for (digits, chinese) in [
        ("0", "零"),
        ("2", "二"),
        ("10", "十"),
        ("15", "十五"),
        ("20", "二十"),
        ("115", "一百一十五"),
        ("200", "二百"),
        ("1001", "一千零一"),
        ("1010", "一千零一十"),
        ("2000", "两千"),
        ("2024", "两千零二十四"),
        ("10005", "一万零五"),
        ("12345", "一万二千三百四十五"),
        ("20000", "两万"),
        ("100500", "十万零五百"),
        ("110000", "十一万"),
        ("100000001", "一亿零一"),
        ("300000000", "三亿"),
    ] {
        assert_eq!(cardinal(digits).as_deref(), Some(chinese), "{digits}");
    }
    assert_eq!(cardinal("12345678901234567"), None);
    assert_eq!(cardinal("12a"), None);
    assert_eq!(decimal("3.14").as_deref(), Some("三点一四"));
    assert_eq!(decimal("3."), None);
    assert_eq!(decimal("3"), None);
}

#[test]
fn words_are_found_and_read_whole() {
    // 行 is háng in a bank and xíng in walking.
    assert_eq!(misaki("银行"), "i↗nxa↗ŋ");
    assert_eq!(misaki("行走"), "ɕi↗ŋʦou↓");
    // Traditional characters read as the same words.
    assert_eq!(misaki("銀行"), "i↗nxa↗ŋ");
    assert_eq!(misaki("你好"), "ni↓ xau↓");
    // jespersen's own entries replace the sources'.
    assert_eq!(misaki("重庆"), "ꭧʰʊ↗ŋʨʰi↘ŋ");
    assert_eq!(misaki("一月"), "i→ɥe↘");
}

#[test]
fn a_word_written_over_several_tokens_is_given_to_the_first() {
    let tokens = [
        ("银", TokenKind::Word),
        ("行", TokenKind::Word),
        ("。", TokenKind::Punctuation),
    ];
    let readings = read_tokens(&tokens, ToneSandhi::Written).unwrap();
    assert_eq!(readings[0].as_ref().map(|reading| reading.tokens), Some(2));
    assert_eq!(readings[1], None);
    assert_eq!(readings[2].as_ref().map(|reading| reading.words.len()), Some(0));
    // A token of several characters whose word runs on into the next token.
    let tokens = [
        ("北京", TokenKind::Word),
        ("银", TokenKind::Word),
        ("行", TokenKind::Word),
    ];
    let readings = read_tokens(&tokens, ToneSandhi::Written).unwrap();
    assert_eq!(readings[0].as_ref().map(|reading| reading.tokens), Some(1));
    assert_eq!(readings[1].as_ref().map(|reading| reading.tokens), Some(2));
    let tokens = [
        ("银", TokenKind::Word),
        ("行朋", TokenKind::Word),
        ("友", TokenKind::Word),
    ];
    let readings = read_tokens(&tokens, ToneSandhi::Written).unwrap();
    assert_eq!(readings[0].as_ref().map(|reading| reading.tokens), Some(3));
}

#[test]
fn other_scripts_and_symbols_are_left_for_others() {
    assert_eq!(
        said("我用Python", ToneSandhi::Written, Notation::Misaki),
        ["wo↓", "jʊ↘ŋ", "?"]
    );
    assert_eq!(said("§", ToneSandhi::Written, Notation::Misaki), ["?"]);
}

#[test]
fn numbers_are_read_in_chinese() {
    // Years digit by digit, dates and days as numbers.
    assert_eq!(misaki("2024年"), "ɚ↘li↗ŋɚ↘sɨ↘ njɛ↗n");
    assert_eq!(misaki("2024/5/12"), "ɚ↘li↗ŋɚ↘sɨ↘njɛ↗nu↓ɥe↘ʂɨ↗ɚ↘ɻɨ↘");
    // A decimal reads its digits one by one, never as the word 一点 (a little).
    assert_eq!(misaki("1.75"), "i→tjɛ↓nʨʰi→u↓");
    assert_eq!(misaki("50%"), "pai↓fə→nꭧɨ→u↓ʂɨ↗");
    assert_eq!(misaki("2/3"), "sa→nfə→nꭧɨ→ɚ↘");
    assert_eq!(misaki("10:05"), "ʂɨ↗tjɛ↓nli↗ŋu↓fə→n");
    assert_eq!(misaki("10:30:00"), "ʂɨ↗tjɛ↓nsa→nʂɨ↗fə→nli↗ŋmjau↓");
    assert_eq!(misaki("10-20"), "ʂɨ↗tau↘ɚ↘ʂɨ↗");
    assert_eq!(misaki("-5"), "fu↘u↓");
    assert_eq!(misaki("+3"), "ꭧə↘ŋsa→n");
    assert_eq!(misaki("1,000,000"), "i→pai↓wa↘n");
    assert_eq!(misaki("１２"), "ʂɨ↗ɚ↘");
    // 2 counting things is 两; telephone numbers and codes are digits.
    assert_eq!(misaki("2个"), "lja↓ŋ kɤ↘");
    assert_eq!(misaki("2月"), "ɚ↘ ɥe↘");
    assert_eq!(misaki("13800138000"), "i→sa→npa→li↗ŋli↗ŋi→sa→npa→li↗ŋli↗ŋli↗ŋ");
    assert_eq!(misaki("007"), "li↗ŋli↗ŋʨʰi→");
    assert_eq!(misaki("25℃"), "ɚ↘ʂɨ↗u↓ ʂɤ↘ʂɨ↘tu↘");
    assert_eq!(misaki("1:2:3:4"), "?");
    assert_eq!(misaki("1.2.3"), "?");
}

#[test]
fn context_decides_some_characters_on_their_own() {
    // 地: the particle after a word of two characters or a character said twice, else the ground.
    assert!(misaki("快乐地玩").contains("kʰwai↘lɤ↘ tɤ"), "{}", misaki("快乐地玩"));
    assert!(misaki("慢慢地走").contains("tɤ"), "{}", misaki("慢慢地走"));
    assert!(misaki("坡地").ends_with("ti↘"), "{}", misaki("坡地"));
    // 个 is neutral after this and that.
    assert!(misaki("这个").ends_with("kɤ"), "{}", misaki("这个"));
    assert!(misaki("三个").ends_with("kɤ↘"), "{}", misaki("三个"));
}

#[test]
fn yi_and_bu_change_before_the_next_tone() {
    assert!(misaki("一辆车").starts_with("i↗"), "{}", misaki("一辆车"));
    assert!(misaki("一本书").starts_with("i↘"), "{}", misaki("一本书"));
    assert!(misaki("不去").starts_with("pu↗"), "{}", misaki("不去"));
    assert!(misaki("不来").starts_with("pu↘"), "{}", misaki("不来"));
    // Counting and ordinals keep 一's own tone; a number written out too.
    assert_eq!(misaki("一二三"), "i→ ɚ↘ sa→n");
    assert!(misaki("第一次").contains("i→"), "{}", misaki("第一次"));
    assert_eq!(misaki("1000000元"), "i→pai↓wa↘n ɥɛ↗n");
    // Between a verb said twice, 一 is neutral.
    assert!(misaki("看一看").contains(" i "), "{}", misaki("看一看"));
    // At the end of a run of speech it keeps its tone.
    assert!(misaki("统一。").contains("i→"), "{}", misaki("统一。"));
}

#[test]
fn spoken_tones_change_next_to_each_other() {
    // A third tone before a third is a second; one not before a pause is only low.
    assert_eq!(spoken("你好"), "ni˧˥ xau˨˩˦");
    assert_eq!(spoken("你来"), "ni˨˩ lai˧˥");
    assert_eq!(said("你好", ToneSandhi::Spoken, Notation::Misaki).concat(), "ni↗xau↓");
    // A neutral tone takes its pitch from the tone before it.
    assert_eq!(spoken("妈妈"), "ma˥˥ma˨");
    assert!(spoken("喜欢").ends_with("xwan˦"), "{}", spoken("喜欢"));
    assert_eq!(spoken("他们"), "tʰa˥˥mən˨");
    assert!(spoken("谢谢").ends_with("ɕje˩"), "{}", spoken("谢谢"));
    // At the start of a run there is nothing to take it from.
    assert_eq!(spoken("吗"), "ma");
    // Punctuation ends a run: the third tone before it keeps its dip.
    assert_eq!(spoken("好，好"), "xau˨˩˦ xau˨˩˦");
}

/// The words of `text` in pinyin or zhuyin, tones as spoken (the spelling is the same either way).
fn spelt(text: &str, notation: Notation) -> String {
    said(text, ToneSandhi::Spoken, notation)
        .into_iter()
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn everyday_words_and_example_words_are_known() {
    // jespersen's own everyday words, with their neutral tones, in both scripts.
    assert_eq!(spelt("东西", Notation::Pinyin), "dōngxi");
    assert_eq!(spelt("東西", Notation::Pinyin), "dōngxi");
    assert_eq!(spelt("头发", Notation::Pinyin), "tóufa");
    assert_eq!(spelt("頭髮", Notation::Pinyin), "tóufa");
    // 開放漢語字典's example words, with their entry's reading, in both scripts.
    assert_eq!(spelt("长大", Notation::Pinyin), "zhǎngdà");
    assert_eq!(spelt("種樹", Notation::Pinyin), "zhòngshù");
    assert_eq!(spelt("受不了", Notation::Pinyin), "shòubuliǎo");
}

#[test]
fn polyphonic_characters_are_read_by_context() {
    for (text, pinyin) in [
        ("快乐地玩", "kuàilè de wán"),
        ("坡地", "pō dì"),
        ("这个", "zhège"),
        ("他长得高", "tā zhǎng de gāo"),
        ("很长", "hěn cháng"),
        ("重来", "chónglái"),
        ("重写", "chóng xiě"),
        ("吃不了", "chī bu liǎo"),
        ("侍候不了", "shì hòu bù liǎo"),
        ("一天一天", "yì tiān yì tiān"),
        ("咳著", "ké zhe"),
        ("教给他", "jiāo gěi tā"),
        ("情理中的事", "qíng lǐ zhōngde shì"),
        ("我得走", "wǒ děi zǒu"),
        ("得了冠军", "déle guànjūn"),
        ("还书", "huán shū"),
        ("还要", "hái yào"),
        ("一只猫", "yì zhī māo"),
        ("只有", "zhǐyǒu"),
        ("种草", "zhòng cǎo"),
        ("数一数", "shǔ yi shu"),
        ("去过", "qù guo"),
        ("选为", "xuǎn wéi"),
        ("睡着", "shuìzháo"),
        ("问问", "wèn wen"),
        ("看一看", "kàn yi kan"),
        ("去不去", "qù bu qù"),
    ] {
        assert_eq!(spelt(text, Notation::Pinyin), pinyin, "{text}");
    }
}

#[test]
fn erhua_is_said_with_the_syllable_before() {
    assert_eq!(spoken("哪儿"), "naɻ˨˩˦");
    assert_eq!(spoken("一点儿"), "i˥˩tjaɻ˨˩˦");
    assert_eq!(spoken("花儿"), "xwaɻ˥˥");
    assert_eq!(spoken("小孩儿"), "ɕjau˨˩ xaɻ˧˥");
    assert_eq!(spoken("一会儿"), "i˧˥xweɻ˥˩");
    assert_eq!(spoken("有空儿"), "jou˨˩ kʰʊɻ˥˩");
    assert_eq!(spoken("今儿"), "tɕiəɻ˥˥");
    assert_eq!(spoken("事儿"), "ʂəɻ˥˩");
    // Written, 儿 stays a syllable of its own, as misaki writes it; both spell it r.
    assert_eq!(misaki("哪儿"), "na↓ɚ");
    assert_eq!(spelt("哪儿", Notation::Pinyin), "nǎr");
    assert_eq!(spelt("哪儿", Notation::Zhuyin), "ㄋㄚˇㄦ");
    assert_eq!(said("哪儿", ToneSandhi::Written, Notation::Pinyin).concat(), "nǎr");
    // 儿 in its own words is a syllable (女儿 nǚ'ér, 儿子).
    assert_eq!(spelt("女儿", Notation::Pinyin), "nǚ'ér");
}

#[test]
fn syllables_are_spelt_in_pinyin_and_zhuyin() {
    for (text, pinyin, zhuyin) in [
        ("银行", "yínháng", "ㄧㄣˊㄏㄤˊ"),
        ("西安", "xī'ān", "ㄒㄧㄢ"),
        ("绿色", "lǜsè", "ㄌㄩˋㄙㄜˋ"),
        ("女孩", "nǚhái", "ㄋㄩˇㄏㄞˊ"),
        ("学生", "xuésheng", "ㄒㄩㄝˊ˙ㄕㄥ"),
        ("知道", "zhīdao", "ㄓ˙ㄉㄠ"),
        ("要求", "yāoqiú", "ㄧㄠㄑㄧㄡˊ"),
        ("贵", "guì", "ㄍㄨㄟˋ"),
        ("穷", "qióng", "ㄑㄩㄥˊ"),
        ("月", "yuè", "ㄩㄝˋ"),
        ("远", "yuǎn", "ㄩㄢˇ"),
        ("云", "yún", "ㄩㄣˊ"),
        ("略", "lüè", "ㄌㄩㄝˋ"),
        ("温", "wēn", "ㄨㄣ"),
        ("翁", "wēng", "ㄨㄥ"),
        ("外", "wài", "ㄨㄞˋ"),
        ("五", "wǔ", "ㄨˇ"),
        ("耳", "ěr", "ㄦˇ"),
        ("呣", "ḿ", "ㄇˊ"),
        ("嗯", "ǹg", "ㄫˋ"),
        ("吗", "ma", "˙ㄇㄚ"),
        ("佛", "fó", "ㄈㄛˊ"),
        ("谁", "shéi", "ㄕㄟˊ"),
        ("哟", "yō", "ㄧㄛ"),
        ("欸", "ǎi", "ㄞˇ"),
    ] {
        assert_eq!(spelt(text, Notation::Pinyin), pinyin, "{text}");
        assert_eq!(spelt(text, Notation::Zhuyin), zhuyin, "{text}");
    }
    // Third-tone sandhi is not spelt; 一 and 不 changes are, as dictionaries write them.
    assert_eq!(spelt("你好", Notation::Pinyin), "nǐ hǎo");
    assert_eq!(spelt("一本书", Notation::Pinyin), "yì běn shū");
}

#[test]
fn on_a_tie_the_longer_last_word_wins() {
    // 我 的 确信 and 我 的确 信 are both three words with two of one character.
    assert_eq!(spelt("我的确信", Notation::Pinyin), "wǒde quèxìn");
    assert_eq!(spelt("质铺和药店", Notation::Pinyin), "zhìpu hé yàodiàn");
}

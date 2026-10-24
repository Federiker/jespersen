//! Characters whose reading on their own depends on the words around them (jespersen's own rules).

use crate::{pinyin::Pinyin, sandhi::RunWord};

/// Personal pronouns and adverbs before which 得 means "must" (我得走 wǒ děi zǒu).
const BEFORE_MUST: [&str; 18] = [
    "我", "你", "他", "她", "它", "我们", "你们", "他们", "咱们", "就", "还", "也", "都", "我們", "你們", "他們",
    "咱們", "還",
];
/// Single-character verbs often said twice or followed by 过 (看看 kànkan, 去过 qùguo).
pub(crate) const VERBS: &str = "数看想走问说谈试听坐等玩读写学找查尝闻摸动歇聊笑洗擦收理帮练比量算讲劝猜逛转跳敲拍吃喝睡来去到见做用住打唱念摇數問說談試聽讀寫學動幫練講勸轉來見搖";
/// What is measured in units (一只猫): numerals, this, that, how many, each.
const MEASURED_AFTER: &str = "零一二三四五六七八九十百千万两几每这那哪萬兩幾這";

/// Changes the readings of characters standing alone in a run of speech where the words around
/// them decide (each rule names its examples):
///
/// - 地 is the particle de after a word of two characters or more, or after a character said
///   twice (快乐地, 慢慢地), and dì (ground) otherwise (坡地: 坡 + 地);
/// - 个 is neutral after this, that and which (这个 zhè ge);
/// - 长 is zhǎng (grow) before 得, 出, 满 and 成 (长得高), cháng (long) otherwise;
/// - 重 is chóng (again) before 来, 写, 做, 演, 建 and 修;
/// - 了 is liǎo after 得 or 不 (吃不了, 侍候不了长衫主顾);
/// - 得 is děi between a pronoun or some adverbs and a word (我得走);
/// - 还 is huán (give back) before 给, 钱, 债 and 书;
/// - 只 is zhī (a measure word) after numerals, this, that and each (一只猫);
/// - 种 is zhòng (plant) before what is planted (种树, 种花);
/// - 数 is shǔ (count) before 着, 了, 过, 一, 不, 得 and 到;
/// - 过 is neutral after a verb of one character (去过 qùguo);
/// - 为 is wéi after 成, 作, 认, 称, 视, 以, 变 and 选 (认为 is a word; 选 为 is not);
/// - 着 is zháo after 睡 (睡着);
/// - 不 is neutral between a verb of one character and one character more (吃不了, 去不去);
/// - a verb of one character said twice is neutral the second time (看看 kànkan, 看一看 kàn yi
///   kan).
pub(crate) fn read_in_context(words: &mut [RunWord]) {
    for index in 0..words.len() {
        let text = |offset: isize| {
            index
                .checked_add_signed(offset)
                .and_then(|at| words.get(at))
                .map(|word| word.characters.as_str())
        };
        let (before, after) = (text(-1), text(1));
        let starts = |word: Option<&str>, set: &str| {
            word.and_then(|word| word.chars().next())
                .is_some_and(|c| set.contains(c))
        };
        let is =
            |word: Option<&str>, set: &str| word.is_some_and(|word| word.chars().count() == 1 && set.contains(word));
        let twice = index
            .checked_sub(2)
            .is_some_and(|first| words[first].characters == words[first + 1].characters);
        let verb_again = is(before, VERBS) && before == text(0)
            || text(-1) == Some("一") && is(text(-2), VERBS) && text(-2) == text(0);
        let reading = match words[index].characters.as_str() {
            _ if verb_again => {
                for syllable in &mut words[index].syllables {
                    syllable.tone = 5;
                }
                continue;
            }
            "地" if twice || before.is_some_and(|word| !words[index - 1].number && word.chars().count() >= 2) => "de5",
            "地" => "di4",
            "个" | "個" if is(before, "这那哪這") => "ge5",
            "长" | "長" if is(after, "得出满成滿") => "zhang3",
            "重" if is(after, "来写做演建修來寫") => "chong2",
            "了" if is(before, "得不") => "liao3",
            "得" if before.is_some_and(|word| BEFORE_MUST.contains(&word)) && after.is_some() => "dei3",
            "还" | "還" if is(after, "给钱债书給錢債書") => "huan2",
            "只" | "隻"
                if before.is_some_and(|word| word.chars().all(|c| MEASURED_AFTER.contains(c))) && after.is_some() =>
            {
                "zhi1"
            }
            "种" | "種" if starts(after, "树花菜地田草粮瓜豆稻麦樹糧麥") => "zhong4",
            "数" | "數" if is(after, "着了过一不得到著過") => "shu3",
            "过" | "過" if is(before, VERBS) => "guo5",
            "为" | "為" if is(before, "成作认称视以变选認稱視變選") => "wei2",
            // Between a verb and what it reaches, or the verb again (吃不了, 去不去).
            "不" if is(before, VERBS) && after.is_some_and(|word| word.chars().count() == 1) => "bu5",
            "着" | "著" if before == Some("睡") => "zhao2",
            _ => continue,
        };
        if let Some(pinyin) = Pinyin::parse(reading) {
            words[index].syllables = vec![pinyin];
        }
    }
}

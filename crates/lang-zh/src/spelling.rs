//! How a syllable is spelt in pinyin (with tone marks) and in zhuyin.

use jespersen_phonemes::Spelling;

use crate::pinyin::{Initial, Pinyin, Rime};

/// The spellings of `pinyin`, with its tone; `erhua` for a neutral 儿 said as part of the syllable
/// before (哪儿 nǎr), which both spell as an r only.
pub(crate) fn spelling(pinyin: Pinyin, erhua: bool) -> Spelling {
    if erhua {
        return Spelling {
            pinyin: "r".to_owned(),
            zhuyin: "ㄦ".to_owned(),
        };
    }
    Spelling {
        pinyin: with_tone_mark(&letters(pinyin), pinyin.tone),
        zhuyin: zhuyin(pinyin),
    }
}

/// A syllable's pinyin letters, without its tone: y and w where it starts with its vowel, the
/// contractions iu, ui and un, and ü written u after j, q, x and y.
fn letters(pinyin: Pinyin) -> String {
    let initial = match pinyin.initial {
        Initial::None => "",
        Initial::B => "b",
        Initial::P => "p",
        Initial::M => "m",
        Initial::F => "f",
        Initial::D => "d",
        Initial::T => "t",
        Initial::N => "n",
        Initial::L => "l",
        Initial::G => "g",
        Initial::K => "k",
        Initial::H => "h",
        Initial::J => "j",
        Initial::Q => "q",
        Initial::X => "x",
        Initial::Zh => "zh",
        Initial::Ch => "ch",
        Initial::Sh => "sh",
        Initial::R => "r",
        Initial::Z => "z",
        Initial::C => "c",
        Initial::S => "s",
    };
    let alone = pinyin.initial == Initial::None;
    let after_jqx = matches!(pinyin.initial, Initial::J | Initial::Q | Initial::X);
    let rime = match pinyin.rime {
        Rime::A => "a",
        Rime::O => "o",
        Rime::E => "e",
        Rime::Eh => "ê",
        Rime::Ai => "ai",
        Rime::Ei => "ei",
        Rime::Ao => "ao",
        Rime::Ou => "ou",
        Rime::An => "an",
        Rime::En => "en",
        Rime::Ang => "ang",
        Rime::Eng => "eng",
        Rime::Ong => "ong",
        Rime::Er => "er",
        Rime::I | Rime::Apical if alone => "yi",
        Rime::I | Rime::Apical => "i",
        Rime::Ia if alone => "ya",
        Rime::Ia => "ia",
        Rime::Ie if alone => "ye",
        Rime::Ie => "ie",
        Rime::Iao if alone => "yao",
        Rime::Iao => "iao",
        Rime::Iou if alone => "you",
        Rime::Iou => "iu",
        Rime::Ian if alone => "yan",
        Rime::Ian => "ian",
        Rime::In if alone => "yin",
        Rime::In => "in",
        Rime::Iang if alone => "yang",
        Rime::Iang => "iang",
        Rime::Ing if alone => "ying",
        Rime::Ing => "ing",
        Rime::Iong if alone => "yong",
        Rime::Iong => "iong",
        Rime::Io if alone => "yo",
        Rime::Io => "io",
        Rime::Iai if alone => "yai",
        Rime::Iai => "iai",
        Rime::U if alone => "wu",
        Rime::U => "u",
        Rime::Ua if alone => "wa",
        Rime::Ua => "ua",
        Rime::Uo if alone => "wo",
        Rime::Uo => "uo",
        Rime::Uai if alone => "wai",
        Rime::Uai => "uai",
        Rime::Uei if alone => "wei",
        Rime::Uei => "ui",
        Rime::Uan if alone => "wan",
        Rime::Uan => "uan",
        Rime::Uen if alone => "wen",
        Rime::Uen => "un",
        Rime::Uang if alone => "wang",
        Rime::Uang => "uang",
        Rime::Ueng if alone => "weng",
        Rime::Ueng => "ueng",
        Rime::V if alone => "yu",
        Rime::V if after_jqx => "u",
        Rime::V => "ü",
        Rime::Ve if alone => "yue",
        Rime::Ve if after_jqx => "ue",
        Rime::Ve => "üe",
        Rime::Van if alone => "yuan",
        Rime::Van if after_jqx => "uan",
        Rime::Van => "üan",
        Rime::Vn if alone => "yun",
        Rime::Vn if after_jqx => "un",
        Rime::Vn => "ün",
        Rime::M => "m",
        Rime::N => "n",
        Rime::Ng => "ng",
    };
    format!("{initial}{rime}")
}

/// `letters` with the tone marked where pinyin marks it: on a or e, else on the o of ou, else on
/// the last vowel; on the m or n of a syllable without a vowel. The neutral tone is unmarked.
fn with_tone_mark(letters: &str, tone: u8) -> String {
    if !(1..=4).contains(&tone) {
        return letters.to_owned();
    }
    let vowels = |c: char| "aeiouüê".contains(c);
    let at = letters
        .find(['a', 'e', 'ê'])
        .or_else(|| letters.find("ou"))
        .or_else(|| letters.rfind(vowels))
        .or_else(|| letters.rfind(['m', 'n']));
    let Some(at) = at else {
        return letters.to_owned();
    };
    let mut out = String::with_capacity(letters.len() + 2);
    for (index, c) in letters.char_indices() {
        if index != at {
            out.push(c);
            continue;
        }
        let marked = match (c, tone) {
            ('a', 1) => "ā",
            ('a', 2) => "á",
            ('a', 3) => "ǎ",
            ('a', 4) => "à",
            ('e', 1) => "ē",
            ('e', 2) => "é",
            ('e', 3) => "ě",
            ('e', 4) => "è",
            ('i', 1) => "ī",
            ('i', 2) => "í",
            ('i', 3) => "ǐ",
            ('i', 4) => "ì",
            ('o', 1) => "ō",
            ('o', 2) => "ó",
            ('o', 3) => "ǒ",
            ('o', 4) => "ò",
            ('u', 1) => "ū",
            ('u', 2) => "ú",
            ('u', 3) => "ǔ",
            ('u', 4) => "ù",
            ('ü', 1) => "ǖ",
            ('ü', 2) => "ǘ",
            ('ü', 3) => "ǚ",
            ('ü', 4) => "ǜ",
            ('ê', 2) => "ế",
            ('ê', 4) => "ề",
            ('n', 2) => "ń",
            ('n', 3) => "ň",
            ('n', 4) => "ǹ",
            ('m', 2) => "ḿ",
            // The rest have no precomposed letter: the letter and a combining mark.
            (c, tone) => {
                out.push(c);
                out.push(['\u{0304}', '\u{0301}', '\u{030C}', '\u{0300}'][usize::from(tone) - 1]);
                continue;
            }
        };
        out.push_str(marked);
    }
    out
}

/// A syllable in zhuyin: its initial, its rime and its tone mark (none for the first tone, ˙
/// before a neutral syllable).
fn zhuyin(pinyin: Pinyin) -> String {
    let initial = match pinyin.initial {
        Initial::None => "",
        Initial::B => "ㄅ",
        Initial::P => "ㄆ",
        Initial::M => "ㄇ",
        Initial::F => "ㄈ",
        Initial::D => "ㄉ",
        Initial::T => "ㄊ",
        Initial::N => "ㄋ",
        Initial::L => "ㄌ",
        Initial::G => "ㄍ",
        Initial::K => "ㄎ",
        Initial::H => "ㄏ",
        Initial::J => "ㄐ",
        Initial::Q => "ㄑ",
        Initial::X => "ㄒ",
        Initial::Zh => "ㄓ",
        Initial::Ch => "ㄔ",
        Initial::Sh => "ㄕ",
        Initial::R => "ㄖ",
        Initial::Z => "ㄗ",
        Initial::C => "ㄘ",
        Initial::S => "ㄙ",
    };
    let rime = match pinyin.rime {
        Rime::A => "ㄚ",
        Rime::O => "ㄛ",
        Rime::E => "ㄜ",
        Rime::Eh => "ㄝ",
        Rime::Ai => "ㄞ",
        Rime::Ei => "ㄟ",
        Rime::Ao => "ㄠ",
        Rime::Ou => "ㄡ",
        Rime::An => "ㄢ",
        Rime::En => "ㄣ",
        Rime::Ang => "ㄤ",
        Rime::Eng => "ㄥ",
        Rime::Ong | Rime::Ueng => "ㄨㄥ",
        Rime::Er => "ㄦ",
        Rime::I => "ㄧ",
        // zhi, chi, shi, ri, zi, ci and si are their initial alone.
        Rime::Apical => "",
        Rime::Ia => "ㄧㄚ",
        Rime::Ie => "ㄧㄝ",
        Rime::Iao => "ㄧㄠ",
        Rime::Iou => "ㄧㄡ",
        Rime::Ian => "ㄧㄢ",
        Rime::In => "ㄧㄣ",
        Rime::Iang => "ㄧㄤ",
        Rime::Ing => "ㄧㄥ",
        Rime::Iong => "ㄩㄥ",
        Rime::Io => "ㄧㄛ",
        Rime::Iai => "ㄧㄞ",
        Rime::U => "ㄨ",
        Rime::Ua => "ㄨㄚ",
        Rime::Uo => "ㄨㄛ",
        Rime::Uai => "ㄨㄞ",
        Rime::Uei => "ㄨㄟ",
        Rime::Uan => "ㄨㄢ",
        Rime::Uen => "ㄨㄣ",
        Rime::Uang => "ㄨㄤ",
        Rime::V => "ㄩ",
        Rime::Ve => "ㄩㄝ",
        Rime::Van => "ㄩㄢ",
        Rime::Vn => "ㄩㄣ",
        Rime::M => "ㄇ",
        Rime::N => "ㄋ",
        Rime::Ng => "ㄫ",
    };
    match pinyin.tone {
        2 => format!("{initial}{rime}ˊ"),
        3 => format!("{initial}{rime}ˇ"),
        4 => format!("{initial}{rime}ˋ"),
        5 => format!("˙{initial}{rime}"),
        _ => format!("{initial}{rime}"),
    }
}

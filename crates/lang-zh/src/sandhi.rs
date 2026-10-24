//! Tones in running speech: the changes a tone undergoes next to others, and the pitch of neutral
//! tones.

use jespersen_phonemes::{Pronunciation, Tone};

use crate::{
    context::VERBS,
    phonemes::{TONES, syllable, with_erhua},
    pinyin::{Pinyin, Rime},
    spelling::spelling,
};

/// Which tones a reading carries.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ToneSandhi {
    /// The tones as spoken: a third tone before another becomes a second (你好 ní hǎo), a third
    /// tone not before a pause is only low (˨˩), 一 and 不 change before the tone that follows
    /// (一个 yí gè, 不是 bú shì), and a neutral tone takes its pitch from the syllable before.
    #[default]
    Spoken,
    /// The tones as dictionaries write them: 一 and 不 change as in the spoken tones, but a third
    /// tone stays a third and neutral tones have no pitch. Close to what misaki writes, which
    /// Kokoro's Chinese voices were trained on.
    Written,
}

/// A word of a run of speech (text between pauses) and where its syllables come from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RunWord {
    pub characters: String,
    pub syllables: Vec<Pinyin>,
    /// A number written out (一 in 十一 keeps its first tone).
    pub number: bool,
}

const HALF_THIRD: Tone = Tone::new(&[2, 1]);
/// A neutral tone's pitch after each of the four tones.
const NEUTRAL_AFTER: [Tone; 4] = [
    Tone::neutral(&[2]),
    Tone::neutral(&[3]),
    Tone::neutral(&[4]),
    Tone::neutral(&[1]),
];

/// The words of a run of speech, each said with its tones.
pub(crate) fn pronounce(words: &[RunWord], sandhi: ToneSandhi) -> Vec<Pronunciation> {
    let written: Vec<u8> = words
        .iter()
        .flat_map(|word| word.syllables.iter().map(|pinyin| pinyin.tone))
        .collect();
    // The tones as dictionaries write them, which pinyin and zhuyin spell.
    let dictionary = yi_and_bu(words, &written);
    let tones: Vec<Tone> = match sandhi {
        ToneSandhi::Written => dictionary.iter().map(|&tone| TONES[usize::from(tone) - 1]).collect(),
        ToneSandhi::Spoken => {
            // A 儿 said with the syllable before is no syllable of its own to the tones (哪儿 nǎr keeps
            // its full third tone at a pause).
            let joined: Vec<bool> = words
                .iter()
                .flat_map(|word| {
                    word.syllables
                        .iter()
                        .enumerate()
                        .map(|(index, pinyin)| index > 0 && pinyin.rime == Rime::Er && pinyin.tone == 5)
                })
                .collect();
            let kept: Vec<u8> = dictionary
                .iter()
                .zip(&joined)
                .filter(|(_, joined)| !**joined)
                .map(|(tone, _)| *tone)
                .collect();
            let mut said = spoken(&kept).into_iter();
            joined
                .iter()
                .map(|&joined| {
                    if joined {
                        TONES[4]
                    } else {
                        said.next().unwrap_or(TONES[4])
                    }
                })
                .collect()
        }
    };
    let mut tones = tones.into_iter().zip(dictionary);
    words
        .iter()
        .map(|word| {
            let mut syllables = Vec::with_capacity(word.syllables.len());
            let mut pinyin = word.syllables.iter().enumerate().peekable();
            while let Some((index, &this)) = pinyin.next() {
                let (tone, written) = tones.next().unzip();
                let this = Pinyin {
                    tone: written.unwrap_or(this.tone),
                    ..this
                };
                let is_erhua = |pinyin: &Pinyin| pinyin.rime == Rime::Er && pinyin.tone == 5;
                // Spoken, a neutral 儿 after a syllable of its word is said as part of it (哪儿 nǎr).
                let joined = sandhi == ToneSandhi::Spoken && pinyin.next_if(|(_, next)| is_erhua(next)).is_some();
                let mut syllable = if joined { with_erhua(this) } else { syllable(this) };
                let mut spelt = spelling(this, index > 0 && is_erhua(&this));
                if joined {
                    tones.next();
                    spelt.pinyin.push('r');
                    spelt.zhuyin.push('ㄦ');
                }
                syllable.tone = tone.or(syllable.tone);
                syllable.spelling = Some(spelt);
                syllables.push(syllable);
            }
            Pronunciation { syllables }
        })
        .collect()
}

/// The `written` tones of `words`, with 一 and 不 on their own taking the tone the next syllable
/// calls for (一个 yí gè, 一天 yì tiān, 不是 bú shì), and 一 between a verb said twice neutral
/// (看一看, but 一天一天 yì tiān yì tiān).
fn yi_and_bu(words: &[RunWord], written: &[u8]) -> Vec<u8> {
    let mut tones = written.to_vec();
    let mut at = 0;
    for (index, word) in words.iter().enumerate() {
        let next = written.get(at + word.syllables.len()).copied();
        // A neutral 一 or 不 (吃不了, as the context made it) does not change.
        if !word.number
            && written.get(at) != Some(&5)
            && let Some(next) = next
        {
            let around = |offset: isize| {
                index
                    .checked_add_signed(offset)
                    .and_then(|index| words.get(index))
                    .map(|word| word.characters.as_str())
            };
            // Counting (一二三), dates (一月) and ordinals (第一) keep 一's own tone.
            let counted = |word: Option<&str>| {
                word.is_some_and(|word| {
                    word.chars()
                        .all(|c| "零一二三四五六七八九十百千万亿两第月号".contains(c))
                })
            };
            match word.characters.as_str() {
                "一" if counted(around(-1)) || counted(around(1)) => {}
                "一" if around(-1).is_some_and(|verb| verb.chars().count() == 1 && VERBS.contains(verb))
                    && around(-1) == around(1) =>
                {
                    tones[at] = 5;
                }
                "一" | "不" if next == 4 => tones[at] = 2,
                "一" if (1..=3).contains(&next) => tones[at] = 4,
                _ => {}
            }
        }
        at += word.syllables.len();
    }
    tones
}

/// The spoken tones, from `tones` with 一 and 不 changed: a third tone before another is said as a
/// second, a third tone not before a pause is only low, and a neutral tone is at the pitch the
/// tone before it leaves.
fn spoken(tones: &[u8]) -> Vec<Tone> {
    let mut tones = tones.to_vec();
    let before = tones.clone();
    for (index, pair) in before.windows(2).enumerate() {
        if pair == [3, 3] {
            tones[index] = 2;
        }
    }
    let mut out = Vec::with_capacity(tones.len());
    for (index, &tone) in tones.iter().enumerate() {
        out.push(match tone {
            3 if index + 1 < tones.len() => HALF_THIRD,
            5 => match index.checked_sub(1).map(|before| tones[before]) {
                Some(before @ 1..=4) => NEUTRAL_AFTER[usize::from(before) - 1],
                Some(_) => NEUTRAL_AFTER[0],
                None => TONES[4],
            },
            tone => TONES[usize::from(tone) - 1],
        });
    }
    out
}

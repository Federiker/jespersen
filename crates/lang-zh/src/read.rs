use jespersen_lexicon::LexiconError;
use jespersen_phonemes::Pronunciation;
use jespersen_text::{Script, TokenKind};

use crate::{
    context::read_in_context,
    lexicon::reading,
    numbers::{cardinal, decimal, digit_by_digit},
    pinyin::Pinyin,
    sandhi::{RunWord, ToneSandhi, pronounce},
    segment::segment,
};

/// How one token, and the tokens after it that belong to the same word, are said.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenReading {
    /// The words said: one for a word (银行), several for a number written out.
    pub words: Vec<Pronunciation>,
    /// How many tokens, from this one on, the words are for: Chinese writes words without spaces,
    /// so a word of two characters is two tokens (银, 行), and 50% is two (50, %).
    pub tokens: usize,
}

/// Characters after a number that count things: a 2 before them is 两 (两个, 两年, 两点).
const COUNTED: &str = "个只本次位天年种条件张点岁份名头辆双对倍斤米段层句碗杯瓶块根家所场周遍片样部台架间户群步";

/// One reading waiting for the end of its run: the token it starts at, how many it covers, and
/// its words.
struct Pending {
    token: usize,
    tokens: usize,
    words: Option<Vec<RunWord>>,
}

/// Reads a sentence's tokens (text and kind). Each token gets its reading, or `None` when it is
/// not Mandarin's to read (Latin letters, read by another language) or cannot be read; a token
/// covered by the reading of one before it gets `None` too.
///
/// Runs of Chinese characters are split into words by the lexicon; numbers are written out in
/// Chinese; punctuation is read as silence, and ends a run of speech for tone sandhi.
///
/// # Errors
///
/// When the embedded lexicon is damaged.
pub fn read_tokens(
    tokens: &[(&str, TokenKind)],
    sandhi: ToneSandhi,
) -> Result<Vec<Option<TokenReading>>, LexiconError> {
    let mut out: Vec<Option<TokenReading>> = vec![None; tokens.len()];
    let mut run: Vec<Pending> = Vec::new();
    let mut index = 0;
    while index < tokens.len() {
        let (text, kind) = tokens[index];
        if Script::of_text(text) == Script::Han {
            let end = (index..tokens.len())
                .find(|&at| Script::of_text(tokens[at].0) != Script::Han)
                .unwrap_or(tokens.len());
            run.extend(han(&tokens[index..end], index)?);
            index = end;
            continue;
        }
        let next = tokens.get(index + 1).map(|token| token.0);
        let said = match kind {
            TokenKind::Number => number(text, next).map(|(said, covered)| (said, covered, true)),
            TokenKind::Symbol => symbol(text).map(|words| (words.to_owned(), 1, false)),
            _ => None,
        };
        if let Some((characters, covered, number)) = said {
            let characters: Vec<char> = characters.chars().collect();
            let words = if number {
                number_word(&characters)?
            } else {
                words(&characters)?
            };
            run.push(Pending {
                token: index,
                tokens: covered,
                words,
            });
            index += covered;
            continue;
        }
        flush(&mut run, &mut out, sandhi);
        if kind == TokenKind::Punctuation {
            out[index] = Some(TokenReading {
                words: Vec::new(),
                tokens: 1,
            });
        }
        index += 1;
    }
    flush(&mut run, &mut out, sandhi);
    Ok(out)
}

/// A run of tokens of Chinese characters starting at token `first`, split into words; a word is
/// given to the token it starts in, with every token it reaches.
fn han(tokens: &[(&str, TokenKind)], first: usize) -> Result<Vec<Pending>, LexiconError> {
    // Each character, and the token it is in.
    let (characters, owners): (Vec<char>, Vec<usize>) = tokens
        .iter()
        .enumerate()
        .flat_map(|(at, (text, _))| text.chars().map(move |c| (c, first + at)))
        .unzip();
    let mut pending: Vec<Pending> = Vec::new();
    let mut start = 0;
    for word in segment(&characters)? {
        let (owner, last) = (owners[start], owners[start + word.length - 1]);
        let read = word.syllables.map(|syllables| RunWord {
            characters: characters[start..start + word.length].iter().collect(),
            syllables,
            number: false,
        });
        match pending.last_mut() {
            // A word that starts inside a token already given to the word before.
            Some(previous) if owner < previous.token + previous.tokens => {
                previous.tokens = previous.tokens.max(last + 1 - previous.token);
                match (&mut previous.words, read) {
                    (Some(words), Some(read)) => words.push(read),
                    (words, _) => *words = None,
                }
            }
            _ => pending.push(Pending {
                token: owner,
                tokens: last + 1 - owner,
                words: read.map(|read| vec![read]),
            }),
        }
        start += word.length;
    }
    Ok(pending)
}

/// Chinese characters split into words; `None` when one cannot be read.
fn words(characters: &[char]) -> Result<Option<Vec<RunWord>>, LexiconError> {
    let mut start = 0;
    let mut out = Vec::new();
    for word in segment(characters)? {
        let Some(syllables) = word.syllables else {
            return Ok(None);
        };
        out.push(RunWord {
            characters: characters[start..start + word.length].iter().collect(),
            syllables,
            number: false,
        });
        start += word.length;
    }
    Ok(Some(out))
}

/// A number written out in Chinese characters, one word of each character's own reading (一点七五
/// is 1.75, not 一点 "a little"); `None` when one cannot be read.
fn number_word(characters: &[char]) -> Result<Option<Vec<RunWord>>, LexiconError> {
    let mut syllables = Vec::with_capacity(characters.len());
    for character in characters {
        match reading(&character.to_string())?.as_deref() {
            Some([syllable]) => syllables.push(*syllable),
            _ => return Ok(None),
        }
    }
    Ok(Some(vec![RunWord {
        characters: characters.iter().collect(),
        syllables,
        number: true,
    }]))
}

/// Gives a run of speech its tones and its readings to their tokens.
fn flush(run: &mut Vec<Pending>, out: &mut [Option<TokenReading>], sandhi: ToneSandhi) {
    join_erhua(run);
    let mut readable: Vec<RunWord> = run
        .iter()
        .filter_map(|pending| pending.words.clone())
        .flatten()
        .collect();
    read_in_context(&mut readable);
    let mut said = pronounce(&readable, sandhi).into_iter();
    for pending in run.drain(..) {
        if let Some(words) = pending.words {
            out[pending.token] = Some(TokenReading {
                words: said.by_ref().take(words.len()).collect(),
                tokens: pending.tokens,
            });
        }
    }
}

/// Joins a 儿 standing alone to the word right before it, as its neutral last syllable (花儿 huār):
/// erhua, which the spoken tones say as part of the syllable before.
fn join_erhua(run: &mut Vec<Pending>) {
    let mut index = 1;
    while index < run.len() {
        let alone = run[index].words.as_deref().is_some_and(
            |words| matches!(words, [word] if matches!(word.characters.as_str(), "儿" | "兒") && !word.number),
        );
        let (before, this) = run.split_at_mut(index);
        let previous = &mut before[index - 1];
        if alone
            && previous.token + previous.tokens == this[0].token
            && let Some(last) = previous.words.as_mut().and_then(|words| words.last_mut())
            && !last.number
            && let Some(er) = Pinyin::parse("er5")
        {
            last.characters.push_str(
                &this[0]
                    .words
                    .as_deref()
                    .map(|words| words[0].characters.clone())
                    .unwrap_or_default(),
            );
            last.syllables.push(er);
            previous.tokens += this[0].tokens;
            run.remove(index);
        } else {
            index += 1;
        }
    }
}

/// A number token in Chinese characters, and how many tokens it takes (2 with a following %).
fn number(text: &str, next: Option<&str>) -> Option<(String, usize)> {
    let text: String = text
        .chars()
        .map(|c| match c {
            '０'..='９' => char::from_u32(u32::from(c) - u32::from('０') + u32::from('0')).unwrap_or(c),
            '−' => '-',
            c => c,
        })
        .collect();
    // Thousands separators (1,000,000).
    let text = if text.contains(',') && text.split(',').skip(1).all(|group| group.len() == 3) {
        text.replace(',', "")
    } else {
        text
    };
    let next = next.unwrap_or_default();
    if matches!(next, "%" | "％") {
        return Some((format!("百分之{}", plain(&text, "")?), 2));
    }
    if let Some(rest) = text.strip_prefix('-') {
        return Some((format!("负{}", plain(rest, next)?), 1));
    }
    if let Some(rest) = text.strip_prefix('+') {
        return Some((format!("正{}", plain(rest, next)?), 1));
    }
    let parts: Vec<&str> = text.split([':', '/', '-']).collect();
    let said = match (parts.as_slice(), text.contains(':'), text.contains('/')) {
        // A time: 10:30 十点三十分, 10:30:15 …十五秒.
        ([hours, minutes, seconds @ ..], true, false) if seconds.len() <= 1 => {
            let seconds = match seconds {
                [seconds] => format!("{}秒", minutes_of(seconds)?),
                _ => String::new(),
            };
            format!("{}点{}分{seconds}", cardinal(hours)?, minutes_of(minutes)?)
        }
        // A date, year first as China writes it: 2024/5/12 二零二四年五月十二日.
        ([year, month, day], false, true) if year.len() == 4 => {
            format!("{}年{}月{}日", digit_by_digit(year), cardinal(month)?, cardinal(day)?)
        }
        // A fraction: 2/3 三分之二.
        ([over, under], false, true) => format!("{}分之{}", cardinal(under)?, cardinal(over)?),
        // A range: 10-20 十到二十, 1990-1995 一九九零到一九九五.
        ([low, high], false, false) => format!("{}到{}", plain(low, next)?, plain(high, next)?),
        ([number], false, false) => plain(number, next)?,
        _ => return None,
    };
    Some((said, 1))
}

/// Minutes or seconds: 05 零五, 30 三十.
fn minutes_of(digits: &str) -> Option<String> {
    match digits.as_bytes() {
        [b'0', b'0'] => Some("零".to_owned()),
        [b'0', _] => Some(format!("零{}", cardinal(digits)?)),
        _ => cardinal(digits),
    }
}

/// A number without signs or separators, read by what follows it (`next`): a year before 年 and
/// identifiers (a leading zero, 11 digits or more) digit by digit, 2 before a counted thing as
/// 两, otherwise as a quantity.
fn plain(number: &str, next: &str) -> Option<String> {
    if number.contains('.') {
        return decimal(number);
    }
    if number.is_empty() || !number.bytes().all(|digit| digit.is_ascii_digit()) {
        return None;
    }
    let year = number.len() == 4 && next.starts_with('年');
    let identifier = number.len() >= 11 || (number.len() > 1 && number.starts_with('0'));
    if year || identifier {
        return Some(digit_by_digit(number));
    }
    if number == "2" && next.chars().next().is_some_and(|c| COUNTED.contains(c)) {
        return Some("两".to_owned());
    }
    cardinal(number)
}

/// Symbols said as words in Chinese.
fn symbol(text: &str) -> Option<&'static str> {
    match text {
        "℃" => Some("摄氏度"),
        "℉" => Some("华氏度"),
        _ => None,
    }
}

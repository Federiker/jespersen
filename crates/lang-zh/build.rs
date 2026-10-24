//! Compiles the embedded Mandarin lexicon, each word's readings in numbered pinyin:
//!
//! - ipa-dict's Simplified and Traditional lists (data/upstream/ipa-dict, from 開放漢語詞典), whose
//!   IPA syllables are turned back into pinyin with 開放詞典's own conversion table
//!   (data/upstream/kfcd-pinyin/pinyinbiao), the one ipa-dict was made with;
//! - each character's reading from Unihan (data/derived/zh/characters.tsv), which replaces
//!   ipa-dict's for single characters;
//! - 開放漢語字典's example words (data/upstream/kfcd-hyzd), for words ipa-dict lacks, each with
//!   its entry's reading for the entry's character, in Traditional characters and in Simplified
//!   ones by 開放詞典's conversion table (data/upstream/kfcd-fanjian);
//! - jespersen's everyday words (data/zh/words.tsv), with their neutral tones, and their Traditional
//!   forms;
//! - jespersen's own entries (data/zh/lexicon.tsv), which replace them all.
//!
//! It also writes the longest word's length in characters, for word segmentation.

use std::{
    collections::{BTreeMap, HashMap},
    env,
    error::Error,
    fs,
    path::PathBuf,
};

#[path = "src/pinyin.rs"]
#[allow(dead_code, reason = "the build script only checks readings")]
mod pinyin;

fn main() -> Result<(), Box<dyn Error>> {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let out = PathBuf::from(env::var("OUT_DIR")?);
    let data = manifest.join("../../data");
    let table = data.join("upstream/kfcd-pinyin/pinyinbiao");
    let simplified = data.join("upstream/ipa-dict/zh_hans.txt");
    let traditional = data.join("upstream/ipa-dict/zh_hant.txt");
    let characters = data.join("derived/zh/characters.tsv");
    let dictionary = data.join("upstream/kfcd-hyzd/hyzd.txt");
    let conversion = data.join("upstream/kfcd-fanjian/fanjian.txt");
    let everyday = data.join("zh/words.tsv");
    let own = data.join("zh/lexicon.tsv");
    for path in [
        &table,
        &simplified,
        &traditional,
        &characters,
        &dictionary,
        &conversion,
        &everyday,
        &own,
    ] {
        println!("cargo::rerun-if-changed={}", path.display());
    }

    let pinyin = ipa_to_pinyin(&fs::read_to_string(&table)?)?;
    let mut words: BTreeMap<String, String> = BTreeMap::new();
    for path in [&simplified, &traditional] {
        for (word, reading) in ipa_dict(&fs::read_to_string(path)?, &pinyin)? {
            words.entry(word).or_insert(reading);
        }
    }
    // Unihan gives a handful of characters readings that are not one Mandarin syllable (瓲 tunwa,
    // a unit): those keep ipa-dict's reading, or none.
    words.extend(
        tsv(&fs::read_to_string(&characters)?)
            .into_iter()
            .filter(|(_, reading)| is_pinyin(reading)),
    );
    let examples = example_words(
        &fs::read_to_string(&dictionary)?,
        &fs::read_to_string(&conversion)?,
        &words,
    )?;
    words.extend(examples);
    words.extend(everyday_words(
        &fs::read_to_string(&everyday)?,
        &fs::read_to_string(&conversion)?,
    )?);
    for (word, reading) in tsv(&fs::read_to_string(&own)?) {
        if !is_pinyin(&reading) || reading.split(' ').count() != word.chars().count() {
            return Err(format!("data/zh/lexicon.tsv: {word} {reading:?} is not one syllable per character").into());
        }
        words.insert(word, reading);
    }
    if words.len() < 80_000 {
        return Err(format!("the Mandarin lexicon has only {} entries", words.len()).into());
    }

    let longest = words.keys().map(|word| word.chars().count()).max().unwrap_or(1);
    fs::write(out.join("longest.rs"), format!("{longest}\n"))?;
    fs::write(
        out.join("mandarin.lex"),
        jespersen_lexicon::compile(words.into_iter().map(|(word, reading)| (word, reading.into_bytes())))?,
    )?;
    Ok(())
}

/// 開放詞典's table, from each IPA syllable (`ʈʂʊŋ˥˥`) to its numbered pinyin (`zhong1`). Where two
/// syllables share an IPA form, the first listed is kept.
fn ipa_to_pinyin(text: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut table = HashMap::new();
    for line in text.lines() {
        let fields: Vec<&str> = line.split('\t').collect();
        let (Some(pinyin), Some(ipa)) = (fields.first(), fields.get(12)) else {
            return Err(format!("pinyinbiao: short line {line:?}").into());
        };
        table.entry((*ipa).to_owned()).or_insert_with(|| (*pinyin).to_owned());
    }
    Ok(table)
}

/// ipa-dict's words of Chinese characters only (not its sayings with commas), each with its first
/// reading in numbered pinyin.
fn ipa_dict(text: &str, pinyin: &HashMap<String, String>) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut words = Vec::new();
    let mut skipped = 0;
    for line in text.lines() {
        let Some((word, readings)) = line.split_once('\t') else {
            return Err(format!("ipa-dict: no tab in {line:?}").into());
        };
        if !word.chars().all(is_han) {
            continue;
        }
        let first = readings.split(", ").next().unwrap_or_default().trim_matches('/');
        let syllables = first
            .split(' ')
            .map(|syllable| {
                pinyin
                    .get(syllable)
                    .map(String::as_str)
                    .ok_or_else(|| format!("ipa-dict: unknown syllable {syllable:?} in {word}"))
            })
            .collect::<Result<Vec<&str>, _>>()?;
        if !syllables
            .iter()
            .all(|syllable| pinyin::Pinyin::parse(syllable).is_some())
        {
            return Err(format!("ipa-dict: {word} reads {syllables:?}, not pinyin jespersen knows").into());
        }
        // A few entries lack a syllable (乒乓球 has two): left out, read from their characters.
        if syllables.len() != word.chars().count() {
            skipped += 1;
            continue;
        }
        words.push((word.to_owned(), syllables.join(" ")));
    }
    if skipped > 50 {
        return Err(format!("ipa-dict: {skipped} words whose syllables do not match their characters").into());
    }
    Ok(words)
}

/// `word<TAB>syllables[<TAB>why]` lines; `#` comments and blank lines skipped.
fn tsv(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .filter_map(|line| {
            let mut fields = line.split('\t');
            Some((fields.next()?.to_owned(), fields.next()?.to_owned()))
        })
        .collect()
}

/// Whether `c` is a Chinese character (the blocks `jespersen_text` calls Han).
fn is_han(c: char) -> bool {
    matches!(c, '〇' | '\u{3400}'..='\u{4DBF}' | '\u{4E00}'..='\u{9FFF}' | '\u{F900}'..='\u{FAFF}' | '\u{20000}'..='\u{3134F}')
}

/// Whether every syllable of `reading` is Mandarin in numbered pinyin.
fn is_pinyin(reading: &str) -> bool {
    reading
        .split(' ')
        .all(|syllable| pinyin::Pinyin::parse(syllable).is_some())
}

/// 開放漢語字典's example words (in Traditional characters, and in Simplified ones by
/// 開放詞典's own conversion table), each read with its entry's reading for the entry's character
/// (its first: the mainland's) and the `words` lexicon's single-character readings for the others, a
/// final 子 neutral.
/// Words already in `words`, or with a character it cannot read, are left out.
fn example_words(
    dictionary: &str,
    conversion: &str,
    words: &BTreeMap<String, String>,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut simplified: HashMap<char, char> = HashMap::new();
    for line in conversion.lines() {
        let mut fields = line.split('\t').map(|field| field.chars());
        if let (Some(mut traditional), Some(mut simple)) = (fields.next(), fields.next())
            && let (Some(traditional), Some(simple)) = (traditional.next(), simple.next())
        {
            simplified.insert(traditional, simple);
        }
    }
    let mut out = Vec::new();
    for line in dictionary.lines() {
        let fields: Vec<&str> = line.split('\t').collect();
        let [traditional, simple, reading, examples, ..] = fields[..] else {
            return Err(format!("hyzd.txt: short line {line:?}").into());
        };
        let (Some(character), Some(simple)) = (traditional.chars().next(), simple.chars().next()) else {
            continue;
        };
        let reading = reading.split('/').next().unwrap_or_default();
        if !is_pinyin(reading) {
            continue;
        }
        for example in examples.split('，').filter(|example| example.chars().count() >= 2) {
            let forms = [
                example.to_owned(),
                example
                    .chars()
                    .map(|c| {
                        if c == character {
                            simple
                        } else {
                            simplified.get(&c).copied().unwrap_or(c)
                        }
                    })
                    .collect(),
            ];
            for form in forms {
                if words.contains_key(&form) || out.iter().any(|(word, _)| *word == form) {
                    continue;
                }
                let syllables: Option<Vec<&str>> = example
                    .chars()
                    .map(|c| {
                        if c == character {
                            Some(reading)
                        } else {
                            words.get(&c.to_string()).map(String::as_str)
                        }
                    })
                    .collect();
                if let Some(mut syllables) = syllables {
                    // 子 ending a word is the neutral suffix (麥子 màizi); words where it is not (原子
                    // yuánzǐ) are ipa-dict's, which these never replace.
                    if example.ends_with('子')
                        && character != '子'
                        && let Some(last) = syllables.last_mut()
                    {
                        *last = "zi5";
                    }
                    out.push((form, syllables.join(" ")));
                }
            }
        }
    }
    Ok(out)
}

/// jespersen's everyday words (data/zh/words.tsv), in Simplified characters and in Traditional ones:
/// given in a third column, or else each character's one Traditional form in 開放詞典's table
/// (`conversion`, Traditional to Simplified); a character with several (发: 發, 髮) needs the column.
fn everyday_words(text: &str, conversion: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut traditional: HashMap<char, Vec<char>> = HashMap::new();
    for line in conversion.lines() {
        let mut fields = line.split('\t').map(|field| field.chars().next());
        if let (Some(Some(old)), Some(Some(simple))) = (fields.next(), fields.next()) {
            traditional.entry(simple).or_default().push(old);
        }
    }
    let mut out = Vec::new();
    let mut unclear = Vec::new();
    for line in text
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
    {
        let fields: Vec<&str> = line.split('\t').collect();
        let [word, reading, ..] = fields[..] else {
            return Err(format!("data/zh/words.tsv: no reading in {line:?}").into());
        };
        if !is_pinyin(reading) || reading.split(' ').count() != word.chars().count() {
            return Err(format!("data/zh/words.tsv: {word} {reading:?} is not one syllable per character").into());
        }
        let old = match fields.get(2) {
            Some(old) => (*old).to_owned(),
            None => word
                .chars()
                .map(|c| match traditional.get(&c).map(Vec::as_slice) {
                    None => Ok(c),
                    Some([old]) => Ok(*old),
                    Some(_) => Err(()),
                })
                .collect::<Result<String, ()>>()
                .unwrap_or_else(|()| {
                    unclear.push(word);
                    word.to_owned()
                }),
        };
        out.push((word.to_owned(), reading.to_owned()));
        if old != word {
            out.push((old, reading.to_owned()));
        }
    }
    if !unclear.is_empty() {
        return Err(format!("data/zh/words.tsv: give the Traditional form of {}", unclear.join(", ")).into());
    }
    Ok(out)
}

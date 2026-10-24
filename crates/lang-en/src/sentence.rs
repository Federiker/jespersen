use jespersen_lexicon::LexiconError;
use jespersen_phonemes::{Pronunciation, Segment, Stress};
use jespersen_text::TokenKind;

use crate::{
    Variety, british,
    function_words::{FunctionWord, before_vowel, function_word, joined, joined_next, reduces_a, segments},
    letters,
    lexicon::{self, cmudict, first, key, pronounce},
    morphology,
    normalise::{self, Spoken},
    phonemes::SCHWA,
    rules, spelling,
};

/// How a word of a sentence is read in running text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reading {
    pub pronunciation: Pronunciation,
    /// Whether eSpeak NG's notation writes this word joined to the next, without a space (in
    /// the: `ɪnðə`). Plain IPA keeps the space.
    pub joined_to_next: bool,
}

/// How each word of a sentence is read in English running text, in order; `None` for a word
/// jespersen cannot pronounce yet.
///
/// Function words take their running-text forms (unstressed or lightly stressed, reduced in plain
/// IPA), "the" and "to" change before a vowel, and some function words join the next word in
/// eSpeak NG's notation. Across words, American English links a final lettER to a vowel with an r
/// and flaps a final /t/ before an unstressed vowel; British English links any final r it does not
/// say. Homographs are told apart by the words around them (to record, a record; to read, had
/// read); other words take their most common pronunciation.
/// British English is read from the American forms with jespersen's own rules (decision 20).
///
/// # Errors
///
/// When an embedded lexicon is damaged.
pub fn read_sentence(words: &[&str], variety: Variety) -> Result<Vec<Option<Reading>>, LexiconError> {
    let lexicon = cmudict()?;
    let british_lexicon = lexicon::british()?;
    let keys: Vec<String> = words.iter().map(|word| key(word)).collect();
    // The phonemes of a word as American English reads them, in the variety read.
    let localise = |segments: Vec<Segment>, key: &str| match variety {
        Variety::American => segments,
        Variety::British(bath) => british::convert(&segments, key, bath),
    };
    // A word in the lexicon: jespersen's own British entries first when reading British English.
    let find = |key: &str| -> Result<Option<Vec<Segment>>, LexiconError> {
        if matches!(variety, Variety::British(_))
            && let Some(segments) = first(&british_lexicon, key)?
        {
            return Ok(Some(segments));
        }
        first(&lexicon, key)
    };
    let mut all = Vec::with_capacity(words.len());
    for (index, (word, key)) in words.iter().zip(&keys).enumerate() {
        let prev = index.checked_sub(1).map(|before| keys[before].as_str());
        let next = keys.get(index + 1).map(String::as_str);
        // A homograph the context decides (to record, a record), else the word as usual.
        let read = match lexicon::homograph(&lexicon, key, prev, next)? {
            Some(mut segments) => {
                rules::weak_vowels(&mut segments, key);
                if variety == Variety::American {
                    rules::reduce_first_kit(&mut segments);
                    rules::syllabic_n(&mut segments, key);
                }
                Some(segments)
            }
            None => read_word(&find, word, key, index + 1 == words.len(), variety)?,
        };
        all.push(read.map(|segments| localise(segments, key)));
    }

    let mut joins = vec![false; words.len()];
    // Words already reduced as the second of a joined pair (had been: bɪn) are not joined again.
    let mut reduced = vec![false; words.len()];
    for index in 0..words.len() {
        let has_next = all.get(index + 1).is_some_and(Option::is_some);
        if has_next && !reduced[index] {
            let next_vowel = all[index + 1]
                .as_ref()
                .and_then(|next| next.first())
                .is_some_and(|first| first.phoneme.is_vowel());
            if let Some(form) = joined(&keys[index], &keys[index + 1]) {
                all[index] = Some(localise(segments(form), &keys[index]));
                joins[index] = true;
                if keys[index + 1] == "a" && reduces_a(&keys[index]) {
                    all[index + 1] = Some(segments(&[(SCHWA, Stress::Unstressed)]));
                }
                if let Some(form) = joined_next(&keys[index], &keys[index + 1]) {
                    all[index + 1] = Some(localise(segments(form), &keys[index + 1]));
                    reduced[index + 1] = true;
                }
            } else if next_vowel && let Some(form) = before_vowel(&keys[index]) {
                all[index] = Some(localise(segments(form), &keys[index]));
            }
        }
        let next = all
            .get(index + 1)
            .and_then(|next| next.as_ref())
            .and_then(|next| next.first().copied());
        if let Some(current) = all[index].as_mut() {
            match (variety, next) {
                (Variety::American, Some(next)) => rules::across(current, next),
                (Variety::American, None) => {}
                (Variety::British(_), next) => british::across(current, next),
            }
        }
    }
    Ok(all
        .into_iter()
        .zip(joins)
        .map(|(segments, joined_to_next)| {
            segments.map(|segments| Reading {
                pronunciation: pronounce(segments, variety),
                joined_to_next,
            })
        })
        .collect())
}

/// How a token of a sentence is read: the words it is said as (none for punctuation, or for a
/// currency sign read after its amount).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenReading {
    pub words: Vec<Pronunciation>,
    /// Whether eSpeak NG's notation writes the token's last word joined to the next token.
    pub joined_to_next: bool,
    /// Whether the words are a hyphenated word's parts (well-known), which eSpeak NG's notation
    /// writes joined and plain IPA with hyphens.
    pub hyphenated: bool,
}

/// How each token (text and kind) of an English sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Unknown words are read from known ones where possible, and otherwise
/// letter by letter with rules learned from CMUdict. Numbers are read as number names (1999:
/// nineteen ninety-nine; British English says "and" in them and reads dates day first), currency
/// after its amount ($5: five dollars), symbols by name (%: percent). The words between
/// punctuation marks are read together, as [`read_sentence`] does.
///
/// # Errors
///
/// When an embedded lexicon is damaged.
pub fn read_tokens(tokens: &[(&str, TokenKind)], variety: Variety) -> Result<Vec<Option<TokenReading>>, LexiconError> {
    let spoken = normalise::spoken(tokens, matches!(variety, Variety::British(_)));
    let mut readings: Vec<Option<TokenReading>> = spoken
        .iter()
        .map(|spoken| {
            (*spoken == Spoken::Silent).then(|| TokenReading {
                words: Vec::new(),
                joined_to_next: false,
                hyphenated: false,
            })
        })
        .collect();
    let mut start = 0;
    for end in 0..=tokens.len() {
        if end == tokens.len() || tokens[end].1 == TokenKind::Punctuation {
            read_run(&spoken[start..end], &mut readings[start..end], variety)?;
            start = end + 1;
        }
    }
    Ok(readings)
}

/// Reads a run of tokens between punctuation marks together.
fn read_run(spoken: &[Spoken], readings: &mut [Option<TokenReading>], variety: Variety) -> Result<(), LexiconError> {
    let words: Vec<&str> = spoken
        .iter()
        .filter_map(Spoken::words)
        .flatten()
        .map(String::as_str)
        .collect();
    let mut read = read_sentence(&words, variety)?.into_iter();
    for (spoken, reading) in spoken.iter().zip(readings) {
        if let Some(words) = spoken.words() {
            let mine: Option<Vec<Reading>> = read.by_ref().take(words.len()).collect();
            *reading = mine.map(|mine| TokenReading {
                joined_to_next: mine.last().is_some_and(|last| last.joined_to_next),
                hyphenated: matches!(spoken, Spoken::Hyphenated(_)),
                words: mine.into_iter().map(|reading| reading.pronunciation).collect(),
            });
        }
    }
    Ok(())
}

/// Lowers every stress above `to` to `to`.
fn lower(mut segments: Vec<Segment>, to: Stress) -> Vec<Segment> {
    for segment in &mut segments {
        segment.stress = segment.stress.min(to);
    }
    segments
}

/// The phonemes of one word of a sentence as American English reads it (from jespersen's British
/// entries, when `find` looks there first), before the rules across words: spelled out if it is
/// an initialism or the letter a on its own (`last` in its run), a function word's running form,
/// the lexicon's, derived from known words, or read with the learned rules.
fn read_word(
    find: &impl Fn(&str) -> Result<Option<Vec<Segment>>, LexiconError>,
    word: &str,
    key: &str,
    last: bool,
    variety: Variety,
) -> Result<Option<Vec<Segment>>, LexiconError> {
    let british = matches!(variety, Variety::British(_));
    // "there" ending its phrase is the place, stressed (he had been there), not "there is".
    if key == "there" && last {
        return find(key);
    }
    if key == "a" && last {
        return Ok(Some(spelling::spell(&['a'], british)));
    }
    let known = find(key)?;
    if let Some(letters) = spelling::spelled(word, known.is_some()) {
        return Ok(Some(spelling::spell(&letters, british)));
    }
    Ok(match function_word(key) {
        Some(FunctionWord::Form(form)) => Some(segments(form)),
        Some(FunctionWord::Lighter) => known.map(|segments| lower(segments, Stress::Secondary)),
        Some(FunctionWord::NoStress) => known.map(|segments| lower(segments, Stress::Unstressed)),
        None => {
            let read = match known {
                Some(segments) => Some(segments),
                None => match morphology::derive(key, find)? {
                    Some(segments) => Some(segments),
                    None => letters::guess(key)?,
                },
            };
            read.map(|mut segments| {
                rules::weak_vowels(&mut segments, key);
                rules::last_secondary(&mut segments);
                if variety == Variety::American {
                    rules::reduce_first_kit(&mut segments);
                    rules::syllabic_n(&mut segments, key);
                }
                segments
            })
        }
    })
}

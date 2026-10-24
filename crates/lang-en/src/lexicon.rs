use jespersen_lexicon::{Lexicon, LexiconError};
use jespersen_phonemes::{Pronunciation, Segment, Stress, syllabify};

use crate::{Variety, arpabet, onsets::legal_onset, phonemes, rules};

/// CMUdict with jespersen's own American entries, compiled by the build script.
static CMUDICT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cmudict.lex"));

/// jespersen's own British entries (data/en-gb/lexicon.tsv), compiled by the build script: words
/// whose British form the rules cannot derive from the American one, written as the American
/// phonemes the rules turn into it.
static BRITISH: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/british.lex"));

/// The American pronunciations of `word` on its own, the preferred one first (see [`first`]), then
/// the others as CMUdict lists them; `None` when it is not there. Case does not matter, nor
/// whether the apostrophe is straight or curly (don’t, don't).
///
/// # Errors
///
/// When the embedded lexicon is damaged, which the tests rule out for the one that ships.
pub fn american(word: &str) -> Result<Option<Vec<Pronunciation>>, LexiconError> {
    let key = key(word);
    let Some(mut values) = cmudict()?.get(&key)? else {
        return Ok(None);
    };
    let preferred = preferred(&key, &values);
    values.swap(0, preferred);
    values
        .into_iter()
        .map(|codes| Ok(pronounce(decode(codes)?, Variety::American)))
        .collect::<Result<_, _>>()
        .map(Some)
}

/// The embedded CMUdict.
pub(crate) fn cmudict() -> Result<Lexicon<'static>, LexiconError> {
    Lexicon::new(CMUDICT)
}

/// The embedded British entries.
pub(crate) fn british() -> Result<Lexicon<'static>, LexiconError> {
    Lexicon::new(BRITISH)
}

/// The phonemes of `key`'s preferred pronunciation in `lexicon`, if it has one, also under its
/// other spellings (colour as color, realise as realize: docs/decisions.md, decision 26).
pub(crate) fn first(lexicon: &Lexicon<'_>, key: &str) -> Result<Option<Vec<Segment>>, LexiconError> {
    for spelling in std::iter::once(key.to_owned()).chain(other_spellings(key)) {
        if let Some(values) = lexicon.get(&spelling)? {
            return values
                .get(preferred(&spelling, &values))
                .map(|codes| decode(codes))
                .transpose();
        }
    }
    Ok(None)
}

/// Which of `key`'s pronunciations is preferred: the first, unless the word spells THOUGHT's vowel
/// au, aw or ough and a later one has that vowel where the first has LOT's (bought, caught, cough:
/// CMUdict lists the merged b ɑ t before b ɔ t). General American keeps THOUGHT in these, as
/// eSpeak NG's notation does; elsewhere CMUdict's ɔ is often regional (dollar, dog).
fn preferred(key: &str, values: &[&[u8]]) -> usize {
    const AA: u8 = 0;
    const AO: u8 = 3;
    let Some(&first) = values.first() else { return 0 };
    if !(key.contains("au") || key.contains("aw") || key.contains("ough")) {
        return 0;
    }
    values
        .iter()
        .position(|other| {
            other.len() == first.len()
                && *other != first
                && first
                    .iter()
                    .zip(*other)
                    .all(|(&a, &b)| a == b || (a / 3 == AA && b / 3 == AO && a % 3 == b % 3))
        })
        .unwrap_or(0)
}

/// The other spellings a word may be in the lexicon under: British spellings as American ones
/// (colour: color, centre: center, realise: realize, travelled: traveled, catalogue: catalog,
/// defence: defense, anaemia: anemia, and some whole words).
fn other_spellings(key: &str) -> Vec<String> {
    const WORDS: &[(&str, &str)] = &[
        ("aeroplane", "airplane"),
        ("cheque", "check"),
        ("draught", "draft"),
        ("gaol", "jail"),
        ("grey", "gray"),
        ("kerb", "curb"),
        ("manoeuvre", "maneuver"),
        ("mould", "mold"),
        ("plough", "plow"),
        ("programme", "program"),
        ("pyjama", "pajama"),
        ("sceptic", "skeptic"),
        ("storey", "story"),
        ("tyre", "tire"),
    ];
    let mut spellings = Vec::new();
    let mut add = |spelling: String| {
        if spelling != key && !spellings.contains(&spelling) {
            spellings.push(spelling);
        }
    };
    for (british, american) in WORDS {
        if let Some(rest) = key.strip_prefix(british) {
            add(format!("{american}{rest}"));
        }
    }
    let replace_last = |from: &str, to: &str| {
        key.rfind(from)
            .map(|at| format!("{}{to}{}", &key[..at], &key[at + from.len()..]))
    };
    // colour, favourite; not our, four or hour, which are found as they are.
    if let Some(at) = key.rfind("our")
        && at > 0
    {
        add(format!("{}or{}", &key[..at], &key[at + 3..]));
    }
    // centre, theatres, fibred: a consonant, then re at the end.
    for (ending, american) in [("re", "er"), ("res", "ers"), ("red", "ered")] {
        if let Some(stem) = key.strip_suffix(ending)
            && stem.ends_with(|c: char| !"aeiou".contains(c))
            && stem.len() > 2
        {
            add(format!("{stem}{american}"));
        }
    }
    for (british, american) in [
        ("isation", "ization"),
        ("ise", "ize"),
        ("ised", "ized"),
        ("ises", "izes"),
        ("ising", "izing"),
        ("iser", "izer"),
    ] {
        if let Some(stem) = key.strip_suffix(british) {
            add(format!("{stem}{american}"));
        }
    }
    for (from, to) in [
        ("yse", "yze"),
        ("lled", "led"),
        ("lling", "ling"),
        ("ller", "ler"),
        ("ogue", "og"),
        ("ence", "ense"),
    ] {
        if let Some(spelling) = replace_last(from, to) {
            add(spelling);
        }
    }
    for (from, to) in [("ae", "e"), ("oe", "e")] {
        if key.len() > 4
            && let Some(spelling) = replace_last(from, to)
        {
            add(spelling);
        }
    }
    spellings
}

/// The lexicon's key for a word: lower case, with a straight apostrophe.
pub(crate) fn key(word: &str) -> String {
    word.to_lowercase().replace('’', "'")
}

/// A pronunciation's phonemes as the lexicon gives them, with the initial-schwa rule applied and
/// only the first primary stress kept: a later one is lowered, its full vowel kept (thirteen,
/// which CMUdict stresses twice).
fn decode(codes: &[u8]) -> Result<Vec<Segment>, LexiconError> {
    let mut segments = codes
        .iter()
        .map(|&code| {
            let (index, stress) = arpabet::decode(code).ok_or(LexiconError::Damaged)?;
            let (phoneme, stress) = phonemes::american(index, stress).ok_or(LexiconError::Damaged)?;
            Ok(Segment::new(phoneme, stress))
        })
        .collect::<Result<Vec<_>, LexiconError>>()?;
    let mut primaries = segments
        .iter_mut()
        .filter(|segment| segment.stress == Stress::Primary)
        .skip(1);
    for later in primaries.by_ref() {
        later.stress = Stress::Unstressed;
    }
    rules::initial_schwa(&mut segments);
    Ok(segments)
}

/// A word's phonemes, with the rules of `variety` within a word applied, syllabified.
pub(crate) fn pronounce(mut segments: Vec<Segment>, variety: Variety) -> Pronunciation {
    match variety {
        Variety::American => rules::american(&mut segments),
        Variety::British(_) => rules::british(&mut segments),
    }
    syllabify(&segments, legal_onset)
}

/// The phonemes of `key` as a homograph in its context (`prev` and `next`, the words around it),
/// if it is one whose context decides (see `homographs.rs`).
pub(crate) fn homograph(
    lexicon: &Lexicon<'_>,
    key: &str,
    prev: Option<&str>,
    next: Option<&str>,
) -> Result<Option<Vec<Segment>>, LexiconError> {
    let variants = lexicon.get(key)?.unwrap_or_default();
    crate::homographs::choose(key, prev, next, &variants)
        .map(|codes| decode(&codes))
        .transpose()
}

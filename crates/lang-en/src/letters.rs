//! Words neither CMUdict nor the morphology can read, read letter by letter with the rules learned
//! from CMUdict (data/derived/en-us/letters.tsv; docs/decisions.md, decision 29).

use jespersen_lexicon::{Lexicon, LexiconError};
use jespersen_phonemes::Segment;

use crate::{arpabet, phonemes, rules};

/// The learned rules, compiled by the build script.
static LETTERS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/letters.lex"));

/// The phonemes the learned rules give `word` (a lexicon key), with exactly one primary stress;
/// `None` for a word with letters the rules do not know (not a–z or an apostrophe, after accents
/// are taken off).
///
/// # Errors
///
/// When the embedded rules are damaged.
pub fn guess(word: &str) -> Result<Option<Vec<Segment>>, LexiconError> {
    let Some(letters) = plain_letters(word) else {
        return Ok(None);
    };
    let lexicon = Lexicon::new(LETTERS)?;
    let lookup = |context: &str| -> Result<Option<Vec<u8>>, LexiconError> {
        Ok(lexicon
            .get(context)?
            .and_then(|values| values.first().map(|value| value.to_vec())))
    };
    let Some(mut codes) = jespersen_lts::predict(&letters, lookup)? else {
        return Ok(None);
    };
    arpabet::one_primary(&mut codes);
    let mut segments = codes
        .into_iter()
        .map(|code| {
            let (index, stress) = arpabet::decode(code).ok_or(LexiconError::Damaged)?;
            let (phoneme, stress) = phonemes::american(index, stress).ok_or(LexiconError::Damaged)?;
            Ok(Segment::new(phoneme, stress))
        })
        .collect::<Result<Vec<_>, LexiconError>>()?;
    rules::initial_schwa(&mut segments);
    Ok(Some(segments))
}

/// The word's letters without accents (café: cafe), if all are a–z or an apostrophe.
fn plain_letters(word: &str) -> Option<Vec<char>> {
    word.chars()
        .map(|letter| {
            let plain = match letter {
                'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' => 'a',
                'ç' | 'ć' | 'č' => 'c',
                'ď' | 'đ' | 'ð' => 'd',
                'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ė' | 'ę' | 'ě' => 'e',
                'ğ' => 'g',
                'ì' | 'í' | 'î' | 'ï' | 'ī' | 'į' | 'ı' => 'i',
                'ł' | 'ľ' | 'ĺ' => 'l',
                'ñ' | 'ń' | 'ň' => 'n',
                'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ő' => 'o',
                'ř' | 'ŕ' => 'r',
                'ś' | 'š' | 'ş' | 'ș' => 's',
                'ť' | 'ţ' | 'ț' => 't',
                'ù' | 'ú' | 'û' | 'ü' | 'ū' | 'ů' | 'ű' | 'ų' => 'u',
                'ý' | 'ÿ' => 'y',
                'ź' | 'ż' | 'ž' => 'z',
                other => other,
            };
            (plain.is_ascii_lowercase() || plain == '\'').then_some(plain)
        })
        .collect::<Option<Vec<char>>>()
        .filter(|letters| letters.iter().any(char::is_ascii_lowercase))
}

use jespersen_phonemes::Pronunciation;
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{locale::Greek, spelling::pronounce};

/// Words said without stress in running text: the articles, the weak pronouns, the particles να,
/// θα, δεν and μην, και, the prepositions of one syllable and their forms with the article.
const UNSTRESSED: [&str; 38] = [
    "ο",
    "η",
    "το",
    "οι",
    "τα",
    "του",
    "της",
    "των",
    "τον",
    "την",
    "τη",
    "τους",
    "τις",
    "μου",
    "σου",
    "μας",
    "σας",
    "με",
    "σε",
    "να",
    "θα",
    "δεν",
    "δε",
    "μην",
    "μη",
    "και",
    "που",
    "για",
    "στο",
    "στη",
    "στην",
    "στον",
    "στα",
    "στους",
    "στις",
    "ως",
    "αν",
    "τοις",
];

/// How each token (text and kind) of a Greek sentence is read, in order; `None` for a token jespersen
/// cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols as
/// words; articles, weak pronouns and particles lose their stress in running text.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Greek, &Reader)
}

/// How Greek says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        pronounce(word)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }
}

use jespersen_phonemes::{Pronunciation, Stress};
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Finnish,
    phonemes::{B, G, I, I_LAX, II_LAX, K, K_FIRST, N, N_LABIAL, N_VELAR, NUCLEI, P, P_FIRST},
    spelling::pronounce,
};

/// Words said without stress in running text: pronouns, ei and en, ja, on, vai.
const UNSTRESSED: [&str; 13] = [
    "ja", "on", "se", "ne", "ei", "en", "et", "hän", "he", "me", "te", "vai", "tuo",
];
/// Words said with a lighter stress (eSpeak NG writes a secondary one): forms of olla, pronouns,
/// mutta, joka and the like.
const LIGHTER: [&str; 26] = [
    "oli", "olivat", "ollut", "olleet", "ole", "olla", "olen", "olet", "olemme", "olette", "ovat", "olisi", "olisivat",
    "minä", "sinä", "mutta", "joka", "jossa", "jonka", "jota", "vähän", "tuota", "tässä", "tämä", "tätä", "eivät",
];

/// How each token (text and kind) of a Finnish sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling, numbers, abbreviations and symbols
/// as words. The words between punctuation marks are read together: function words lose their
/// stress, and eSpeak NG's notation writes ŋ and m for n before k, g, p and b across words and
/// ɪ for an unstressed i.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Finnish, &Reader)
}

/// How Finnish says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        pronounce(word)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    fn run(&self, words: &mut [&mut Pronunciation]) {
        let long_i = NUCLEI
            .iter()
            .find(|(letters, _)| *letters == "ii")
            .map(|(_, phoneme)| *phoneme);
        for said in words.iter_mut() {
            for syllable in &mut said.syllables {
                if syllable.stress == Stress::Unstressed && syllable.nucleus == [I] {
                    syllable.nucleus = vec![I_LAX];
                } else if syllable.stress == Stress::Unstressed
                    && Some(syllable.nucleus[..].first().copied()).flatten() == long_i
                {
                    syllable.nucleus = vec![II_LAX];
                }
            }
        }
        // n ending a word before k or g starting the next is ŋ, before p or b it is m (talon katolla
        // tˈaloŋ kˈatolla, niin paljon nˈiːm pˈaljon).
        for at in 1..words.len() {
            let first = words[at]
                .syllables
                .first()
                .and_then(|syllable| syllable.onset.first())
                .copied();
            let n = if first.is_some_and(|first| [K, K_FIRST, G].contains(&first)) {
                N_VELAR
            } else if first.is_some_and(|first| [P, P_FIRST, B].contains(&first)) {
                N_LABIAL
            } else {
                continue;
            };
            if let Some(last) = words[at - 1].syllables.last_mut()
                && let Some(end) = last.coda.last_mut()
                && *end == N
            {
                *end = n;
            }
        }
    }
}

use jespersen_phonemes::{Phoneme, Pronunciation, Stress};
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    Variety,
    locale::Spanish,
    phonemes::{
        B, B_SOFT, CH, D, D_SOFT, E, E_OPEN, F, G, G_SOFT, K, LL_YEISMO, M, N, N_LABIAL, N_VELAR, NY, P, T, X, Y,
        is_nasal,
    },
    spelling::say,
};

/// Function words said without stress in running text: articles, object pronouns, unstressed
/// possessives, most prepositions, and the conjunctions and relatives without an accent.
const UNSTRESSED: [&str; 35] = [
    "el", "la", "los", "las", "lo", "le", "les", "me", "te", "se", "nos", "os", "mi", "mis", "tu", "tus", "su", "sus",
    "de", "del", "a", "al", "en", "con", "por", "sin", "y", "e", "o", "u", "que", "si", "quien", "quienes", "tras",
];
/// Function words of two syllables said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 26] = [
    "porque", "pero", "como", "cuando", "donde", "sobre", "entre", "desde", "hasta", "para", "hacia", "aunque",
    "mientras", "ante", "nuestro", "nuestra", "nuestros", "nuestras", "vuestro", "vuestra", "vuestros", "vuestras",
    "cuanto", "cuanta", "cuantos", "cuantas",
];

/// How each token (text and kind) of a Spanish sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings,
/// méxico → méjico), numbers, abbreviations and symbols as words. The words between punctuation
/// marks are read together: function words lose their stress, and eSpeak NG's notation writes
/// the allophones running speech has (β ð ɣ between sounds, ŋ and m for n) and its rhythmic
/// secondary stresses.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)], variety: Variety) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Spanish, &Reader { variety })
}

/// How Spanish says its words, for the shared reader.
struct Reader {
    variety: Variety,
}

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        say(word, self.variety)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    fn run(&self, words: &mut [&mut Pronunciation]) {
        for said in words.iter_mut() {
            // eSpeak NG's ɛ is for a full stress only (entre ˌentɾe in a sentence).
            for syllable in &mut said.syllables {
                if syllable.stress != Stress::Primary && syllable.nucleus == [E_OPEN] {
                    syllable.nucleus = vec![E];
                }
            }
            rhythm(said);
        }
        allophones(words, self.variety);
    }
}

/// The allophones of running speech, written in eSpeak NG's notation only (plain IPA keeps the
/// phonemes): n is ŋ before g and x and m before b and p, also across words (un gato, un beso); b,
/// d and g are β ð ɣ except after a pause or a nasal, b and g before most consonants (obtuso), and a
/// d ending a word unless a vowel follows (usted es, but usted dijo).
fn allophones(run: &mut [&mut Pronunciation], variety: Variety) {
    // eSpeak NG's Latin American voice writes m before f too (enfrente emfɾˈɛnte), its Castilian one n.
    let labial: &[Phoneme] = match variety {
        Variety::Castilian => &[B, B_SOFT, P],
        Variety::LatinAmerican => &[B, B_SOFT, P, F],
    };
    // Where each phoneme is: word, syllable, part (0 onset, 1 nucleus, 2 coda), index.
    let mut places: Vec<(usize, usize, usize, usize)> = Vec::new();
    for (word, said) in run.iter().enumerate() {
        for (number, syllable) in said.syllables.iter().enumerate() {
            for (part, phonemes) in [&syllable.onset, &syllable.nucleus, &syllable.coda]
                .into_iter()
                .enumerate()
            {
                places.extend((0..phonemes.len()).map(|index| (word, number, part, index)));
            }
        }
    }
    let get = |run: &[&mut Pronunciation], (word, syllable, part, index): (usize, usize, usize, usize)| -> Phoneme {
        let syllable = &run[word].syllables[syllable];
        [&syllable.onset, &syllable.nucleus, &syllable.coda][part][index]
    };
    let set = |run: &mut [&mut Pronunciation],
               (word, syllable, part, index): (usize, usize, usize, usize),
               phoneme: Phoneme| {
        let syllable = &mut run[word].syllables[syllable];
        [&mut syllable.onset, &mut syllable.nucleus, &mut syllable.coda][part][index] = phoneme;
    };
    for at in 0..places.len() {
        let phoneme = get(run, places[at]);
        let next = places.get(at + 1).map(|&place| get(run, place));
        if phoneme == N {
            if next.is_some_and(|next| [G, G_SOFT, X].contains(&next)) {
                set(run, places[at], N_VELAR);
            } else if next.is_some_and(|next| labial.contains(&next)) {
                set(run, places[at], N_LABIAL);
            }
        }
    }
    // Latin American ll is written jj between vowels inside a word only, ʝ elsewhere (llegó ʝeɣˈo,
    // la llama la ʝˈama, calle kˈajje).
    for at in 0..places.len() {
        let after_vowel = at
            .checked_sub(1)
            .is_some_and(|before| places[before].0 == places[at].0 && get(run, places[before]).is_vowel());
        if get(run, places[at]) == LL_YEISMO && !after_vowel {
            set(run, places[at], Y);
        }
    }
    for at in 1..places.len() {
        let phoneme = get(run, places[at]);
        let before = get(run, places[at - 1]);
        // A d ending a word stays d unless a vowel follows (usted es: ð; usted. or usted dijo: d).
        let word_final = places.get(at + 1).is_none_or(|next| next.0 != places[at].0);
        let vowel_next = places.get(at + 1).is_some_and(|&next| get(run, next).is_vowel());
        // Before a stop or a nasal, b and g stay stops (obtuso, ignoran); before s they soften
        // (absoluto aβs).
        let stop_next = places
            .get(at + 1)
            .is_some_and(|&next| [P, T, K, B, D, G, CH, M, N, NY].contains(&get(run, next)));
        let soft = match phoneme {
            _ if is_nasal(before) => continue,
            phoneme if phoneme == B && !stop_next => B_SOFT,
            phoneme if phoneme == D && (!word_final || vowel_next) => D_SOFT,
            phoneme if phoneme == G && !stop_next => G_SOFT,
            _ => continue,
        };
        set(run, places[at], soft);
    }
}

/// The rhythmic secondary stresses of a word, which eSpeak NG's notation marks: every other
/// syllable from the first, up to two before the stressed one (ˌestaβlˌeθimjˈɛnto, ˌimpoɾtˈante).
fn rhythm(said: &mut Pronunciation) {
    let Some(stressed) = said
        .syllables
        .iter()
        .position(|syllable| syllable.stress == Stress::Primary)
    else {
        return;
    };
    for syllable in said.syllables.iter_mut().take(stressed.saturating_sub(1)).step_by(2) {
        if syllable.stress == Stress::Unstressed {
            syllable.stress = Stress::Rhythmic;
        }
    }
}

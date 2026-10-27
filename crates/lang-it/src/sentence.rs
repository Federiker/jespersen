use jespersen_phonemes::{Phoneme, Pronunciation, Stress, Syllable};
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Italian,
    phonemes::{CH, E, E_IMPERFECT, E_OPEN, G, G_FIRST, G_LONG, I, I_FINAL, N, N_VELAR, O, O_OPEN, U, U_LAX},
    spelling::pronounce,
};

/// Function words said without stress in running text: articles and their forms with
/// prepositions, object pronouns, prepositions, conjunctions, ha and ho. è, the verb, keeps its
/// stress and its open ɛ (qual era è cosa dura), unlike the conjunction e.
const UNSTRESSED: [&str; 49] = [
    "il", "lo", "la", "i", "gli", "le", "un", "di", "a", "da", "in", "con", "su", "per", "tra", "fra", "e", "ed", "o",
    "od", "ma", "che", "se", "ne", "ci", "vi", "mi", "ti", "si", "li", "me", "te", "del", "al", "dal", "nel", "sul",
    "col", "non", "quel", "più", "fu", "chi", "ha", "ho", "hai", "han", "tu", "ve",
];
/// Words said with a lighter stress (eSpeak NG writes a secondary one): una, articles joined to a
/// preposition, demonstratives, possessives, and the commonest forms of essere and avere.
const LIGHTER: [&str; 69] = [
    "una", "perché", "come", "dei", "dello", "della", "degli", "delle", "allo", "alla", "agli", "alle", "ai", "dallo",
    "dalla", "dai", "dagli", "dalle", "nei", "sullo", "sulla", "sui", "sugli", "sulle", "questo", "questa", "questi",
    "queste", "suo", "sua", "suoi", "sue", "era", "erano", "sono", "sei", "siamo", "abbiamo", "aveva", "avevo",
    "avevano", "avete", "ebbe", "senza", "verso", "loro", "mentre", "mio", "mia", "miei", "mie", "tuo", "tua", "tuoi",
    "tue", "quello", "quella", "quelli", "quelle", "nostro", "nostra", "nostri", "nostre", "vostro", "vostra",
    "vostri", "vostre", "sia", "siete",
];

/// How each token (text and kind) of an Italian sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling and jespersen's word list, numbers,
/// abbreviations and symbols as words, and an elided word with the word after it (l'amico
/// laˈmiko). The words between punctuation marks are read together: function words lose their
/// stress, and eSpeak NG's notation writes the allophones it does (ʊ, ɪ, ŋ).
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Italian, &Reader)
}

/// How Italian says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        pronounce(word)
    }

    fn elide(&self, head: &str, word: Pronunciation) -> Option<Pronunciation> {
        elide(head, word)
    }

    /// An elision at the end of the text (po' for poco): its last vowel takes the stress and an
    /// open quality (pò).
    fn elided_alone(&self, head: &str) -> Option<Pronunciation> {
        let mut stressed: String = head.to_owned();
        if let Some(last) = stressed.pop() {
            stressed.push(match last {
                'a' => 'à',
                'e' => 'è',
                'i' => 'ì',
                'o' => 'ò',
                'u' => 'ù',
                other => other,
            });
        }
        pronounce(&stressed)
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    fn run(&self, words: &mut [&mut Pronunciation]) {
        allophones(words);
    }
}

/// An elided word said with the word after it: its sounds unstressed, its last consonant starting
/// the next word's first syllable (l'amico la-mi-co, dell'acqua del-lac-qua, un'altra u-nal-tra); c'
/// before e or i is tʃ (c'era).
fn elide(head: &str, word: Pronunciation) -> Option<Pronunciation> {
    let lower = head.to_lowercase();
    let starts_front = word
        .syllables
        .first()
        .and_then(|syllable| syllable.nucleus.first())
        .is_some_and(|vowel| [E, E_OPEN, I].contains(vowel));
    let mut head = if lower == "c" && starts_front {
        Pronunciation {
            syllables: vec![Syllable {
                onset: vec![CH],
                ..Syllable::default()
            }],
        }
    } else {
        pronounce(&lower)?
    };
    for syllable in &mut head.syllables {
        syllable.stress = Stress::Unstressed;
    }
    let mut syllables = head.syllables;
    let mut rest = word.syllables.into_iter();
    let mut first = rest.next()?;
    if let Some(last) = syllables.last_mut()
        && first.onset.is_empty()
    {
        if last.nucleus.is_empty() {
            // A head of consonants only (l', c', s'): they start the word.
            first.onset = std::mem::take(&mut last.onset);
            syllables.pop();
        } else if let Some(consonant) = last.coda.pop() {
            first.onset.insert(0, consonant);
        }
    }
    syllables.push(first);
    syllables.extend(rest);
    Some(Pronunciation { syllables })
}

/// The allophones of running speech eSpeak NG's notation writes: unstressed u as ʊ, an unstressed
/// final i as ɪ, open vowels closed when unstressed, and n before g inside a word as ŋ. Plain IPA
/// keeps the phonemes.
fn allophones(run: &mut [&mut Pronunciation]) {
    for said in run.iter_mut() {
        let last = said.syllables.len().saturating_sub(1);
        for (number, syllable) in said.syllables.iter_mut().enumerate() {
            let stressed = syllable.stress == Stress::Primary;
            // Under a lighter stress open vowels stay open (ˌɛra); unstressed, they close.
            let lighter = syllable.stress == Stress::Secondary;
            for vowel in &mut syllable.nucleus {
                *vowel = match *vowel {
                    vowel if vowel == U && !stressed => U_LAX,
                    // After a consonant only: the article i stays i.
                    vowel
                        if vowel == I
                            && !stressed
                            && number == last
                            && syllable.coda.is_empty()
                            && !syllable.onset.is_empty() =>
                    {
                        I_FINAL
                    }
                    vowel if vowel == E_OPEN && !stressed && !lighter => E,
                    vowel if vowel == O_OPEN && !stressed && !lighter => O,
                    vowel if vowel == E_IMPERFECT && !stressed => E,
                    vowel => vowel,
                };
            }
        }
    }
    // n before g, inside a word only (inglese iŋɡlˈeze, but un gatto ʊn ɡˈatːo).
    for said in run.iter_mut() {
        let mut phonemes: Vec<&mut Phoneme> = said
            .syllables
            .iter_mut()
            .flat_map(|syllable| {
                syllable
                    .onset
                    .iter_mut()
                    .chain(&mut syllable.nucleus)
                    .chain(&mut syllable.coda)
            })
            .collect();
        for at in 0..phonemes.len().saturating_sub(1) {
            if *phonemes[at] == N && [G, G_FIRST, G_LONG].contains(phonemes[at + 1]) {
                *phonemes[at] = N_VELAR;
            }
        }
    }
}

//! American function words in running text (docs/decisions.md, decision 27). jespersen's own list,
//! from textbook phonetics (weak forms in plain IPA) and eSpeak NG's observed behaviour (which
//! words are unstressed or lightly stressed, and the vowels its notation writes for them).

use jespersen_phonemes::Stress::{self, Primary, Secondary, Unstressed};
use jespersen_phonemes::{Phoneme, Segment};

use crate::phonemes::{
    B, CURE_WEAK, D, DH, DRESS, F, FACE, FLEECE, FOOT_WEAK, GOAT, GOOSE, GOOSE_WEAK, H, HAPPY, K, KIT, L, LETTER, LOT,
    M, MOUTH, N, NORTH_WEAK, NURSE_WEAK, ON, P, PRICE, R, S, SCHWA, SCHWA_OPEN, SH, SQUARE_WEAK, START_WEAK, STRUT,
    STRUT_WEAK, T, THE_BEFORE_VOWEL, TRAP, TRAP_WEAK, V, W, Y, Z,
};

/// How a function word is read in running text.
pub enum FunctionWord {
    /// Always this pronunciation.
    Form(&'static [(Phoneme, Stress)]),
    /// The lexicon's first pronunciation, its primary stress lowered to secondary.
    Lighter,
    /// The lexicon's first pronunciation, without stress (contractions such as she's, you'd).
    NoStress,
}

/// How `word` (lower case) is read in running text, if it is a function word.
pub fn function_word(word: &str) -> Option<FunctionWord> {
    use FunctionWord::{Form, Lighter};
    const U: Stress = Unstressed;
    Some(match word {
        "a" => Form(&[(SCHWA_OPEN, U)]),
        // Before its vowel, written ɐn in eSpeak NG's notation.
        "an" => Form(&[(SCHWA_OPEN, U), (N, U)]),
        "and" => Form(&[(TRAP_WEAK, U), (N, U), (D, U)]),
        "the" => Form(&[(DH, U), (SCHWA, U)]),
        "to" => Form(&[(T, U), (SCHWA, U)]),
        "of" => Form(&[(STRUT_WEAK, U), (V, U)]),
        "in" => Form(&[(KIT, U), (N, U)]),
        "at" => Form(&[(TRAP_WEAK, U), (T, U)]),
        "for" => Form(&[(F, U), (NORTH_WEAK, U)]),
        "from" => Form(&[(F, U), (R, U), (STRUT_WEAK, U), (M, U)]),
        "with" => Form(&[(W, U), (KIT, U), (DH, U)]),
        "by" => Form(&[(B, U), (PRICE, U)]),
        "as" => Form(&[(TRAP_WEAK, U), (Z, U)]),
        "is" => Form(&[(KIT, U), (Z, U)]),
        "was" => Form(&[(W, U), (STRUT_WEAK, U), (Z, U)]),
        "were" => Form(&[(W, U), (NURSE_WEAK, U)]),
        "be" => Form(&[(B, U), (FLEECE, U)]),
        "are" => Form(&[(START_WEAK, U)]),
        "am" => Form(&[(TRAP_WEAK, U), (M, U)]),
        "has" => Form(&[(H, U), (SCHWA_OPEN, U), (Z, U)]),
        "have" => Form(&[(H, U), (TRAP_WEAK, U), (V, U)]),
        "had" => Form(&[(H, U), (TRAP_WEAK, U), (D, U)]),
        "can" => Form(&[(K, U), (TRAP_WEAK, U), (N, U)]),
        "could" => Form(&[(K, U), (FOOT_WEAK, U), (D, U)]),
        "will" => Form(&[(W, U), (KIT, U), (L, U)]),
        "would" => Form(&[(W, U), (FOOT_WEAK, U), (D, U)]),
        "i" => Form(&[(PRICE, U)]),
        "you" => Form(&[(Y, U), (GOOSE, U)]),
        "he" => Form(&[(H, U), (FLEECE, U)]),
        "she" => Form(&[(SH, U), (FLEECE, U)]),
        "it" => Form(&[(KIT, U), (T, U)]),
        "we" => Form(&[(W, U), (FLEECE, U)]),
        "they" => Form(&[(DH, U), (FACE, U)]),
        "her" => Form(&[(H, U), (NURSE_WEAK, U)]),
        "my" => Form(&[(M, U), (PRICE, U)]),
        "your" => Form(&[(Y, U), (CURE_WEAK, U)]),
        "his" => Form(&[(H, U), (KIT, U), (Z, U)]),
        "its" => Form(&[(KIT, U), (T, U), (S, U)]),
        "their" | "there" => Form(&[(DH, U), (SQUARE_WEAK, U)]),
        "this" => Form(&[(DH, U), (KIT, U), (S, U)]),
        "that" => Form(&[(DH, U), (TRAP, U), (T, U)]),
        "these" => Form(&[(DH, U), (FLEECE, U), (Z, U)]),
        "those" => Form(&[(DH, U), (GOAT, U), (Z, U)]),
        "what" => Form(&[(W, U), (STRUT_WEAK, U), (T, U)]),
        "when" => Form(&[(W, U), (DRESS, U), (N, U)]),
        "if" => Form(&[(KIT, U), (F, U)]),
        "than" => Form(&[(DH, U), (SCHWA_OPEN, U), (N, U)]),
        "or" => Form(&[(NORTH_WEAK, U)]),
        "on" => Form(&[(ON, Secondary), (N, U)]),
        "into" => Form(&[(KIT, Secondary), (N, U), (T, U), (GOOSE_WEAK, U)]),
        "upon" => Form(&[(SCHWA, U), (P, U), (LOT, Secondary), (N, U)]),
        "without" => Form(&[(W, U), (KIT, U), (DH, U), (MOUTH, Secondary), (T, U)]),
        "because" => Form(&[(B, U), (KIT, U), (K, U), (STRUT, Primary), (Z, U)]),
        "onto" => Form(&[(ON, Secondary), (N, U), (T, U), (GOOSE_WEAK, U)]),
        "been" | "shall" | "should" | "might" | "me" | "him" | "us" | "them" | "our" | "which" | "who" | "where"
        | "why" | "how" | "not" | "so" | "but" | "up" | "about" | "over" | "under" | "down" | "some" | "any"
        | "though" | "isn't" | "aren't" | "it'll" | "go" | "make" | "made" | "while" | "before" => Lighter,
        "i'm" | "i'd" | "i'll" | "i've" | "you're" | "you'd" | "you'll" | "you've" | "he's" | "he'd" | "he'll"
        | "she's" | "she'd" | "she'll" | "it's" | "it'd" | "we're" | "we'd" | "we'll" | "we've" | "they're"
        | "they'd" | "they'll" | "they've" | "that's" | "there's" | "won't" | "through" | "per" => {
            FunctionWord::NoStress
        }
        _ => return None,
    })
}

/// A form's segments.
pub fn segments(form: &[(Phoneme, Stress)]) -> Vec<Segment> {
    form.iter()
        .map(|&(phoneme, stress)| Segment::new(phoneme, stress))
        .collect()
}

/// How `word` is read when the next word starts with a vowel, if that changes it: "the" and "to"
/// (the apple, to eat).
pub fn before_vowel(word: &str) -> Option<&'static [(Phoneme, Stress)]> {
    match word {
        "the" => Some(&[(DH, Unstressed), (THE_BEFORE_VOWEL, Unstressed)]),
        "to" => Some(&[(T, Unstressed), (GOOSE_WEAK, Unstressed)]),
        _ => None,
    }
}

/// The form of `word` when eSpeak NG's notation writes it joined to the next word, `next` (in
/// the, on the, at a), if it does.
pub fn joined(word: &str, next: &str) -> Option<&'static [(Phoneme, Stress)]> {
    const U: Stress = Unstressed;
    match (word, next) {
        ("in", "the") => Some(&[(KIT, U), (N, U)]),
        ("on", "the") => Some(&[(ON, U), (N, U)]),
        ("of", "the") => Some(&[(STRUT_WEAK, U), (V, U)]),
        ("of", "a") => Some(&[(SCHWA, U), (V, U)]),
        ("for", "the" | "a") => Some(&[(F, U), (LETTER, U)]),
        ("from", "the") => Some(&[(F, U), (R, U), (STRUT, U), (M, U)]),
        ("with", "the") => Some(&[(W, U), (KIT, U), (DH, U)]),
        ("was", "the" | "a") => Some(&[(W, U), (STRUT, U), (Z, U)]),
        ("had", "been") => Some(&[(H, U), (SCHWA_OPEN, U), (D, U)]),
        ("has", "been") => Some(&[(H, U), (SCHWA_OPEN, U), (Z, U)]),
        ("have", "been") => Some(&[(H, U), (SCHWA_OPEN, U), (V, U)]),
        ("to", "be") => Some(&[(T, U), (SCHWA, U)]),
        ("i", "am") => Some(&[(PRICE, U)]),
        ("there", "was") => Some(&[(DH, U), (SQUARE_WEAK, U)]),
        ("did", "not") => Some(&[(D, U), (KIT, U), (D, U)]),
        ("do", "not") => Some(&[(D, U), (GOOSE, U)]),
        ("out", "of") => Some(&[(MOUTH, Secondary), (T, U)]),
        ("that", "the") => Some(&[(DH, U), (TRAP, U), (T, U)]),
        ("that", "a") => Some(&[(DH, U), (TRAP, Secondary), (T, U)]),
        ("at", "a") => Some(&[(TRAP_WEAK, U), (T, U)]),
        _ => None,
    }
}

/// Whether "a" after `word`, joined to it, is a plain schwa (at a: æɾə) rather than ɐ (was a).
pub fn reduces_a(word: &str) -> bool {
    matches!(word, "at" | "of" | "for" | "that")
}

/// The form of `next` when it is joined to `word` and reduced with it (had been: hɐdbɪn), if it
/// is.
pub fn joined_next(word: &str, next: &str) -> Option<&'static [(Phoneme, Stress)]> {
    const U: Stress = Unstressed;
    match (word, next) {
        ("had" | "has" | "have", "been") => Some(&[(B, U), (KIT, U), (N, U)]),
        ("to", "be") => Some(&[(B, U), (HAPPY, U)]),
        ("i", "am") => Some(&[(SCHWA_OPEN, U), (M, U)]),
        ("there", "was") => Some(&[(W, U), (STRUT_WEAK, Secondary), (Z, U)]),
        ("did" | "do", "not") => Some(&[(N, U), (LOT, Secondary), (T, U)]),
        ("out", "of") => Some(&[(SCHWA, U), (V, U)]),
        _ => None,
    }
}

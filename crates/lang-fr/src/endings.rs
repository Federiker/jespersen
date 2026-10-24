//! A French word's written ending turned into what is said: its silent letters dropped, -er, -ez
//! and -et respelt as the vowel they are (parler parlé, billet billè), and the consonant a liaison
//! would say kept aside (les lé + z, petit peti + t, trop tro + p).

use jespersen_phonemes::Phoneme;

use crate::phonemes::{LIAISON_P, LIAISON_T, LIAISON_Z, LIAISON_ZS};

/// Words in -ent that say it (nouns, adjectives, adverbs), where verbs leave it silent (ils
/// parlent). Adverbs and nouns in -ment say it unless they are listed among the verbs below.
const SAID_ENT: [&str; 51] = [
    "absent",
    "accent",
    "accident",
    "adolescent",
    "agent",
    "argent",
    "ardent",
    "client",
    "compétent",
    "confident",
    "content",
    "continent",
    "couvent",
    "décent",
    "différent",
    "éloquent",
    "équivalent",
    "évident",
    "excellent",
    "fréquent",
    "impatient",
    "imprudent",
    "incident",
    "innocent",
    "insolent",
    "intelligent",
    "occident",
    "orient",
    "parent",
    "patient",
    "permanent",
    "président",
    "présent",
    "prudent",
    "récent",
    "régent",
    "résident",
    "serpent",
    "souvent",
    "talent",
    "torrent",
    "urgent",
    "violent",
    "omniprésent",
    "transparent",
    "apparent",
    "ingrédient",
    "coefficient",
    "pourcent",
    "adjacent",
    "concurrent",
];

/// Verbs in -ment whose -ent is silent (ils aiment, elles dorment).
const VERB_MENT: [&str; 28] = [
    "aiment",
    "animent",
    "calment",
    "consomment",
    "dorment",
    "endorment",
    "estiment",
    "expriment",
    "ferment",
    "forment",
    "fument",
    "impriment",
    "informent",
    "nomment",
    "priment",
    "résument",
    "sèment",
    "suppriment",
    "transforment",
    "allument",
    "assument",
    "affirment",
    "confirment",
    "rament",
    "réclament",
    "proclament",
    "abîment",
    "riment",
];

/// Words whose ending the rules would misread: et é, est è (with t in a liaison), es è, and six and
/// dix, said siz before a vowel, si before a consonant and sis at the end.
const SPECIAL: [(&str, &str, Option<Phoneme>); 5] = [
    ("et", "é", None),
    ("est", "è", Some(LIAISON_T)),
    ("es", "è", Some(LIAISON_Z)),
    ("six", "si", Some(LIAISON_ZS)),
    ("dix", "di", Some(LIAISON_ZS)),
];

/// Whether the -ent of `word` is said (moment, souvent) rather than a verb's silent ending.
fn says_ent(word: &str) -> bool {
    SAID_ENT.contains(&word) || (word.ends_with("ment") && !VERB_MENT.contains(&word))
}

/// The number of vowel groups (syllables, roughly) in `word`.
fn vowel_groups(word: &[char]) -> usize {
    let vowel = |c: &char| "aàâeéèêëiîïoôuùûüyœ".contains(*c);
    let mut groups = 0;
    let mut previous = false;
    for c in word {
        let now = vowel(c);
        if now && !previous {
            groups += 1;
        }
        previous = now;
    }
    groups
}

/// Words whose silent final t or d is said t in a liaison, as eSpeak NG says them: adjectives
/// before their noun, a few adverbs and conjunctions, and verbs (petit ami, quand il, ils sont
/// arrivés). Other words keep it silent (le chat est, souvent à).
const LIAISON_T_WORDS: [&str; 20] = [
    "petit", "grand", "tout", "quand", "vingt", "dont", "segond", "saint", "peut", "veut", "doit", "soit", "fait",
    "sait", "dit", "met", "ont", "sont", "font", "vont",
];

/// `word` (in lower case) as its ending is said, and the consonant it says in a liaison.
pub(crate) fn ending(word: &str) -> (String, Option<Phoneme>) {
    if let Some((_, said, liaison)) = SPECIAL.iter().find(|(written, ..)| *written == word) {
        return ((*said).to_owned(), *liaison);
    }
    let mut letters: Vec<char> = word.chars().collect();
    let single = vowel_groups(&letters) <= 1;
    let vowel = |c: char| "aàâeéèêëiîïoôuùûüyœ".contains(c);
    let replace = |letters: &mut Vec<char>, length: usize, with: &str| {
        letters.truncate(letters.len() - length);
        letters.extend(with.chars());
    };
    // -ez is é (chez, nez, parlez), and so is the -es of les, des, ces and the like.
    let lone_es = single && letters.len() > 2 && letters.ends_with(&['e', 's']) && !vowel(letters[letters.len() - 3]);
    if letters.ends_with(&['e', 'z']) || lone_es {
        replace(&mut letters, 2, "é");
        return (letters.into_iter().collect(), Some(LIAISON_Z));
    }
    // A final s, x or z is silent and said z in a liaison (les amis, deux enfants).
    let mut liaison = None;
    if letters.len() > 2 && matches!(letters.last(), Some('s' | 'x' | 'z')) {
        letters.pop();
        liaison = Some(LIAISON_Z);
    }
    // A verb's -ent is silent and says t in a liaison (ils parlent, elles jouent); a word with no
    // vowel before it says it (dent, vent).
    let without_s: String = letters.iter().collect();
    let stem_has_vowel = letters.len() > 3 && letters[..letters.len() - 3].iter().any(|&c| vowel(c));
    // vient and tient (revient, appartient) are jɛ̃, not a plural verb's ending.
    let vient = letters.ends_with(&['v', 'i', 'e', 'n', 't']) || letters.ends_with(&['t', 'i', 'e', 'n', 't']);
    if stem_has_vowel && !vient && letters.ends_with(&['e', 'n', 't']) && !says_ent(&without_s) {
        replace(&mut letters, 2, "");
        return (letters.into_iter().collect(), liaison.or(Some(LIAISON_T)));
    }
    let says_t = LIAISON_T_WORDS.contains(&without_s.as_str()) || without_s.ends_with("ait");
    match letters[..] {
        // -er is é in words of more than one syllable (parler, premier; but mer, fer), -ed é
        // (pied), -et è (billet, met).
        [.., 'e', 'r'] if !single => replace(&mut letters, 2, "é"),
        [.., 'e', 'd'] => replace(&mut letters, 2, "é"),
        [.., 'e', 't'] => {
            replace(&mut letters, 2, "è");
            if says_t {
                liaison = liaison.or(Some(LIAISON_T));
            }
        }
        // A final t or d is silent (petit, grand, chat), but not in -ct (direct); the g of -gt goes
        // with it (doigt).
        [.., c, 't' | 'd'] if c != 'c' => {
            letters.pop();
            if letters.last() == Some(&'g') {
                letters.pop();
            }
            if says_t {
                liaison = liaison.or(Some(LIAISON_T));
            }
        }
        // A final p is silent (trop, beaucoup, champ), said in a liaison after trop and beaucoup.
        [.., c, 'p'] if c != 'p' => {
            letters.pop();
            if matches!(without_s.as_str(), "trop" | "beaucoup") {
                liaison = liaison.or(Some(LIAISON_P));
            }
        }
        // A final g or c after n (sang, long, blanc) and a b after m (plomb) are silent.
        [.., 'n', 'g' | 'c'] | [.., 'm', 'b'] => {
            letters.pop();
        }
        _ => {}
    }
    (letters.into_iter().collect(), liaison)
}

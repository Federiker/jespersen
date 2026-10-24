//! How a Spanish word is said from its spelling: letters to phonemes, then stress, then
//! syllables. Spanish spelling gives both nearly completely; the few words it does not (México)
//! are respelt in the exceptions list.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::{
    Variety,
    abbreviations::respelling,
    phonemes::{
        A, B, CH, D, E, E_OPEN, F, G, GLIDE_I, GLIDE_U, I, K, L, LL, LL_YEISMO, M, N, NY, O, OFFGLIDE_I,
        OFFGLIDE_I_AFTER_U, OFFGLIDE_U, P, S, T, TAP, THETA, TRILL, U, X, Y, legal_onset,
    },
};

/// A letter after its accent is taken off, and whether it had one (á, é, í, ó, ú).
fn base(c: char) -> (char, bool) {
    match c {
        'á' => ('a', true),
        'é' => ('e', true),
        'í' => ('i', true),
        'ó' => ('o', true),
        'ú' => ('u', true),
        'à' | 'â' | 'ä' => ('a', false),
        'è' | 'ê' | 'ë' => ('e', false),
        'ì' | 'î' | 'ï' => ('i', false),
        'ò' | 'ô' | 'ö' => ('o', false),
        'ù' | 'û' => ('u', false),
        c => (c, false),
    }
}

/// One sound of a word as the letters give it, before stress: a phoneme, and for a vowel whether
/// it was written with an accent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Sound {
    phoneme: Phoneme,
    accent: bool,
}

/// The sounds of a word in lower case, `None` when it has a letter Spanish does not read.
fn sounds(word: &str, variety: Variety) -> Option<Vec<Sound>> {
    let letters: Vec<(char, bool)> = word.chars().map(base).collect();
    let letter = |at: usize| letters.get(at).map(|(c, _)| *c);
    let is_vowel = |c: Option<char>| c.is_some_and(|c| "aeiouü".contains(c));
    let front = |c: Option<char>| matches!(c, Some('e' | 'i'));
    let theta = match variety {
        Variety::Castilian => THETA,
        Variety::LatinAmerican => S,
    };
    let mut out: Vec<Sound> = Vec::with_capacity(letters.len());
    let push = |out: &mut Vec<Sound>, phoneme| out.push(Sound { phoneme, accent: false });
    let mut at = 0;
    while at < letters.len() {
        let (c, accent) = letters[at];
        let next = letter(at + 1);
        let mut step = 1;
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'ü' => {
                let phoneme = match c {
                    'a' => A,
                    'e' => E,
                    'i' => I,
                    'o' => O,
                    _ => U,
                };
                out.push(Sound { phoneme, accent });
            }
            'y' => {
                // A vowel alone (y, "and") or after a vowel at the end or before a consonant (rey,
                // hoy, muy); a consonant before a vowel (yo, mayo).
                let after_vowel = out
                    .last()
                    .filter(|sound| sound.phoneme.is_vowel())
                    .map(|sound| sound.phoneme);
                match after_vowel {
                    _ if is_vowel(next) => push(&mut out, Y),
                    Some(vowel) if vowel == U => push(&mut out, OFFGLIDE_I_AFTER_U),
                    Some(_) => push(&mut out, OFFGLIDE_I),
                    None => push(&mut out, I),
                }
            }
            'b' | 'v' => push(&mut out, B),
            'c' if next == Some('h') => {
                push(&mut out, CH);
                step = 2;
            }
            'c' if front(next) => push(&mut out, theta),
            'c' | 'k' => push(&mut out, K),
            'q' => {
                push(&mut out, K);
                // qu before e and i: the u is silent.
                if next == Some('u') && front(letter(at + 2)) {
                    step = 2;
                }
            }
            'd' => push(&mut out, D),
            'f' => push(&mut out, F),
            'g' if front(next) => push(&mut out, X),
            'g' => {
                push(&mut out, G);
                // gu before e and i: the u is silent (guerra); gü is said (pingüino).
                if next == Some('u') && front(letter(at + 2)) {
                    step = 2;
                }
            }
            'h' => {}
            'j' => push(&mut out, X),
            'l' if next == Some('l') => {
                push(
                    &mut out,
                    match variety {
                        Variety::Castilian => LL,
                        Variety::LatinAmerican => LL_YEISMO,
                    },
                );
                step = 2;
            }
            'l' => push(&mut out, L),
            'm' => push(&mut out, M),
            'n' => push(&mut out, N),
            'ñ' => push(&mut out, NY),
            'p' => push(&mut out, P),
            'r' if next == Some('r') => {
                push(&mut out, TRILL);
                step = 2;
            }
            // r starting a word or after l, n or s is trilled (rosa, honra, alrededor, Israel).
            'r' if at == 0 || matches!(letters[at - 1].0, 'l' | 'n' | 's') => push(&mut out, TRILL),
            'r' => push(&mut out, TAP),
            's' => push(&mut out, S),
            't' => push(&mut out, T),
            'w' => push(&mut out, GLIDE_U),
            'x' => {
                push(&mut out, K);
                push(&mut out, S);
            }
            'z' => push(&mut out, theta),
            _ => return None,
        }
        at += step;
    }
    // s before another s, as seseo makes sc and xc (descendencia, excesos), is said once.
    out.dedup_by(|this, before| this.phoneme == S && before.phoneme == S);
    Some(glides(out))
}

/// i and u without an accent next to another vowel are glides: before it, j and w (cielo, bueno;
/// in ui and iu the first, cuidado, ciudad); after it, the offglides of rey and euro. Two vowels
/// that are not i or u, an accented í or ú, and i or u after a word's or a cluster's r or l stay apart
/// (le-o, pa-ís, ru-i-do, cri-a-do).
fn glides(mut sounds: Vec<Sound>) -> Vec<Sound> {
    let vowel = |sound: Option<&Sound>| sound.is_some_and(|sound| sound.phoneme.is_vowel());
    let high = |sound: &Sound| !sound.accent && (sound.phoneme == I || sound.phoneme == U);
    for at in 0..sounds.len() {
        if !high(&sounds[at]) {
            continue;
        }
        let before = at.checked_sub(1).map(|before| &sounds[before]);
        let after = sounds.get(at + 1);
        // After a trilled r, or an r or l that starts a word or follows a consonant, i and u stay
        // vowels of their own (ruido ru-i-do, criado cri-a-do, luego lu-e-go), as eSpeak NG says
        // them; after r or l between vowels they glide (interior).
        let liquid_onset = |at: usize| {
            let consonant_before = at
                .checked_sub(1)
                .is_none_or(|before| !sounds[before].phoneme.is_vowel());
            sounds[at].phoneme == TRILL || ([TAP, L].contains(&sounds[at].phoneme) && consonant_before)
        };
        let after_r = at.checked_sub(1).is_some_and(liquid_onset);
        let rising = vowel(after) && !after_r && !(before.is_some_and(|before| vowel(Some(before)) && !high(before)));
        let falling = vowel(before) && before.is_some_and(|before| !high(before)) && !rising;
        sounds[at].phoneme = match (sounds[at].phoneme, rising, falling) {
            (phoneme, true, _) if phoneme == I => GLIDE_I,
            (_, true, _) => GLIDE_U,
            (phoneme, _, true) if phoneme == I && before.is_some_and(|before| before.phoneme == U) => {
                OFFGLIDE_I_AFTER_U
            }
            (phoneme, _, true) if phoneme == I => OFFGLIDE_I,
            (_, _, true) => OFFGLIDE_U,
            (phoneme, ..) => phoneme,
        };
    }
    sounds
}

/// How `word` (in any case) is said, stressed by the spelling: the vowel with a written accent,
/// else the last syllable but one in words ending in a vowel, n or s, else the last syllable.
/// `None` for a word with a letter Spanish does not read.
pub(crate) fn pronounce(word: &str, variety: Variety) -> Option<Pronunciation> {
    let lower = word.to_lowercase();
    let sounds = sounds(&lower, variety)?;
    let nuclei: Vec<usize> = (0..sounds.len()).filter(|&at| sounds[at].phoneme.is_vowel()).collect();
    // A word without a vowel (pst, shh) is said as its consonants.
    if nuclei.is_empty() {
        let segments: Vec<Segment> = sounds
            .iter()
            .map(|sound| Segment::new(sound.phoneme, Stress::Unstressed))
            .collect();
        return (!segments.is_empty()).then(|| syllabify(&segments, legal_onset));
    }
    let stressed = sounds.iter().position(|sound| sound.accent).or_else(|| {
        let last = lower.chars().last().map(|c| base(c).0);
        let penultimate = last.is_some_and(|c| "aeiouns".contains(c)) && nuclei.len() > 1;
        Some(nuclei[nuclei.len() - if penultimate { 2 } else { 1 }])
    })?;
    let segments: Vec<Segment> = sounds
        .iter()
        .enumerate()
        .map(|(at, sound)| {
            Segment::new(
                sound.phoneme,
                if at == stressed {
                    Stress::Primary
                } else {
                    Stress::Unstressed
                },
            )
        })
        .collect();
    let mut said = syllabify(&segments, legal_onset);
    open_e(&mut said);
    Some(said)
}

/// A stressed e in the last syllable but one, closed by n, is ɛ in eSpeak NG's notation (tengo,
/// cuenta, siento, entre).
fn open_e(said: &mut Pronunciation) {
    let count = said.syllables.len();
    if count >= 2
        && let Some(syllable) = said.syllables.get_mut(count - 2)
        && syllable.stress == Stress::Primary
        && syllable.nucleus == [E]
        && syllable.coda.first() == Some(&N)
    {
        syllable.nucleus = vec![E_OPEN];
    }
}

/// Adjectives ending in -mente, which are not adverbs made with -mente.
const NOT_ADVERBS: [&str; 5] = ["mente", "demente", "clemente", "inclemente", "vehemente"];

/// How a word is said: from jespersen's respellings (méxico → méjico) or its spelling, with an adverb
/// in -mente said as its adjective and mente, each stressed (rápidamente rápida-mente).
pub(crate) fn say(word: &str, variety: Variety) -> Option<Pronunciation> {
    let lower = word.to_lowercase();
    let lower = respelling(&lower).unwrap_or(&lower);
    if let Some(adjective) = lower.strip_suffix("mente")
        && adjective.chars().count() >= 3
        && !NOT_ADVERBS.contains(&lower)
        && let (Some(mut adjective), Some(mut mente)) = (pronounce(adjective, variety), pronounce("mente", variety))
    {
        // eSpeak NG keeps the e of -mente close (rápidamente ˈraβiðamˈente).
        for syllable in &mut mente.syllables {
            if syllable.nucleus == [E_OPEN] {
                syllable.nucleus = vec![E];
            }
        }
        adjective.syllables.extend(mente.syllables);
        return Some(adjective);
    }
    pronounce(lower, variety)
}

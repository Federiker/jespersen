//! How a Latvian word is said from its spelling: the vowels as written, long with a macron; o as
//! uo in native words and o in loanwords (skola skˈuola, telefons tˈelefoːns); ē open before a
//! back vowel, e and ē before l, r or n and a consonant in a last syllable, as eSpeak NG reads
//! them in native words (pilsēta, zēns, zelts), closed elsewhere; ie, ai, ei, au and ui; the palatals ķ, ģ, ļ and ņ, č, š, ž, dz and dž; and the
//! stress on the first syllable.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, AA, AE, AEAE, AI, AU, B, CH, D, DZ, DZH, E, EE, EI, F, G, GJ, H, I, IE, II, J, K, KJ, L, L_LONG, LJ, M, N, NG,
    NJ, O, OO, P, R, S, SH, T, TS, U, UI, UO, UU, V, Z, ZH, legal_onset,
};

fn is_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'ā' | 'e' | 'ē' | 'i' | 'ī' | 'o' | 'ō' | 'ò' | 'u' | 'ū' | 'y' | 'ä' | 'ǟ'
    )
}

/// The endings of loanwords, whose o is o rather than uo (telefons, direktors, ekonomika).
const LOAN_ENDINGS: [&str; 24] = [
    "ons", "ors", "ika", "ikas", "ija", "ijas", "iju", "ence", "ences", "āde", "ēka", "ēkas", "isms", "ists", "loģija",
    "grāfs", "grāfija", "ūra", "ūras", "oss", "ējs", "ejs", "ojs", "oja",
];

/// Whether `word` reads like a loanword: a loanword's ending, two vowels side by side that make
/// no Latvian diphthong (bibliotēka, teātris), jespersen's list's o (pròcenti), or a letter native
/// words do not have (f, h, x, w, q, y).
fn is_loan(word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();
    let hiatus = chars.windows(2).any(|pair| {
        is_vowel(pair[0])
            && is_vowel(pair[1])
            && !matches!(pair, ['i', 'e'] | ['a' | 'e' | 'u' | 'o', 'i'] | ['a', 'u'])
    });
    hiatus
        || chars.iter().any(|c| matches!(c, 'ò' | 'ō'))
        || LOAN_ENDINGS.iter().any(|ending| word.ends_with(ending))
        || word.chars().any(|c| matches!(c, 'f' | 'h' | 'x' | 'w' | 'q' | 'y'))
}

/// Whether an e or ē at `at` in a native word is open, as eSpeak NG reads it: before a consonant
/// and a back vowel (vēstule, pilsēta, cena), but not a palatal or postalveolar one nor tr (meža,
/// četras); and before l, r or n and a dental (zelts, zēns, bērni).
fn open(word: &[char], at: usize) -> bool {
    let rest = &word[at + 1..];
    let cluster: Vec<char> = rest.iter().copied().take_while(|&c| !is_vowel(c)).collect();
    if cluster
        .iter()
        .any(|c| matches!(c, 'ķ' | 'ģ' | 'ļ' | 'ņ' | 'j' | 'š' | 'ž' | 'č'))
        || cluster.starts_with(&['t', 'r'])
    {
        return false;
    }
    if matches!(cluster[..], ['l' | 'r' | 'n', 'n' | 't' | 's' | 'd', ..]) {
        return true;
    }
    match rest.get(cluster.len()) {
        Some(&next) if !cluster.is_empty() => {
            matches!(next, 'a' | 'ā' | 'u' | 'ū') || (next == 'o' && rest.get(cluster.len() + 1) != Some(&'i'))
        }
        _ => false,
    }
}

/// The phoneme of the vowel letter at `at`, and how many letters it takes.
fn vowel(word: &[char], at: usize, loan: bool) -> (Phoneme, usize) {
    let next = word.get(at + 1).copied();
    match (word[at], next) {
        ('i', Some('e')) => (IE, 2),
        ('a', Some('i')) => (AI, 2),
        ('e', Some('i')) => (EI, 2),
        ('a', Some('u')) => (AU, 2),
        ('u', Some('i')) => (UI, 2),
        ('a', _) => (A, 1),
        ('ā', _) => (AA, 1),
        ('e', _) if !loan && open(word, at) => (AE, 1),
        ('ē', _) if !loan && open(word, at) => (AEAE, 1),
        ('e', _) => (E, 1),
        ('ē', _) => (EE, 1),
        ('ä', _) => (AE, 1),
        ('ǟ', _) => (AEAE, 1),
        ('i' | 'y', _) => (I, 1),
        ('ī', _) => (II, 1),
        ('u', _) => (U, 1),
        ('ū', _) => (UU, 1),
        ('ō', _) => (OO, 1),
        ('ò', _) => (O, 1),
        // A loanword's o is long after the first syllable (telefons, futbols), short in it and
        // beside another vowel or j (hokejs, bibliotēka, birojs).
        ('o', _) if loan => {
            let first_syllable = !word[..at].iter().any(|&c| is_vowel(c));
            let beside_vowel =
                (at > 0 && is_vowel(word[at - 1])) || next.is_some_and(|next| is_vowel(next) || next == 'j');
            (if first_syllable || beside_vowel { O } else { OO }, 1)
        }
        _ => (UO, 1),
    }
}

/// The sounds of `word`'s letters, in lower case; `None` for a letter Latvian does not read.
fn sounds(word: &[char]) -> Option<Vec<Phoneme>> {
    let text: String = word.iter().collect();
    let loan = is_loan(&text);
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(word.len());
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        let next = word.get(at + 1).copied();
        if is_vowel(c) {
            let (vowel, length) = vowel(word, at, loan);
            sounds.push(vowel);
            at += length;
            continue;
        }
        let (sound, length): (&[Phoneme], usize) = match (c, next) {
            ('d', Some('ž')) => (&[DZH], 2),
            ('d', Some('z')) => (&[DZ], 2),
            ('c', Some('h')) => (&[H], 2),
            ('n', Some('k' | 'g' | 'ķ' | 'ģ')) => (&[NG], 1),
            ('b', _) => (&[B], 1),
            ('c', _) => (&[TS], 1),
            ('č', _) => (&[CH], 1),
            ('d', _) => (&[D], 1),
            ('f', _) => (&[F], 1),
            ('g', _) => (&[G], 1),
            ('ģ', _) => (&[GJ], 1),
            ('h', _) => (&[H], 1),
            ('j', _) => (&[J], 1),
            ('k' | 'q', _) => (&[K], 1),
            ('ķ', _) => (&[KJ], 1),
            // l before a voiced consonant is ll in eSpeak NG's notation (kalna kˈallna).
            ('l', Some('b' | 'd' | 'g' | 'm' | 'n' | 'v' | 'z' | 'ž' | 'j' | 'r')) => (&[L_LONG], 1),
            ('l', _) => (&[L], 1),
            ('ļ', _) => (&[LJ], 1),
            ('m', _) => (&[M], 1),
            ('n', _) => (&[N], 1),
            ('ņ', _) => (&[NJ], 1),
            ('p', _) => (&[P], 1),
            ('r', _) => (&[R], 1),
            ('s', _) => (&[S], 1),
            ('š', _) => (&[SH], 1),
            ('t', _) => (&[T], 1),
            ('v' | 'w', _) => (&[V], 1),
            ('x', _) => (&[K, S], 1),
            ('z', _) => (&[Z], 1),
            ('ž', _) => (&[ZH], 1),
            ('\'' | '’' | '-', _) => (&[], 1),
            _ => return None,
        };
        sounds.extend_from_slice(sound);
        at += length;
    }
    Some(sounds)
}

/// How `word` is said, stressed on its first syllable; `None` for a letter Latvian does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let chars: Vec<char> = word.to_lowercase().chars().collect();
    let sounds = sounds(&chars)?;
    if sounds.is_empty() {
        return None;
    }
    let first_vowel = sounds.iter().position(|sound| sound.is_vowel());
    let segments: Vec<Segment> = sounds
        .iter()
        .enumerate()
        .map(|(at, &sound)| {
            let stress = if Some(at) == first_vowel {
                Stress::Primary
            } else {
                Stress::Unstressed
            };
            Segment::new(sound, stress)
        })
        .collect();
    Some(syllabify(&segments, legal_onset))
}

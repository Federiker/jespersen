//! How a Catalan word is said from its spelling, in the central variety eSpeak NG reads: the
//! stress where the accent marks it, else on the last syllable but one of a word ending in a
//! vowel, -s, -en or -in and on the last of the others; unstressed a and e as ə and o as u; a
//! stressed e or o open or closed by what follows it (porta pˈɔrtə, cosa kˈɔzə but famosa
//! fəmˈozə), as eSpeak NG reads them; ll, ny, l·l, ix, tx, tg and tj, ig at the end; a final r, t
//! and p unsaid where Catalan drops them (cantar, moment, camp).

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, B, B_HARD, B_SOFT, CH, D, DZ, DZH, E, E_OPEN, F, G, G_HARD, I, J, K, L, LATENT_K, LATENT_P, LL, M, N, NG, NY, O,
    O_OPEN, P, R, R_TAP, S, SCHWA, SH, T, U, W, Z, ZH, legal_onset,
};

/// The accent a vowel letter carries.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Accent {
    Plain,
    /// à, è, ò: stressed, and open.
    Grave,
    /// é, í, ó, ú: stressed, and closed.
    Acute,
    /// ï, ü: a vowel of its own (raïm), or a u said after g and q (pingüí).
    Diaeresis,
}

/// What a letter, or a group of them, comes to before the stress is known.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Unit {
    /// A vowel letter without its accent, and where it is in the word.
    Vowel {
        letter: char,
        accent: Accent,
        at: usize,
    },
    Sound(Phoneme),
    /// A final r (or r before a final s), said or not by the stress and the vowel before it.
    FinalR,
    /// A letter not said, which still counts for the stress (the t of moments).
    Silent,
    /// The b of bl, doubled after the stressed vowel (poble pˈɔbblə) and soft elsewhere (oblidar).
    DoubledB,
}

/// `c` without its accent, and the accent.
fn plain(c: char) -> (char, Accent) {
    match c {
        'à' => ('a', Accent::Grave),
        'è' => ('e', Accent::Grave),
        'ò' => ('o', Accent::Grave),
        'é' => ('e', Accent::Acute),
        'í' => ('i', Accent::Acute),
        'ó' => ('o', Accent::Acute),
        'ú' => ('u', Accent::Acute),
        'ï' => ('i', Accent::Diaeresis),
        'ü' => ('u', Accent::Diaeresis),
        _ => (c, Accent::Plain),
    }
}

fn is_vowel(c: Option<char>) -> bool {
    matches!(c, Some('a' | 'e' | 'i' | 'o' | 'u'))
}

fn is_front(c: Option<char>) -> bool {
    matches!(c, Some('e' | 'i'))
}

/// The units of `word`'s letters (in lower case, without accents, which `accents` gives); `None`
/// for a letter Catalan does not read.
fn units(word: &[char], accents: &[Accent]) -> Option<Vec<Unit>> {
    let letter = |at: usize| word.get(at).copied();
    let before = |at: usize| at.checked_sub(1).and_then(letter);
    // Whether the letter at `at` ends the word, or only a final s follows it.
    let ends = |at: usize| at + 1 == word.len() || (at + 2 == word.len() && word[at + 1] == 's');
    let mut units: Vec<Unit> = Vec::with_capacity(word.len());
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        let next = letter(at + 1);
        let accent = accents[at];
        // An h between vowels keeps them apart (ahir əˈi).
        let after_vowel = matches!(units.last(), Some(Unit::Vowel { .. })) && before(at) != Some('h');
        let vowel = Unit::Vowel { letter: c, accent, at };
        let (unit, step): (&[Unit], usize) = match c {
            // u after g and q: unsaid before e and i (guerra, que), w before a and o (aigua,
            // quatre) and when it has a diaeresis (pingüí).
            'u' if matches!(before(at), Some('g' | 'q')) && is_vowel(next) => {
                if is_front(next) && accent == Accent::Plain {
                    (&[Unit::Silent], 1)
                } else {
                    (&[Unit::Sound(W)], 1)
                }
            }
            'i' | 'u' if accent != Accent::Plain => (&[vowel], 1),
            // The i of ix and of a final ig after a vowel is unsaid (caixa, vaig).
            'i' if after_vowel && (next == Some('x') || (next == Some('g') && ends(at + 1))) => (&[Unit::Silent], 1),
            // i and u beside a vowel are glides (noia, feina, veure, ciutat), and i before one at
            // the start (iogurt).
            'i' if after_vowel || (at == 0 && is_vowel(next)) => (&[Unit::Sound(J)], 1),
            'u' if after_vowel => (&[Unit::Sound(W)], 1),
            'a' | 'e' | 'o' | 'i' | 'u' => (&[vowel], 1),
            'y' => (
                &[Unit::Vowel {
                    letter: 'i',
                    accent,
                    at,
                }],
                1,
            ),
            // b: p at the end and before a voiceless consonant, but not after m (amb); doubled before
            // l after a vowel (poble pˈɔbblə).
            'b' if before(at) == Some('m') && ends(at) => (&[Unit::Sound(B)], 1),
            'b' if after_vowel && next == Some('l') => (&[Unit::DoubledB], 1),
            'b' if ends(at) || matches!(next, Some('s' | 't' | 'c' | 'p' | 'f')) => (&[Unit::Sound(P)], 1),
            'b' | 'v' => (&[Unit::Sound(B)], 1),
            // The c of xc and sc before e and i is said with the x or s (excessos, consciència).
            'c' if is_front(next) && matches!(before(at), Some('x' | 's')) => (&[Unit::Silent], 1),
            'c' if is_front(next) => (&[Unit::Sound(S)], 1),
            // A final c after n is said only before a vowel (banc baŋ), and never before an s.
            'c' if before(at) == Some('n') && at + 1 == word.len() => (&[Unit::Sound(LATENT_K)], 1),
            'c' if before(at) == Some('n') && ends(at) => (&[Unit::Silent], 1),
            'c' | 'k' | 'q' => (&[Unit::Sound(K)], 1),
            'd' if next == Some('j') => (&[Unit::Sound(DZH)], 2),
            'd' if ends(at) => (&[Unit::Sound(T)], 1),
            'd' => (&[Unit::Sound(D)], 1),
            'f' => (&[Unit::Sound(F)], 1),
            // g: ʒ before e and i; tʃ in a final ig (vaig, mig); unsaid in a final ng (sang).
            'g' if is_front(next) => (&[Unit::Sound(ZH)], 1),
            'g' if before(at) == Some('i') && ends(at) => (&[Unit::Sound(CH)], 1),
            'g' if before(at) == Some('n') && ends(at) => (&[Unit::Silent], 1),
            'g' if ends(at) => (&[Unit::Sound(K)], 1),
            'g' => (&[Unit::Sound(G)], 1),
            'j' => (&[Unit::Sound(ZH)], 1),
            'l' if next == Some('l') => (&[Unit::Sound(LL)], 2),
            'l' if next == Some('·') => (&[Unit::Sound(L)], 2),
            'l' => (&[Unit::Sound(L)], 1),
            'm' => (&[Unit::Sound(M)], 1),
            'n' if next == Some('y') => (&[Unit::Sound(NY)], 2),
            'n' if matches!(next, Some('c' | 'g' | 'k' | 'q')) && !is_front(letter(at + 2)) => (&[Unit::Sound(NG)], 1),
            'n' => (&[Unit::Sound(N)], 1),
            'p' if before(at) == Some('m') && at + 1 == word.len() => (&[Unit::Sound(LATENT_P)], 1),
            'p' if before(at) == Some('m') && (ends(at) || next == Some('t')) => (&[Unit::Silent], 1),
            'p' => (&[Unit::Sound(P)], 1),
            // r: trilled at the start, doubled and after n, l and s; a tap between vowels and after
            // a consonant; trilled before a consonant.
            'r' if next == Some('r') => (&[Unit::Sound(R)], 2),
            'r' if ends(at) => (&[Unit::FinalR], 1),
            'r' if at == 0 || matches!(before(at), Some('n' | 'l' | 's')) => (&[Unit::Sound(R)], 1),
            'r' if is_vowel(next) => (&[Unit::Sound(R_TAP)], 1),
            'r' => (&[Unit::Sound(R)], 1),
            // s: z between vowels and before a voiced consonant (casa, esbarzer).
            's' if next == Some('s') => (&[Unit::Sound(S)], 2),
            's' if at > 0 && (after_vowel || matches!(units.last(), Some(Unit::Sound(J | W)))) && is_vowel(next) => {
                (&[Unit::Sound(Z)], 1)
            }
            's' if matches!(next, Some('b' | 'd' | 'g' | 'l' | 'm' | 'n' | 'r' | 'v')) => (&[Unit::Sound(Z)], 1),
            // A final s after ny is ʃ (anys ˈaɲʃ).
            's' if before(at) == Some('y') && at + 1 == word.len() => (&[Unit::Sound(SH)], 1),
            's' | 'ç' => (&[Unit::Sound(S)], 1),
            't' if next == Some('x') => (&[Unit::Sound(CH)], 2),
            't' if next == Some('z') => (&[Unit::Sound(DZ)], 2),
            't' if next == Some('j') || (next == Some('g') && is_front(letter(at + 2))) => (&[Unit::Sound(DZH)], 2),
            // t is said as the consonant after it in tll and tm (ametlla, setmana).
            't' if next == Some('l') && letter(at + 2) == Some('l') => (&[Unit::Sound(LL)], 1),
            't' if next == Some('m') => (&[Unit::Sound(M)], 1),
            // A final t after n and l is unsaid (moment, molt), as before a final s after n, l and r
            // (moments, certs).
            't' if matches!(before(at), Some('n' | 'l')) && ends(at) => (&[Unit::Silent], 1),
            't' if before(at) == Some('r') && next == Some('s') && at + 2 == word.len() => (&[Unit::Silent], 1),
            't' => (&[Unit::Sound(T)], 1),
            'w' => (&[Unit::Sound(W)], 1),
            // x: ʃ at the start, after i and after a consonant (xarxa, caixa); ɡz in ex- before a
            // vowel, ks elsewhere (exemple, explicar, taxi).
            'x' if at == 0 || before(at) == Some('i') || !is_vowel(before(at)) => (&[Unit::Sound(SH)], 1),
            'x' if at == 1 && word[0] == 'e' && is_vowel(next) => (&[Unit::Sound(G_HARD), Unit::Sound(Z)], 1),
            'x' => (&[Unit::Sound(K), Unit::Sound(S)], 1),
            'z' => (&[Unit::Sound(Z)], 1),
            'h' | '\'' | '’' | '-' | '·' => (&[], 1),
            _ => return None,
        };
        units.extend_from_slice(unit);
        at += step;
    }
    Some(units)
}

/// Which of the vowels (from 0, among `count`) is stressed when no accent says: the last but one
/// when the word ends in a vowel, a vowel and s, or en or in; the last otherwise.
fn stressed(units: &[Unit], word: &[char], count: usize) -> usize {
    let is_vowel_unit = |unit: Option<&Unit>| matches!(unit, Some(Unit::Vowel { .. }));
    let length = units.len();
    let before_last = length.checked_sub(2).and_then(|at| units.get(at));
    let open_end = is_vowel_unit(units.last())
        || (word.last() == Some(&'s') && is_vowel_unit(before_last))
        || (word.last() == Some(&'n') && matches!(before_last, Some(Unit::Vowel { letter: 'e' | 'i', .. })));
    if open_end && count >= 2 { count - 2 } else { count - 1 }
}

/// Endings after a stressed e that eSpeak NG reads closed: the consonants before the next vowel
/// (with a plural s dropped), and whether the word ends there.
const CLOSED_E: [(&str, bool); 28] = [
    ("", true),
    ("nt", true),
    ("r", true),
    ("ll", true),
    ("st", true),
    ("r", false),
    ("ss", false),
    ("ll", false),
    ("nt", false),
    ("ct", false),
    ("st", false),
    ("br", false),
    ("v", false),
    ("rv", false),
    ("ndr", false),
    ("ntr", false),
    ("rm", false),
    ("ng", false),
    ("tg", false),
    ("tj", false),
    ("mbr", false),
    ("pt", false),
    ("spr", false),
    ("mpl", false),
    ("mpr", false),
    ("q", false),
    ("nd", false),
    ("ctr", false),
];

/// The same for a stressed o.
const CLOSED_O: [(&str, bool); 24] = [
    ("r", true),
    ("n", true),
    ("ll", true),
    ("x", true),
    ("x", false),
    ("nt", true),
    ("rn", true),
    ("s", false),
    ("r", false),
    ("n", false),
    ("ss", false),
    ("m", false),
    ("rm", false),
    ("rr", false),
    ("ll", false),
    ("nd", false),
    ("mbr", false),
    ("ntr", false),
    ("tz", false),
    ("mpr", false),
    ("bt", false),
    ("ns", false),
    ("ls", false),
    ("p", false),
];

/// Whether a stressed e or o at `at` is closed, by the letters after it (cosa kˈɔzə, famosa
/// fəmˈozə; dent dˈen, set sˈɛt).
fn closed(word: &[char], at: usize) -> bool {
    let mut rest = &word[at + 1..];
    // An unsaid i before x counts for nothing (angoixa).
    if rest.starts_with(&['i', 'x']) {
        rest = &rest[1..];
    }
    // o before r and i is open (il·lusòria, satisfactori).
    if word[at] == 'o' && rest.starts_with(&['r', 'i']) {
        return false;
    }
    let length = rest.iter().take_while(|&&c| !is_vowel(Some(c))).count();
    let end = length == rest.len();
    let mut cluster: String = rest[..length].iter().filter(|&&c| c != 'h').collect();
    if end && cluster.len() > 1 && cluster.ends_with('s') && !cluster.ends_with("ss") {
        cluster.pop();
    }
    let table: &[(&str, bool)] = if word[at] == 'e' { &CLOSED_E } else { &CLOSED_O };
    table.contains(&(cluster.as_str(), end))
}

/// The vowel a unit's letter is said as, stressed or not.
fn vowel_sound(word: &[char], letter: char, accent: Accent, at: usize, stressed: bool) -> Phoneme {
    match (letter, stressed) {
        ('a', true) => A,
        ('a' | 'e', false) => SCHWA,
        ('o' | 'u', false) | ('u', true) => U,
        ('i', _) => I,
        ('e', true) if accent == Accent::Acute || (accent == Accent::Plain && closed(word, at)) => E,
        ('e', true) => E_OPEN,
        (_, true) if accent == Accent::Acute || (accent == Accent::Plain && closed(word, at)) => O,
        _ => O_OPEN,
    }
}

/// How a final r is said (a tap before a vowel in the next word, which the reading of a sentence
/// makes): a tap in per; trilled after an unstressed vowel (ésser, màrtir), after u
/// (pur), in a word of one syllable in -ar or -or (mar, cor), in -ilar, -ular and -rror (similar,
/// regular, terror) and before an enclitic pronoun (menjar-se); otherwise it is unsaid (cantar,
/// carrer, dormir, color).
fn final_r(word: &[char], before: Option<char>, stressed_last: bool, one_syllable: bool) -> Option<Phoneme> {
    let text: String = word.iter().collect();
    if text == "per" {
        return Some(R_TAP);
    }
    let text = text.strip_suffix('s').unwrap_or(&text);
    let said = !stressed_last
        || before == Some('u')
        || (one_syllable && matches!(before, Some('a' | 'o')))
        || ["ilar", "ular", "rror"].iter().any(|ending| text.ends_with(ending));
    said.then_some(R)
}

/// The segments of `word`, its final r said when `keep_r` (before an enclitic pronoun); `None`
/// for a letter Catalan does not read.
pub(crate) fn segments(word: &str, keep_r: bool) -> Option<Vec<Segment>> {
    let (chars, accents): (Vec<char>, Vec<Accent>) = word.to_lowercase().chars().map(plain).unzip();
    let units = units(&chars, &accents)?;
    let vowels: Vec<usize> = (0..units.len())
        .filter(|&at| matches!(units[at], Unit::Vowel { .. }))
        .collect();
    let stress = vowels
        .iter()
        .position(|&at| {
            matches!(
                units[at],
                Unit::Vowel {
                    accent: Accent::Grave | Accent::Acute,
                    ..
                }
            )
        })
        .or_else(|| (!vowels.is_empty()).then(|| stressed(&units, &chars, vowels.len())));
    let stressed_at = stress.map(|stress| vowels[stress]);
    let last_vowel = vowels.last().copied();
    let mut segments: Vec<Segment> = Vec::with_capacity(units.len());
    for (index, unit) in units.iter().enumerate() {
        let phoneme = match *unit {
            Unit::Vowel { letter, accent, at } => vowel_sound(&chars, letter, accent, at, Some(index) == stressed_at),
            Unit::Sound(phoneme) => phoneme,
            Unit::FinalR if keep_r => R,
            Unit::FinalR => {
                let before = match index.checked_sub(1).map(|at| units[at]) {
                    Some(Unit::Vowel { letter, .. }) => Some(letter),
                    _ => None,
                };
                let Some(said) = final_r(&chars, before, stressed_at == last_vowel, vowels.len() == 1) else {
                    continue;
                };
                said
            }
            Unit::Silent => continue,
            Unit::DoubledB => {
                let after_stress = units[..index]
                    .iter()
                    .rposition(|unit| matches!(unit, Unit::Vowel { .. }))
                    .is_some_and(|vowel| Some(vowel) == stressed_at);
                if after_stress {
                    segments.push(Segment::new(B_HARD, Stress::Unstressed));
                    B_HARD
                } else {
                    B_SOFT
                }
            }
        };
        let stress = if Some(index) == stressed_at {
            Stress::Primary
        } else {
            Stress::Unstressed
        };
        segments.push(Segment::new(phoneme, stress));
    }
    Some(segments)
}

/// The endings of the adjective that make a word in -ment an adverb (solament, generalment,
/// probablement); other words in -ment are nouns (moment, pensament).
const ADVERB_STEMS: [&str; 12] = [
    "l", "ble", "ple", "bre", "osa", "iva", "nta", "nt", "sola", "ica", "ça", "ssa",
];

/// An adverb in -ment keeps the stress of its adjective, and eSpeak NG marks both (solament
/// sˈɔləmˈen): the adjective's segments and those of ment, or `None` for another word.
fn adverb(word: &str) -> Option<Vec<Segment>> {
    let stem = word.strip_suffix("ment")?;
    let accented = stem
        .chars()
        .any(|c| plain(c).1 == Accent::Grave || plain(c).1 == Accent::Acute);
    if stem.chars().count() < 3 || !(accented || ADVERB_STEMS.iter().any(|ending| stem.ends_with(ending))) {
        return None;
    }
    let mut segments = self::segments(stem, false)?;
    segments.extend(self::segments("ment", false)?);
    Some(segments)
}

/// How `word` is said; `None` for a letter Catalan does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let lower = word.to_lowercase();
    let segments = adverb(&lower).map_or_else(|| segments(&lower, false), Some)?;
    if segments.is_empty() {
        return None;
    }
    Some(syllabify(&segments, legal_onset))
}

/// Whether `phoneme` is a voiced b, d or g that softens after a vowel (cada kˈaðə).
pub(crate) fn softens(phoneme: Phoneme) -> bool {
    [B, D, G].contains(&phoneme)
}

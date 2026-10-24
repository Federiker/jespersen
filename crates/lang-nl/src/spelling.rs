//! How a Dutch word is said from its spelling: a vowel long in an open syllable and when doubled
//! (maken, maan), short in a closed one (man); oe, eu, ui, ei and ij, ou and au; g and ch as ɣ
//! and x, sch as sx at the start of a stem and s elsewhere (school, mensch), w as ʋ, the
//! consonants unvoiced at the end of a word; an unstressed e as ə, -ig as əx and -lijk as lək;
//! the stress on the first syllable of the stem, after an unstressed prefix (be-, ge-, ver-), and
//! on the endings of loanwords (-teit, -eren, -ij, -ie).

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, A_LONG, AU, B, D, E, E_BEFORE_R, E_LONG, EI, EU, F, GH, H, I, IE, J, K, L, M, N, NG, O, O_LONG, OE, P, R, S,
    SCHWA, SH, T, U, UI, UU, V, W, W_GLIDE, X, Z, legal_onset,
};

/// A vowel letter, or a group of them said as one vowel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    A,
    E,
    I,
    O,
    U,
    Aa,
    Ee,
    Oo,
    Uu,
    Ie,
    Oe,
    Eu,
    Ui,
    Ei,
    Au,
}

/// What a letter, or a group of them, comes to before the stress is known.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Unit {
    /// A vowel, and where its letters start and end.
    Vowel {
        kind: Kind,
        at: usize,
        end: usize,
    },
    Sound(Phoneme),
}

fn is_vowel(c: Option<char>) -> bool {
    matches!(
        c,
        Some('a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'é' | 'è' | 'ë' | 'ï' | 'ö' | 'ü' | 'á' | 'ó' | 'ú' | 'í')
    )
}

/// Whether a voiced consonant before `next` is unvoiced: at the end of a word and before a
/// voiceless consonant (hond hˈɔnt, zegt).
fn unvoiced_before(next: Option<char>) -> bool {
    next.is_none() || matches!(next, Some('t' | 's' | 'k' | 'p' | 'f' | 'c'))
}

/// The units of one part of a word (a prefix or a stem), in lower case; `offset` is where the part
/// starts in the word.
fn units(word: &[char], offset: usize) -> Option<Vec<Unit>> {
    let letter = |at: usize| word.get(at).copied();
    let mut units: Vec<Unit> = Vec::with_capacity(word.len());
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        let next = letter(at + 1);
        let after = letter(at + 2);
        let vowel = |kind: Kind, length: usize| Unit::Vowel {
            kind,
            at: offset + at,
            end: offset + at + length,
        };
        let rest: String = word[at..].iter().collect();
        let (unit, step): (Vec<Unit>, usize) = match c {
            _ if rest.starts_with("eeuw") => (vec![vowel(Kind::Ee, 2), Unit::Sound(W_GLIDE)], 4),
            _ if rest.starts_with("ieuw") => (vec![vowel(Kind::Ie, 2), Unit::Sound(W_GLIDE)], 4),
            _ if rest.starts_with("ouw") || rest.starts_with("auw") => (vec![vowel(Kind::Au, 3)], 3),
            _ if rest.starts_with("aai") => (vec![vowel(Kind::Aa, 2), Unit::Sound(J)], 3),
            _ if rest.starts_with("ooi") => (vec![vowel(Kind::Oo, 2), Unit::Sound(J)], 3),
            _ if rest.starts_with("oei") => (vec![vowel(Kind::Oe, 2), Unit::Sound(J)], 3),
            'u' if next == Some('w') => (vec![vowel(Kind::Uu, 1), Unit::Sound(W_GLIDE)], 2),
            'a' if next == Some('a') => (vec![vowel(Kind::Aa, 2)], 2),
            'e' if next == Some('e') => (vec![vowel(Kind::Ee, 2)], 2),
            'o' if next == Some('o') => (vec![vowel(Kind::Oo, 2)], 2),
            'u' if next == Some('u') => (vec![vowel(Kind::Uu, 2)], 2),
            'i' if next == Some('e') => (vec![vowel(Kind::Ie, 2)], 2),
            'o' if next == Some('e') => (vec![vowel(Kind::Oe, 2)], 2),
            'e' if next == Some('u') => (vec![vowel(Kind::Eu, 2)], 2),
            'u' if next == Some('i') => (vec![vowel(Kind::Ui, 2)], 2),
            'e' | 'i' if next == Some(if c == 'e' { 'i' } else { 'j' }) => (vec![vowel(Kind::Ei, 2)], 2),
            'a' | 'o' if next == Some('u') => (vec![vowel(Kind::Au, 2)], 2),
            'a' => (vec![vowel(Kind::A, 1)], 1),
            'e' | 'ë' => (vec![vowel(Kind::E, 1)], 1),
            'i' | 'ï' | 'y' => (vec![vowel(Kind::I, 1)], 1),
            'o' | 'ö' => (vec![vowel(Kind::O, 1)], 1),
            'u' | 'ü' => (vec![vowel(Kind::U, 1)], 1),
            // ssch, in the older spelling, is s (tusschen).
            's' if rest.starts_with("ssch") => (vec![Unit::Sound(S)], 4),
            // sch is sx at the start of a part (school, verschijnen) and s elsewhere (mensch,
            // tusschen).
            's' if next == Some('c') && after == Some('h') => {
                if at == 0 {
                    (vec![Unit::Sound(S), Unit::Sound(X)], 3)
                } else {
                    (vec![Unit::Sound(S)], 3)
                }
            }
            'c' if next == Some('h') => (vec![Unit::Sound(X)], 2),
            'c' if next == Some('k') => (vec![Unit::Sound(K)], 2),
            'c' if matches!(next, Some('e' | 'i' | 'y')) => (vec![Unit::Sound(S)], 1),
            'q' if next == Some('u') => (vec![Unit::Sound(K), Unit::Sound(W)], 2),
            'c' | 'k' | 'q' => (vec![Unit::Sound(K)], 1),
            'p' if next == Some('h') => (vec![Unit::Sound(F)], 2),
            't' if next == Some('h') => (vec![Unit::Sound(T)], 2),
            'd' if next == Some('t') => (vec![Unit::Sound(T)], 2),
            // -tion is ʃɔn (station).
            't' if rest.starts_with("tion") && at > 0 => (vec![Unit::Sound(SH)], 2),
            // -tie after a vowel or n is tsi (natie, advertentie), after c si (functie).
            't' if next == Some('i')
                && after == Some('e')
                && matches!(word[at + 3..], [] | ['s'])
                && at > 0
                && (is_vowel(letter(at - 1)) || matches!(letter(at - 1), Some('n' | 'c'))) =>
            {
                if letter(at - 1) == Some('c') {
                    (vec![Unit::Sound(S)], 1)
                } else {
                    (vec![Unit::Sound(T), Unit::Sound(S)], 1)
                }
            }
            'n' if next == Some('g') => (vec![Unit::Sound(NG)], 2),
            'n' if next == Some('k') => (vec![Unit::Sound(NG)], 1),
            // g before a final d is x (jeugd).
            'g' if unvoiced_before(next) || (next == Some('d') && after.is_none()) => (vec![Unit::Sound(X)], 1),
            'g' => (vec![Unit::Sound(GH)], 1),
            'd' if unvoiced_before(next) => (vec![Unit::Sound(T)], 1),
            'b' if unvoiced_before(next) => (vec![Unit::Sound(P)], 1),
            'v' if unvoiced_before(next) => (vec![Unit::Sound(F)], 1),
            'z' if unvoiced_before(next) => (vec![Unit::Sound(S)], 1),
            'b' => (vec![Unit::Sound(B)], 1),
            'd' => (vec![Unit::Sound(D)], 1),
            'f' => (vec![Unit::Sound(F)], 1),
            'h' => (vec![Unit::Sound(H)], 1),
            'j' => (vec![Unit::Sound(J)], 1),
            'l' => (vec![Unit::Sound(L)], 1),
            'm' => (vec![Unit::Sound(M)], 1),
            'n' => (vec![Unit::Sound(N)], 1),
            'p' => (vec![Unit::Sound(P)], 1),
            'r' => (vec![Unit::Sound(R)], 1),
            // s between vowels is z (filosofie, basis).
            's' if at > 0 && is_vowel(letter(at - 1)) && is_vowel(next) => (vec![Unit::Sound(Z)], 1),
            's' => (vec![Unit::Sound(S)], 1),
            't' => (vec![Unit::Sound(T)], 1),
            'v' => (vec![Unit::Sound(V)], 1),
            'w' => (vec![Unit::Sound(W)], 1),
            'x' => (vec![Unit::Sound(K), Unit::Sound(S)], 1),
            'z' => (vec![Unit::Sound(Z)], 1),
            '\'' | '’' | '-' => (Vec::new(), 1),
            _ => return None,
        };
        // A doubled consonant is said once (bakken, zeggen).
        let doubled = step == 1 && next == Some(c) && !is_vowel(Some(c));
        units.extend(unit);
        at += if doubled { 2 } else { step };
    }
    Some(units)
}

/// The unstressed prefixes (beginnen, gezicht, verstaan, ontmoeten).
const PREFIXES: [&str; 5] = ["be", "ge", "ver", "ont", "her"];

/// Words that start like a prefix but are not one (geven, beter, herfst).
const NOT_PREFIXED: [&str; 14] = [
    "geven", "geef", "geel", "gele", "geld", "beter", "beste", "beter", "bed", "beek", "beeld", "herfst", "heren",
    "herder",
];

/// The unstressed prefix `word` starts with, if any.
fn prefix(word: &str) -> Option<&'static str> {
    if NOT_PREFIXED.iter().any(|stem| word.starts_with(stem)) {
        return None;
    }
    PREFIXES.iter().copied().find(|prefix| {
        let Some(stem) = word.strip_prefix(prefix) else {
            return false;
        };
        let mut letters = stem.chars();
        let (first, second) = (letters.next(), letters.next());
        stem.chars().count() >= 3
            && stem.chars().any(|c| is_vowel(Some(c)))
            // Not a vowel the prefix's e would join (beide, geur), nor a doubled consonant (bedden).
            && !(prefix.ends_with('e') && matches!(first, Some('i' | 'u' | 'e')))
            && !(first.is_some() && first == second && !is_vowel(first))
    })
}

/// The endings of loanwords that take the stress, and which vowel from the end is stressed: 0
/// for the last (universiteit, bakkerij), 1 for the one before (studeren, informatie).
const STRESSED_ENDINGS: [(&str, usize); 26] = [
    ("ee", 0),
    ("eeën", 1),
    ("tie", 1),
    ("ties", 1),
    ("ion", 0),
    ("oor", 0),
    ("teit", 0),
    ("teiten", 1),
    ("eren", 1),
    ("eert", 0),
    ("eerde", 1),
    ("ij", 0),
    ("ijen", 1),
    ("ie", 0),
    ("ieën", 1),
    ("ist", 0),
    ("isten", 1),
    ("isme", 1),
    ("eur", 0),
    ("ent", 0),
    ("ant", 0),
    ("ief", 0),
    ("aal", 0),
    ("eel", 0),
    ("iek", 0),
    ("uur", 0),
];

/// The endings that are never stressed, though they look like a loanword's (-lijk, -heid).
const UNSTRESSED_ENDINGS: [&str; 4] = ["lijk", "heid", "lijke", "lijken"];

/// Which vowel unit is stressed: the one an acute accent is on (jespersen's list: waaróm), a
/// loanword ending's, else the first of the stem after the prefix.
fn stressed(units: &[Unit], word: &str, stem: usize, accent: Option<usize>) -> Option<usize> {
    let vowels: Vec<usize> = (0..units.len())
        .filter(|&index| matches!(units[index], Unit::Vowel { .. }))
        .collect();
    if let Some(accent) = accent {
        return vowels
            .iter()
            .copied()
            .find(|&index| matches!(units[index], Unit::Vowel { at, end, .. } if at <= accent && accent < end));
    }
    if !UNSTRESSED_ENDINGS.iter().any(|ending| word.ends_with(ending))
        && let Some(&(_, back)) = STRESSED_ENDINGS
            .iter()
            .filter(|(ending, _)| word.ends_with(ending))
            .max_by_key(|(ending, _)| ending.len())
        && vowels.len() > 1 + back
    {
        return Some(vowels[vowels.len() - 1 - back]);
    }
    vowels
        .iter()
        .copied()
        .find(|&index| matches!(units[index], Unit::Vowel { at, .. } if at >= stem))
        .or_else(|| vowels.first().copied())
}

/// Whether a single vowel letter is in an open syllable: before one consonant and a vowel, or at
/// the end of the word (maken, na); ch, ng, nk and sch close it (lachen).
fn open(word: &[char], end: usize) -> bool {
    let rest = &word[end..];
    let cluster: Vec<char> = rest.iter().copied().take_while(|&c| !is_vowel(Some(c))).collect();
    match cluster[..] {
        [] => true,
        // c before e or i is one consonant, s (procent).
        ['c'] if matches!(rest.get(1), Some('e' | 'i')) => true,
        ['c' | 'x' | 'q', ..] => false,
        [_] => cluster.len() < rest.len(),
        _ => false,
    }
}

/// The phoneme of a vowel unit: `stressed` or not, `schwa` when an unstressed e or an ending's i
/// is reduced, `before_r` for the long e of meer.
fn vowel_phoneme(kind: Kind, open: bool, schwa: bool, before_r: bool) -> Phoneme {
    match kind {
        _ if schwa => SCHWA,
        Kind::A if open => A_LONG,
        Kind::A => A,
        Kind::E | Kind::Ee if before_r && (open || kind == Kind::Ee) => E_BEFORE_R,
        Kind::E if open => E_LONG,
        Kind::E => E,
        Kind::I if open => IE,
        Kind::I => I,
        Kind::O if open => O_LONG,
        Kind::O => O,
        Kind::U if open => UU,
        Kind::U => U,
        Kind::Aa => A_LONG,
        Kind::Ee => E_LONG,
        Kind::Oo => O_LONG,
        Kind::Uu => UU,
        Kind::Ie => IE,
        Kind::Oe => OE,
        Kind::Eu => EU,
        Kind::Ui => UI,
        Kind::Ei => EI,
        Kind::Au => AU,
    }
}

/// `c` without an acute accent, which marks the stress (é, jespersen's list: waaróm), or a grave
/// one, which keeps an unstressed e full (duizènd); and which it had.
fn plain(c: char) -> (char, Option<char>) {
    match c {
        'á' => ('a', Some('´')),
        'é' => ('e', Some('´')),
        'í' => ('i', Some('´')),
        'ó' => ('o', Some('´')),
        'ú' => ('u', Some('´')),
        'è' => ('e', Some('`')),
        // The older spelling's circumflex marks a contraction (moê, elkaâr).
        'â' | 'à' => ('a', None),
        'ê' => ('e', None),
        'ô' => ('o', None),
        'û' => ('u', None),
        _ => (c, None),
    }
}

/// How `word` is said; `None` for a letter Dutch does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let (chars, marks): (Vec<char>, Vec<Option<char>>) = word.to_lowercase().chars().map(plain).unzip();
    let lower: String = chars.iter().collect();
    let accent = marks.iter().position(|&mark| mark == Some('´'));
    let prefix = if accent.is_some() { None } else { prefix(&lower) };
    let stem = prefix.map_or(0, |prefix| prefix.chars().count());
    let mut units = if stem > 0 {
        self::units(&chars[..stem], 0)?
    } else {
        Vec::new()
    };
    units.extend(self::units(&chars[stem..], stem)?);
    let stressed = stressed(&units, &lower, stem, accent);
    let mut segments: Vec<Segment> = Vec::with_capacity(units.len());
    for (index, unit) in units.iter().enumerate() {
        let stress = if Some(index) == stressed {
            Stress::Primary
        } else {
            Stress::Unstressed
        };
        let phoneme = match *unit {
            Unit::Vowel { kind, at, end, .. } => {
                let unstressed = stress == Stress::Unstressed;
                let after_stress = stressed.is_some_and(|stressed| index > stressed);
                let rest: String = chars[end..].iter().collect();
                // An unstressed e is ə after the stress, in a prefix and after a doubled consonant
                // (bakkerij), but not elsewhere before the stress (universiteit); so are the i of -ig and the ij of -lijk after the stress
                // (twintig tʋˈɪntəx, vriendelijk).
                let schwa = unstressed
                    && marks[at] != Some('`')
                    && ((kind == Kind::E
                        && (after_stress || at < stem || (at >= 2 && chars[at - 1] == chars[at - 2])))
                        || (after_stress
                            && kind == Kind::I
                            && (rest == "g" || rest.starts_with("ge") || rest == "gs"))
                        || (after_stress
                            && kind == Kind::Ei
                            && chars[..at].ends_with(&['l'])
                            && rest.starts_with('k')));
                // Before r and a vowel or the end (meer, studeren; but begeerte eː).
                let before_r =
                    chars.get(end) == Some(&'r') && (end + 1 == chars.len() || is_vowel(chars.get(end + 1).copied()));
                // In a part of its own a prefix's vowel is closed by the part's end.
                let open = at >= stem && open(&chars, end);
                vowel_phoneme(kind, open, schwa, before_r)
            }
            Unit::Sound(phoneme) => phoneme,
        };
        segments.push(Segment::new(
            phoneme,
            if phoneme.is_vowel() { stress } else { Stress::Unstressed },
        ));
    }
    if segments.is_empty() {
        return None;
    }
    let mut said = syllabify(&segments, legal_onset);
    rhythm(&mut said);
    Some(said)
}

/// eSpeak NG's rhythmic stresses: every other syllable from the first, up to two before the
/// stress (ˌynivˌɛrsitˈɛɪt), and a secondary one after it.
fn rhythm(said: &mut Pronunciation) {
    let Some(stressed) = said
        .syllables
        .iter()
        .position(|syllable| syllable.stress == Stress::Primary)
    else {
        return;
    };
    for syllable in said.syllables.iter_mut().take(stressed.saturating_sub(1)).step_by(2) {
        syllable.stress = Stress::Rhythmic;
    }
    // After the stress, a full vowel two syllables on takes a secondary stress (Amsterdam
    // ˈɑmstərdˌɑm, vergadering vərɣˈaːdərˌɪŋ).
    for syllable in said.syllables.iter_mut().skip(stressed + 2).step_by(2) {
        if syllable.nucleus.iter().all(|vowel| *vowel != SCHWA) {
            syllable.stress = Stress::Secondary;
        }
    }
}

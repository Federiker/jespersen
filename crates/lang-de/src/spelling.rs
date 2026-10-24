//! How a German word is said from its spelling: the vowels long before one consonant and a vowel,
//! after h, doubled or in ie, and short before two consonants; ch as ç or x by the vowel before
//! it, sp and st as ʃp and ʃt at the start of a stem, s as z before a vowel, b, d, g and s
//! unvoiced at the end of a syllable, -ig as ɪç, an unstressed e as ə and -er as ɐ; the stress on
//! the first syllable of the stem, after an unstressed prefix (be-, ge-, ver-), and on the
//! suffixes of loanwords (-ion, -ität, -ieren, -ei).

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, A_LONG, ACH, AE_LONG, AI, AU, B, CH, D, DZH, E, E_LONG, ER, EU, F, G, H, I, I_LONG, ICH, J, K, L, M, N, NG, O,
    O_LONG, OE, OE_LONG, P, PF, R, R_AFTER_U, R_TAP, R_VOCALIC, S, SCHWA, SH, T, TS, U, U_LONG, UE, UE_LONG, V, Z,
    legal_onset,
};

/// A vowel letter, or a digraph said as one vowel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    A,
    E,
    I,
    O,
    U,
    Ae,
    Oe,
    Ue,
    Ai,
    Au,
    Eu,
}

/// What a letter, or a group of them, comes to before the stress is known.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Unit {
    /// A vowel: what it is, where its letters start and end, and its length when the spelling says
    /// it (ie, aa, ah; ss after it).
    Vowel {
        kind: Kind,
        at: usize,
        end: usize,
        long: Option<bool>,
    },
    Sound(Phoneme),
    /// An r, said by what is around it once the vowels are known.
    R {
        onset: bool,
    },
}

fn is_vowel(c: Option<char>) -> bool {
    matches!(c, Some('a' | 'e' | 'i' | 'o' | 'u' | 'ä' | 'ö' | 'ü' | 'y'))
}

/// Whether a b, d or g (`c`) keeps its voice before `next`: before a vowel, and before l or r, but
/// a d after a consonant (Adler, gleich, but endlich).
fn keeps_voice(c: char, next: Option<char>, before: Option<char>) -> bool {
    is_vowel(next) || (matches!(next, Some('l' | 'r')) && (c != 'd' || before.is_none() || is_vowel(before)))
}

/// The units of one part of a word (a prefix or a stem), in lower case.
#[expect(clippy::too_many_lines, reason = "one arm per spelling, read in order")]
fn units(word: &[char], offset: usize) -> Option<Vec<Unit>> {
    let letter = |at: usize| word.get(at).copied();
    let before = |at: usize| at.checked_sub(1).and_then(letter);
    let mut units: Vec<Unit> = Vec::with_capacity(word.len());
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        let next = letter(at + 1);
        let after = letter(at + 2);
        let vowel = |kind: Kind, length: usize, long: Option<bool>| Unit::Vowel {
            kind,
            at: offset + at,
            end: offset + at + length,
            long,
        };
        let back = matches!(
            units.last(),
            Some(Unit::Vowel {
                kind: Kind::A | Kind::O | Kind::U | Kind::Au,
                ..
            })
        );
        let (unit, step): (Vec<Unit>, usize) = match (c, next) {
            ('a' | 'e', Some('i' | 'y')) => (vec![vowel(Kind::Ai, 2, None)], 2),
            ('a', Some('u')) => (vec![vowel(Kind::Au, 2, None)], 2),
            ('e' | 'ä', Some('u')) => (vec![vowel(Kind::Eu, 2, None)], 2),
            // -ie and -ien after l are i and ə (Familie famˈiːliə).
            ('i', Some('e')) if before(at) == Some('l') && matches!(word[at + 2..], [] | ['n']) => {
                (vec![vowel(Kind::I, 1, Some(false)), vowel(Kind::E, 1, None)], 2)
            }
            ('i', Some('e')) => (vec![vowel(Kind::I, 2, Some(true))], 2),
            ('a', Some('a')) | ('e', Some('e')) | ('o', Some('o')) => {
                let kind = match c {
                    'a' => Kind::A,
                    'e' => Kind::E,
                    _ => Kind::O,
                };
                (vec![vowel(kind, 2, Some(true))], 2)
            }
            // A vowel before h and no vowel after it is long, the h unsaid (sehen, Sohn).
            ('a' | 'e' | 'i' | 'o' | 'u' | 'ä' | 'ö' | 'ü', Some('h')) => {
                let kind = kind_of(c);
                (vec![vowel(kind, 2, Some(true))], 2)
            }
            // i before another vowel in a loanword is j (Nation natsjˈoːn).
            ('i', Some('a' | 'o' | 'u')) if at > 0 && !is_vowel(before(at)) => (vec![Unit::Sound(J)], 1),
            ('a' | 'e' | 'i' | 'o' | 'u' | 'ä' | 'ö' | 'ü', _) => (vec![vowel(kind_of(c), 1, None)], 1),
            ('y', _) if at == 0 && is_vowel(next) => (vec![Unit::Sound(J)], 1),
            ('y', _) => (vec![vowel(Kind::Ue, 1, None)], 1),
            ('s', Some('c')) if after == Some('h') => (vec![Unit::Sound(SH)], 3),
            ('s', Some('p' | 't')) if at == 0 => (vec![Unit::Sound(SH)], 1),
            ('s', Some('s')) => (vec![Unit::Sound(S)], 2),
            ('s', _)
                if is_vowel(next)
                    && (at == 0
                        || is_vowel(before(at))
                        || matches!(before(at), Some('l' | 'm' | 'n' | 'r'))
                        || (before(at) == Some('g') && at >= 2 && letter(at - 2) == Some('n'))) =>
            {
                (vec![Unit::Sound(Z)], 1)
            }
            ('s' | 'ß', _) => (vec![Unit::Sound(S)], 1),
            ('t', Some('s')) if after == Some('c') && letter(at + 3) == Some('h') => (vec![Unit::Sound(CH)], 4),
            ('d', Some('s')) if after == Some('c') && letter(at + 3) == Some('h') => (vec![Unit::Sound(DZH)], 4),
            ('t', Some('z')) => (vec![Unit::Sound(TS)], 2),
            ('t', Some('h')) | ('d', Some('t')) => (vec![Unit::Sound(T)], 2),
            // -tion, -tient: ts (Nation, Patient).
            ('t', Some('i')) if matches!(after, Some('o' | 'e' | 'a')) && at > 0 && letter(at + 3) == Some('n') => {
                (vec![Unit::Sound(TS), Unit::Sound(J)], 2)
            }
            ('c', Some('h')) if after == Some('s') && letter(at + 3) != Some('t') => {
                (vec![Unit::Sound(K), Unit::Sound(S)], 3)
            }
            ('c', Some('h')) if at == 0 => (vec![Unit::Sound(K)], 2),
            ('c', Some('h')) => (vec![Unit::Sound(if back { ACH } else { ICH })], 2),
            ('c', Some('k')) => (vec![Unit::Sound(K)], 2),
            ('c', _) if matches!(next, Some('e' | 'i' | 'ä' | 'y')) => (vec![Unit::Sound(TS)], 1),
            ('q', Some('u')) => (vec![Unit::Sound(K), Unit::Sound(V)], 2),
            ('c' | 'k' | 'q', _) => (vec![Unit::Sound(K)], 1),
            ('p', Some('h')) => (vec![Unit::Sound(F)], 2),
            ('p', Some('f')) => (vec![Unit::Sound(PF)], 2),
            ('n', Some('g')) if !is_vowel(after) || at > 0 => (vec![Unit::Sound(NG)], 2),
            ('n', Some('k')) => (vec![Unit::Sound(NG)], 1),
            // -ig at the end of a word or before a consonant is ɪç (König, ruhig, beschäftigt).
            ('g', _)
                if before(at) == Some('i')
                    && !matches!(at.checked_sub(2).and_then(letter), Some('e' | 'a'))
                    && !keeps_voice('g', next, before(at)) =>
            {
                (vec![Unit::Sound(ICH)], 1)
            }
            ('b', _) if !keeps_voice('b', next, before(at)) && next != Some('b') => (vec![Unit::Sound(P)], 1),
            ('d', _) if !keeps_voice('d', next, before(at)) && next != Some('d') => (vec![Unit::Sound(T)], 1),
            ('g', _) if !keeps_voice('g', next, before(at)) && next != Some('g') => (vec![Unit::Sound(K)], 1),
            ('v', _) if !is_vowel(next) => (vec![Unit::Sound(F)], 1),
            ('v', _) if at == 0 => (vec![Unit::Sound(F)], 1),
            ('r', _) => (
                vec![Unit::R { onset: is_vowel(next) }],
                if next == Some('r') { 2 } else { 1 },
            ),
            ('b', _) => (vec![Unit::Sound(B)], 1),
            ('d', _) => (vec![Unit::Sound(D)], 1),
            ('f', _) => (vec![Unit::Sound(F)], 1),
            ('g', _) => (vec![Unit::Sound(G)], 1),
            ('h', _) => (vec![Unit::Sound(H)], 1),
            ('j', _) => (vec![Unit::Sound(J)], 1),
            ('l', _) => (vec![Unit::Sound(L)], 1),
            ('m', _) => (vec![Unit::Sound(M)], 1),
            ('n', _) => (vec![Unit::Sound(N)], 1),
            ('p', _) => (vec![Unit::Sound(P)], 1),
            ('t', _) => (vec![Unit::Sound(T)], 1),
            ('v' | 'w', _) => (vec![Unit::Sound(V)], 1),
            ('x', _) => (vec![Unit::Sound(K), Unit::Sound(S)], 1),
            ('z', _) => (vec![Unit::Sound(TS)], 1),
            ('\'' | '’' | '-', _) => (Vec::new(), 1),
            _ => return None,
        };
        // A doubled consonant is said once (Mutter, kommen).
        let doubled = step == 1 && next == Some(c) && !is_vowel(Some(c)) && c != 's';
        units.extend(unit);
        at += if doubled { 2 } else { step };
    }
    Some(units)
}

fn kind_of(c: char) -> Kind {
    match c {
        'a' => Kind::A,
        'e' => Kind::E,
        'i' => Kind::I,
        'o' => Kind::O,
        'u' => Kind::U,
        'ä' => Kind::Ae,
        'ö' => Kind::Oe,
        _ => Kind::Ue,
    }
}

/// The unstressed prefixes of verbs and their nouns (beginnen, Gebäude, verstehen).
const UNSTRESSED_PREFIXES: [&str; 7] = ["be", "ge", "er", "ver", "zer", "ent", "emp"];

/// The stressed prefixes of separable verbs and their nouns, after which a stem starts afresh
/// (aufstehen ˈaʊfʃteːən, Absicht ˈapzɪçt).
const STRESSED_PREFIXES: [&str; 15] = [
    "ab", "an", "auf", "aus", "bei", "ein", "mit", "nach", "vor", "weg", "um", "durch", "fort", "los", "zurück",
];

/// The stressed prefixes with a long vowel.
const LONG_PREFIXES: [&str; 3] = ["vor", "nach", "los"];

/// Words that start like a prefix but are not one (geben, Erde, Abend).
const NOT_PREFIXED: [&str; 21] = [
    "geben", "gebe", "gehen", "gegen", "gegend", "gestern", "erde", "ernte", "erbe", "besen", "beten", "becher",
    "abend", "abends", "ander", "andere", "anderen", "anders", "anna", "mittel", "einander",
];

/// Whether `stem` starts as a German syllable can: with a vowel, or with consonants that can begin
/// a word (bei-nchen cannot).
fn starts_syllable(stem: &str) -> bool {
    let letters: Vec<char> = stem.chars().take(2).collect();
    match letters[..] {
        [first, ..] if is_vowel(Some(first)) => true,
        [_, second] if is_vowel(Some(second)) => true,
        [first, second] => {
            let pair: String = [first, second].iter().collect();
            ONSETS.contains(&pair.as_str())
        }
        _ => false,
    }
}

/// The pairs of consonants a German word can start with.
const ONSETS: [&str; 23] = [
    "bl", "br", "ch", "dr", "fl", "fr", "gl", "gr", "kl", "kn", "kr", "pf", "ph", "pl", "pr", "qu", "sc", "sp", "st",
    "th", "tr", "wr", "zw",
];

/// The prefixes `word` starts with, and where its stem starts: the unstressed prefix last
/// (Vorbereitung: vor, be).
fn prefixes(word: &str) -> Vec<&'static str> {
    let mut found = Vec::new();
    let mut rest = word;
    if NOT_PREFIXED.iter().any(|stem| word.starts_with(stem)) {
        return found;
    }
    // A stem left must have a vowel, two letters at least, and not start with one that would make
    // a diphthong with the prefix (Bein, Geist) or a doubled consonant (Bett).
    let stem_left = |stem: &str, prefix: &str| {
        let mut letters = stem.chars();
        let first = letters.next();
        let second = letters.next();
        stem.chars().count() >= 2
            && stem.chars().any(|c| is_vowel(Some(c)))
            && starts_syllable(stem)
            && !(prefix.ends_with('e') && matches!(first, Some('i' | 'u' | 'y')))
            && !(first.is_some() && first == second && !is_vowel(first))
    };
    for _ in 0..2 {
        let prefix = STRESSED_PREFIXES
            .iter()
            .chain(&UNSTRESSED_PREFIXES)
            .filter(|prefix| rest.starts_with(*prefix) && stem_left(&rest[prefix.len()..], prefix))
            .max_by_key(|prefix| prefix.len());
        let Some(prefix) = prefix else {
            break;
        };
        let unstressed = UNSTRESSED_PREFIXES.contains(prefix);
        // A stressed prefix needs a stem of three letters at least (an-na, ab-er).
        if !unstressed && rest[prefix.len()..].chars().count() < 3 {
            break;
        }
        found.push(*prefix);
        rest = &rest[prefix.len()..];
        if unstressed {
            break;
        }
    }
    found
}

/// The endings of loanwords that take the stress, and which vowel from the end is stressed: 0 for
/// the last (Nation, Universität, Bäckerei), 1 for the one before (studieren, Maschine).
const STRESSED_ENDINGS: [(&str, usize); 30] = [
    ("ion", 0),
    ("ionen", 1),
    ("ität", 0),
    ("itäten", 1),
    ("ei", 0),
    ("eien", 1),
    ("ieren", 1),
    ("iert", 0),
    ("ierte", 1),
    ("ierten", 1),
    ("ierst", 0),
    ("ismus", 1),
    ("ist", 0),
    ("isten", 1),
    ("istin", 1),
    ("ent", 0),
    ("enten", 1),
    ("ant", 0),
    ("anten", 1),
    ("enz", 0),
    ("anz", 0),
    ("ös", 0),
    ("är", 0),
    ("eur", 0),
    ("ell", 0),
    ("ine", 1),
    ("ade", 1),
    ("ur", 0),
    ("ie", 0),
    ("ien", 0),
];

/// The adverbs made of da-, wo-, hier-, her-, hin- or vor- and a preposition, stressed on the
/// preposition (dabei, darauf, herein, vorbei).
const ADVERB_HEADS: [&str; 9] = ["dar", "da", "wor", "wo", "hier", "her", "hin", "vor", "zu"];
/// The prepositions with a short vowel, also at the end of an adverb.
const SHORT_TAILS: [&str; 7] = ["an", "in", "um", "mit", "ab", "ob", "von"];
const ADVERB_TAILS: [&str; 22] = [
    "an", "auf", "aus", "bei", "durch", "für", "gegen", "hinter", "in", "ein", "mit", "nach", "neben", "über", "um",
    "unter", "von", "vor", "zu", "zwischen", "rück", "sammen",
];

/// Where `word`'s letters (in lower case) start its stem when it is an adverb stressed on its
/// second part (dabei: 2).
fn adverb(word: &str) -> Option<usize> {
    ADVERB_HEADS.iter().find_map(|head| {
        let tail = word.strip_prefix(head)?;
        ADVERB_TAILS
            .iter()
            .any(|ending| tail == *ending || (tail.starts_with(ending) && tail.len() <= ending.len() + 1))
            .then_some(head.chars().count())
    })
}

/// The vowel units' indices (among `units`) whose letters start at or after `at`.
fn vowels_from(units: &[Unit], at: usize) -> Vec<usize> {
    (0..units.len())
        .filter(|&index| matches!(units[index], Unit::Vowel { at: start, .. } if start >= at))
        .collect()
}

/// Which vowel unit is stressed: an acute accent's (from jespersen's list), a loanword ending's, the
/// second part's of an adverb, else the first of the stem after the prefixes.
fn stressed(units: &[Unit], word: &str, accent: Option<usize>, stem: usize) -> Option<usize> {
    let vowels = vowels_from(units, 0);
    if let Some(accent) = accent {
        return vowels
            .iter()
            .copied()
            .find(|&index| matches!(units[index], Unit::Vowel { at, end, .. } if at <= accent && accent < end));
    }
    // -ie after l is i and ə, stressed on the i (Familie famˈiːliə).
    if (word.ends_with("lie") || word.ends_with("lien")) && vowels.len() >= 3 {
        return Some(vowels[vowels.len() - 3]);
    }
    if let Some(&(_, back)) = STRESSED_ENDINGS.iter().find(|(ending, _)| word.ends_with(ending))
        && vowels.len() > 1 + back
    {
        return Some(vowels[vowels.len() - 1 - back]);
    }
    let start = adverb(word).unwrap_or(stem);
    vowels_from(units, start)
        .first()
        .copied()
        .or_else(|| vowels.first().copied())
}

/// Endings after which a stressed vowel before one final consonant is short: the function words
/// are listed in the sentence reader; these are the endings of words (Erlebnis, Ärztin, Museum).
const SHORT_FINAL: [&str; 6] = ["nis", "in", "um", "us", "is", "ig"];

/// Whether the vowel unit at `index` is long: as its spelling says (ie, ah, aa), else long before
/// one consonant and a vowel or at the end and short before two consonants, ch, sch, ng or x,
/// but long before the b or g of an inflected stem (lebt, sagte, Obst). After the stress i, u and
/// ü are short, as in endings (ruhigen, König, Achtung), unless spelt long.
fn long(units: &[Unit], index: usize, word: &[char], after_stress: bool) -> bool {
    let Unit::Vowel { kind, end, long, .. } = units[index] else {
        return false;
    };
    if let Some(long) = long {
        return long;
    }
    if matches!(kind, Kind::Ai | Kind::Au | Kind::Eu) {
        return true;
    }
    let rest = &word[end..];
    let mut cluster: Vec<char> = rest.iter().copied().take_while(|&c| !is_vowel(Some(c))).collect();
    // ph and th are one consonant (Philosophie); t before i and a vowel is ts (Nation).
    if matches!(cluster[..], ['p' | 't', 'h', ..]) {
        cluster.remove(1);
    }
    if cluster == ['t'] && rest.get(1) == Some(&'i') && is_vowel(rest.get(2).copied()) {
        return false;
    }
    let at_end = cluster.len() == rest.len();
    if cluster.first() == Some(&'ß') {
        return true;
    }
    if after_stress && matches!(kind, Kind::I | Kind::U | Kind::Ue) {
        return false;
    }
    match cluster[..] {
        ['c' | 'x' | 'q', ..] => false,
        [c] if at_end => {
            let text: String = word.iter().collect();
            !SHORT_FINAL.iter().any(|ending| text.ends_with(ending)) && c != 'ß'
        }
        [c, second, ..] if c == second => false,
        // A stop before l or r and a vowel leaves the vowel long (übrig, Adler).
        ['b' | 'd' | 'g' | 'p' | 't' | 'k', 'r' | 'l'] if !at_end => true,
        [] | [_] | ['b' | 'g', 't' | 's'] | ['b' | 'g', 's', 't'] | ['b' | 'g', 't', 'e' | 'n'] => true,
        _ => false,
    }
}

/// The phoneme of a vowel unit.
fn vowel_phoneme(kind: Kind, long: bool, schwa: bool) -> Phoneme {
    match (kind, long) {
        (Kind::E, _) if schwa => SCHWA,
        (Kind::A, true) => A_LONG,
        (Kind::A, false) => A,
        (Kind::E, true) => E_LONG,
        (Kind::E | Kind::Ae, false) => E,
        (Kind::Ae, true) => AE_LONG,
        (Kind::I, true) => I_LONG,
        (Kind::I, false) => I,
        (Kind::O, true) => O_LONG,
        (Kind::O, false) => O,
        (Kind::U, true) => U_LONG,
        (Kind::U, false) => U,
        (Kind::Oe, true) => OE_LONG,
        (Kind::Oe, false) => OE,
        (Kind::Ue, true) => UE_LONG,
        (Kind::Ue, false) => UE,
        (Kind::Ai, _) => AI,
        (Kind::Au, _) => AU,
        (Kind::Eu, _) => EU,
    }
}

/// `c` without an acute accent, which jespersen's list writes on a stressed vowel (gében), and
/// whether it had one.
fn plain(c: char) -> (char, bool) {
    match c {
        'á' => ('a', true),
        'é' => ('e', true),
        'í' => ('i', true),
        'ó' => ('o', true),
        'ú' => ('u', true),
        _ => (c, false),
    }
}

/// How `word` is said; `None` for a letter German does not read.
#[expect(clippy::too_many_lines, reason = "the steps of a word's reading, in order")]
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let (chars, accents): (Vec<char>, Vec<bool>) = word.to_lowercase().chars().map(plain).unzip();
    let lower: String = chars.iter().collect();
    let accent = accents.iter().position(|&accent| accent);
    let prefixes = if accent.is_some() { Vec::new() } else { prefixes(&lower) };
    // The units of each prefix and of the stem, a consonant doubled across them said once.
    let mut units: Vec<Unit> = Vec::new();
    let mut start = 0;
    let mut parts: Vec<(usize, usize)> = Vec::new();
    for prefix in &prefixes {
        parts.push((start, start + prefix.chars().count()));
        start += prefix.chars().count();
    }
    // An adverb's two parts are said apart (davon dɑːfˈɔn).
    let adverb_head = (prefixes.is_empty() && accent.is_none())
        .then(|| adverb(&lower))
        .flatten();
    if let Some(head) = adverb_head {
        // The r of dar- and wor- starts the second part (daran dɑːrˈan).
        let split = if lower[..head].ends_with('r') { head - 1 } else { head };
        parts.push((0, split));
        start = split;
    }
    parts.push((start, chars.len()));
    for &(from, to) in &parts {
        let part = self::units(&chars[from..to], from)?;
        if let (Some(last), Some(first)) = (units.last(), part.first())
            && last == first
            && !matches!(first, Unit::Vowel { .. })
        {
            units.pop();
        }
        units.extend(part);
    }
    // The stem starts after the unstressed prefixes; a stressed one keeps the stress (Absicht).
    let stem = prefixes
        .iter()
        .zip(&parts)
        .filter(|(prefix, _)| UNSTRESSED_PREFIXES.contains(prefix))
        .map(|(_, &(_, to))| to)
        .next_back()
        .unwrap_or(0);
    let stressed = stressed(&units, &lower, accent, stem);
    // After a stressed prefix the stem keeps a secondary stress (Vorzimmer fˈoːɾtsˌɪmɜ, einmal).
    let stem_start = parts.last().map_or(0, |&(from, _)| from);
    let secondary = (stem_start > 0
        && stressed.is_some_and(|index| matches!(units[index], Unit::Vowel { at, .. } if at < stem_start)))
    .then(|| vowels_from(&units, stem_start).first().copied())
    .flatten();
    // The short prepositions ending an adverb keep their short vowel (daran dɑːrˈan).
    let short_tail = adverb(&lower).is_some_and(|head| SHORT_TAILS.contains(&&lower[head..]));
    let unstressed_prefix_end = prefixes
        .iter()
        .position(|prefix| UNSTRESSED_PREFIXES.contains(prefix))
        .map(|index| parts[index]);
    let mut segments: Vec<Segment> = Vec::with_capacity(units.len());
    for (index, unit) in units.iter().enumerate() {
        let stress = if Some(index) == stressed {
            Stress::Primary
        } else if Some(index) == secondary {
            Stress::Secondary
        } else {
            Stress::Unstressed
        };
        let phoneme = match *unit {
            Unit::Vowel {
                kind, at, long: spelt, ..
            } => {
                let after_stress = stressed.is_some_and(|stressed| index > stressed)
                    && secondary.is_none_or(|secondary| index != secondary);
                let in_prefix = unstressed_prefix_end.filter(|&(from, to)| from <= at && at < to);
                // be- and ge- have ə, er-, ver-, zer-, ent- and emp- a short ɛ.
                let schwa = kind == Kind::E
                    && spelt.is_none()
                    && stress == Stress::Unstressed
                    && (after_stress || in_prefix.is_some_and(|(from, to)| to - from == 2 && chars[from] != 'e'));
                // A stressed prefix has its own length (vor fˈoːɾ, nach, los; ab, an, mit short).
                let stressed_prefix = prefixes
                    .iter()
                    .zip(&parts)
                    .find(|(prefix, part)| !UNSTRESSED_PREFIXES.contains(prefix) && part.0 <= at && at < part.1)
                    .map(|(prefix, _)| *prefix);
                let long = if let Some(prefix) = stressed_prefix {
                    LONG_PREFIXES.contains(&prefix)
                } else {
                    in_prefix.is_none()
                        && !(short_tail && Some(index) == stressed)
                        // An unstressed a before the stress is short (Nation, Natur).
                        && !(kind == Kind::A
                            && spelt.is_none()
                            && stress == Stress::Unstressed
                            && !after_stress
                            && adverb_head.is_none_or(|head| at >= head))
                        && long(&units, index, &chars, after_stress)
                };
                vowel_phoneme(kind, long, schwa)
            }
            Unit::Sound(phoneme) => phoneme,
            Unit::R { onset: true } => match index.checked_sub(1).map(|before| units[before]) {
                Some(Unit::Vowel { .. }) | None => R,
                Some(Unit::Sound(sound)) if [F, V, SH, ACH, ICH, S, Z].contains(&sound) => R,
                _ => R_TAP,
            },
            Unit::R { onset: false } => match segments.last().map(|segment| segment.phoneme) {
                Some(last) if last == SCHWA => {
                    if let Some(segment) = segments.last_mut() {
                        segment.phoneme = ER;
                    }
                    continue;
                }
                Some(last) if last == U => R_AFTER_U,
                _ => R_VOCALIC,
            },
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
/// stress (ˌuːniːvˌɛɾziːtˈɛːt).
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
}

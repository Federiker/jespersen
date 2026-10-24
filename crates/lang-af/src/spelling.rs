//! How an Afrikaans word is said from its spelling: a vowel long in an open syllable and when
//! doubled (vader, skool), short in a closed one (dag); ee as iə and oo as uə, a short i as ə; oe,
//! eu, ui, ei and y, ou; ê, ô and û long; g as x, v as f, w as v but after t, k, s and d; the
//! consonants unvoiced at the end of a word; an unstressed e as ə; the stress on the first
//! syllable of the stem, after an unstressed prefix (be-, ge-, ver-), and on the endings of
//! loanwords (-eer, -teit, -ie).

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, A_LONG, A_NASAL, AAI, AI, B, D, E, E_LONG, E_LONG_BEFORE_R, E_NASAL, E_OPEN, EE, EI, EU, F, G, H, IE, IE_LONG,
    IJ, J, K, L, M, N, NG, O, O_LONG, O_NASAL, OE, OE_LONG, OEI, OO, OOI, OU, P, R, S, SCHWA, SH, T, U, U_LONG, UI, UU,
    V, W, X, Z, legal_onset,
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
    Ou,
    Ai,
    /// ij, in the older spelling; ooi; the o of oë.
    Ij,
    Ooi,
    OeLong,
    Aai,
    Oei,
    /// ê, ô and û.
    LongE,
    LongO,
    LongU,
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
        Some('a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'ê' | 'ô' | 'û' | 'ë' | 'ï' | 'ö' | 'ü' | 'î')
    )
}

/// Whether a voiced consonant before `next` is unvoiced: at the end of a word and before a
/// voiceless consonant (hond hˈɔnt).
fn unvoiced_before(next: Option<char>) -> bool {
    next.is_none() || matches!(next, Some('t' | 's' | 'k' | 'p' | 'f'))
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
        let vowel = |kind: Kind, length: usize| Unit::Vowel {
            kind,
            at: offset + at,
            end: offset + at + length,
        };
        let rest: String = word[at..].iter().collect();
        let (unit, step): (Vec<Unit>, usize) = match c {
            _ if rest.starts_with("aai") => (vec![vowel(Kind::Aai, 3)], 3),
            _ if rest.starts_with("ooi") => (vec![vowel(Kind::Ooi, 3)], 3),
            // oë is u (voël).
            'o' if next == Some('ë') => (vec![vowel(Kind::OeLong, 1)], 1),
            _ if rest.starts_with("oei") => (vec![vowel(Kind::Oei, 3)], 3),
            // eë is one vowel, and a final ae is a (gereën xərˈiən, dae dˈɑː).
            'e' if next == Some('ë') => (vec![vowel(Kind::Ee, 2)], 2),
            'a' if next == Some('e') && at + 2 == word.len() => (vec![vowel(Kind::Aa, 2)], 2),
            'a' if next == Some('a') => (vec![vowel(Kind::Aa, 2)], 2),
            'a' if next == Some('i') => (vec![vowel(Kind::Ai, 2)], 2),
            'e' if next == Some('e') => (vec![vowel(Kind::Ee, 2)], 2),
            'o' if next == Some('o') => (vec![vowel(Kind::Oo, 2)], 2),
            'u' if next == Some('u') => (vec![vowel(Kind::Uu, 2)], 2),
            'i' if next == Some('e') => (vec![vowel(Kind::Ie, 2)], 2),
            'o' if next == Some('e') => (vec![vowel(Kind::Oe, 2)], 2),
            'e' if next == Some('u') => (vec![vowel(Kind::Eu, 2)], 2),
            'u' if next == Some('i') => (vec![vowel(Kind::Ui, 2)], 2),
            'e' if next == Some('i') => (vec![vowel(Kind::Ei, 2)], 2),
            'o' | 'a' if next == Some('u') => (vec![vowel(Kind::Ou, 2)], 2),
            // ij, in the older spelling, is ɛɪ (kijk), but əj in eSpeak NG's notation at the end of a
            // word (hij hˈəj, nabij).
            'i' if next == Some('j') => {
                let last = at + 2 == word.len();
                (vec![vowel(if last { Kind::Ij } else { Kind::Ei }, 2)], 2)
            }
            'y' => (vec![vowel(Kind::Ei, 1)], 1),
            'a' => (vec![vowel(Kind::A, 1)], 1),
            'e' | 'ë' => (vec![vowel(Kind::E, 1)], 1),
            'i' | 'ï' | 'î' => (vec![vowel(Kind::I, 1)], 1),
            'o' | 'ö' => (vec![vowel(Kind::O, 1)], 1),
            'u' | 'ü' => (vec![vowel(Kind::U, 1)], 1),
            'ê' => (vec![vowel(Kind::LongE, 1)], 1),
            'ô' => (vec![vowel(Kind::LongO, 1)], 1),
            'û' => (vec![vowel(Kind::LongU, 1)], 1),
            's' if next == Some('j') => (vec![Unit::Sound(SH)], 2),
            'c' if next == Some('h') => (vec![Unit::Sound(X)], 2),
            'c' if matches!(next, Some('e' | 'i' | 'y')) => (vec![Unit::Sound(S)], 1),
            'q' if next == Some('u') => (vec![Unit::Sound(K), Unit::Sound(W)], 2),
            'c' | 'k' | 'q' => (vec![Unit::Sound(K)], 1),
            'p' if next == Some('h') => (vec![Unit::Sound(F)], 2),
            't' if next == Some('h') => (vec![Unit::Sound(T)], 2),
            'n' if next == Some('g') => (vec![Unit::Sound(NG)], 2),
            'n' if next == Some('k') => (vec![Unit::Sound(NG)], 1),
            // g after r or l and before a vowel is ɡ (berge bˈærɡə).
            'g' if at > 0 && matches!(letter(at - 1), Some('r' | 'l')) && is_vowel(next) => (vec![Unit::Sound(G)], 1),
            'g' => (vec![Unit::Sound(X)], 1),
            // d is unsaid in -onder (honderd hˈɔnərt, onder; but ander, kinders).
            'd' if rest.starts_with("der") && at >= 2 && word[at - 2..at] == ['o', 'n'] => (Vec::new(), 1),
            'd' if unvoiced_before(next) => (vec![Unit::Sound(T)], 1),
            'b' if unvoiced_before(next) => (vec![Unit::Sound(P)], 1),
            'b' => (vec![Unit::Sound(B)], 1),
            'd' => (vec![Unit::Sound(D)], 1),
            // v is f at the start of a part, v elsewhere (vader fˈɑːdər, universiteit).
            'v' if at > 0 => (vec![Unit::Sound(V)], 1),
            'f' | 'v' => (vec![Unit::Sound(F)], 1),
            'h' => (vec![Unit::Sound(H)], 1),
            'j' => (vec![Unit::Sound(J)], 1),
            // A final rm and lm take a ə (warm vˈɐrəm, film).
            'r' | 'l' if word[at + 1..] == ['m'] => {
                (vec![Unit::Sound(if c == 'r' { R } else { L }), Unit::Sound(SCHWA)], 1)
            }
            'l' => (vec![Unit::Sound(L)], 1),
            'm' => (vec![Unit::Sound(M)], 1),
            'n' => (vec![Unit::Sound(N)], 1),
            'p' => (vec![Unit::Sound(P)], 1),
            'r' => (vec![Unit::Sound(R)], 1),
            's' => (vec![Unit::Sound(S)], 1),
            't' => (vec![Unit::Sound(T)], 1),
            // w is v, but w after a t, k, s or d starting a word (water vˈɑːtər, twee; antwoord v).
            'w' if at == 1 && matches!(letter(0), Some('t' | 'k' | 's' | 'd')) => (vec![Unit::Sound(W)], 1),
            'w' => (vec![Unit::Sound(V)], 1),
            'x' => (vec![Unit::Sound(K), Unit::Sound(S)], 1),
            'z' => (vec![Unit::Sound(Z)], 1),
            '\'' | '’' | '-' => (Vec::new(), 1),
            _ => return None,
        };
        // A doubled consonant is said once (sakke, eggo).
        let doubled = step == 1 && next == Some(c) && !is_vowel(Some(c));
        units.extend(unit);
        at += if doubled { 2 } else { step };
    }
    Some(units)
}

/// The unstressed prefixes (begin, gesig, verstaan, ontmoet).
const PREFIXES: [&str; 5] = ["be", "ge", "ver", "ont", "her"];

/// Words that start like a prefix but are not one (gee, beter, herfs).
const NOT_PREFIXED: [&str; 12] = [
    "gee", "geel", "geld", "gesel", "beter", "beste", "bed", "beeld", "bees", "herfs", "here", "verf",
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
        stem.chars().count() >= 2
            && stem.chars().any(|c| is_vowel(Some(c)))
            // Not a vowel the prefix's e would join (beide, geur), nor a doubled consonant.
            && !(prefix.ends_with('e') && matches!(first, Some('i' | 'u' | 'e')))
            && !(first.is_some() && first == second && !is_vowel(first))
            // Nor two consonants no syllable starts with (berge).
            && (is_vowel(first) || is_vowel(second) || first == Some('s') || matches!(second, Some('l' | 'r' | 'w')))
    })
}

/// The endings of loanwords that take the stress, and which vowel from the end is stressed: 0
/// for the last (universiteit, studeer, filosofie), 1 for the one before (nasie).
const STRESSED_ENDINGS: [(&str, usize); 23] = [
    ("eer", 0),
    ("eerde", 1),
    ("teit", 0),
    ("fie", 0),
    ("gie", 0),
    ("mie", 0),
    ("sie", 1),
    ("aans", 0),
    ("ade", 1),
    ("oor", 0),
    ("teek", 0),
    ("ant", 0),
    ("ent", 0),
    ("ief", 0),
    ("aal", 0),
    ("eel", 0),
    ("iek", 0),
    ("oen", 0),
    ("eur", 0),
    ("ier", 0),
    ("ist", 0),
    ("isme", 1),
    ("uur", 0),
];

/// Which vowel unit is stressed: the one an acute accent is on (jespersen's list), a loanword
/// ending's, else the first of the stem after the prefix.
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
    // An ending's first letter must not close a diphthong before it (baie is not -ie).
    let fits = |ending: &str| {
        word.ends_with(ending)
            && !(ending.starts_with(['i', 'e']) && word[..word.len() - ending.len()].ends_with(['a', 'o', 'e', 'u']))
    };
    if let Some(&(_, back)) = STRESSED_ENDINGS
        .iter()
        .filter(|(ending, _)| fits(ending))
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
/// the end of the word (vader, na); ng, nk and sj close it.
fn open(word: &[char], end: usize) -> bool {
    let rest = &word[end..];
    let cluster: Vec<char> = rest.iter().copied().take_while(|&c| !is_vowel(Some(c))).collect();
    match cluster[..] {
        ['x' | 'q', ..] => false,
        [] => true,
        // th is one consonant (Botha).
        ['t', 'h'] | [_] => cluster.len() < rest.len(),
        _ => false,
    }
}

/// The phoneme of a vowel unit: `open` for one in an open syllable, `schwa` for a reduced e, and
/// `next` and `after` the letters after it: a short e is æ before k, l, r and g (ek, elf, kerk,
/// weg); ie and oe are long before r and a vowel or the end (vier, diere, snoer), a closed e before
/// a final r (ver fˈæːr).
fn vowel_phoneme(kind: Kind, open: bool, schwa: bool, next: Option<char>, after: Option<char>) -> Phoneme {
    let final_r = next == Some('r') && after.is_none();
    let before_r = next == Some('r') && (after.is_none() || is_vowel(after));
    match kind {
        _ if schwa => SCHWA,
        Kind::A if open => A_LONG,
        Kind::A => A,
        Kind::E if open => EE,
        Kind::E if final_r => E_LONG_BEFORE_R,
        Kind::E if matches!(next, Some('k' | 'l' | 'r' | 'g')) => E_OPEN,
        Kind::E => E,
        Kind::I if open => IE,
        Kind::I => SCHWA,
        Kind::O if open => OO,
        Kind::O => O,
        Kind::U if open => UU,
        Kind::U => U,
        Kind::Aa => A_LONG,
        Kind::Ee => EE,
        Kind::Oo => OO,
        Kind::Uu => UU,
        Kind::Ie | Kind::Oe if before_r => {
            if kind == Kind::Ie {
                IE_LONG
            } else {
                OE_LONG
            }
        }
        Kind::Ie => IE,
        Kind::Oe => OE,
        Kind::Eu => EU,
        Kind::Ui => UI,
        Kind::Ei => EI,
        Kind::Ij => IJ,
        Kind::Ou => OU,
        Kind::Ooi => OOI,
        Kind::Aai => AAI,
        Kind::Oei => OEI,
        Kind::OeLong => OE_LONG,
        Kind::Ai => AI,
        Kind::LongE if next == Some('r') => E_LONG_BEFORE_R,
        Kind::LongE => E_LONG,
        Kind::LongO => O_LONG,
        Kind::LongU => U_LONG,
    }
}

/// `c` without an acute accent, which marks the stress in jespersen's list, and whether it had one.
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

/// How `word` is said; `None` for a letter Afrikaans does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let (chars, accents): (Vec<char>, Vec<bool>) = word.to_lowercase().chars().map(plain).unzip();
    let lower: String = chars.iter().collect();
    let accent = accents.iter().position(|&accent| accent);
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
            Unit::Vowel { kind, at, end } => {
                let after_stress = stressed.is_some_and(|stressed| index > stressed);
                // An unstressed e is ə, but at the start of the word (persent pərsˈɛnt, eksamen).
                let schwa = stress == Stress::Unstressed && kind == Kind::E && at > 0;
                // In a part of its own a prefix's vowel is closed by the part's end.
                let open = at >= stem && open(&chars, end);
                // A stressed a, e or o before a final ns or nse is nasal (ons õns, mense mˈẽnsə, kans
                // kˈɑ̃ns; but konferensie).
                let nasal = stress == Stress::Primary
                    && chars.get(end) == Some(&'n')
                    && chars.get(end + 1) == Some(&'s')
                    && matches!(chars.get(end + 2), None | Some('e'));
                // A final unstressed o is u (euro ˈøəru), one in a closed syllable ə (motor mˈʊətər).
                if nasal && matches!(kind, Kind::A | Kind::Aa) {
                    A_NASAL
                } else if nasal && kind == Kind::E && !schwa {
                    E_NASAL
                } else if nasal && kind == Kind::O {
                    O_NASAL
                } else if kind == Kind::O && after_stress && end == chars.len() {
                    OE
                } else if kind == Kind::O
                    && after_stress
                    && !open
                    && chars.get(end) == Some(&'r')
                    && end + 1 == chars.len()
                {
                    SCHWA
                } else if kind == Kind::A && after_stress && end == chars.len() {
                    // A final unstressed a is short (auto ˈəʊtɐ).
                    A
                } else {
                    vowel_phoneme(kind, open, schwa, chars.get(end).copied(), chars.get(end + 1).copied())
                }
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

/// eSpeak NG's secondary stress after the stress: a full vowel two syllables on (opgewonde
/// ˈɔpxəvˌɔndə).
fn rhythm(said: &mut Pronunciation) {
    let Some(stressed) = said
        .syllables
        .iter()
        .position(|syllable| syllable.stress == Stress::Primary)
    else {
        return;
    };
    for syllable in said.syllables.iter_mut().skip(stressed + 2).step_by(2) {
        if syllable.nucleus.iter().all(|vowel| *vowel != SCHWA) {
            syllable.stress = Stress::Secondary;
        }
    }
}

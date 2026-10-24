//! How a Swedish word is said from its spelling: a stressed vowel long before one consonant or at
//! the end and short before two (hus, hundra); o as uː or ɔ, å as oː or ɔ, u as ʉː or ɵ; the sj
//! sound in sj, skj, stj and sk before a front vowel, the tj sound in tj, kj and k before one, j
//! for g, dj, gj, hj and lj at the start; rs and rt as ʂ and ʈ, ng as ŋ; an unstressed e in a last
//! syllable as ə; the stress on the first syllable, as eSpeak NG reads it, but on -tion.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, A_LONG, AA, AE_LONG, B, D, E, E_LONG, F, G, H, I, I_LONG, J, K, L, LONG, M, N, NG, O, OE, OE_LONG, OO, P, R, RS,
    RT, S, SCHWA, SJ, T, TJ, U, U_LONG, V, Y, Y_LONG, legal_onset,
};

fn is_vowel(c: Option<char>) -> bool {
    matches!(c, Some('a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'å' | 'ä' | 'ö'))
}

/// `c` without an acute accent, which marks the stress (idé, jespersen's list: miljón), and whether
/// it had one.
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

/// The front vowels, before which k, g and sk soften at the start of a word (kök, göra, skön).
fn is_front(c: Option<char>) -> bool {
    matches!(c, Some('e' | 'i' | 'y' | 'ä' | 'ö'))
}

/// What a letter, or a group of them, comes to before the stress is known.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Unit {
    /// A vowel letter, and where it is in the word.
    Vowel {
        letter: char,
        at: usize,
    },
    Sound(Phoneme),
}

/// The units of `word`'s letters, in lower case; `None` for a letter Swedish does not read.
fn units(word: &[char]) -> Option<Vec<Unit>> {
    let letter = |at: usize| word.get(at).copied();
    let rest_is = |at: usize, text: &str| word[at..].iter().copied().take(text.chars().count()).eq(text.chars());
    let mut units: Vec<Unit> = Vec::with_capacity(word.len());
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        let next = letter(at + 1);
        let after = letter(at + 2);
        let start = at == 0;
        let (unit, step): (Vec<Unit>, usize) = match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'å' | 'ä' | 'ö' => (vec![Unit::Vowel { letter: c, at }], 1),
            // -tion and -sion: the sj sound and uːn (nation nasxˈuːn).
            't' | 's' if rest_is(at + 1, "ion") && at > 0 => (vec![Unit::Sound(SJ)], 2),
            's' if rest_is(at, "skj") || rest_is(at, "stj") || rest_is(at, "sch") => (vec![Unit::Sound(SJ)], 3),
            's' if next == Some('j') => (vec![Unit::Sound(SJ)], 2),
            's' if start && next == Some('k') && is_front(after) => (vec![Unit::Sound(SJ)], 2),
            'c' if next == Some('h') => (vec![Unit::Sound(SJ)], 2),
            't' | 'k' if next == Some('j') => (vec![Unit::Sound(TJ)], 2),
            'k' if start && is_front(next) => (vec![Unit::Sound(TJ)], 1),
            'g' if start && is_front(next) => (vec![Unit::Sound(J)], 1),
            // g after r or l, at the end or before e or i, is j (berg, torget, Sverige).
            'g' if at > 0
                && matches!(letter(at - 1), Some('r' | 'l'))
                && (next.is_none() || matches!(next, Some('e' | 'i'))) =>
            {
                (vec![Unit::Sound(J)], 1)
            }
            'd' | 'g' | 'h' | 'l' if start && next == Some('j') => (vec![Unit::Sound(J)], 2),
            'n' if next == Some('g') => (vec![Unit::Sound(NG)], 2),
            'n' if next == Some('k') => (vec![Unit::Sound(NG)], 1),
            'g' if next == Some('n') && at > 0 => (vec![Unit::Sound(NG)], 1),
            'r' if next == Some('s') => (vec![Unit::Sound(RS)], 2),
            'r' if next == Some('t') => (vec![Unit::Sound(RT)], 2),
            'c' if next == Some('k') => (vec![Unit::Sound(K)], 2),
            'c' if is_front(next) => (vec![Unit::Sound(S)], 1),
            'q' if next == Some('u') => (vec![Unit::Sound(K), Unit::Sound(V)], 2),
            'c' | 'k' | 'q' => (vec![Unit::Sound(K)], 1),
            // g before a final t is k (dimmigt).
            'g' if next == Some('t') && after.is_none() => (vec![Unit::Sound(K)], 1),
            'p' if next == Some('h') => (vec![Unit::Sound(F)], 2),
            't' if next == Some('h') => (vec![Unit::Sound(T)], 2),
            'd' if next == Some('t') => (vec![Unit::Sound(T)], 2),
            'b' => (vec![Unit::Sound(B)], 1),
            'd' => (vec![Unit::Sound(D)], 1),
            'f' => (vec![Unit::Sound(F)], 1),
            'g' => (vec![Unit::Sound(G)], 1),
            'h' => (vec![Unit::Sound(H)], 1),
            'j' => (vec![Unit::Sound(J)], 1),
            'l' => (vec![Unit::Sound(L)], 1),
            'm' => (vec![Unit::Sound(M)], 1),
            'n' => (vec![Unit::Sound(N)], 1),
            'p' => (vec![Unit::Sound(P)], 1),
            'r' => (vec![Unit::Sound(R)], 1),
            's' | 'z' => (vec![Unit::Sound(S)], 1),
            't' => (vec![Unit::Sound(T)], 1),
            'v' | 'w' => (vec![Unit::Sound(V)], 1),
            'x' => (vec![Unit::Sound(K), Unit::Sound(S)], 1),
            '\'' | '’' | '-' => (Vec::new(), 1),
            _ => return None,
        };
        units.extend(unit);
        // A doubled consonant is said once, a doubled t or k between vowels long (vatten vˈatːən,
        // flicka; but tack, kvinna, soppa).
        let doubled = step == 1 && next == Some(c) && !is_vowel(Some(c));
        let ck = c == 'c' && next == Some('k') && letter(at + 2) != Some('e');
        if (doubled || ck) && matches!(c, 't' | 'k' | 'c') && is_vowel(letter(at + 2)) {
            units.push(Unit::Sound(LONG));
        }
        at += if doubled { 2 } else { step };
    }
    Some(units)
}

/// Whether a vowel letter at `at` is followed by one consonant (or none) before a vowel or the end:
/// then a stressed vowel is long (hus, gata); before two it is short (hundra). ng, ck and x count
/// as two; a final m shortens it (fem), rd, rn and rl keep it long (barn).
fn long_before(word: &[char], at: usize) -> bool {
    let rest = &word[at + 1..];
    let cluster: Vec<char> = rest.iter().copied().take_while(|&c| !is_vowel(Some(c))).collect();
    match cluster[..] {
        ['x', ..] => false,
        // A final m or j leaves the vowel short (fem, som, hej).
        ['m' | 'j'] => cluster.len() < rest.len(),
        // rd, rn and rl leave a, o and å long (barn bˈɑːrn, bord; but gärna).
        ['r', 'd' | 'n' | 'l'] => matches!(word[at], 'a' | 'o' | 'å'),
        // sj, tj and kj are one sound; th one consonant.
        [] | [_] | ['s' | 't' | 'k', 'j', ..] | ['t', 'h'] => true,
        _ => false,
    }
}

/// The phoneme of a vowel letter, long or short, `reduced` for an unstressed e in a last syllable.
fn vowel_phoneme(letter: char, long: bool, reduced: bool) -> Phoneme {
    match (letter, long) {
        ('e', _) if reduced => SCHWA,
        ('a', true) => A_LONG,
        ('a', false) => A,
        ('e', true) => E_LONG,
        ('e' | 'ä', false) => E,
        ('i', true) => I_LONG,
        ('i', false) => I,
        ('o', true) => OO,
        ('o' | 'å', false) => O,
        ('u', true) => U_LONG,
        ('u', false) => U,
        ('y', true) => Y_LONG,
        ('y', false) => Y,
        ('å', true) => AA,
        ('ä', true) => AE_LONG,
        ('ö', true) => OE_LONG,
        _ => OE,
    }
}

/// How `word` is said; `None` for a letter Swedish does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let (chars, accents): (Vec<char>, Vec<bool>) = word.to_lowercase().chars().map(plain).unzip();
    let lower: String = chars.iter().collect();
    let accent = accents.iter().position(|&accent| accent);
    let units = units(&chars)?;
    let vowels: Vec<usize> = (0..units.len())
        .filter(|&index| matches!(units[index], Unit::Vowel { .. }))
        .collect();
    // The stress is on the first vowel, but on an accented one and on loanword endings.
    // be- is unstressed (berätta bərˈɛtːa).
    let prefixed = lower.starts_with("be")
        && chars.get(2).is_some_and(|&c| !is_vowel(Some(c)))
        && chars.get(3).is_some_and(|&c| is_vowel(Some(c)))
        && vowels.len() > 2;
    let stressed = if let Some(accent) = accent {
        vowels
            .iter()
            .copied()
            .find(|&index| matches!(units[index], Unit::Vowel { at, .. } if at == accent))
    } else if let Some(&(_, back)) = STRESSED_ENDINGS
        .iter()
        .filter(|(ending, _)| lower.ends_with(ending))
        .max_by_key(|(ending, _)| ending.len())
        && vowels.len() > back + 1
    {
        vowels.get(vowels.len() - 1 - back).copied()
    } else if prefixed {
        vowels.get(1).copied()
    } else {
        vowels.first().copied()
    };
    let last_vowel = vowels.last().copied();
    let mut segments: Vec<Segment> = Vec::with_capacity(units.len());
    for (index, unit) in units.iter().enumerate() {
        let stress = if Some(index) == stressed {
            Stress::Primary
        } else {
            Stress::Unstressed
        };
        let phoneme = match *unit {
            Unit::Vowel { letter, at } => {
                // An unstressed a is short (gata ɡˈɑːta, lärare), and so are the i of -ig (fattig) and
                // o, u and å before a final consonant (morgon mˈɔrɡɔn, femton, minus).
                let ig = letter == 'i' && chars[at + 1..] == ['g'];
                let closed_end = chars[at + 1..].len() == 1 && !is_vowel(chars.get(at + 1).copied());
                let long = long_before(&chars, at)
                    && !(stress == Stress::Unstressed
                        && (letter == 'a' || ig || (closed_end && matches!(letter, 'o' | 'u' | 'å'))));
                // An unstressed e in the last syllable is ə at the end and before one consonant
                // (pojke, vatten, fågel; but universitet).
                let reduced = stress == Stress::Unstressed
                    && Some(index) == last_vowel
                    && chars[at + 1..].len() <= 1
                    && (chars.len() == at + 1 || matches!(chars[at + 1], 'n' | 'l' | 'r' | 't' | 's'));
                // So is one before rn and the e of be- (bönderna, berätta bərˈɛtːa).
                let reduced = reduced
                    || (letter == 'e' && stress == Stress::Unstressed && chars[at + 1..].starts_with(&['r', 'n']))
                    || (prefixed && at == 1);
                vowel_phoneme(letter, long, reduced)
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

/// The endings of loanwords that take the stress, and which vowel from the end is stressed: 0
/// for the last (nation nasxˈuːn, procent), 1 for the one before (konferensen).
const STRESSED_ENDINGS: [(&str, usize); 7] = [
    ("tion", 0),
    ("sion", 0),
    ("ent", 0),
    ("ensen", 1),
    ("är", 0),
    ("ären", 1),
    ("tionen", 1),
];

/// eSpeak NG's secondary stresses after the stress: a full vowel two syllables on, and every
/// other one after it (arbetar ˈarbeːtˌar, fiskarna fˈɪskarnˌa).
pub(crate) fn rhythm(said: &mut Pronunciation) {
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

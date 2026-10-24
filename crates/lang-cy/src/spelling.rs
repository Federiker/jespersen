//! How a Welsh word is said from its spelling: the stress on the last syllable but one; y as ɨ in a
//! last syllable and ʌ before it; u as ɨ, w as a vowel or a glide; the diphthongs (ai, ae, au,
//! ei, eu, oe, wy, aw, ew, iw); ch, dd, ff, ll, ng, ph, rh and th; f as v; a stressed vowel long
//! before b, ch, d, dd, f, ff, g, s and th or at the end and short before the other consonants,
//! long with a circumflex (dŵr).

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, A_LONG, AE, AI, AU, AW, B, CH, D, DH, DZH, E, E_LONG, EI, EU, EW, F, G, H, I, I_LONG, IW, J, K, L, LL, M, N, NG,
    O, O_LONG, OE, P, R, RH, S, SH, T, TH, U, U_LONG, V, W, W_LONG, W_VOWEL, WY, X, X_FRONT, Y, legal_onset,
};

/// A vowel letter, or a pair of them said as one diphthong.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    A,
    E,
    I,
    O,
    U,
    W,
    Y,
    Diphthong(Phoneme),
}

/// What a letter, or a group of them, comes to before the stress is known.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Unit {
    /// A vowel, where its letters start and end, and whether a circumflex makes it long.
    Vowel {
        kind: Kind,
        at: usize,
        end: usize,
        long: bool,
    },
    Sound(Phoneme),
}

/// `c` without a circumflex (long) or an acute accent (stressed), and which it had: 'ˆ' or '´'.
fn plain(c: char) -> (char, Option<char>) {
    match c {
        'â' => ('a', Some('ˆ')),
        'ê' => ('e', Some('ˆ')),
        'î' => ('i', Some('ˆ')),
        'ô' => ('o', Some('ˆ')),
        'û' => ('u', Some('ˆ')),
        'ŵ' => ('w', Some('ˆ')),
        'ŷ' => ('y', Some('ˆ')),
        'á' => ('a', Some('´')),
        'é' => ('e', Some('´')),
        'í' => ('i', Some('´')),
        'ó' => ('o', Some('´')),
        'ú' => ('u', Some('´')),
        'ẃ' => ('w', Some('´')),
        'ý' => ('y', Some('´')),
        'ä' | 'à' => ('a', None),
        'ë' | 'è' => ('e', None),
        'ï' | 'ì' => ('i', None),
        'ö' | 'ò' => ('o', None),
        'ü' | 'ù' => ('u', None),
        'ẅ' | 'ẁ' => ('w', None),
        'ÿ' | 'ỳ' => ('y', None),
        _ => (c, None),
    }
}

fn is_vowel(c: Option<char>) -> bool {
    matches!(c, Some('a' | 'e' | 'i' | 'o' | 'u' | 'w' | 'y'))
}

/// The units of `word`'s letters, in lower case and without accents (`marks` holds them); `None`
/// for a letter Welsh does not read.
fn units(word: &[char], marks: &[Option<char>]) -> Option<Vec<Unit>> {
    let letter = |at: usize| word.get(at).copied();
    let mut units: Vec<Unit> = Vec::with_capacity(word.len());
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        let next = letter(at + 1);
        let before = at.checked_sub(1).and_then(letter);
        let vowel = |kind: Kind, length: usize| Unit::Vowel {
            kind,
            at,
            end: at + length,
            long: marks[at] == Some('ˆ'),
        };
        let diphthong = |phoneme: Phoneme| (vec![vowel(Kind::Diphthong(phoneme), 2)], 2);
        let (unit, step): (Vec<Unit>, usize) = match (c, next) {
            ('a', Some('i')) => diphthong(AI),
            ('a', Some('e')) => diphthong(AE),
            ('a', Some('u')) => diphthong(AU),
            ('a', Some('w')) => diphthong(AW),
            ('e', Some('i')) => diphthong(EI),
            ('e', Some('u' | 'y')) => diphthong(EU),
            ('e', Some('w')) => diphthong(EW),
            ('o', Some('e' | 'i' | 'u')) => diphthong(OE),
            ('i' | 'y' | 'u', Some('w')) if !is_vowel(letter(at + 2)) => diphthong(IW),
            ('w', Some('y')) if !is_vowel(before) => diphthong(WY),
            // w is a glide before another vowel and between g and a consonant (dweud, gwraig).
            ('w', Some(next))
                if (is_vowel(Some(next)) && next != 'w')
                    || (before == Some('g') && matches!(next, 'l' | 'r' | 'n')) =>
            {
                (vec![Unit::Sound(W)], 1)
            }
            // i before another vowel is j (astudio, cariad).
            ('i', Some(next)) if is_vowel(Some(next)) && next != 'w' && at + 1 < word.len() => {
                (vec![Unit::Sound(J)], 1)
            }
            ('a', _) => (vec![vowel(Kind::A, 1)], 1),
            ('e', _) => (vec![vowel(Kind::E, 1)], 1),
            ('i', _) => (vec![vowel(Kind::I, 1)], 1),
            ('o', _) => (vec![vowel(Kind::O, 1)], 1),
            ('u', _) => (vec![vowel(Kind::U, 1)], 1),
            ('w', _) => (vec![vowel(Kind::W, 1)], 1),
            ('y', _) => (vec![vowel(Kind::Y, 1)], 1),
            // si before a vowel is ʃ (siarad).
            ('s', Some('i')) if is_vowel(letter(at + 2)) => (vec![Unit::Sound(SH)], 2),
            // ch right after e or i is ç in eSpeak NG's notation (dechrau, chwech).
            ('c', Some('h')) if matches!(before, Some('e' | 'i')) => (vec![Unit::Sound(X_FRONT)], 2),
            ('c', Some('h')) => (vec![Unit::Sound(X)], 2),
            ('d', Some('d')) => (vec![Unit::Sound(DH)], 2),
            ('f', Some('f')) | ('p', Some('h')) => (vec![Unit::Sound(F)], 2),
            ('n', Some('g')) => (vec![Unit::Sound(NG)], 2),
            ('l', Some('l')) => (vec![Unit::Sound(LL)], 2),
            ('r', Some('h')) => (vec![Unit::Sound(RH)], 2),
            ('t', Some('h')) => (vec![Unit::Sound(TH)], 2),
            ('t', Some('s')) if letter(at + 2) == Some('h') => (vec![Unit::Sound(CH)], 3),
            ('j', _) => (vec![Unit::Sound(DZH)], 1),
            ('b', _) => (vec![Unit::Sound(B)], 1),
            ('c' | 'k', _) => (vec![Unit::Sound(K)], 1),
            ('d', _) => (vec![Unit::Sound(D)], 1),
            ('f' | 'v', _) => (vec![Unit::Sound(V)], 1),
            ('g', _) => (vec![Unit::Sound(G)], 1),
            ('h', _) => (vec![Unit::Sound(H)], 1),
            ('l', _) => (vec![Unit::Sound(L)], 1),
            ('m', _) => (vec![Unit::Sound(M)], 1),
            ('n', _) => (vec![Unit::Sound(N)], 1),
            ('p', _) => (vec![Unit::Sound(P)], 1),
            ('r', _) => (vec![Unit::Sound(R)], 1),
            ('s' | 'z', _) => (vec![Unit::Sound(S)], 1),
            ('t', _) => (vec![Unit::Sound(T)], 1),
            ('x', _) => (vec![Unit::Sound(K), Unit::Sound(S)], 1),
            ('\'' | '’' | '-', _) => (Vec::new(), 1),
            _ => return None,
        };
        // A doubled r, m, t, p, c or s is said once (torri); a doubled n is said twice, as eSpeak
        // NG does (ysgrifennwch).
        let doubled = step == 1 && next == Some(c) && matches!(c, 'r' | 'm' | 't' | 'p' | 'c' | 's');
        units.extend(unit);
        at += if doubled { 2 } else { step };
    }
    Some(units)
}

/// Whether a stressed vowel whose letters end at `end` is long: at the end of the word, and
/// before one of b, ch, d, dd, f, ff, g, s and th; i, u and y also before n, l and r (un ˈɨːn,
/// mil, miliwn; but car, hwn).
fn long_before(word: &[char], end: usize, high: bool) -> bool {
    let rest = &word[end..];
    let cluster: Vec<char> = rest.iter().copied().take_while(|&c| !is_vowel(Some(c))).collect();
    match cluster[..] {
        [] | ['c' | 't', 'h', ..] | ['d', 'd'] | ['f', 'f'] | ['b' | 'd' | 'g' | 'f' | 's'] => true,
        // y is short before n (bryn brˈøn, cyn).
        ['n'] => high && word[end - 1] != 'y',
        ['l' | 'r'] => high,
        _ => false,
    }
}

/// The phoneme of a vowel unit, `stressed` or not, `long`, and `last` when it is the word's last
/// vowel (where y is ɨ, and an unstressed ɨ ø in eSpeak NG's notation).
fn vowel_phoneme(kind: Kind, stressed: bool, long: bool, last: bool) -> Phoneme {
    let long = long && stressed;
    match kind {
        Kind::Diphthong(phoneme) => phoneme,
        Kind::A if long => A_LONG,
        Kind::A => A,
        Kind::E if long => E_LONG,
        Kind::E => E,
        Kind::I if long => I_LONG,
        Kind::I => I,
        Kind::O if long => O_LONG,
        Kind::O => O,
        Kind::W if long => W_LONG,
        Kind::W => W_VOWEL,
        Kind::Y if !last => Y,
        Kind::U | Kind::Y if long => U_LONG,
        Kind::U | Kind::Y => U,
    }
}

/// How `word` is said; `None` for a letter Welsh does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let (chars, marks): (Vec<char>, Vec<Option<char>>) = word.to_lowercase().chars().map(plain).unzip();
    let units = units(&chars, &marks)?;
    let vowels: Vec<usize> = (0..units.len())
        .filter(|&index| matches!(units[index], Unit::Vowel { .. }))
        .collect();
    // The stress is on the last vowel but one, or where an acute accent is (caniatáu).
    let accent = marks.iter().position(|&mark| mark == Some('´'));
    let accented = accent.and_then(|accent| {
        vowels
            .iter()
            .copied()
            .find(|&index| matches!(units[index], Unit::Vowel { at, end, .. } if at <= accent && accent < end))
    });
    let stressed = accented.or_else(|| {
        vowels
            .len()
            .checked_sub(2)
            .map(|at| vowels[at])
            .or(vowels.first().copied())
    });
    let last_vowel = vowels.last().copied();
    let mut segments: Vec<Segment> = Vec::with_capacity(units.len());
    for (index, unit) in units.iter().enumerate() {
        let is_stressed = Some(index) == stressed;
        let phoneme = match *unit {
            Unit::Vowel { kind, end, long, .. } => {
                let high = matches!(kind, Kind::I | Kind::U | Kind::Y);
                let long = long || long_before(&chars, end, high);
                vowel_phoneme(kind, is_stressed, long, Some(index) == last_vowel)
            }
            Unit::Sound(phoneme) => phoneme,
        };
        let stress = if is_stressed && phoneme.is_vowel() {
            Stress::Primary
        } else {
            Stress::Unstressed
        };
        segments.push(Segment::new(phoneme, stress));
    }
    if segments.is_empty() {
        return None;
    }
    Some(syllabify(&segments, legal_onset))
}

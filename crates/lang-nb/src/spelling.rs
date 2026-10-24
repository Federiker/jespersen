//! How a Norwegian Bokmål word is said from its spelling: a stressed vowel long before one
//! consonant or at the end and short before two (hus, gutt); o as uː or ɔ, å as oː or ɔ, u as ʉ, e
//! as æ before r; ei, øy and au; the sj sound in sj, skj and sk before a front vowel, the kj sound
//! in kj and k before one, tj, j for g before a front vowel and for gj, hj and lj; rs as ʃ, nd as
//! nn, hv as v; a doubled consonant long; an unstressed e as ə; the stress on the first syllable,
//! as eSpeak NG reads it, but after be-.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, A_FINAL, A_LONG, A_UNSTRESSED, AA, AE, AE_LONG, AU, B, D, E, E_BEFORE_R, E_LONG, EG, EI, F, G, H, I, I_LONG, J,
    K, KJ, L, M, N, NG, O, OE, OE_LONG, OEY, OO, P, R, S, SCHWA, SCHWA_FINAL, SJ, T, TJ, U, U_LONG, V, Y, Y_LONG,
    doubled, legal_onset,
};

fn is_vowel(c: Option<char>) -> bool {
    matches!(c, Some('a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'å' | 'æ' | 'ø'))
}

/// `c` without an acute accent, which marks the stress (idé, jespersen's list), and whether it had
/// one.
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

/// The front vowels, before which k, g and sk soften (kirke, gi, ski).
fn is_front(c: Option<char>) -> bool {
    matches!(c, Some('e' | 'i' | 'y' | 'æ' | 'ø'))
}

/// What a letter, or a group of them, comes to before the stress is known.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Unit {
    /// A vowel letter, or ei, øy or au (as e, ø and a with `diphthong`), and where it is.
    Vowel {
        letter: char,
        at: usize,
        diphthong: bool,
    },
    Sound(Phoneme),
}

/// The units of `word`'s letters, in lower case; `None` for a letter Norwegian does not read.
#[expect(clippy::too_many_lines, reason = "one arm per spelling, read in order")]
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
        let ends = |at: usize| at + 1 == word.len();
        let (unit, step): (Vec<Unit>, usize) = match c {
            // eg ending a word of one syllable is a diphthong (jeg jˈɛːɪ).
            'e' if next == Some('g') && at + 2 == word.len() && word[..at].iter().all(|&c| !is_vowel(Some(c))) => (
                vec![Unit::Vowel {
                    letter: 'g',
                    at,
                    diphthong: true,
                }],
                2,
            ),
            'e' | 'a' if next == Some(if c == 'e' { 'i' } else { 'u' }) => (
                vec![Unit::Vowel {
                    letter: c,
                    at,
                    diphthong: true,
                }],
                2,
            ),
            'ø' if next == Some('y') => (
                vec![Unit::Vowel {
                    letter: c,
                    at,
                    diphthong: true,
                }],
                2,
            ),
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'å' | 'æ' | 'ø' => (
                vec![Unit::Vowel {
                    letter: c,
                    at,
                    diphthong: false,
                }],
                1,
            ),
            // -sjon: ʃuːn (stasjon).
            's' if rest_is(at, "skj") => (vec![Unit::Sound(SJ)], 3),
            's' if next == Some('j') => (vec![Unit::Sound(SJ)], 2),
            's' if start && next == Some('k') && is_front(after) => (vec![Unit::Sound(SJ)], 2),
            's' if rest_is(at, "stj") => (vec![Unit::Sound(S), Unit::Sound(TJ)], 3),
            't' if next == Some('j') => (vec![Unit::Sound(TJ)], 2),
            'k' if next == Some('j') => (vec![Unit::Sound(KJ)], 2),
            'k' if start && is_front(next) => (vec![Unit::Sound(KJ)], 1),
            // g before a front vowel is j at the start and after be- (gi, begynne bəjˈynna).
            'g' if (start || (at == 2 && word[..2] == ['b', 'e'])) && is_front(next) => (vec![Unit::Sound(J)], 1),
            'g' | 'h' | 'l' if start && next == Some('j') => (vec![Unit::Sound(J)], 2),
            'h' if next == Some('v') => (vec![Unit::Sound(V)], 2),
            'n' if next == Some('g') => (vec![Unit::Sound(NG)], 2),
            'n' if next == Some('k') => (vec![Unit::Sound(NG)], 1),
            'g' if next == Some('n') && at > 0 => (vec![Unit::Sound(NG)], 1),
            // rs is ʃ, after the r inside a word (norsk nˈɔrʃk, but fors fˈɔʃ).
            'r' if next == Some('s') && ends(at + 1) => (vec![Unit::Sound(SJ)], 2),
            'r' if next == Some('s') => (vec![Unit::Sound(R), Unit::Sound(SJ)], 2),
            // A final d after r is unsaid (bord bˈuːr); after n or l it lengthens them (land lˈɑnn,
            // hundre, and in the older spelling vilde, kunde, manden).
            'd' if ends(at) && at > 0 && letter(at - 1) == Some('r') => (Vec::new(), 1),
            'd' if at > 0 && matches!(letter(at - 1), Some('n' | 'l')) && !matches!(next, Some('d' | 's')) => (
                vec![Unit::Sound(doubled(if letter(at - 1) == Some('n') { N } else { L }))],
                1,
            ),
            'c' if next == Some('k') => (vec![Unit::Sound(K)], 2),
            'c' if is_front(next) => (vec![Unit::Sound(S)], 1),
            'q' if next == Some('u') => (vec![Unit::Sound(K), Unit::Sound(V)], 2),
            'c' | 'k' | 'q' => (vec![Unit::Sound(K)], 1),
            'p' if next == Some('h') => (vec![Unit::Sound(F)], 2),
            't' if next == Some('h') => (vec![Unit::Sound(T)], 2),
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
        // A doubled consonant is long: ː after a stop (gutt ɡˈʉtː), the consonant twice in eSpeak
        // NG's notation for l, m, n and r between vowels (kvinne kvˈɪnna; but vann vˈɑn).
        let doubled_letter = step == 1 && next == Some(c) && !is_vowel(Some(c));
        let ck = c == 'c' && next == Some('k');
        if let Some(Unit::Sound(sound)) = units.last().copied()
            && (doubled_letter || ck)
        {
            let is_stop = matches!(c, 'p' | 't' | 'k' | 'b' | 'd' | 'g' | 'c');
            if is_stop || is_vowel(letter(at + 2)) {
                units.push(Unit::Sound(doubled(sound)));
            }
        }
        at += if doubled_letter { 2 } else { step };
    }
    Some(units)
}

/// Whether a vowel letter at `at` is followed by one consonant (or none) before a vowel or the end:
/// then a vowel is long (hus, gate); before two it is short (gutt, nasjon). ng and x count as
/// two, rd as one (bord).
fn long_before(word: &[char], at: usize) -> bool {
    let rest = &word[at + 1..];
    let cluster: Vec<char> = rest.iter().copied().take_while(|&c| !is_vowel(Some(c))).collect();
    match cluster[..] {
        ['x', ..] => false,
        // A final m leaves the vowel short (som, fem).
        ['m'] => cluster.len() < rest.len(),
        [] | [_] | ['t', 'h'] | ['r', 'd'] => true,
        _ => false,
    }
}

/// Where a vowel letter is, for its phoneme.
#[derive(Clone, Copy)]
#[expect(clippy::struct_excessive_bools, reason = "independent facts about one vowel")]
struct Place {
    /// Before one consonant or none (long), before r, unstressed and reduced, ending the word, in
    /// a word of one syllable.
    long: bool,
    before_r: bool,
    reduced: bool,
    last: bool,
    one_syllable: bool,
}

/// The phoneme of a vowel letter at `place`: an e before r is æ, and æː in a word of one syllable
/// (der, her); a reduced e is ə, written a at the end of the word.
fn vowel_phoneme(letter: char, place: Place) -> Phoneme {
    let Place {
        long,
        before_r,
        reduced,
        last,
        one_syllable,
    } = place;
    match (letter, long) {
        ('e', _) if reduced => {
            if last {
                SCHWA_FINAL
            } else {
                SCHWA
            }
        }
        ('e', true) if before_r && one_syllable => E_BEFORE_R,
        ('e', false) if before_r => AE,
        // A final a in a word of one syllable is ɑ in eSpeak NG's notation (da dˈɑ, fra).
        ('a', true) if last && one_syllable => A_FINAL,
        ('a', true) => A_LONG,
        ('a', false) => A,
        ('e', true) => E_LONG,
        ('e', false) => E,
        ('æ', true) => AE_LONG,
        ('æ', false) => AE,
        ('i', true) => I_LONG,
        ('i', false) => I,
        ('o', true) => OO,
        ('o' | 'å', false) => O,
        ('u', true) => U_LONG,
        ('u', false) => U,
        ('y', true) => Y_LONG,
        ('y', false) => Y,
        ('å', true) => AA,
        ('ø', true) => OE_LONG,
        _ => OE,
    }
}

/// How `word` is said; `None` for a letter Norwegian does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let (chars, accents): (Vec<char>, Vec<bool>) = word.to_lowercase().chars().map(plain).unzip();
    let lower: String = chars.iter().collect();
    let accent = accents.iter().position(|&accent| accent);
    let units = units(&chars)?;
    let vowels: Vec<usize> = (0..units.len())
        .filter(|&index| matches!(units[index], Unit::Vowel { .. }))
        .collect();
    // The stress is on the first vowel, but on an accented one and after be- (begynne bəjˈynna).
    let prefixed = lower.starts_with("be")
        && chars.get(2).is_some_and(|&c| !is_vowel(Some(c)))
        && chars.get(3).is_some_and(|&c| is_vowel(Some(c)))
        && vowels.len() > 2;
    let stressed = if let Some(accent) = accent {
        vowels
            .iter()
            .copied()
            .find(|&index| matches!(units[index], Unit::Vowel { at, .. } if at == accent))
    } else if prefixed {
        vowels.get(1).copied()
    } else {
        vowels.first().copied()
    };
    let mut segments: Vec<Segment> = Vec::with_capacity(units.len());
    for (index, unit) in units.iter().enumerate() {
        let stress = if Some(index) == stressed {
            Stress::Primary
        } else {
            Stress::Unstressed
        };
        let phoneme = match *unit {
            Unit::Vowel {
                letter,
                diphthong: true,
                ..
            } => match letter {
                'e' => EI,
                'g' => EG,
                'a' => AU,
                _ => OEY,
            },
            Unit::Vowel { letter, at, .. } => {
                let unstressed = stress == Stress::Unstressed;
                let long = long_before(&chars, at);
                let before_r = chars.get(at + 1) == Some(&'r');
                // An unstressed e in the last syllable, or in be-, is ə (skole, begynne).
                let reduced =
                    unstressed && letter == 'e' && (Some(index) == vowels.last().copied() || (prefixed && at == 1));
                // An unstressed final a is short, written a in eSpeak NG's notation (barna bˈɑrna).
                if letter == 'a' && unstressed && at + 1 == chars.len() {
                    A_UNSTRESSED
                } else {
                    let place = Place {
                        long,
                        before_r,
                        reduced,
                        last: at + 1 == chars.len(),
                        one_syllable: vowels.len() == 1,
                    };
                    vowel_phoneme(letter, place)
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

/// eSpeak NG's secondary stresses after the stress: a full vowel two syllables on, and every
/// other one after it (universitet ˈʉːniːvˌarʃiːtət, million).
fn rhythm(said: &mut Pronunciation) {
    let Some(stressed) = said
        .syllables
        .iter()
        .position(|syllable| syllable.stress == Stress::Primary)
    else {
        return;
    };
    for syllable in said.syllables.iter_mut().skip(stressed + 2).step_by(2) {
        if syllable
            .nucleus
            .iter()
            .all(|vowel| ![SCHWA, SCHWA_FINAL].contains(vowel))
        {
            syllable.stress = Stress::Secondary;
        }
    }
}

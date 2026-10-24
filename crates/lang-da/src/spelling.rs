//! How a Danish word is said from its spelling: a vowel long in an open syllable and short before
//! two consonants; a as ɑ before r and before m, ng, k, g and v, short i as e before n, m, k, l
//! and s with a consonant, e as ɛ before r, short o as ʌ, u as ɔ before n, m, f and g; ej and eg
//! as ɑj, øj as ʌj, ov and og as ɒw, av as ɑw; a soft d after a vowel, d unsaid after n and l, g
//! and v after a vowel as j and w (dag, tyve); r after a vowel as ɐ̯, the unstressed -er, -erne
//! and -re as ɐ, -et as əð, -igt as it, t after a consonant before ə as d; the older spelling's
//! aa as å; the stress on the first syllable, after be-, ge- and for-, on the endings of
//! loanwords (-ion, -tet, -ere) and on é (café).

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, A_BACK, A_DARK, AA, AA_SHORT, AE, B, D, DH, E, E_LONG, E_PLAIN, E_PREFIX, ER, ER_AFTER_E, F, G, H, I, I_LONG,
    I_LONG_PLAIN, I_LOW, I_PLAIN, J, K, L, M, N, NG, O, O_CLOSE, O_CLOSE_PLAIN, O_OPEN, O_PLAIN, O_RR, O_SHORT, OE, P,
    R, R_VOCALIC, S, SCHWA, SH, T, U, U_LONG, U_LONG_PLAIN, U_LOW, U_PLAIN, V, W, Y, Y_LONG, Y_LONG_PLAIN, Y_PLAIN,
    Y_SHORT, legal_onset,
};

fn is_vowel(c: Option<char>) -> bool {
    matches!(c, Some('a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'æ' | 'ø' | 'å' | 'ä'))
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
    /// An r after a vowel, said ɐ̯ or with the vowel once the stress is known.
    CodaR,
}

/// The units of `word`'s letters, in lower case; `None` for a letter Danish does not read.
fn units(word: &[char]) -> Option<Vec<Unit>> {
    let letter = |at: usize| word.get(at).copied();
    let before = |at: usize| at.checked_sub(1).and_then(letter);
    let rest_is = |at: usize, text: &str| word[at..].iter().copied().take(text.chars().count()).eq(text.chars());
    let mut units: Vec<Unit> = Vec::with_capacity(word.len());
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        let next = letter(at + 1);
        let after_vowel = is_vowel(before(at));
        let (unit, step): (Vec<Unit>, usize) = match c {
            // Loanwords from German may keep their ä (enormität).
            'ä' => (vec![Unit::Vowel { letter: 'æ', at }], 1),
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'æ' | 'ø' | 'å' => (vec![Unit::Vowel { letter: c, at }], 1),
            // -tion is ʃon (nation naʃˈʔon).
            't' if rest_is(at + 1, "ion") && at > 0 => (vec![Unit::Sound(SH)], 2),
            's' if next == Some('j') => (vec![Unit::Sound(SH)], 2),
            'c' if next == Some('h') => (vec![Unit::Sound(SH)], 2),
            'h' if matches!(next, Some('v' | 'j')) => (Vec::new(), 1),
            'n' if next == Some('g') => (vec![Unit::Sound(NG)], 2),
            'n' if next == Some('k') => (vec![Unit::Sound(NG)], 1),
            // d: soft after a vowel (gade, med), unsaid after n and l (mand, vilde) and at the end
            // after r (bord).
            'd' if next == Some('t') => (vec![Unit::Sound(T)], 2),
            'd' if matches!(before(at), Some('n' | 'l')) => (Vec::new(), 1),
            'd' if before(at) == Some('r') && next.is_none() => (Vec::new(), 1),
            'd' if after_vowel && !matches!(next, Some('r' | 'l' | 'n' | 'j')) => (vec![Unit::Sound(DH)], 1),
            // g after a vowel: j after a, e, æ and ø, unsaid after i and y, w after o, u and å
            // (dag, sige, tog), but ɡ before a consonant (sagt).
            'g' if after_vowel
                && (next.is_none() || is_vowel(next))
                && !(at == 2 && matches!(word[..2], ['b' | 'g', 'e'])) =>
            {
                match before(at) {
                    Some('i' | 'y') => (Vec::new(), 1),
                    Some('o' | 'u' | 'å') => (vec![Unit::Sound(W)], 1),
                    _ => (vec![Unit::Sound(J)], 1),
                }
            }
            // v after a vowel at the end or before a consonant is w (syv, navn), unsaid after l
            // there (selv, halvtreds).
            'v' if before(at) == Some('l') && !is_vowel(next) => (Vec::new(), 1),
            'v' if after_vowel && !is_vowel(next) => (vec![Unit::Sound(W)], 1),
            // r after a vowel waits for the stress (der dˈεɐ̯, far fˈɑ); after b it is ɐ̯ in eSpeak
            // NG's notation (brød).
            'r' if after_vowel && !is_vowel(next) => (vec![Unit::CodaR], 1),
            // g after r at the end or before e is w (borg bˈɒɒw, bjerget).
            // -igt is it (dejligt dˈ?ɑjlʔit).
            'g' if before(at) == Some('i') && next == Some('t') && at > 2 && at + 2 == word.len() => (Vec::new(), 1),
            'g' if before(at) == Some('r') && matches!(next, None | Some('e')) => (vec![Unit::Sound(W)], 1),
            'r' if before(at) == Some('b') => (vec![Unit::Sound(R_VOCALIC)], 1),
            'c' if matches!(next, Some('e' | 'i' | 'y')) => (vec![Unit::Sound(S)], 1),
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
        // A doubled consonant is said once (komme, ikke).
        let doubled = step == 1 && next == Some(c) && !is_vowel(Some(c));
        at += if doubled { 2 } else { step };
    }
    Some(units)
}

/// Whether a vowel letter at `at` is long: before one consonant and a vowel or at the end, and in
/// a word of one syllable before a final d, s or r, l after i, u and y, or n after i (hus, tid,
/// smil, min; but kan, vel, op).
fn long(word: &[char], at: usize, one_syllable: bool) -> bool {
    let rest = &word[at + 1..];
    let cluster: Vec<char> = rest.iter().copied().take_while(|&c| !is_vowel(Some(c))).collect();
    match cluster[..] {
        [c] if cluster.len() == rest.len() => {
            one_syllable
                && (matches!(c, 'd' | 's' | 'r')
                    || (c == 'l' && matches!(word[at], 'i' | 'u' | 'y'))
                    || (c == 'n' && word[at] == 'i'))
        }
        [] | [_] => true,
        // A final rd keeps the vowel long (bord bˈʔoɐ̯).
        ['r', 'd'] => cluster.len() == rest.len(),
        _ => false,
    }
}

/// The unstressed prefixes (begynde, gestalt, forstå).
const PREFIXES: [&str; 3] = ["be", "ge", "for"];

/// The endings of loanwords that take the stress, and which vowel from the end is stressed: 0 for
/// the last (nation, universitet), 1 for the one before (studere).
const STRESSED_ENDINGS: [(&str, usize); 12] = [
    ("ion", 0),
    ("ioner", 1),
    ("tet", 0),
    ("ere", 1),
    ("erer", 1),
    ("erede", 2),
    ("eret", 1),
    ("ist", 0),
    ("ent", 0),
    ("ant", 0),
    ("ur", 0),
    ("eri", 0),
];

/// Whether `word` starts with an unstressed prefix whose stem starts as a syllable can.
fn prefixed(word: &[char]) -> Option<usize> {
    let text: String = word.iter().collect();
    PREFIXES.iter().find_map(|prefix| {
        let stem = text.strip_prefix(prefix)?;
        let mut letters = stem.chars();
        let (first, second) = (letters.next(), letters.next());
        let starts = is_vowel(first) && !(prefix.ends_with('e') && matches!(first, Some('j' | 'i' | 'e')))
            || is_vowel(second)
            || matches!(second, Some('l' | 'r' | 'j' | 'v'))
            || first == Some('s');
        (stem.chars().count() >= 3 && stem.chars().any(|c| is_vowel(Some(c))) && starts).then(|| prefix.chars().count())
    })
}

/// `c` with an acute accent taken off, which marks the stress in jespersen's list and in loanwords
/// (café), and whether it had one.
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

/// The phoneme of a stressed or full vowel `letter`, `long` or short, before `next` and `after`.
fn full_vowel(letter: char, long: bool, next: Option<char>, after: Option<char>) -> Phoneme {
    let dark_before = matches!(next, Some('m' | 'k' | 'v')) || (next == Some('n') && matches!(after, Some('g' | 'k')));
    match letter {
        'a' if next == Some('r') => A_BACK,
        'a' if !long && dark_before => A_DARK,
        'a' => A,
        // e before an r that ends the syllable is ɛ (der, her; but mere).
        'e' if next == Some('r') && !is_vowel(after) => E,
        'e' if long => E_LONG,
        'æ' if long || next == Some('r') => AE,
        'e' | 'æ' => E,
        'i' if long => I,
        'i' if matches!(next, Some('n' | 'k' | 'd' | 't' | 'm'))
            || (next == Some('s') && !is_vowel(after) && after.is_some()) =>
        {
            I_LOW
        }
        // A short i before l is e but before ld (til, stille; vilde).
        'i' if next == Some('l') && after != Some('d') => I_LOW,
        'i' => I,
        'o' if next == Some('r') && !long && !matches!(after, Some('d') | None) => O_RR,
        'o' if next == Some('r') && !long => O_OPEN,
        // A short o before t is o (otte ˈʔotə).
        'o' if next == Some('t') && !long => O,
        'o' if long => O,
        'o' => O_SHORT,
        'u' if !long && matches!(next, Some('n' | 'm' | 'f' | 'g')) => U_LOW,
        'u' => U,
        'y' if long => Y,
        'y' => Y_SHORT,
        'ø' => OE,
        _ if long => AA,
        _ => AA_SHORT,
    }
}

/// How `word` is said; `None` for a letter Danish does not read.
#[expect(clippy::too_many_lines, reason = "the steps of a word's reading, in order")]
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    // The older spelling writes å as aa (paa).
    let lower = word.to_lowercase().replace("aa", "å");
    let (chars, accents): (Vec<char>, Vec<bool>) = lower.chars().map(plain).unzip();
    let lower: String = chars.iter().collect();
    let accent = accents.iter().position(|&accent| accent);
    let units = units(&chars)?;
    let vowels: Vec<usize> = (0..units.len())
        .filter(|&index| matches!(units[index], Unit::Vowel { .. }))
        .collect();
    let stem = if accent.is_none() { prefixed(&chars) } else { None };
    let vowel_at = |index: usize| match units[index] {
        Unit::Vowel { at, .. } => at,
        _ => 0,
    };
    let stressed = if let Some(accent) = accent {
        vowels.iter().copied().find(|&index| vowel_at(index) == accent)
    } else if let Some(&(_, back)) = STRESSED_ENDINGS
        .iter()
        .filter(|(ending, _)| lower.ends_with(ending))
        .max_by_key(|(ending, _)| ending.len())
        && vowels.len() > back + 1
    {
        vowels.get(vowels.len() - 1 - back).copied()
    } else if let Some(stem) = stem {
        vowels.iter().copied().find(|&index| vowel_at(index) >= stem)
    } else {
        vowels.first().copied()
    };
    let last_vowel = vowels.last().copied();
    let mut segments: Vec<Segment> = Vec::with_capacity(units.len());
    let mut index = 0;
    while index < units.len() {
        let stress = if Some(index) == stressed {
            Stress::Primary
        } else {
            Stress::Unstressed
        };
        let mut phonemes: Vec<Phoneme> = Vec::new();
        match units[index] {
            Unit::Vowel { letter, at } => {
                let next = chars.get(at + 1).copied();
                let after = chars.get(at + 2).copied();
                let in_prefix = stem.is_some_and(|stem| at < stem);
                // ej, eg, øj, øg, ov, og and av are diphthongs (vej, øje, over, navn).
                let diphthong = match (letter, next) {
                    ('e', Some('j' | 'g')) if !is_vowel(after) || next == Some('j') => Some([A_DARK, J]),
                    ('ø', Some('j' | 'g')) if !is_vowel(after) || next == Some('j') => Some([O_SHORT, J]),
                    ('o', Some('v' | 'g')) if !is_vowel(after) || next == Some('v') => Some([O_OPEN, W]),
                    ('a', Some('v')) if !is_vowel(after) => Some([A_DARK, W]),
                    _ => None,
                };
                if let Some(diphthong) = diphthong {
                    phonemes.extend(diphthong);
                    segments.extend(phonemes.iter().map(|&phoneme| {
                        Segment::new(phoneme, if phoneme.is_vowel() { stress } else { Stress::Unstressed })
                    }));
                    // The j, g or v is said with the vowel.
                    index += 2;
                    continue;
                }
                let one_syllable = vowels.len() == 1;
                // A stressed last syllable of a longer word is long before one consonant (nation).
                let long = long(&chars, at, one_syllable)
                    || (stress == Stress::Primary && !one_syllable && chars[at + 1..].len() == 1);
                let unstressed = stress == Stress::Unstressed;
                let phoneme = if in_prefix {
                    // be- and ge- have ʔe, for- ʔʌ in eSpeak NG's notation (begynde bʔeɡˈʔœnə).
                    if letter == 'e' { E_PREFIX } else { O_SHORT }
                } else if unstressed && letter == 'i' && chars[at + 1..] == ['n', 'd'] {
                    // -ind after the stress is ən (tusind tˈʔusən).
                    SCHWA
                } else if unstressed && letter == 'e' && next == Some('r') && Some(index) != last_vowel {
                    // After the stress it is ɐ with the r (bagerne bˈ?ɑkʔʌnə), before it ɛ.
                    if stressed.is_some_and(|stressed| index > stressed) {
                        ER
                    } else {
                        E
                    }
                } else if !unstressed || letter != 'e' {
                    full_vowel(letter, long, next, after)
                } else {
                    SCHWA
                };
                phonemes.push(phoneme);
            }
            Unit::Sound(phoneme) => {
                // -et and -ed after the stress are əð (meget, intet).
                let final_t = phoneme == T
                    && index + 1 == units.len()
                    && index > 0
                    && Some(index - 1) == last_vowel
                    && Some(index - 1) != stressed
                    && chars.ends_with(&['e', 't']);
                // t after a consonant but n before an unstressed e is d (femten fˈɛmdən, næsten).
                let soft_t = phoneme == T
                    && index > 0
                    && matches!(units[index - 1], Unit::Sound(before) if before != N)
                    && matches!(units.get(index + 1), Some(Unit::Vowel { letter: 'e', .. }))
                    && Some(index + 1) != stressed;
                phonemes.push(if final_t {
                    DH
                } else if soft_t {
                    D
                } else {
                    phoneme
                });
            }
            Unit::CodaR => {
                // After a and å, and in an -er- after the stress, the r is in the vowel (far fˈɑ, år,
                // bagerne); elsewhere ɐ̯.
                let previous = index.checked_sub(1).map(|previous| units[previous]);
                let absorbed = match previous {
                    Some(Unit::Vowel { letter: 'a' | 'å', .. }) => true,
                    Some(Unit::Vowel { .. }) if segments.last().is_some_and(|segment| segment.phoneme == ER) => true,
                    // In for- and before a consonant but d, o and r are ɒɒ (forstå, kort kˈɒɒt).
                    Some(Unit::Vowel { letter: 'o', at }) => {
                        (stem.is_some() && at < 3)
                            || (chars.get(at + 2).is_some_and(|&c| !is_vowel(Some(c)) && c != 'd')
                                && Some(index - 1) == stressed)
                    }
                    _ => false,
                };
                if !absorbed {
                    phonemes.push(R_VOCALIC);
                }
            }
        }
        segments.extend(
            phonemes
                .iter()
                .map(|&phoneme| Segment::new(phoneme, if phoneme.is_vowel() { stress } else { Stress::Unstressed })),
        );
        index += 1;
    }
    if segments.is_empty() {
        return None;
    }
    unstressed_r(&mut segments);
    let mut said = syllabify(&segments, legal_onset);
    rhythm(&mut said);
    Some(said)
}

/// The unstressed -er, -ers and -re at the end of a word are ɐ (efter ˈεftʔʌ, kære kˈɛʔʌ), after a
/// consonant with the r said (mindre).
fn unstressed_r(segments: &mut Vec<Segment>) {
    let length = segments.len();
    let phonemes: Vec<Phoneme> = segments.iter().map(|segment| segment.phoneme).collect();
    let er = Segment::new(ER, Stress::Unstressed);
    match phonemes[..] {
        [.., schwa, r] if schwa == SCHWA && r == R_VOCALIC => {
            segments.truncate(length - 2);
            segments.push(er);
        }
        [.., schwa, r, s] if schwa == SCHWA && r == R_VOCALIC && s == S => {
            segments.truncate(length - 3);
            segments.extend([er, Segment::new(S, Stress::Unstressed)]);
        }
        [.., before, r, schwa] if r == R && schwa == SCHWA => {
            segments.truncate(length - if before.is_vowel() { 2 } else { 1 });
            // After a long e it is ʌ without the glottal stop (studere stuðˈʔeʌ).
            segments.push(if before == E_LONG {
                Segment::new(ER_AFTER_E, Stress::Unstressed)
            } else {
                er
            });
        }
        _ => {}
    }
}

/// eSpeak NG's rhythmic stresses: every other syllable from the first up to two before the stress,
/// and any vowel but ə two syllables after it (ˌʔuniʋˌεɐ̯sitˈʔet, christian kʁˈʔistʔiˌan, arbejder
/// ˈɑb?ɑjdˌʔʌ).
pub(crate) fn rhythm(said: &mut Pronunciation) {
    let Some(stressed) = said
        .syllables
        .iter()
        .position(|syllable| syllable.stress == Stress::Primary)
    else {
        return;
    };
    for syllable in said.syllables.iter_mut().take(stressed.saturating_sub(1)).step_by(2) {
        syllable.stress = Stress::Secondary;
    }
    for syllable in said.syllables.iter_mut().skip(stressed + 2).step_by(2) {
        if syllable.nucleus.iter().all(|vowel| *vowel != SCHWA) {
            syllable.stress = Stress::Secondary;
        }
    }
    // An unstressed vowel is written without eSpeak NG's glottal stop (universitet: ni).
    for syllable in &mut said.syllables {
        if syllable.stress == Stress::Unstressed {
            for vowel in &mut syllable.nucleus {
                *vowel = unstressed_form(*vowel);
            }
        }
    }
}

/// A vowel as eSpeak NG writes it without the stress.
pub(crate) fn unstressed_form(vowel: Phoneme) -> Phoneme {
    [
        (I, I_PLAIN),
        (U, U_PLAIN),
        (O, O_PLAIN),
        (Y, Y_PLAIN),
        (E_LONG, E_PLAIN),
        (I_LONG, I_LONG_PLAIN),
        (U_LONG, U_LONG_PLAIN),
        (Y_LONG, Y_LONG_PLAIN),
        (O_CLOSE, O_CLOSE_PLAIN),
    ]
    .iter()
    .find(|(stressed, _)| *stressed == vowel)
    .map_or(vowel, |&(_, plain)| plain)
}

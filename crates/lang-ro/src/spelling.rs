//! How a Romanian word is said from its spelling: ă, â and î, ș and ț; c and g soft before e and
//! i, hard in che, chi, ghe and ghi; the diphthongs ea and oa, i and u as glides beside a vowel, a
//! final i after a consonant only softening it (lupi lupʲ); and, as eSpeak NG reads it, the stress
//! on the last syllable but one of a word ending in a vowel and on the last of one ending in a
//! consonant, the article -ul passed over (totul), with rhythmic stresses before it.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, B, CH, D, DZH, E, EA, F, G, H, I, J, K, L, M, N, NG, O, OA, OFFGLIDE_I, OFFGLIDE_U, P, R, R_TAP, S, SCHWA, SH,
    SOFT, SOFT_SHORT, T, TS, U, U_IN_OUA, V, W, Y, Y_BEFORE_I, Z, ZH, legal_onset,
};

fn is_vowel_letter(c: Option<char>) -> bool {
    matches!(c, Some('a' | 'ă' | 'â' | 'e' | 'i' | 'î' | 'o' | 'u'))
}

fn is_front(c: Option<char>) -> bool {
    matches!(c, Some('e' | 'i'))
}

/// The sounds of `word`'s letters (in lower case, without accents; `acute` is where an acute
/// accent marked the stress); `None` for a letter Romanian does not read.
fn letters(word: &[char], acute: Option<usize>) -> Option<Vec<Phoneme>> {
    let letter = |at: usize| word.get(at).copied();
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(word.len());
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        let next = letter(at + 1);
        // Whether the sound before is a vowel (an i said j, as in mediul, is not).
        let previous_vowel = sounds.last().is_some_and(|sound: &Phoneme| sound.is_vowel());
        let has_other_vowel = word
            .iter()
            .enumerate()
            .any(|(other, &c)| other != at && is_vowel_letter(Some(c)));
        let (phonemes, step): (&[Phoneme], usize) = match (c, next) {
            ('e', Some('a')) => (&[EA], 2),
            ('o', Some('a')) => (&[OA], 2),
            // eea is e, a glide and a (aceea atʃˈeɪa).
            ('e', Some('e')) if letter(at + 2) == Some('a') => (&[E, OFFGLIDE_I, A], 3),
            ('c' | 'g', Some('h')) if is_front(letter(at + 2)) => (if c == 'c' { &[K] } else { &[G] }, 2),
            // ci and gi before a, o and u: the i only softens (ciorbă, giuvaier).
            ('c' | 'g', Some('i')) if matches!(letter(at + 2), Some('a' | 'o' | 'u')) => {
                (if c == 'c' { &[CH] } else { &[DZH] }, 2)
            }
            ('c', _) if is_front(next) => (&[CH], 1),
            ('g', _) if is_front(next) => (&[DZH], 1),
            ('c' | 'k' | 'q', _) => (&[K], 1),
            ('g', _) => (&[G], 1),
            // i: a glide after a vowel (mai, câine) and before one (piață, iarnă), but a vowel of its
            // own when it bears the stress (Românía), in iei (Victoriei) and in a final iu (știu);
            // at the end after a consonant it only softens the consonant (lupi), shortly after -ur
            // and k or g (dealuri, ochi).
            ('i', _) if acute == Some(at) => (&[I], 1),
            ('i', Some('i')) if letter(at + 2) == Some('i') => (&[I, J, I], 3),
            ('i', Some(next)) if previous_vowel && is_vowel_letter(Some(next)) => (&[J], 1),
            ('i', Some('u')) if letter(at + 2).is_none() || is_vowel_letter(letter(at + 2)) => (&[I], 1),
            // A final oi ends in a full j, and so is stressed (apoi apˈoj).
            ('i', None) if at > 0 && letter(at - 1) == Some('o') => (&[J], 1),
            ('i', _) if previous_vowel => (&[OFFGLIDE_I], 1),
            // îi, the pronoun, is two vowels.
            ('î', Some('i')) if word.len() == 2 => (&[Y, I], 2),
            // -ie in a word of one syllable is i and e (fie, scrie).
            ('i', Some('e'))
                if at > 0 && letter(at + 2).is_none() && !word[..at].iter().any(|&c| is_vowel_letter(Some(c))) =>
            {
                (&[I], 1)
            }
            ('i', Some('e')) if letter(at + 2) == Some('i') => (&[I], 1),
            ('i', Some(next)) if is_vowel_letter(Some(next)) && next != 'i' => (&[J], 1),
            ('i', None) if has_other_vowel && at > 0 => {
                let short = matches!(letter(at - 1), Some('k' | 'g' | 'h' | 'c'))
                    || (at >= 2 && letter(at - 2) == Some('u') && letter(at - 1) == Some('r'));
                (if short { &[SOFT_SHORT] } else { &[SOFT] }, 1)
            }
            // u: a glide before a vowel (ziua), the second half of au, eu, ou and iu.
            // ouă is o, u and ă (două).
            ('u', Some('ă')) if at > 0 && letter(at - 1) == Some('o') => (&[U_IN_OUA], 1),
            // ou keeps its diphthong before a vowel (plouat).
            ('u', Some(_)) if at > 0 && letter(at - 1) == Some('o') => (&[OFFGLIDE_U], 1),
            ('u', Some(next)) if previous_vowel && is_vowel_letter(Some(next)) => (&[W], 1),
            ('u', _) if previous_vowel => (&[OFFGLIDE_U], 1),
            // â and î before i and u, which eSpeak NG writes y (câine, râul).
            ('â' | 'î', Some('i' | 'u')) => (&[Y_BEFORE_I], 1),
            // r between vowels is a tap, but not in -uri (dealuri).
            ('r', _)
                if previous_vowel
                    && is_vowel_letter(next)
                    && !(at > 0 && letter(at - 1) == Some('u') && next == Some('i') && letter(at + 2).is_none()) =>
            {
                (&[R_TAP], 1)
            }
            // ex before a vowel is eɡz (exemplu).
            ('x', Some(next)) if at == 1 && word[0] == 'e' && is_vowel_letter(Some(next)) => (&[G, Z], 1),
            // n is ŋ before g (lângă, mingea), but not before k (încă).
            ('n', Some('g')) => (&[NG], 1),
            _ => (
                match c {
                    'a' => &[A],
                    'ă' => &[SCHWA],
                    'â' | 'î' => &[Y],
                    'b' => &[B],
                    'd' => &[D],
                    'e' => &[E],
                    'f' => &[F],
                    'h' => &[H],
                    'i' | 'y' => &[I],
                    'j' => &[ZH],
                    'l' => &[L],
                    'm' => &[M],
                    'n' => &[N],
                    'o' => &[O],
                    'p' => &[P],
                    'r' => &[R],
                    's' => &[S],
                    'ș' | 'ş' => &[SH],
                    't' => &[T],
                    'ț' | 'ţ' => &[TS],
                    'u' => &[U],
                    'v' | 'w' => &[V],
                    'x' => &[K, S],
                    'z' => &[Z],
                    '\'' | '’' | '-' => &[],
                    _ => return None,
                },
                1,
            ),
        };
        sounds.extend_from_slice(phonemes);
        at += step;
    }
    Some(sounds)
}

/// Words in -ea stressed on it (cafea, avea); in other words a final -ea is the article, and the
/// stress the bare word's (curtea kˈurtea).
const FINAL_EA: [&str; 14] = [
    "cafea", "avea", "vedea", "putea", "bea", "stea", "perdea", "lalea", "mărgea", "părea", "plăcea", "tăcea", "cădea",
    "ședea",
];

/// Which nucleus (from 0) is stressed among `count`: the last but one when the word ends in a
/// vowel or a diphthong, the last when it ends in a consonant, in ai, ou and eu (tramvai, erou)
/// or in the -u of the past (făcu, văzu); the first in the teens (doisprezece), the last but one
/// in -em (mergem).
fn stressed(sounds: &[Phoneme], count: usize, word: &str) -> usize {
    if word.ends_with("sprezece") {
        return 0;
    }
    let last = sounds.last().copied();
    let vowel_end = last.is_some_and(|last| last.is_vowel() || [OFFGLIDE_I, OFFGLIDE_U].contains(&last));
    let letters: Vec<char> = word.chars().rev().take(3).collect();
    let past = matches!(letters[..], ['u', consonant, vowel] if !is_vowel_letter(Some(consonant)) && is_vowel_letter(Some(vowel)));
    let final_stress = !vowel_end
        || FINAL_EA.contains(&word)
        || last == Some(OFFGLIDE_U)
        || (count > 1 && (word.ends_with("ai") || past));
    if count < 2 || (final_stress && !word.ends_with("em")) {
        count.saturating_sub(1)
    } else {
        count - 2
    }
}

/// `c` without an acute accent, which marks the stress (jespersen's word list: așá), or a grave one,
/// which the older spelling writes on some final vowels (erà, făcù) and is read as the plain
/// vowel; and whether it had an acute accent.
fn plain(c: char) -> (char, bool) {
    match c {
        'á' => ('a', true),
        'é' => ('e', true),
        'í' => ('i', true),
        'ó' => ('o', true),
        'ú' => ('u', true),
        'à' => ('a', false),
        'è' => ('e', false),
        'ì' => ('i', false),
        'ò' => ('o', false),
        'ù' => ('u', false),
        _ => (c, false),
    }
}

/// How `word` is said; `None` for a letter Romanian does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let (chars, acute): (Vec<char>, Vec<bool>) = word.to_lowercase().chars().map(plain).unzip();
    let lower: String = chars.iter().collect();
    let acute_at = acute.iter().position(|&acute| acute);
    let sounds = letters(&chars, acute_at)?;
    if sounds.is_empty() {
        return None;
    }
    let nuclei: Vec<usize> = (0..sounds.len()).filter(|&at| sounds[at].is_vowel()).collect();
    if nuclei.is_empty() {
        let segments: Vec<Segment> = sounds
            .iter()
            .map(|&sound| Segment::new(sound, Stress::Unstressed))
            .collect();
        return Some(syllabify(&segments, legal_onset));
    }
    // The articles -ul and -ului after a consonant are passed over: the stress is the bare word's
    // (totul tˈotul, avionul, împăratului; but biroul).
    let after_consonant = |end: usize| {
        chars.len() > end
            && (!is_vowel_letter(chars.get(chars.len() - end - 1).copied()) || chars[chars.len() - end - 1] == 'i')
    };
    // The letters of the article and its vowels.
    let article = if lower.ends_with("ului") && after_consonant(4) && nuclei.len() > 2 {
        Some((4, 2))
    } else if lower.ends_with("ul") && after_consonant(2) && nuclei.len() > 1 {
        Some((2, 1))
    } else {
        None
    };
    // An acute accent marks the stressed vowel: the vowels said up to it.
    let marked = acute_at
        .and_then(|at| letters(&chars[..=at], acute_at))
        .map(|before| before.iter().filter(|sound| sound.is_vowel()).count());
    let stress = if let Some(marked) = marked.filter(|&marked| marked > 0) {
        (marked - 1).min(nuclei.len() - 1)
    } else if let Some((length, vowels)) = article {
        let base: String = chars[..chars.len() - length].iter().collect();
        stressed(&sounds[..sounds.len() - length], nuclei.len() - vowels, &base)
    } else {
        stressed(&sounds, nuclei.len(), &lower)
    };
    let stressed_at = nuclei[stress];
    let segments: Vec<Segment> = sounds
        .iter()
        .enumerate()
        .map(|(at, &sound)| {
            let stress = if at == stressed_at {
                Stress::Primary
            } else {
                Stress::Unstressed
            };
            Segment::new(sound, stress)
        })
        .collect();
    let mut said = syllabify(&segments, legal_onset);
    // eSpeak NG's rhythmic stresses: every other syllable from the first, up to two before the
    // stress (ˌakopeɾˈiʃul, ˌunivˌersitˈatea).
    let primary = said
        .syllables
        .iter()
        .position(|syllable| syllable.stress == Stress::Primary);
    if let Some(primary) = primary {
        for syllable in said.syllables.iter_mut().take(primary.saturating_sub(1)).step_by(2) {
            syllable.stress = Stress::Rhythmic;
        }
    }
    // The teens take a secondary stress on -ze- (doisprezece dˈoɪsprezˌetʃe).
    let count = said.syllables.len();
    if lower.ends_with("sprezece")
        && count >= 2
        && let Some(syllable) = said.syllables.get_mut(count - 2)
    {
        syllable.stress = Stress::Rhythmic;
    }
    Some(said)
}

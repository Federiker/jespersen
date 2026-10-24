//! How an Icelandic word is said from its spelling: the stress on the first syllable, its vowel
//! long before one consonant (or p, t, k or s and j, r or v) and short before two; á, é, ó, ú, æ,
//! au, ei and ey; p, t and k aspirated at the start and unaspirated elsewhere, preaspirated when
//! doubled and in tl and tn; k and g palatal before a front vowel, g as ɣ, j or x after a vowel;
//! ll as tl and nn as tn after a long vowel; l, n and r voiceless at the end and before a voiceless
//! consonant as eSpeak NG writes them; hv as kv, hj as ç, hl, hn and hr voiceless.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, A_LONG, AI, AI_LONG, AU, AU_LONG, C, C_ASP, C_PLAIN, CC, DH, E, E_LONG, EI, EI_LONG, F, GH, H, I, I_LONG, II,
    II_LONG, J, K, K_ASP, K_PLAIN, L, L_VOICELESS, M, M_VOICELESS, N, N_VOICELESS, N_VOICELESS_H, NG, NG_VOICELESS, NY,
    NY_VOICELESS, O, O_LONG, OE, OE_LONG, OEY, OEY_LONG, OI, OU, OU_LONG, P, P_ASP, PREASPIRATION, R, R_BETWEEN,
    R_VOICELESS, R_VOICELESS_H, S, SCHWA, T, T_ASP, T_PLAIN, TH, U, U_LONG, UU, UU_LONG, V, X, doubled, legal_onset,
};

fn is_vowel(c: Option<char>) -> bool {
    matches!(
        c,
        Some('a' | 'á' | 'e' | 'é' | 'i' | 'í' | 'o' | 'ó' | 'u' | 'ú' | 'y' | 'ý' | 'æ' | 'œ' | 'ö')
    )
}

/// The front vowels and j, before which k and g are palatal (kenna, gefa, kjöt).
fn is_front(c: Option<char>) -> bool {
    matches!(c, Some('e' | 'é' | 'i' | 'í' | 'y' | 'ý' | 'æ' | 'j'))
}

/// The vowels that are long by nature or diphthongs, after which nn is said tn (einn, steinn).
fn is_long_kind(word: &[char], at: usize) -> bool {
    matches!(word[at], 'á' | 'é' | 'í' | 'ó' | 'ú' | 'ý' | 'æ' | 'œ')
        || (at > 0 && matches!(word[at - 1..=at], ['a', 'u'] | ['e', 'i' | 'y']))
}

/// The voiceless consonants, before which l and r lose their voice (kalt, kort).
fn is_voiceless(c: Option<char>) -> bool {
    matches!(c, Some('p' | 't' | 'k' | 'f' | 's'))
}

/// A vowel's phoneme, long or short.
fn vowel_phoneme(word: &[char], at: usize, long: bool) -> (Phoneme, usize) {
    let next = word.get(at + 1).copied();
    let pick = |short: Phoneme, lengthened: Phoneme| if long { lengthened } else { short };
    match (word[at], next) {
        ('a', Some('u')) => (pick(OEY, OEY_LONG), 2),
        ('e', Some('i' | 'y')) => (pick(EI, EI_LONG), 2),
        ('a', _) => (pick(A, A_LONG), 1),
        ('á', _) => (pick(AU, AU_LONG), 1),
        ('e' | 'é', _) => (pick(E, E_LONG), 1),
        ('i' | 'y', _) => (pick(I, I_LONG), 1),
        ('í' | 'ý', _) => (pick(II, II_LONG), 1),
        ('o', _) => (pick(O, O_LONG), 1),
        ('ó', _) => (pick(OU, OU_LONG), 1),
        ('u', _) => (pick(U, U_LONG), 1),
        ('ú', _) => (pick(UU, UU_LONG), 1),
        // œ, in the older spelling, is æ (bœklinga).
        ('æ' | 'œ', _) => (pick(AI, AI_LONG), 1),
        _ => (pick(OE, OE_LONG), 1),
    }
}

/// The vowel a short one is said as before ng and nk: a as au, e as ei, i, y and u as their accented
/// vowels, o as ou and ö as au (langur, lengi, ungur).
fn before_ng_phoneme(c: char) -> Option<Phoneme> {
    Some(match c {
        'a' => AU,
        'e' => EI,
        'i' | 'y' => II,
        'o' => OU,
        'u' => UU,
        'ö' => OEY,
        _ => return None,
    })
}

/// The diphthong a, e or o becomes before a g said j, long or short.
fn before_gi_phoneme(c: char, long: bool) -> Option<Phoneme> {
    Some(match (c, long) {
        ('a', false) => AI,
        ('a', true) => AI_LONG,
        ('e', false) => EI,
        ('e', true) => EI_LONG,
        ('o', _) => OI,
        _ => return None,
    })
}

/// Whether the stressed vowel ending at `end` is long: before no consonant or one, or before p, t,
/// k or s and j, r or v (hús, akra, sitja); short before two (barn, ekki). x counts as two.
fn long_before(word: &[char], end: usize) -> bool {
    let cluster: Vec<char> = word[end..]
        .iter()
        .copied()
        .take_while(|&c| !is_vowel(Some(c)))
        .collect();
    match cluster[..] {
        ['x', ..] => false,
        [] | [_] => true,
        ['p' | 't' | 'k' | 's', 'j' | 'r' | 'v'] => is_vowel(word.get(end + 2).copied()),
        _ => false,
    }
}

/// The phonemes of `word`'s letters, in lower case, stressed on the first vowel; `None` for a
/// letter Icelandic does not read.
#[expect(clippy::too_many_lines, reason = "one arm per spelling, read in order")]
fn segments(word: &[char]) -> Option<Vec<Segment>> {
    let letter = |at: usize| word.get(at).copied();
    let mut segments: Vec<Segment> = Vec::with_capacity(word.len() + 2);
    let mut stressed = false;
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        let next = letter(at + 1);
        let after = letter(at + 2);
        let before = at.checked_sub(1).and_then(letter);
        let start = at == 0;
        let vowel_next = is_vowel(next);
        let push = |segments: &mut Vec<Segment>, phonemes: &[Phoneme]| {
            segments.extend(
                phonemes
                    .iter()
                    .map(|&phoneme| Segment::new(phoneme, Stress::Unstressed)),
            );
        };
        if c == 'é' {
            push(&mut segments, &[J]);
        }
        if is_vowel(Some(c)) {
            let length = if (c == 'a' && next == Some('u')) || (c == 'e' && matches!(next, Some('i' | 'y'))) {
                2
            } else {
                1
            };
            // An unstressed e before one consonant and a vowel is long too (fallegur fˈadlɛːɣyr).
            let open_e =
                stressed && c == 'e' && length == 1 && long_before(word, at + 1) && is_vowel(word.get(at + 2).copied());
            let long = (!stressed && long_before(word, at + length)) || open_e;
            // Before ng and nk a vowel is said as its accented one (konungur kˈɔːnuŋɡyr, langur).
            let before_ng = matches!(word.get(at + length..at + length + 2), Some(['n', 'g' | 'k']));
            // Before gi, where the g is said j, a, e and o end in i (segir sˈeɪːjɪr, bogi).
            let before_gi = !stressed
                && length == 1
                && word.get(at + 1) == Some(&'g')
                && word.get(at + 2) == Some(&'i')
                && word.get(at + 3).is_none_or(|&c| c != 'g');
            let (phoneme, step) = match (before_ng_phoneme(c), before_gi_phoneme(c, long)) {
                (Some(phoneme), _) if before_ng && length == 1 => (phoneme, 1),
                (_, Some(phoneme)) if before_gi => (phoneme, 1),
                _ => vowel_phoneme(word, at, long),
            };
            let stress = if stressed { Stress::Unstressed } else { Stress::Primary };
            segments.push(Segment::new(phoneme, stress));
            stressed = true;
            at += step;
            continue;
        }
        let doubled_letter = next == Some(c);
        // The ə eSpeak NG says between a stop, f or þ and an r before a vowel or j (brauð, akra,
        // þrjú; but liggr).
        if c == 'r'
            && matches!(before, Some('p' | 't' | 'k' | 'b' | 'd' | 'g' | 'f' | 'þ'))
            && (vowel_next || next == Some('j'))
        {
            push(&mut segments, &[SCHWA]);
        }
        let (phonemes, step): (Vec<Phoneme>, usize) = match c {
            'h' if start && next == Some('v') => (vec![K_ASP, V], 2),
            'h' if next == Some('j') => (vec![CC], 2),
            'h' if start && next == Some('l') => (vec![L_VOICELESS], 2),
            'h' if start && next == Some('n') => (vec![N_VOICELESS_H], 2),
            'h' if start && next == Some('r') => (vec![R_VOICELESS_H], 2),
            'h' => (vec![H], 1),
            'k' if start && next == Some('v') => (vec![K, V], 2),
            'k' if start && next == Some('j') => (vec![C_ASP, J], 2),
            'k' if start && is_front(next) => (vec![C_ASP], 1),
            'k' if start => (vec![K_ASP], 1),
            'k' if doubled_letter => (vec![PREASPIRATION, if is_front(after) { C } else { K }], 2),
            'k' if next == Some('j') => (vec![C_PLAIN], 2),
            'g' if start && next == Some('j') => (vec![C], 2),
            'g' if start && is_front(next) => (vec![C], 1),
            'g' if start => (vec![K], 1),
            'g' if doubled_letter => (vec![if is_front(after) && after != Some('j') { C } else { K }], 2),
            // g after r is x at the end and before t and s (margt mˈarxd, borg).
            'g' if before == Some('r') && (next.is_none() || matches!(next, Some('t' | 's'))) => (vec![X], 1),
            'g' if before == Some('n') => (vec![if is_front(next) { C } else { K }], 1),
            'g' if is_vowel(before) && matches!(next, Some('i' | 'j')) => (vec![J], 1),
            'g' if is_vowel(before) && (vowel_next || matches!(next, Some('ð' | 'r'))) => (vec![GH], 1),
            'g' if is_vowel(before) && (next.is_none() || matches!(next, Some('t' | 's'))) => (vec![X], 1),
            'g' | 'k' if is_front(next) => (vec![C], 1),
            'p' if start => (vec![P_ASP], 1),
            'p' if doubled_letter => (vec![PREASPIRATION, P], 2),
            't' if start => (vec![T_ASP], 1),
            't' if doubled_letter => (vec![PREASPIRATION, T], 2),
            // tl and tn are preaspirated (litla lˈɪhdla, vatn ʋˈahdhn#).
            't' if matches!(next, Some('l' | 'n')) && at > 0 => (vec![PREASPIRATION, T], 1),
            'p' => (vec![P], 1),
            't' | 'd' => (vec![T], if doubled_letter { 2 } else { 1 }),
            'b' => (vec![P], if doubled_letter { 2 } else { 1 }),
            'f' if start => (vec![F], 1),
            'f' if doubled_letter => (vec![F], 2),
            // f is p before l and n (tafla tˈabla, höfn), v between vowels and before r and ð.
            'f' if matches!(next, Some('l' | 'n')) => (vec![P], 1),
            'f' if is_vowel(before) && (vowel_next || matches!(next, Some('r' | 'ð' | 'j'))) => (vec![V], 1),
            'f' => (vec![F], 1),
            'v' | 'w' => (vec![V], 1),
            's' if doubled_letter && (is_vowel(after) || after.is_none()) => (vec![S, doubled(S)], 2),
            's' if doubled_letter => (vec![S], 2),
            's' | 'z' => (vec![S], 1),
            'þ' => (vec![TH], 1),
            'ð' => (vec![DH], 1),
            'j' => (vec![J], 1),
            'x' => (vec![K_PLAIN, S], 1),
            'c' | 'g' | 'k' | 'q' => (vec![K], 1),
            // ll is tl: voiced before a vowel or j (fullur, milljón), voiceless otherwise (bíll).
            'l' if doubled_letter && (is_vowel(after) || after == Some('j')) => (vec![T, L], 2),
            'l' if doubled_letter => (vec![T_PLAIN, L_VOICELESS], 2),
            // A final l, and an l before a final voiceless consonant, is voiceless (sól, kalt, hjálp).
            'l' if next.is_none() || (is_voiceless(next) && after.is_none()) => (vec![L_VOICELESS], 1),
            'l' => (vec![L], 1),
            'm' if doubled_letter && is_vowel(after) => (vec![M, doubled(M)], 2),
            'm' if doubled_letter => (vec![M], 2),
            // m and n before p, t and k are voiceless (lampi lˈam#bɪ, vant ʋˈan#d, banki).
            'm' if matches!(next, Some('p' | 't' | 'k')) => (vec![M_VOICELESS], 1),
            'm' => (vec![M], 1),
            // nn is tn after a long vowel or a diphthong (einn ˈeɪdn), long between vowels.
            'n' if doubled_letter && at > 0 && is_long_kind(word, at - 1) => (vec![T, N], 2),
            'n' if doubled_letter && is_vowel(after) => (vec![N, doubled(N)], 2),
            'n' if doubled_letter => (vec![N], 2),
            'n' if next == Some('k') && is_front(after) => (vec![NY_VOICELESS], 1),
            'n' if next == Some('k') => (vec![NG_VOICELESS], 1),
            'n' if next == Some('g') && is_front(after) => (vec![NY], 1),
            'n' if next == Some('g') => (vec![NG], 1),
            'n' if matches!(next, Some('t' | 'p')) => (vec![N_VOICELESS], 1),
            'n' if matches!(before, Some('t' | 'p' | 'f')) => (vec![N_VOICELESS_H], 1),
            'n' => (vec![N], 1),
            // rn is rtn̥ (barn bˈardn#), r voiceless before a voiceless stop and s (kort, fersk),
            // and between vowels ɾr in eSpeak NG's notation (fara fˈaːɾra, kennari).
            'r' if next == Some('n') => (vec![R, T, N_VOICELESS], 2),
            'r' if doubled_letter => (vec![R], 2),
            'r' if matches!(next, Some('p' | 't' | 'k' | 's')) => (vec![R_VOICELESS], 1),
            'r' if vowel_next && is_vowel(before) => (vec![R_BETWEEN], 1),
            'r' => (vec![R], 1),
            '\'' | '’' | '-' => (Vec::new(), 1),
            _ => return None,
        };
        push(&mut segments, &phonemes);
        at += step;
    }
    Some(segments)
}

/// How `word` is said; `None` for a letter Icelandic does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    // A macron, in foreign names, leaves a plain vowel (Unalāshka).
    let chars: Vec<char> = word
        .to_lowercase()
        .chars()
        .map(|c| if c == 'ā' { 'a' } else { c })
        .collect();
    let segments = segments(&chars)?;
    if segments.is_empty() {
        return None;
    }
    Some(syllabify(&segments, legal_onset))
}

//! How an Irish word is said from its spelling: the stress on the first syllable (or on a long
//! second one after a short first, as in the south: foláir); a consonant broad or slender as the
//! vowel next to it is a, o, u or e, i; the vowel groups (ao, ea, ei, ia, io, oi, ua, ui, aoi, eoi,
//! uai…), with the síneadh fada marking a long vowel; lenition (bh, ch, dh, fh, gh, mh, ph, sh,
//! th) and eclipsis (mb, gc, nd, bhf, ng, bp, dt) at the start of a word; a or o before bh and mh
//! as aʊ, a before dh and gh and a consonant as aɪ; the unstressed vowels as ə, ɐ and ɪ; the
//! final -adh, -igh and -idh; and the ə between l, n or r and a following b, ch, g or m (orm
//! ˈɔɹəm).

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, A_BACK, AA, AI, AU, B, BJ, D, D_PLAIN, DH, DH_FINAL, DJ, DJ_PLAIN, DZH, E, EE, F, FJ, G, GH, GJ, GJ_PLAIN,
    GLIDE, H, HJ, I, IA, II, J, K, KJ, KJ_PLAIN, L, LJ, LJ_PLAIN, M, MJ, N, NG, NJ, NJ_PLAIN, O, OO, P, PJ, R,
    R_APPROX, RJ, S, SCHWA, SCHWA_A, SH, T, TJ, TSH, U, U_TENSE, UA, UU, V, VJ, W, X, XJ, XJ_PLAIN, legal_onset,
};

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'á' | 'é' | 'í' | 'ó' | 'ú')
}

/// Whether a vowel group carries a síneadh fada.
fn is_long(group: &[char]) -> bool {
    group.iter().any(|c| matches!(c, 'á' | 'é' | 'í' | 'ó' | 'ú'))
}

/// e and i make the consonants next to them slender.
fn is_front(c: char) -> bool {
    matches!(c, 'e' | 'i' | 'é' | 'í')
}

/// A run of letters: vowels said as one vowel, or the consonants between two such runs.
#[derive(Clone, Debug)]
struct Run {
    letters: Vec<char>,
    vowel: bool,
}

fn runs(word: &[char]) -> Vec<Run> {
    let mut runs: Vec<Run> = Vec::new();
    for &c in word {
        match runs.last_mut() {
            Some(run) if run.vowel == is_vowel(c) => run.letters.push(c),
            _ => runs.push(Run {
                letters: vec![c],
                vowel: is_vowel(c),
            }),
        }
    }
    runs
}

/// What surrounds a vowel group: the consonants before and after it (empty at the edges of the
/// word) and whether those after end the word.
#[expect(
    clippy::struct_excessive_bools,
    reason = "independent facts about where the group is"
)]
struct Around<'a> {
    before: &'a [char],
    after: &'a [char],
    last: bool,
    /// Whether the group comes before the stressed one.
    pretonic: bool,
    /// The vowel group after the consonants after it, if any.
    then: &'a [char],
    /// Whether the consonants before it are only a t, n or h a word before adds (i n-aon).
    prefixed: bool,
    /// Whether the stressed group is long (or ia, ua), and whether a diphthong took in all the
    /// consonants before this group (domhan d̪ˈaʊən).
    stress_long: bool,
    hiatus: bool,
}

/// A short a (of a, ai or ea) before `after`: eSpeak NG's back A before most single consonants,
/// ch, ng and r with c, n or s, its front a before n and m at the end, nn, rr, r with another
/// consonant, and the lenited consonants (mac mˈAk, glas ɡlˈAs, rang, marcach; bean bˈan, fear
/// fˈar, ainm, ceart).
fn short_a(group: &[char], around: &Around) -> Phoneme {
    let after = around.after;
    let back = match group {
        ['e', 'a'] => matches!(after, ['c', 'h', ..]),
        ['a', 'i'] => matches!(after, ['d' | 'l' | 'b', ..] | ['t'] | ['t', 'h', ..]),
        _ => match after {
            [_, 'h', ..] => matches!(after, ['c' | 't', ..]),
            // Also before r and the consonant a ə comes before (arm ˈAɹəm).
            // And before a final n (glan ɡlˈAn, gan).
            ['r' | 'n'] | ['r', 'b' | 'g' | 'm' | 's' | 'c' | 'n', ..] | ['n', 'g', ..] => true,
            ['m'] | ['l', 'l'] if !around.last => true,
            [] | ['l', 'l', ..] | ['r' | 'n' | 'm', ..] => false,
            _ => true,
        },
    };
    if back { A_BACK } else { A }
}

/// The vowel of a stressed group without a síneadh fada, and how many of the consonants after it
/// it takes in (bh, mh, dh and gh that make a diphthong).
fn stressed_short(group: &[char], around: &Around) -> (Vec<Phoneme>, usize) {
    let after = around.after;
    // a or o before bh or mh is aʊ (labhair, domhan, leabhar), a before dh or gh and another
    // consonant aɪ (Tadhg), ai before nt, ms or ll aɪ (caint, aimsir, caillte).
    // But ea before mh is a and v (neamh nʲˈav).
    if matches!(group, ['a' | 'o'] | ['e', 'a'])
        && matches!(after, ['b' | 'm', 'h', ..])
        && !(group == ['e', 'a'] && after[0] == 'm')
    {
        return (vec![AU], 2);
    }
    if matches!(group, ['a'] | ['a', 'i'])
        && matches!(after, ['d' | 'g', 'h', ..])
        && !(around.last && after.len() == 2)
    {
        return (vec![AI], 2);
    }
    // o before dh or gh is aʊ (foghluim, bogha).
    if group == ['o'] && matches!(after, ['d' | 'g', 'h', ..]) {
        return (vec![AU], 2);
    }
    if matches!(group, ['a', 'i']) && matches!(after, ['n', 't', ..] | ['m', 's', ..] | ['l', 'l', ..]) {
        return (vec![AI], 0);
    }
    // a before a final m, nn or ll, and o before a final nn, is aʊ in the south (am, ann, ceann,
    // gheall; conn).
    if around.last
        && (matches!(group, ['a'] | ['e', 'a']) && matches!(after, ['m'] | ['m', 'm'] | ['n', 'n'] | ['l', 'l'])
            || group == ['o'] && after == ['n', 'n'])
    {
        return (vec![AU], 0);
    }
    // a and o before a final rd are long (bord bˠoːrd̪, ard).
    if around.last && after == ['r', 'd'] {
        match group {
            ['a'] => return (vec![AA], 0),
            ['o'] => return (vec![OO], 0),
            _ => {}
        }
    }
    if let Some(vowels) = ao(group, around) {
        return (vowels, 0);
    }
    let velar = matches!(around.before, [.., 'c' | 'g'] | [.., 'c' | 'g', 'h']);
    let vowel = match group {
        ['a'] | ['e', 'a'] | ['a', 'i'] => short_a(group, around),
        ['e', 'o', ..] => {
            if matches!(after, ['c', 'h', ..]) {
                O
            } else {
                OO
            }
        }
        ['a', 'e'] => EE,
        // iai is iː and a glide (bhriain vrˈiːi̯n, ndiaigh), uai before g, dh or gh u and a
        // glide (uaidh ˈui̯ɡ, ghruaig).
        ['i', 'a', 'i'] => return (vec![II, GLIDE], 0),
        ['u', 'a', 'i'] if matches!(after, ['g', ..] | ['d', 'h', ..]) => return (vec![U_TENSE, GLIDE], 0),
        ['i', 'a', ..] => IA,
        // A final ua is uːə (nua nˈuːə).
        ['u', 'a'] if after.is_empty() => return (vec![UU, SCHWA], 0),
        ['u', 'a', ..] => UA,
        ['i', 'u'] | ['u'] => U,
        ['i', 'o'] => {
            if matches!(after, ['s', ..]) {
                I
            } else {
                O
            }
        }
        ['o', 'i'] => match after {
            ['r'] if around.last => E,
            ['r', _, ..] | ['l' | 's', ..] => O,
            // oi after c or g has a ə before it (coimeád kəˈɪmʲɑːd̪, gcoinnibh).
            _ if velar => return (vec![SCHWA, I], 0),
            _ => I,
        },
        // ui after a broad labial or velar has a ə before it (cuid kəˈɪdʒ, chuir xəˈɪr), but not
        // after the eclipsis bhf (bhfuil wˈɪl).
        ['u', 'i']
            if (matches!(around.before.last(), Some('b' | 'c' | 'f' | 'g' | 'm' | 'p'))
                || matches!(around.before, [.., 'b' | 'c' | 'd' | 'f' | 'g' | 'm', 'h']))
                && around.before != ['b', 'h', 'f'] =>
        {
            return (vec![SCHWA, I], 0);
        }
        ['e'] | ['e', 'i'] => E,
        ['o'] => O,
        _ => I,
    };
    (vec![vowel], 0)
}

/// ao and aoi: eː and iː, with a ə before them after a consonant, a w after g, m or p (caol
/// kəˈeːəl, caoi kəˈiː, gaol ɡwˈeːəl), and ao a ə after it before a sounded consonant (aon
/// ˈeːən; gaoth ɡəˈeː).
fn ao(group: &[char], around: &Around) -> Option<Vec<Phoneme>> {
    let vowel = match group {
        ['a', 'o'] => EE,
        ['a', 'o', 'i'] => II,
        _ => return None,
    };
    let mut vowels: Vec<Phoneme> = Vec::with_capacity(3);
    match around.before {
        // Nothing before ao, or only the n, t or h a word before adds (i n-aon, a haon əhˈeːn).
        ['n' | 't'] if around.prefixed => {}
        [] | ['h'] => {}
        [.., 'b' | 'g' | 'm' | 'p'] | [.., 'b' | 'g' | 'm', 'h'] => vowels.push(W),
        _ => vowels.push(SCHWA),
    }
    vowels.push(vowel);
    if vowel == EE && !around.after.is_empty() && !(around.last && matches!(around.after, ['t', 'h'])) {
        vowels.push(SCHWA);
    }
    Some(vowels)
}

/// The long vowel of a group with a síneadh fada (á ɑː, é eː, í iː, ó oː, ú uː), with the glide
/// i̯ after á and ó before a slender consonant (amháin, dóibh).
fn long(group: &[char], around: &Around) -> Option<Vec<Phoneme>> {
    let accented = group.iter().find(|c| matches!(c, 'á' | 'é' | 'í' | 'ó' | 'ú'))?;
    let vowel = match accented {
        'á' => AA,
        'é' => EE,
        'í' => II,
        'ó' => OO,
        _ => UU,
    };
    // úi before g has the glide too (cúige kˈuːi̯ɡʲə).
    let glide = group.last() == Some(&'i')
        && !around.after.is_empty()
        && (matches!(accented, 'á' | 'ó') && !(around.after == ['n'] && around.then.first() == Some(&'i'))
            || *accented == 'ú' && around.after[0] == 'g');
    Some(match group {
        _ if glide => vec![vowel, GLIDE],
        // ío before a broad consonant is iːə (níor nʲˈiːər, bhíodh vˈiːəχ).
        ['í', 'o'] if !around.after.is_empty() && !matches!(around.after, ['m', 'h', ..]) => vec![II, SCHWA],
        // The old spelling's eó is əoː (beó bəˈoː, eólus).
        ['e', 'ó'] => vec![SCHWA, OO],
        _ => vec![vowel],
    })
}

/// The vowel of an unstressed group: ɪ for i, ei, oi and ui and for ai before nn, ng, dh, gh,
/// th, s and c (againn, tháinig, turais), ə for ai before r after th, bh or mh and before m
/// (athair, measaim), ɐ for other ai and for a or ea before a consonant (obair, seisean), ə
/// otherwise (arsa, eile, agus). Before the stress a short group is ɐ if the stressed one is
/// long (foláir fɐlˈɑːi̯r, fadó) and ə if short (amach əmˈAx).
fn unstressed(group: &[char], around: &Around) -> Phoneme {
    let after = around.after;
    match group {
        ['a', 'o', ..] => EE,
        ['e', 'o', ..] => OO,
        ['i', 'a', ..] => IA,
        ['u', 'a', ..] => UA,
        ['o'] | ['e', 'a'] | ['a' | 'o', 'i'] if around.pretonic && around.stress_long => SCHWA_A,
        ['a', 'i'] | ['e', 'a', 'i'] if matches!(after, ['r', ..]) => {
            if matches!(around.before, [.., 't' | 'b' | 'm', 'h']) {
                SCHWA
            } else {
                SCHWA_A
            }
        }
        ['a', 'i'] | ['e', 'a', 'i'] if !after.is_empty() => match after {
            ['n', 'n' | 'g', ..] | ['d' | 'g' | 't', 'h', ..] | ['s' | 'c'] => I,
            ['m'] => SCHWA,
            _ => SCHWA_A,
        },
        ['e', 'i'] if around.pretonic => E,
        ['o', 'i'] if around.pretonic => O,
        ['i'] | ['e' | 'o' | 'u' | 'a', 'i'] | ['e', 'a', 'i'] | ['i', 'o'] => I,
        // After an aʊ that took in the consonants, a is ə (domhan d̪ˈaʊən; leabhar).
        ['a'] | ['e', 'a'] if !after.is_empty() && !around.pretonic && (!around.hiatus || after[0] == 'r') => SCHWA_A,
        _ => SCHWA,
    }
}

/// The consonants of a run in `word` (lower case, without its letters skipped by a diphthong),
/// `slender` or broad, at the `initial` or `last` place of the word.
#[expect(clippy::struct_excessive_bools, reason = "independent facts about where the run is")]
struct Consonants<'a> {
    letters: &'a [char],
    slender: bool,
    initial: bool,
    last: bool,
    /// The vowel group after them, if any.
    next: Option<&'a [char]>,
    /// The consonants after the vowel group after them, if any.
    then: &'a [char],
    /// Whether the vowel after them is stressed.
    next_stressed: bool,
    /// Whether they hold the f of a future or conditional ending (-fadh, -fidh, -fainn…).
    future: bool,
    /// What comes before them: a long vowel, a long stressed one ending in á, ó, ú or i (rádh,
    /// réidh), a vowel group with a (a, ai, ea) or éi, and an unstressed vowel after r, n or l
    /// (cuireadh).
    after_long: bool,
    after_long_open: bool,
    after_a: bool,
    after_sonorant: bool,
    /// Whether the word starts with an elided d' (d'ól).
    elided: bool,
}

/// Eclipsis and the t before s at the start of a word: mb m, gc ɡ, nd n, bhf v, ng ŋ, bp b, dt d,
/// ts t (mbaile, gcathair, bhfear, dtí, tsaol), and the letters left after them.
fn eclipsis(letters: &[char]) -> Option<(Phoneme, usize)> {
    match letters {
        ['b', 'h', 'f', ..] => Some((V, 3)),
        ['m', 'b', ..] => Some((M, 2)),
        ['g', 'c', ..] => Some((G, 2)),
        ['n', 'd', ..] => Some((N, 2)),
        ['n', 'g', ..] => Some((NG, 2)),
        ['b', 'p', ..] => Some((B, 2)),
        ['d', 't', ..] => Some((D, 2)),
        ['t', 's', ..] => Some((T, 2)),
        _ => None,
    }
}

/// A consonant `slender` (marked or, where eSpeak NG leaves its ʲ out, `plain`) or `broad`.
fn plain_at_end(soft: bool, unmarked: bool, slender: Phoneme, plain: Phoneme, broad: Phoneme) -> Phoneme {
    match (soft, unmarked) {
        (true, false) => slender,
        (true, true) => plain,
        (false, _) => broad,
    }
}

/// The phonemes of a consonant run.
#[expect(
    clippy::too_many_lines,
    reason = "one match over the consonant letters, kept together"
)]
fn consonants(run: &Consonants) -> Vec<Phoneme> {
    let letters = run.letters;
    let slender = run.slender;
    let mut phonemes: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    if run.initial
        && let Some((phoneme, step)) = eclipsis(letters)
    {
        // Slender only right before its vowel (gcreideamh ɡrˈɛdʒɐv).
        let soft = slender && step == letters.len();
        let front = |phoneme: Phoneme| match phoneme {
            phoneme if phoneme == G && soft => GJ,
            phoneme if phoneme == D && soft => DZH,
            phoneme if phoneme == N && soft => NJ,
            phoneme => phoneme,
        };
        // bhf before ui is w (bhfuil wˈɪl).
        let ui = matches!(run.next, Some(['u', 'i', ..]));
        phonemes.push(if phoneme == V && ui { W } else { front(phoneme) });
        at = step;
    }
    while at < letters.len() {
        let c = letters[at];
        let next = letters.get(at + 1).copied();
        let first = at == 0 && run.initial;
        let end = at + 1 >= letters.len() || (next == Some('h') && at + 2 >= letters.len());
        let final_ = end && run.last;
        // eSpeak NG marks a slender k, g, l, n or ch only next to its vowel: the last consonant
        // before one, the first after the last (rince ɹˈɪnkʲə, aigne ˈaɡnʲə).
        // At the end of a word, only a consonant on its own (bhail vˈAlʲ; caint kˈaɪntʃ).
        let soft = slender
            && if run.next.is_some() {
                end
            } else {
                phonemes.is_empty() && end
            };
        // Before e, é and ei, a slender l or n is plain in its notation (leis lˈɛʃ, léir), and d
        // before e and ei (de dˈɛ; dé dʲˈeː).
        let before_e = end && matches!(run.next, Some(['e' | 'é'] | ['e' | 'é', 'i' | 'í' | 'o' | 'ó', ..]));
        let before_short_e = end && run.next == Some(&['e'][..]);
        // A slender labial before a back vowel (eá, io, iu, iú) is marked with ʲ in eSpeak NG's
        // notation (fiú fʲˈuː, an mheán vʲˈɑːn).
        let labial_glide = soft
            && run.next.is_some()
            && matches!(run.next, Some(['i', 'á' | 'o' | 'ó' | 'u' | 'ú', ..] | ['e', 'á', ..]))
            && !(run.next == Some(&['i', 'o'][..]) && matches!(run.then, ['s', ..]));
        let (phoneme, step): (Option<Phoneme>, usize) = match (c, next) {
            // bh and mh between vowels before a stressed broad one are w (abhaile, amháin).
            ('b' | 'm', Some('h'))
                if !run.initial
                    && letters.len() == 2
                    && run.next_stressed
                    && run.next.is_some_and(|next| !is_front(next[0])) =>
            {
                (Some(W), 2)
            }
            // A final broad bh is doubled in eSpeak NG's notation (dubh d̪ˈʊvv, scríobh).
            ('b', Some('h')) if final_ && !slender && !run.initial => {
                phonemes.push(V);
                (Some(V), 2)
            }
            ('b' | 'm', Some('h')) if labial_glide => (Some(VJ), 2),
            ('b' | 'm', Some('h')) => (Some(V), 2),
            ('c', Some('h')) => (Some(plain_at_end(soft, final_, XJ, XJ_PLAIN, X)), 2),
            // dh and gh: j slender at the start of a word, ʁ broad and inside a word (dhéanamh,
            // ghá, rígheacht), a slender dh inside one dʒh in eSpeak NG's notation (oídhche).
            ('d', Some('h')) if slender && !first && !final_ && run.next.is_some() && at + 2 >= letters.len() => {
                (Some(J), 2)
            }
            ('d', Some('h')) if slender && !first && !final_ => {
                phonemes.push(DZH);
                (Some(H), 2)
            }
            // At the end of a word, dh and gh are silent after a long stressed á, ó, ú or i (rádh,
            // réidh; but slígh ʃlˈiːʁ, béadh bˈeːχ), a slender one ɡ (-igh, -idh), and -adh χ
            // except after r, n or l and an unstressed vowel (murchadh, tógadh; cuireadh kəˈɪrɐ).
            ('d' | 'g', Some('h')) if final_ => (
                match (slender, c) {
                    _ if run.after_long_open => (c == 'g').then_some(GH),
                    (true, _) => Some(GJ_PLAIN),
                    (false, 'd') if run.after_sonorant => None,
                    (false, 'd') => Some(DH_FINAL),
                    (false, _) => Some(GH),
                },
                2,
            ),
            ('d' | 'g', Some('h')) => (
                Some(match slender {
                    true if first
                        && soft
                        && c == 'g'
                        && matches!(run.next, Some(['i' | 'e', 'a' | 'á' | 'o' | 'ó' | 'u' | 'ú', ..])) =>
                    {
                        phonemes.push(GH);
                        J
                    }
                    true if first && soft => J,
                    false if first && c == 'd' => DH,
                    _ => GH,
                }),
                2,
            ),
            // fh is silent, and so is a final th (maith mˈA, bheith).
            ('f', Some('h')) => (None, 2),
            ('t', Some('h')) if final_ => (None, 2),
            ('p', Some('h')) if labial_glide => (Some(FJ), 2),
            ('p', Some('h')) => (Some(F), 2),
            // A slender sh or th is hʲ at the start of a word, and after a long vowel or gh
            // inside one (sheanáin, thighearna; ndóthin, ríghthe).
            ('s' | 't', Some('h'))
                if soft
                    && (first
                        || run.next.is_some() && (run.after_long && at == 0 || at > 0 && letters[at - 1] == 'h')) =>
            {
                (Some(HJ), 2)
            }
            ('s' | 't', Some('h')) => (Some(H), 2),
            ('n', Some('g')) => (Some(NG), 2),
            // A final slender nn is ŋ (linn, againn).
            ('n', Some('n')) if slender && run.last && at + 2 >= letters.len() => (Some(NG), 2),
            // A slender nn before a vowel is n and a glide j in eSpeak NG's notation (rinne
            // ɹˈɪnjə, cruinniú).
            ('n', Some('n'))
                if slender
                    && at + 2 >= letters.len()
                    && run.next.is_some()
                    && (!run.next_stressed || matches!(run.next, Some(['i', 'u', ..]))) =>
            {
                phonemes.push(N);
                (Some(J), 2)
            }
            // dl is l (codladh kˈɔlɐ).
            ('d', Some('l')) if !run.initial => (None, 1),
            // So is a slender b or m inside a word (dhíbirt jˈiːbʲɪrtʃ, coimeád).
            ('b', _) if labial_glide || slender && at == 0 && !run.initial && run.next.is_some() => (Some(BJ), 1),
            ('b', _) => (Some(B), 1),
            ('c' | 'k', _) => (Some(plain_at_end(soft, final_, KJ, KJ_PLAIN, K)), 1),
            // A slender d is dʲ before ea and éa at the start of a word, plain before e, and dʒ
            // elsewhere (déanamh, de; deoch, idir).
            // The d of d' is plain in eSpeak NG's notation (d'ól dˈoːl).
            ('d', _) if first && run.elided => (Some(if slender { DJ_PLAIN } else { D_PLAIN }), 1),
            // A slender d is dʲ at the start of a word and after l, n or r, plain before e, and dʒ
            // elsewhere and before eo, io and iu (déanamh, gairdín, de; deoch, idir).
            ('d', _) if slender => (
                Some(match run.next {
                    _ if before_short_e && first => DJ_PLAIN,
                    Some(['e', 'o', ..] | ['i', 'o' | 'u', ..]) => DZH,
                    _ if first || at > 0 && matches!(letters[at - 1], 'l' | 'n' | 'r') => DJ,
                    _ => DZH,
                }),
                1,
            ),
            ('d', _) => (Some(D), 1),
            // The f of the future and the conditional is h (déarfadh dʲˈeːrhɐχ, fhéadfadh).
            ('f', _) if run.future => (Some(H), 1),
            ('f', _) if labial_glide => (Some(FJ), 1),
            ('f' | 'v', _) => (Some(F), 1),
            // A final slender g is broad (tháinig hˈɑːnɪɡ).
            ('g', _) => (Some(plain_at_end(soft, final_, GJ, GJ_PLAIN, G)), 1),
            ('h', _) => (Some(H), 1),
            ('j', _) => (Some(DZH), 1),
            // A slender l is marked before t too (fáilte fˈɑːi̯lʲtʃə), and plain before e except
            // after a and éi (míle mˈiːlɛ, eile; bhaile vˈAlʲɛ, chéile).
            ('l', _) if slender && matches!(next, Some('t' | 's')) => (Some(LJ), 1),
            ('l', _) => {
                let plain = before_e && !run.after_a && (run.initial || matches!(run.next, Some(['e', ..])));
                (Some(plain_at_end(soft, plain, LJ, LJ_PLAIN, L)), 1)
            }
            ('m', _) if labial_glide || soft && at == 0 && !run.initial && run.next.is_some() => (Some(MJ), 1),
            ('m', _) => (Some(M), 1),
            ('n', _) => (Some(plain_at_end(soft, final_, NJ, NJ_PLAIN, N)), 1),
            ('p', _) if labial_glide => (Some(PJ), 1),
            ('p', _) => (Some(P), 1),
            // r is ɹ at the start of a word, and broad before a vowel (rud, troid, cora, mórán,
            // freagra; triall, éirinn, móir).
            ('r', _) => {
                let broad = !slender && end && run.next.is_some();
                (
                    Some(match () {
                        () if first || broad => R_APPROX,
                        () if slender => RJ,
                        () => R,
                    }),
                    1,
                )
            }
            ('s' | 'z', _) => (Some(if slender { SH } else { S }), 1),
            // A slender t is c, and tʃ after n, l, r or s and before r (tír, duit, tugtí;
            // caint, triall).
            ('t', _) if slender => (
                Some(
                    if matches!(next, Some('r' | 'n')) || at > 0 && matches!(letters[at - 1], 'n' | 'l' | 'r' | 's') {
                        TSH
                    } else {
                        TJ
                    },
                ),
                1,
            ),
            ('t', _) => (Some(T), 1),
            ('w', _) => (Some(W), 1),
            ('x', _) => {
                phonemes.push(K);
                (Some(S), 1)
            }
            ('y', _) => (Some(J), 1),
            ('q', _) => (Some(K), 1),
            _ => (None, 1),
        };
        // A doubled consonant is said once (ceann, fearr, giolla).
        let doubled = step == 1 && next == Some(c);
        if let Some(phoneme) = phoneme
            && (phonemes.last() != Some(&phoneme) || phoneme == V)
        {
            phonemes.push(phoneme);
        }
        at += if doubled { 2 } else { step };
    }
    phonemes
}

/// Whether `rest`, what follows an f inside a word, is a future or conditional ending.
fn is_future_ending(rest: &[char]) -> bool {
    const ENDINGS: [&str; 17] = [
        "adh", "eadh", "idh", "aidh", "inn", "ainn", "á", "eá", "imid", "aimid", "ar", "ear", "aimis", "imis", "aidís",
        "idís", "í",
    ];
    let rest: String = rest.iter().collect();
    ENDINGS.contains(&rest.as_str())
}

/// Where the letters of `runs[index]` end in the word.
fn run_end(runs: &[Run], index: usize) -> usize {
    runs[..=index].iter().map(|run| run.letters.len()).sum()
}

/// Whether a ə comes between the consonants of `cluster` after a short stressed vowel: l, n or r
/// before b, bh, ch, g, m or mh (orm, dearg, ainm, dorcha, Albain).
fn epenthesis(cluster: &[char]) -> bool {
    matches!(cluster, ['l' | 'n' | 'r', 'b' | 'g' | 'm', ..]) && !matches!(cluster, ['n', 'g', ..])
}

/// Which vowel group of `runs` the stress falls on: the first, or the second when it is long and
/// the first short, as in the south (foláir, amháin, dritháir), but not a final í or ín (hataí);
/// `marked` overrides it.
fn stressed_group(runs: &[Run], marked: Option<usize>) -> Option<usize> {
    let groups: Vec<usize> = (0..runs.len()).filter(|&index| runs[index].vowel).collect();
    if let Some(marked) = marked {
        return groups.iter().copied().find(|&index| index >= marked);
    }
    let first = *groups.first()?;
    // A final -ís takes the stress (airís, bhídís).
    if let Some(&last) = groups.last()
        && last != first
        && runs[last].letters == ['í']
        && runs.get(last + 1).is_some_and(|run| run.letters == ['s'])
        && last + 2 == runs.len()
    {
        return Some(last);
    }
    let long = |index: usize| is_long(&runs[index].letters);
    // Not when the first makes a diphthong with bh, mh, dh or gh before a consonant (amhrán
    // ˈaʊɹˌɑːn; but amháin).
    let diphthong = runs
        .get(first + 1)
        .is_some_and(|run| matches!(run.letters[..], ['b' | 'm' | 'd' | 'g', 'h', _, ..]));
    if let Some(&second) = groups.get(1)
        && !long(first)
        && !diphthong
        && long(second)
        && !(runs[second].letters.contains(&'í') && second + 2 >= runs.len())
    {
        return Some(second);
    }
    Some(first)
}

/// How `word` is said; `None` for a word without a letter Irish reads. A ˈ in a respelling marks
/// the stress (amach → aˈmach).
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    said(word, false)
}

/// How an elided word without its vowel is said: its consonants alone (d' d̪, bh' v).
pub(crate) fn pronounce_bare(word: &str) -> Option<Pronunciation> {
    said(word, true)
}

/// `word` said, `bare` when it may have no vowel.
#[expect(
    clippy::too_many_lines,
    reason = "one pass over the word's letter runs, kept together"
)]
fn said(word: &str, bare: bool) -> Option<Pronunciation> {
    let lower = word.to_lowercase();
    let mut chars: Vec<char> = Vec::with_capacity(lower.len());
    let mut marked: Option<usize> = None;
    let elided = (lower.starts_with("d'") || lower.starts_with("d’")) && lower.chars().count() > 2;
    // The t, n or h a word before adds, written with a hyphen (an t-am, i n-aon).
    let prefixed = matches!(lower.as_bytes(), [b't' | b'n' | b'h', b'-', ..]);
    for c in lower.chars() {
        match c {
            'ˈ' => marked = Some(chars.len()),
            '-' | '\'' | '’' => {}
            c if c.is_alphabetic() => chars.push(c),
            _ => return None,
        }
    }
    let runs = runs(&chars);
    // The index of the run that holds the marked letter.
    let marked_run = marked.map(|marked| {
        let mut letters = 0;
        runs.iter()
            .position(|run| {
                letters += run.letters.len();
                letters > marked
            })
            .unwrap_or(runs.len())
    });
    let stressed = stressed_group(&runs, marked_run);
    // A stressed final -ís leaves a long first syllable its stress too, as eSpeak NG says it
    // (bhídís vˈiːdʒˈiːʃ; but airís ɐrˈiːʃ, dtagaidís).
    let groups: Vec<usize> = (0..runs.len()).filter(|&index| runs[index].vowel).collect();
    let also_stressed = groups.first().copied().filter(|&first| {
        marked_run.is_none()
            && stressed.is_some_and(|stressed| stressed != first && runs[stressed].letters == ['í'])
            && is_long(&runs[first].letters)
    });
    let mut segments: Vec<Segment> = Vec::with_capacity(chars.len());
    let mut skip = 0;
    let mut hiatus = false;
    let mut after_au = false;
    let stress_long =
        stressed.is_some_and(|at| is_long(&runs[at].letters) || matches!(runs[at].letters[..], ['i' | 'u', 'a', ..]));
    for (index, run) in runs.iter().enumerate() {
        let last = index + 1 == runs.len();
        if run.vowel {
            let before = index.checked_sub(1).map_or(&[][..], |at| &runs[at].letters[..]);
            let after = runs.get(index + 1).map_or(&[][..], |next| &next.letters[..]);
            let around = Around {
                before,
                after,
                last: index + 2 >= runs.len(),
                pretonic: stressed.is_some_and(|stressed| index < stressed),
                then: runs.get(index + 2).map_or(&[][..], |then| &then.letters[..]),
                prefixed: index == 1 && prefixed,
                stress_long,
                hiatus,
            };
            let is_stressed = Some(index) == stressed || Some(index) == also_stressed;
            let (vowels, taken) = match long(&run.letters, &around) {
                Some(vowels) => (vowels, 0),
                None if is_stressed => stressed_short(&run.letters, &around),
                None => match ao(&run.letters, &around) {
                    Some(vowels) => (vowels, 0),
                    None => (vec![unstressed(&run.letters, &around)], 0),
                },
            };
            skip = taken;
            after_au = taken > 0 && vowels.contains(&AU);
            // The stress goes on the group's vowel that is not ə (cuid kəˈɪdʒ, níor nʲˈiːər).
            let stressed_at = vowels.iter().position(|&vowel| vowel.is_vowel() && vowel != SCHWA);
            // An á after the stress keeps a lighter one (amhrán ˈaʊɹˌɑːn, Uachtarán).
            let lighter = stressed.is_some_and(|stressed| index > stressed) && run.letters.contains(&'á');
            for (position, &vowel) in vowels.iter().enumerate() {
                let stress = match () {
                    () if Some(position) != stressed_at => Stress::Unstressed,
                    () if is_stressed => Stress::Primary,
                    () if lighter => Stress::Secondary,
                    () => Stress::Unstressed,
                };
                segments.push(Segment::new(vowel, stress));
            }
            continue;
        }
        let taken = std::mem::take(&mut skip);
        hiatus = after_au && taken >= run.letters.len();
        let letters = &run.letters[taken.min(run.letters.len())..];
        let previous_stressed_short = index
            .checked_sub(1)
            .is_some_and(|at| Some(at) == stressed && !is_long(&runs[at].letters));
        let next = runs.get(index + 1).map(|next| &next.letters[..]);
        let slender = match (next, index.checked_sub(1)) {
            (Some(next), _) => is_front(next[0]),
            (None, Some(at)) => runs[at].letters.last().is_some_and(|&c| is_front(c)),
            (None, None) => false,
        };
        let said = consonants(&Consonants {
            letters,
            slender,
            initial: index == 0,
            last,
            next,
            next_stressed: Some(index + 1) == stressed,
            then: runs.get(index + 2).map_or(&[][..], |then| &then.letters[..]),
            future: index > 0 && letters.last() == Some(&'f') && is_future_ending(&chars[run_end(&runs, index)..]),
            after_long: index.checked_sub(1).is_some_and(|at| is_long(&runs[at].letters)),
            after_long_open: index.checked_sub(1).is_some_and(|at| {
                Some(at) == stressed
                    && is_long(&runs[at].letters)
                    && matches!(runs[at].letters.last(), Some('á' | 'ó' | 'ú' | 'í' | 'i'))
            }),
            after_a: index
                .checked_sub(1)
                .is_some_and(|at| matches!(runs[at].letters[..], ['a'] | ['a' | 'é' | 'u', 'i'] | ['e', 'a', ..])),
            after_sonorant: index
                .checked_sub(2)
                .is_some_and(|at| Some(at + 1) != stressed && matches!(runs[at].letters.last(), Some('r' | 'n' | 'l'))),
            elided: index == 0 && elided,
        });
        // A ə also comes between an initial g, c, m or t and n (gnó ɡənˈoː, mná).
        let split = (previous_stressed_short && taken == 0 && epenthesis(letters)
            || index == 0 && matches!(letters, ['g' | 'c' | 'm' | 't', 'n', ..] | ['m', 'h', 'n', ..]))
            && said.len() >= 2;
        for (position, &phoneme) in said.iter().enumerate() {
            if split && position == 1 {
                segments.push(Segment::new(SCHWA, Stress::Unstressed));
            }
            // The r before the ə is between vowels, so ɹ (orm ˈɔɹəm).
            let phoneme = if split && position == 0 && phoneme == R && !slender {
                R_APPROX
            } else {
                phoneme
            };
            segments.push(Segment::new(phoneme, Stress::Unstressed));
        }
    }
    if segments.is_empty() || !bare && !segments.iter().any(|segment| segment.phoneme.is_vowel()) {
        return None;
    }
    Some(syllabify(&segments, legal_onset))
}

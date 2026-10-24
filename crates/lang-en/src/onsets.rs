//! English phonotactics: which consonant clusters may start a syllable, for syllabification by
//! maximal onset. American: no /j/ after coronals (new, menu), which CMUdict also follows.

use jespersen_phonemes::Phoneme;

use crate::phonemes::{B, D, F, G, H, K, L, M, N, NG, P, R, S, SH, T, TH, V, W, Y};

/// Whether `cluster` may start an English syllable.
pub fn legal_onset(cluster: &[Phoneme]) -> bool {
    match cluster {
        [single] => *single != NG,
        [first, second] => two(*first, *second),
        // s + p/t/k + liquid or glide: spring, strong, scream, splash, squash, spew, skew.
        [s, stop, last] if *s == S => matches!((*stop, *last), (P, R | L | Y) | (T, R) | (K, R | W | Y)),
        _ => false,
    }
}

fn two(first: Phoneme, second: Phoneme) -> bool {
    match second {
        // pray, bray, tray, dray, cry, gray, fry, three, shrimp.
        _ if second == R => [P, B, T, D, K, G, F, TH, SH].contains(&first),
        // play, blue, clay, glue, fly, slow.
        _ if second == L => [P, B, K, G, F, S].contains(&first),
        // twin, dwell, quick, Gwen, swim, thwart.
        _ if second == W => [T, D, K, G, S, TH].contains(&first),
        // pure, beauty, cute, argue, few, view, music, huge.
        _ if second == Y => [P, B, K, G, F, V, M, H].contains(&first),
        // spin, stop, skip, smile, snow, sphere.
        _ => first == S && [P, T, K, M, N, F].contains(&second),
    }
}

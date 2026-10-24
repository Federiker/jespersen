//! CMUdict's ARPAbet phones and their one-byte encoding in the compiled lexicon. Shared with the
//! build script (`build.rs` includes this file), so it depends on nothing else in the crate.

/// The 39 phones, in a fixed order: a phone's code is `index * 3 + stress` (stress 0 for
/// consonants; 0, 1 or 2 for vowels, as in CMUdict).
pub const PHONES: [&str; 39] = [
    "AA", "AE", "AH", "AO", "AW", "AY", "B", "CH", "D", "DH", "EH", "ER", "EY", "F", "G", "HH", "IH", "IY", "JH", "K",
    "L", "M", "N", "NG", "OW", "OY", "P", "R", "S", "SH", "T", "TH", "UH", "UW", "V", "W", "Y", "Z", "ZH",
];

/// Whether the phone at `index` is a vowel (and so carries a stress digit).
pub const fn is_vowel(index: usize) -> bool {
    matches!(index, 0..=5 | 10..=12 | 16 | 17 | 24 | 25 | 32 | 33)
}

/// The code of one CMUdict phone such as `AH0` or `T`; `None` for anything else.
#[cfg_attr(not(test), allow(dead_code, reason = "used by the build script"))]
pub fn encode(phone: &str) -> Option<u8> {
    let (name, stress) = match phone.as_bytes().last() {
        Some(digit @ b'0'..=b'2') => (&phone[..phone.len() - 1], digit - b'0'),
        _ => (phone, 0),
    };
    let index = PHONES.iter().position(|&known| known == name)?;
    let has_digit = name.len() < phone.len();
    if is_vowel(index) != has_digit {
        return None;
    }
    u8::try_from(index * 3).ok().map(|base| base + stress)
}

/// The phone index and stress (0–2) of a code; `None` for a code no phone has.
pub fn decode(code: u8) -> Option<(usize, u8)> {
    let index = usize::from(code / 3);
    let stress = code % 3;
    (index < PHONES.len() && (stress == 0 || is_vowel(index))).then_some((index, stress))
}

/// Gives a word of codes exactly one primary stress, as learned letter-to-sound rules may give it
/// none or several: the first primary stays and later ones become secondary; with none, the first
/// secondary becomes primary, or else the first vowel that is not a schwa, or else the first vowel.
pub fn one_primary(codes: &mut [u8]) {
    const AH: usize = 2;
    let vowels: Vec<usize> = (0..codes.len())
        .filter(|&at| is_vowel(usize::from(codes[at] / 3)))
        .collect();
    let stress = |code: u8| code % 3;
    let primaries: Vec<usize> = vowels.iter().copied().filter(|&at| stress(codes[at]) == 1).collect();
    if let Some((_, later)) = primaries.split_first() {
        for &at in later {
            // Primary (1) to secondary (2).
            codes[at] += 1;
        }
        return;
    }
    let chosen = vowels
        .iter()
        .copied()
        .find(|&at| stress(codes[at]) == 2)
        .or_else(|| vowels.iter().copied().find(|&at| usize::from(codes[at] / 3) != AH))
        .or_else(|| vowels.first().copied());
    if let Some(at) = chosen {
        codes[at] = codes[at] / 3 * 3 + 1;
    }
}

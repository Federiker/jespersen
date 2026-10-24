//! British English from the American phonemes (docs/decisions.md, decision 20): jespersen's own
//! rules, which the word's spelling informs where the American vowels merge what RP keeps apart
//! (LOT and PALM, THOUGHT and cloth, NEAR and KIT before r).

use jespersen_phonemes::{Phoneme, Segment, Stress};

use crate::{
    BathVowel,
    phonemes::{self as us, british as gb},
    spellings::{Group, vowel_groups},
};

/// Stems whose TRAP before n and a consonant is BATH in RP (plant, can't, command); n with other
/// consonants is TRAP far more often (fantasy, antler), so these are listed.
const BATH_BEFORE_N: &[&str] = &[
    "advantag",
    "aunt",
    "can't",
    "chant",
    "command",
    "demand",
    "enchant",
    "grant",
    "plant",
    "remand",
    "shan't",
    "slant",
    "supplant",
    "transplant",
];

/// Words the BATH rule would catch that RP says with TRAP.
const TRAP_ANYWAY: &[&str] = &["alas", "crass", "gas", "lass", "mass", "math", "maths", "pasta"];

/// Words whose last vowel is the vowel of one.
const ONE_WORDS: &[&str] = &["one", "someone", "anyone", "everyone", "no-one", "noone"];

/// The British phonemes of a word read as American, `spelling` its lower-case letters.
pub fn convert(american: &[Segment], spelling: &str, bath: BathVowel) -> Vec<Segment> {
    let vowels = american.iter().filter(|segment| segment.phoneme.is_vowel()).count();
    let groups = vowel_groups(spelling);
    let groups = (groups.len() == vowels).then_some(groups);
    let ends_ary = ["ary", "ery", "ory"].iter().any(|ending| spelling.ends_with(ending));
    let last_vowel_index = american
        .iter()
        .rposition(|segment| segment.phoneme.is_vowel())
        .unwrap_or(usize::MAX);

    let mut out: Vec<Segment> = Vec::with_capacity(american.len() + 1);
    // The American phoneme of the vowel last written, for an r that follows it.
    let mut last_vowel: Option<Phoneme> = None;
    let mut vowel_index: usize = 0;
    for (index, segment) in american.iter().enumerate() {
        let next = american.get(index + 1).map(|next| next.phoneme);
        if segment.phoneme == us::R
            && let Some(vowel) = last_vowel
            && out.last().is_some_and(|last| last.phoneme.is_vowel())
            && !next.is_some_and(Phoneme::is_vowel)
        {
            // An r after a vowel and before a consonant or the end colours the vowel and is not said.
            let group = groups
                .as_ref()
                .and_then(|groups| groups.get(vowel_index.saturating_sub(1)));
            let (coloured, schwa) = r_coloured(vowel, group);
            if let Some(last) = out.last_mut() {
                last.phoneme = coloured;
            }
            if schwa {
                out.push(Segment::new(us::SCHWA, Stress::Unstressed));
            }
            if next.is_none() {
                out.push(Segment::new(gb::R_SILENT, Stress::Unstressed));
            }
            last_vowel = None;
            continue;
        }
        if !segment.phoneme.is_vowel() {
            out.push(*segment);
            last_vowel = None;
            continue;
        }
        let group = groups.as_ref().map(|groups| &groups[vowel_index]);
        vowel_index += 1;
        let r_coloured_before_vowel =
            (segment.phoneme == us::NURSE || segment.phoneme == us::LETTER) && next.is_some_and(Phoneme::is_vowel);
        let before_r_vowel = r_coloured_before_vowel
            || (next == Some(us::R) && american.get(index + 2).is_some_and(|after| after.phoneme.is_vowel()));
        let mut stress = segment.stress;
        let mut phoneme = if before_r_vowel {
            before_r(segment.phoneme, group)
        } else {
            vowel(segment.phoneme, group, || {
                bath_context(american, index, spelling).then(|| bath_vowel(bath))
            })
        };
        // -ary, -ery, -ory lose their stress and full vowel: secretary, territory, library.
        if ends_ary && stress != Stress::Primary && index + 3 == american.len() {
            phoneme = us::SCHWA;
            stress = Stress::Unstressed;
        }
        // Their final y is weak whatever stress the lexicon gives it (library).
        if ends_ary && index + 1 == american.len() && (phoneme == gb::FLEECE || phoneme == us::HAPPY) {
            phoneme = us::HAPPY;
            stress = Stress::Unstressed;
        }
        // The last vowel of one, someone, anyone, everyone (eSpeak NG: ɒ), and because (RP ɒ).
        if segment.phoneme == us::STRUT && index == last_vowel_index {
            if ONE_WORDS.contains(&spelling) {
                phoneme = gb::ONE;
            } else if spelling == "because" {
                phoneme = gb::LOT;
            }
        }
        // Yod after t, d, n and th where the spelling has u (tune, duke, new, enthusiasm).
        if phoneme == gb::GOOSE
            && out
                .last()
                .is_some_and(|last| [us::T, us::D, us::N, us::TH].contains(&last.phoneme))
            && group.is_some_and(|group| {
                group.letters.contains('u') || (group.letters.ends_with('e') && group.after == Some('w'))
            })
        {
            out.push(Segment::new(us::Y, Stress::Unstressed));
        }
        out.push(Segment {
            phoneme,
            stress,
            ..*segment
        });
        // lettER and NURSE before a vowel keep their r (memory, hurry).
        if r_coloured_before_vowel {
            out.push(Segment::new(us::R, Stress::Unstressed));
        }
        // A final r-coloured or weak vowel may link to the next word with an r.
        if next.is_none() && takes_linking_r(segment.phoneme) {
            out.push(Segment::new(gb::R_SILENT, Stress::Unstressed));
        }
        last_vowel = Some(segment.phoneme);
    }
    out
}

/// A vowel's British phoneme, away from r. `bath` gives BATH's phoneme where TRAP is BATH.
fn vowel(american: Phoneme, group: Option<&Group>, bath: impl FnOnce() -> Option<Phoneme>) -> Phoneme {
    match american {
        p if p == us::LOT => lot_or_palm(group),
        p if p == us::THOUGHT => {
            // The cloth words, spelt o alone: dog, long, off, gone.
            if group.is_some_and(|group| group.letters == "o") {
                gb::LOT
            } else {
                gb::NORTH
            }
        }
        p if p == us::TRAP => bath().unwrap_or(gb::TRAP),
        p if p == us::GOAT => gb::GOAT,
        p if p == us::GOOSE => gb::GOOSE,
        p if p == us::FLEECE => gb::FLEECE,
        p if p == us::NURSE => gb::NURSE,
        p if p == us::LETTER => us::SCHWA,
        p if p == us::TRAP_WEAK => gb::TRAP_WEAK,
        p if p == us::STRUT_WEAK => gb::LOT_WEAK,
        p if p == us::NURSE_WEAK => gb::NURSE_WEAK,
        p if p == us::NORTH_WEAK || p == us::CURE_WEAK => gb::NORTH_WEAK,
        p if p == us::START_WEAK => gb::START_WEAK,
        p if p == us::SQUARE_WEAK => gb::SQUARE_WEAK,
        p if p == us::ON => gb::LOT,
        other => other,
    }
}

/// A vowel before r and another vowel, where the r stays: starry, story, during, period, vary;
/// very, carry, mirror, hurry keep their short vowels.
fn before_r(american: Phoneme, group: Option<&Group>) -> Phoneme {
    let spelt = |letter: char| group.is_some_and(|group| group.letters.contains(letter));
    match american {
        p if p == us::LOT => lot_or_palm(group),
        p if p == us::THOUGHT || p == us::GOAT => gb::NORTH,
        p if p == us::FOOT || p == us::GOOSE => gb::CURE,
        p if p == us::KIT || p == us::FLEECE => {
            if spelt('e') {
                gb::NEAR
            } else {
                us::KIT
            }
        }
        p if p == us::DRESS || p == us::FACE => {
            if spelt('a') {
                gb::SQUARE
            } else {
                us::DRESS
            }
        }
        p if p == us::NURSE => {
            if group.is_some_and(|group| group.doubled_r) {
                us::STRUT
            } else {
                gb::NURSE
            }
        }
        p if p == us::LETTER => us::SCHWA,
        other => vowel(other, group, || None),
    }
}

/// The vowel an r after it turns into, and whether a schwa follows it (fire, our).
fn r_coloured(american: Phoneme, group: Option<&Group>) -> (Phoneme, bool) {
    match american {
        p if p == us::LOT || p == us::TRAP => (gb::START, false),
        p if p == us::THOUGHT || p == us::GOAT => (gb::NORTH, false),
        p if p == us::KIT || p == us::FLEECE || p == us::HAPPY => (gb::NEAR, false),
        p if p == us::DRESS || p == us::FACE => (gb::SQUARE, false),
        p if p == us::FOOT || p == us::GOOSE => (gb::CURE, false),
        p if p == us::STRUT || p == us::NURSE => (gb::NURSE, false),
        p if p == us::PRICE || p == us::MOUTH => (p, true),
        p if p == us::SCHWA => (us::SCHWA, false),
        other => (vowel(other, group, || None), false),
    }
}

/// LOT where the vowel is spelt o, or a after w or qu (watch, quality); PALM for other a (father,
/// calm, spa).
fn lot_or_palm(group: Option<&Group>) -> Phoneme {
    match group {
        Some(group) if group.letters.contains('a') && !matches!(group.before, Some('w')) && !group.after_qu => {
            gb::START
        }
        _ => gb::LOT,
    }
}

/// Whether the vowel at `index`, TRAP in American, is BATH in RP: stressed, before f or th
/// closing the syllable, s and a consonant or the end, m p (example), n with s or ch (dance,
/// branch), or n with t or d in a listed stem (plant, command).
fn bath_context(american: &[Segment], index: usize, spelling: &str) -> bool {
    if american[index].stress != Stress::Primary || TRAP_ANYWAY.contains(&spelling) || spelling.contains("astic") {
        return false;
    }
    let at = |offset: usize| american.get(index + offset).map(|segment| segment.phoneme);
    let closes = |offset: usize| !at(offset).is_some_and(Phoneme::is_vowel);
    match at(1) {
        Some(p) if p == us::F || p == us::TH => closes(2),
        Some(p) if p == us::S => at(2).is_some_and(|after| !after.is_vowel()) || at(2).is_none(),
        Some(p) if p == us::M => at(2) == Some(us::P),
        Some(p) if p == us::N => match at(2) {
            Some(after) if after == us::S || after == us::CH => true,
            Some(after) if after == us::T || after == us::D => {
                BATH_BEFORE_N.iter().any(|stem| spelling.starts_with(stem))
            }
            _ => false,
        },
        _ => false,
    }
}

fn bath_vowel(bath: BathVowel) -> Phoneme {
    match bath {
        BathVowel::ByNotation => gb::BATH,
        BathVowel::Broad => gb::BATH_BROAD,
        BathVowel::Flat => gb::BATH_FLAT,
    }
}

/// Whether a word-final vowel (American) is said with an r that links to a following vowel:
/// lettER, NURSE and the weak forms of for, or, are, your, there (not her or were, which eSpeak
/// NG's British voice does not link).
fn takes_linking_r(american: Phoneme) -> bool {
    [
        us::LETTER,
        us::NURSE,
        us::NORTH_WEAK,
        us::START_WEAK,
        us::CURE_WEAK,
        us::SQUARE_WEAK,
    ]
    .contains(&american)
}

/// Across a word boundary: a final silent r links to a following vowel in eSpeak NG's notation
/// (bˈʌtəɹ and); before anything else, or at the end, it goes.
pub fn across(segments: &mut Vec<Segment>, next: Option<Segment>) {
    if segments.last().is_some_and(|last| last.phoneme == gb::R_SILENT) {
        if next.is_some_and(|next| next.phoneme.is_vowel()) {
            if let Some(last) = segments.last_mut() {
                last.phoneme = gb::R_LINKING;
            }
        } else {
            segments.pop();
        }
    }
}

//! Contextual rules for American English, applied to a word's phonemes before syllabification.
//! The allophones they choose differ only in eSpeak NG's notation; plain IPA writes the phoneme.

use jespersen_phonemes::{Segment, Stress};

use crate::phonemes::{
    D, FLEECE, HAPPY, HAPPY_CLOSED, KIT, KIT_REDUCED, LETTER, LETTER_LINKING, N, N_SYLLABIC, R, S, SCHWA, SCHWA_OPEN,
    SCHWA_REDUCED, SCHWA_SYLLABIC, T, T_FLAP, T_GLOTTAL, Z,
};
use crate::spellings::vowel_groups;

/// A word from the lexicon that starts with a schwa: eSpeak NG writes it ɐ (ago, about).
pub fn initial_schwa(segments: &mut [Segment]) {
    if let Some(first) = segments.first_mut()
        && first.phoneme == SCHWA
    {
        first.phoneme = SCHWA_OPEN;
    }
}

/// Within one word, in this order:
///
/// - The vowel of a final -ted or -ded is reduced (planted, folded, waited; for the first
///   syllable's, see [`reduce_first_kit`]).
/// - Unstressed FLEECE before a consonant is closed (anything, happiness; not happy, previous).
/// - FLEECE before r is KIT (here: hɪɹ), and lettER before a vowel is linked with an r (history,
///   memory).
/// - /t/ after a vowel or /ɹ/ and before an unstressed vowel is flapped (butter, party; not
///   attack, winter, Latin). /d/ is left alone, as in eSpeak NG's notation.
pub fn american(segments: &mut [Segment]) {
    let vowels: Vec<usize> = (0..segments.len())
        .filter(|&index| segments[index].phoneme.is_vowel())
        .collect();
    if vowels.len() >= 2
        && let [.., before, vowel, last] = segments
        && last.phoneme == D
        && vowel.stress == Stress::Unstressed
        && (before.phoneme == T || before.phoneme == D)
    {
        if vowel.phoneme == KIT {
            vowel.phoneme = KIT_REDUCED;
        } else if vowel.phoneme == SCHWA {
            vowel.phoneme = SCHWA_REDUCED;
        }
    }
    close_happy(segments);
    for index in 0..segments.len().saturating_sub(1) {
        let next = segments[index + 1].phoneme;
        let segment = &mut segments[index].phoneme;
        // NEAR is ɪɹ in General American (here, hear), as near is in CMUdict.
        if *segment == FLEECE && next == R {
            *segment = KIT;
        }
        if *segment == LETTER && next.is_vowel() {
            *segment = LETTER_LINKING;
        }
    }
    for index in 1..segments.len().saturating_sub(1) {
        let before = segments[index - 1].phoneme;
        let after = segments[index + 1];
        if segments[index].phoneme == T
            && (before.is_vowel() || before == R)
            && after.phoneme.is_vowel()
            && after.stress == Stress::Unstressed
            // A full vowel whose stress was lowered (thirteen) keeps its t.
            && after.phoneme != FLEECE
            // Nor before an unstressed vowel and n (Latin, curtain, important).
            && segments.get(index + 2).is_none_or(|n| n.phoneme != N)
        {
            segments[index].phoneme = T_FLAP;
        }
    }
}

/// Across a word boundary, when the next word starts with `next`: a final lettER before a vowel
/// is linked with an r (butter and), and a final /t/ after a vowel or /ɹ/ before an unstressed
/// vowel is flapped (it is, at a).
pub fn across(segments: &mut [Segment], next: Segment) {
    if !next.phoneme.is_vowel() {
        return;
    }
    match segments {
        [.., last] if last.phoneme == LETTER => last.phoneme = LETTER_LINKING,
        [.., before, last]
            if last.phoneme == T
                && (before.phoneme.is_vowel() || before.phoneme == R)
                && next.stress == Stress::Unstressed =>
        {
            last.phoneme = T_FLAP;
        }
        _ => {}
    }
}

/// Unstressed FLEECE before a consonant in the same word is closed (anything, happiness; not
/// happy, previous), in American and British English alike.
fn close_happy(segments: &mut [Segment]) {
    for index in 0..segments.len().saturating_sub(1) {
        if segments[index].phoneme == HAPPY && !segments[index + 1].phoneme.is_vowel() {
            segments[index].phoneme = HAPPY_CLOSED;
        }
    }
}

/// British English, within one word: unstressed FLEECE before a consonant is closed. There is no
/// flapping and no reduced ᵻ, as eSpeak NG's British voice writes it.
pub fn british(segments: &mut [Segment]) {
    close_happy(segments);
}

/// An unstressed schwa that the spelling writes i, or e in -ed, -es, -est, -et, -ess or a first
/// prefix (record, return, event; not hello), or the a of -age, is KIT: village, language, promised,
/// forest, difficult, folded, as RP and eSpeak NG's notation say it. Not before bl (horrible).
/// A final y the lexicon stresses lightly after the main stress is happY (starry). For words from
/// the lexicon, in either variety; the spelling must match the vowels one to one.
pub fn weak_vowels(segments: &mut [Segment], spelling: &str) {
    let groups = vowel_groups(spelling);
    let vowels: Vec<usize> = (0..segments.len())
        .filter(|&at| segments[at].phoneme.is_vowel())
        .collect();
    if groups.len() != vowels.len() || groups.is_empty() {
        return;
    }
    let last = groups.len() - 1;
    // A final y the lexicon gives secondary stress after the main one is happY (starry, library).
    if let (Some(&at), Some(group)) = (vowels.last(), groups.last())
        && group.letters == "y"
        && segments[at].phoneme == FLEECE
        && segments[at].stress == Stress::Secondary
        && segments[..at].iter().any(|segment| segment.stress == Stress::Primary)
    {
        segments[at].phoneme = HAPPY;
        segments[at].stress = Stress::Unstressed;
    }
    for (position, (&at, group)) in vowels.iter().zip(&groups).enumerate() {
        if segments[at].phoneme != SCHWA || segments[at].stress != Stress::Unstressed {
            continue;
        }
        let kit = match group.letters.as_str() {
            "i" | "y" => group.after != Some('b'),
            "e" => {
                (position == 0
                    && ["re", "de", "be", "pre", "se", "ex", "e"]
                        .iter()
                        .any(|prefix| spelling.starts_with(prefix)))
                    || (position == last
                        && ["ed", "es", "est", "et", "ess"]
                            .iter()
                            .any(|ending| spelling.ends_with(ending)))
            }
            "a" => position == last && spelling.ends_with("age"),
            _ => false,
        };
        if kit {
            segments[at].phoneme = KIT;
        }
    }
}

/// American English, for words from the lexicon (not function words' running forms): unstressed
/// KIT in a first syllable that starts with a consonant, before a stressed one, is reduced, as
/// eSpeak NG's notation writes it (behind, return; not eleven, example, because).
pub fn reduce_first_kit(segments: &mut [Segment]) {
    let vowels: Vec<usize> = (0..segments.len())
        .filter(|&index| segments[index].phoneme.is_vowel())
        .collect();
    if let [first, second, ..] = vowels[..]
        && first > 0
        && segments[first].phoneme == KIT
        && segments[first].stress == Stress::Unstressed
        && segments[second].stress != Stress::Unstressed
    {
        segments[first].phoneme = KIT_REDUCED;
    }
}

/// A secondary stress on a closed last syllable after the main stress is dropped, its full vowel
/// kept: lighthouse, whitewash, Sherlock, someone (not Mexico or studio, whose last syllable is
/// open). For words from the lexicon, in either variety, as eSpeak NG's notation writes them.
pub fn last_secondary(segments: &mut [Segment]) {
    let Some(last) = segments.iter().rposition(|segment| segment.phoneme.is_vowel()) else {
        return;
    };
    let closed = segments[last + 1..].iter().any(|segment| !segment.phoneme.is_vowel());
    if closed
        && segments[last].stress == Stress::Secondary
        && segments[..last].iter().any(|segment| segment.stress == Stress::Primary)
    {
        segments[last].stress = Stress::Unstressed;
    }
}

/// American English, for words from the lexicon: t, an unstressed schwa and n at the end after a
/// stressed vowel, written -tten, -tton or -ten (button, kitten, written, eaten; not Latin or
/// curtain), are a glottal stop and a syllabic n in eSpeak NG's notation (bˈʌʔn̩); plain IPA keeps
/// tən.
pub fn syllabic_n(segments: &mut [Segment], spelling: &str) {
    let written = ["ten", "ton", "tens", "tons"]
        .iter()
        .any(|ending| spelling.ends_with(ending));
    let end = match segments {
        [.., last] if last.phoneme == S || last.phoneme == Z => segments.len() - 1,
        _ => segments.len(),
    };
    if written
        && end >= 4
        && segments[end - 1].phoneme == N
        && segments[end - 2].phoneme == SCHWA
        && segments[end - 3].phoneme == T
        && segments[end - 4].phoneme.is_vowel()
        && segments[end - 4].stress != Stress::Unstressed
    {
        segments[end - 3].phoneme = T_GLOTTAL;
        segments[end - 2].phoneme = SCHWA_SYLLABIC;
        segments[end - 1].phoneme = N_SYLLABIC;
    }
}

//! Where a Russian word is stressed, which its spelling does not show: an acute accent or ё
//! written in the text, jespersen's own list, then the endings whose stress is predictable (-а́ние,
//! -и́ровать, the vowel before -ость, -тель and -ция, an adjective's stem), and otherwise the last
//! vowel but one.

use crate::{
    spelling::is_vowel,
    words::{ACUTE, Stressed, lookup},
};

/// Endings with the stressed vowel marked (-а́ние, -и́ровать), the infinitive's and the past's
/// vowel included (сказа́ть, говори́л).
const MARKED_ENDINGS: &[&str] = &[
    "а́ние",
    "а́ния",
    "а́нию",
    "а́нием",
    "а́нии",
    "а́ний",
    "а́ниям",
    "а́ниями",
    "а́ниях",
    "я́ние",
    "я́ния",
    "я́нии",
    "е́ние",
    "е́ния",
    "е́нию",
    "е́нием",
    "е́нии",
    "е́ний",
    "е́ниям",
    "е́ниями",
    "е́ниях",
    "и́ровать",
    "и́ровал",
    "и́ровала",
    "и́ровали",
    "и́рую",
    "и́рует",
    "и́руют",
    "и́рование",
    "и́рования",
    "и́рованный",
    "ова́ть",
    "а́ть",
    "я́ть",
    "е́ть",
    "и́ть",
    "у́ть",
    "а́л",
    "а́ла",
    "а́ло",
    "а́ли",
    "и́л",
    "и́ла",
    "и́ло",
    "и́ли",
    "е́л",
    "е́ла",
    "е́ли",
    "а́ет",
    "а́ют",
    "а́ешь",
    "я́ет",
    "и́зм",
    "и́зма",
    "и́зму",
    "и́змом",
    "и́зме",
    "и́змы",
    "и́ст",
    "и́ста",
    "и́сту",
    "и́стом",
    "и́сте",
    "и́сты",
    "и́стов",
    "и́стам",
    "и́стами",
    "и́стах",
    "и́стка",
    "и́стки",
    "и́сткой",
    "и́сток",
    "щено́",
    "щена́",
    "щены́",
    "ждено́",
    "ждена́",
    "ждены́",
    "быва́ть",
    "быва́ю",
    "быва́ешь",
    "быва́ет",
    "быва́ют",
    "быва́л",
    "быва́ла",
    "быва́ло",
    "быва́ли",
];

/// Endings stressed on the vowel before them: -ость, -тель, -ство, -ция and -ия with their forms,
/// -ывать and -ивать, the present participle's -ющий, and an adjective's endings (кра́сный).
const AFTER_STRESS: &[&str] = &[
    "ость",
    "ости",
    "остью",
    "остей",
    "остям",
    "остями",
    "остях",
    "тель",
    "теля",
    "телю",
    "телем",
    "теле",
    "тели",
    "телей",
    "телям",
    "телями",
    "телях",
    "тельный",
    "тельная",
    "тельное",
    "тельные",
    "тельно",
    "тельного",
    "тельной",
    "тельным",
    "тельных",
    "ство",
    "ства",
    "ству",
    "ством",
    "стве",
    "ция",
    "ции",
    "цию",
    "цией",
    "ций",
    "ия",
    "ии",
    "ию",
    "ией",
    "иям",
    "иями",
    "иях",
    "ывать",
    "ываю",
    "ывает",
    "ывают",
    "ывал",
    "ывала",
    "ывали",
    "ивать",
    "иваю",
    "ивает",
    "ивают",
    "ивал",
    "ивала",
    "ивали",
    "ющий",
    "ющая",
    "ющее",
    "ющие",
    "ющего",
    "ющей",
    "ющим",
    "ющих",
    "ый",
    "ий",
    "ая",
    "яя",
    "ое",
    "ее",
    "ые",
    "ие",
    "ого",
    "его",
    "ому",
    "ему",
    "ыми",
    "ими",
    "ых",
    "их",
    "ую",
    "юю",
    "гнуть",
    "гнут",
    "гнута",
    "гнуты",
    "гнул",
    "гнула",
    "гнули",
    "гнуло",
];

/// The endings of perfective verbs in -ить and -нуть, which вы- takes the stress from.
const PERFECTIVE_ENDINGS: [&str; 10] = ["ить", "ил", "ила", "ило", "или", "нуть", "нул", "нула", "нуло", "нули"];

fn vowels(letters: &str) -> usize {
    letters.chars().filter(|&c| is_vowel(c)).count()
}

/// The stress the endings give `word`, if one of them ends it.
fn by_ending(word: &str) -> Option<usize> {
    let marked = MARKED_ENDINGS
        .iter()
        .filter_map(|ending| {
            let letters: String = ending.chars().filter(|&c| c != ACUTE).collect();
            let stem = word.strip_suffix(letters.as_str())?;
            let before = ending.split(ACUTE).next().map_or(0, vowels);
            Some((letters.len(), vowels(stem) + before - 1))
        })
        .max_by_key(|(length, _)| *length);
    let after = AFTER_STRESS
        .iter()
        .filter_map(|ending| {
            let stem = word.strip_suffix(ending)?;
            Some((ending.len(), vowels(stem).checked_sub(1)?))
        })
        .max_by_key(|(length, _)| *length);
    // Before an ending, -ическ-, -еческ-, -ованн- and the participle's -аем- are stressed on their
    // first vowel (истори́ческий, челове́ческий, организо́ванный, получа́емый), -ственн-, -ствова-
    // and -тель- on the vowel before them (госуда́рственный, сопу́тствовать, учи́тельница).
    let inside = [
        ("ическ", 0),
        ("еческ", 0),
        ("ованн", 0),
        ("аем", 0),
        ("яем", 0),
        ("ственн", 1),
        ("ствов", 1),
        ("ствуе", 1),
        ("ствую", 1),
        ("тель", 1),
        ("тысячн", 0),
    ]
    .iter()
    .find_map(|&(suffix, back)| {
        let at = word
            .rfind(suffix)
            .filter(|&at| vowels(&word[at + suffix.len()..]) <= 2)?;
        vowels(&word[..at]).checked_sub(back)
    });
    match (inside, marked, after) {
        (None, Some((marked_length, stress)), Some((after_length, _))) if marked_length >= after_length => Some(stress),
        (Some(stress), _, _) | (None, _, Some((_, stress))) | (None, Some((_, stress)), None) => Some(stress),
        (None, None, None) => None,
    }
}

/// `word` (as written) with its stress: marked in the text, listed, by its ending, on вы-, or on
/// its last vowel but one.
pub(crate) fn stressed(word: &str) -> Stressed {
    let lower = word.to_lowercase();
    if lower.contains(ACUTE) || lower.contains('ё') {
        let letters: String = lower.chars().filter(|&c| c != ACUTE).collect();
        let marked = lower.find(ACUTE).map(|at| vowels(&lower[..at]) - 1);
        let yo = letters.find('ё').map(|at| vowels(&letters[..at]));
        return Stressed {
            stress: marked.or(yo),
            letters,
        };
    }
    if let Some(said) = lookup(&lower) {
        return said;
    }
    // The endings are read without -ся and -сь (отправля́ется).
    let base = ["ся", "сь"]
        .iter()
        .find_map(|reflexive| lower.strip_suffix(reflexive))
        .filter(|base| vowels(base) > 0)
        .unwrap_or(&lower);
    let count = vowels(base);
    // вы- before a consonant takes the stress where the endings give none (вы́росла), and from
    // the perfective's -ить and -нуть (вы́ступил, вы́тянуть).
    let prefixed = base
        .strip_prefix("вы")
        .and_then(|rest| rest.chars().next())
        .is_some_and(|c| !is_vowel(c));
    let perfective = PERFECTIVE_ENDINGS.iter().any(|ending| base.ends_with(ending));
    let stress = if prefixed && perfective {
        Some(0)
    } else {
        by_ending(base).filter(|&stress| stress < count).or(match count {
            0 => None,
            1 => Some(0),
            _ if prefixed => Some(0),
            _ => Some(count - 2),
        })
    };
    Stressed { letters: lower, stress }
}

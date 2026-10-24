//! How a Russian word is said from its spelling once its stress is known: consonants hard, or soft
//! before е, ё, и, ю, я and ь (ж, ш and ц always hard, ч and щ always soft); е, ё, ю and я as j
//! and a vowel at the start, after a vowel and after ь and ъ; the unstressed vowels reduced by
//! their distance from the stress (о as ɐ right before it and ə elsewhere, е and я as ɪ); voicing
//! assimilated inside the word and lost at its end; and the consonant groups said shorter than
//! written (сч as щ, тс as ц, -тся as ца, стн as сн, -ого as -ово).

use jespersen_phonemes::{Notation, Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A_CLITIC, A_FINAL, A_PRETONIC, A_REDUCED, A_STRESSED, B, B_SOFT, CH, D, D_SOFT, E_OPEN, E_REDUCED, E_STRESSED,
    E_UNSTRESSED, F, G, I, I_FINAL, I_LAX, I_REDUCED, I_START, J, J_SIGN, K, L, L_SILENT, L_SOFT_SIGN, M, N, N_SOFT,
    N_SOFT_SIGN, O_CLITIC, O_PRETONIC, O_REDUCED, O_STRESSED, P, P_SOFT, R, R_SOFT_SIGN, S, S_SOFT, SH, SHCH, T,
    T_SOFT, TS, U, U_REDUCED, V, V_SOFT, V_SOFT_SIGN, X, Y, YA_FINAL, YA_J, YA_J_FINAL, YA_J_REDUCED, YA_PRETONIC,
    YA_REDUCED, YO, YU, YU_J, YU_J_REDUCED, YU_REDUCED, Z, Z_SOFT, ZH, devoice, is_obstruent, is_voiced, legal_onset,
    soften, voice,
};

/// The vowel letters, in the order stress indexes count them.
pub(crate) const VOWELS: &str = "аеёиоуыэюя";

/// Words ending in -ого whose г is said (много, дорого); the rest say it as в (его, нового).
const OGO_KEPT: [&str; 8] = [
    "много",
    "немного",
    "строго",
    "дорого",
    "недорого",
    "убого",
    "полого",
    "отлого",
];

/// Whether `c` is a vowel letter.
pub(crate) fn is_vowel(c: char) -> bool {
    VOWELS.contains(c)
}

/// The consonant groups said shorter than they are written, and г before к and ч said as х. The
/// vowels stay as they are, so a stress counted in vowels keeps its place; -тся is spelt ця so
/// that its vowel is reduced as я is (учится ˈutʃʲitsʌ).
fn respell(word: &str) -> String {
    let mut said = word.to_owned();
    for ending in ["ться", "тся"] {
        if let Some(stem) = said.strip_suffix(ending) {
            said = format!("{stem}ця");
        }
    }
    // тс and дс are ц, but before a soft vowel (отсюда ʌtʲsʲˈu"da).
    for written in ["тьс", "дьс", "тс", "дс"] {
        let mut rest = said.as_str();
        let mut joined = String::with_capacity(said.len());
        while let Some(at) = rest.find(written) {
            let after = rest[at + written.len()..].chars().next();
            joined.push_str(&rest[..at]);
            joined.push_str(if softens(after) { written } else { "ц" });
            rest = &rest[at + written.len()..];
        }
        joined.push_str(rest);
        said = joined;
    }
    for (written, spoken) in [
        // The г of сегодня is в, as in -ого (сегодняшний).
        ("сегодн", "севодн"),
        ("сч", "щ"),
        ("зч", "щ"),
        ("жч", "щ"),
        ("вств", "ств"),
        ("стн", "сн"),
        ("здн", "зн"),
        ("стл", "сл"),
        ("рдц", "рц"),
        ("гк", "хк"),
        ("гч", "хч"),
    ] {
        said = said.replace(written, spoken);
    }
    let ogo = said.ends_with("ого") || said.ends_with("его");
    if ogo && (said.chars().count() > 3 && !OGO_KEPT.contains(&word) || said == "его") {
        let mut letters: Vec<char> = said.chars().collect();
        let g = letters.len() - 2;
        letters[g] = 'в';
        said = letters.into_iter().collect();
    }
    said
}

/// What the sound before a vowel letter makes of it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Before {
    /// A hard consonant.
    Hard,
    /// ж, ш or ц, after which и is said ы and е э.
    Hush,
    /// A soft consonant, ч or щ.
    Soft,
    /// Nothing: the start of the word, or a vowel (и in поиск).
    Vowel,
    /// ь or ъ.
    Sign,
}

/// Where a vowel stands from the stress.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Place {
    Stressed,
    /// Right before the stress, or at the start of the word before it.
    Pretonic,
    /// Further before the stress.
    Before,
    /// After the stress, not at the end.
    After,
    /// After the stress, the word's last letter.
    Final,
    /// In a word said without stress (что, за).
    Clitic,
}

/// One sound before the vowels are known.
#[derive(Clone, Copy, Debug)]
enum Unit {
    Consonant(Phoneme),
    Vowel { letter: char, before: Before, start: bool },
}

/// The consonant a letter stands for, before softening.
fn consonant(c: char) -> Option<Phoneme> {
    Some(match c {
        'б' => B,
        'в' => V,
        'г' => G,
        'д' => D,
        'ж' => ZH,
        'з' => Z,
        'й' => J,
        'к' => K,
        'л' => L,
        'м' => M,
        'н' => N,
        'п' => P,
        'р' => R,
        'с' => S,
        'т' => T,
        'ф' => F,
        'х' => X,
        'ц' => TS,
        'ч' => CH,
        'ш' => SH,
        'щ' => SHCH,
        _ => return None,
    })
}

/// Whether `c` softens the consonant before it.
fn softens(c: Option<char>) -> bool {
    matches!(c, Some('ь' | 'е' | 'ё' | 'и' | 'ю' | 'я'))
}

/// The letters of `word` as consonants and vowels; `None` for a letter Russian does not read.
fn units(word: &[char]) -> Option<Vec<Unit>> {
    let mut units = Vec::with_capacity(word.len() + 2);
    let mut before = Before::Vowel;
    for (at, &c) in word.iter().enumerate() {
        let next = word.get(at + 1).copied();
        if let Some(phoneme) = consonant(c) {
            let hush = [ZH, SH, TS].contains(&phoneme);
            let softened = !hush && phoneme != J && softens(next);
            let after_sign = word.get(at + 2).copied();
            let said = match phoneme {
                // The л of -лнц- is not said (солнце).
                _ if c == 'л' && word.get(at + 1..at + 3) == Some(&['н', 'ц']) => L_SILENT,
                _ if next == Some('ь') && phoneme == N => N_SOFT_SIGN,
                // A final ль stays soft in eSpeak NG's notation (роль rˈoɭʲ).
                _ if next == Some('ь') && phoneme == L && after_sign.is_some() => L_SOFT_SIGN,
                _ if next == Some('ь') && phoneme == R && after_sign.is_some_and(|c| consonant(c).is_some()) => {
                    R_SOFT_SIGN
                }
                _ if next == Some('ь') && phoneme == V => V_SOFT_SIGN,
                _ if softened => soften(phoneme),
                _ => phoneme,
            };
            units.push(Unit::Consonant(said));
            before = if hush {
                Before::Hush
            } else if phoneme == J {
                Before::Vowel
            } else if [CH, SHCH].contains(&phoneme) || softened {
                Before::Soft
            } else {
                Before::Hard
            };
        } else if is_vowel(c) {
            units.push(Unit::Vowel {
                letter: c,
                before,
                start: at == 0,
            });
            before = Before::Vowel;
        } else if matches!(c, 'ь' | 'ъ') {
            before = Before::Sign;
        } else if !matches!(c, '-' | '\'' | '’') {
            return None;
        }
    }
    soften_before_soft(&mut units);
    Some(units)
}

/// A consonant softened by the soft one after it, from the end of the word back: с, з and т
/// before a soft dental (участие utʃʲˈɑsʲtʲijɪ, работница rabˈotʲnʲitsa, Россия rʌsʲsʲˈija), м
/// before a soft p or b (империя ɪmʲpʲˈerʲija), when a vowel follows.
fn soften_before_soft(units: &mut [Unit]) {
    for index in (0..units.len().saturating_sub(2)).rev() {
        let (Unit::Consonant(phoneme), Unit::Consonant(next), Unit::Vowel { .. }) =
            (units[index], units[index + 1], units[index + 2])
        else {
            continue;
        };
        let dental = [S, Z, T].contains(&phoneme) && [T_SOFT, D_SOFT, N_SOFT, S_SOFT, Z_SOFT].contains(&next);
        let labial = phoneme == M && [P_SOFT, B_SOFT].contains(&next);
        if dental || labial {
            units[index] = Unit::Consonant(soften(phoneme));
        }
    }
}

/// The phonemes of vowel `letter` after `before`, at `place`, `start`ing the word or not: the j
/// that е, ё, ю and я stand for at the start, after a vowel and after ь and ъ (and и and о after
/// ь), then the vowel.
fn vowel(letter: char, before: Before, place: Place, start: bool) -> Vec<Phoneme> {
    use Place::{After, Before as Ahead, Clitic, Final, Pretonic, Stressed};
    let stressed = place == Stressed;
    let iotated = matches!(before, Before::Vowel | Before::Sign);
    // ь and ъ give a j of their own before я, ю and е in eSpeak NG's notation (семья sʲimʲjˈja).
    let sign = if before == Before::Sign && matches!(letter, 'я' | 'ю' | 'е') {
        vec![J_SIGN]
    } else {
        Vec::new()
    };
    let single = |phoneme: Phoneme| [sign.clone(), vec![phoneme]].concat();
    let with_j = |phoneme: Phoneme| [sign.clone(), vec![J, phoneme]].concat();
    match (letter, before) {
        ('я', _) if iotated => single(match place {
            Stressed => YA_J,
            Final => YA_J_FINAL,
            _ => YA_J_REDUCED,
        }),
        ('ю', _) if iotated => single(if stressed { YU_J } else { YU_J_REDUCED }),
        ('е', _) if iotated => with_j(match place {
            Stressed => E_STRESSED,
            Final | Clitic => I_LAX,
            _ if start => I_LAX,
            _ => E_REDUCED,
        }),
        ('ё', _) if iotated => with_j(YO),
        ('и' | 'о', Before::Sign) => with_j(vowel(letter, Before::Soft, place, start)[0]),
        ('а', Before::Soft) | ('я', _) => single(match place {
            Stressed => A_STRESSED,
            Pretonic | Clitic => YA_PRETONIC,
            Ahead | After => YA_REDUCED,
            Final => YA_FINAL,
        }),
        ('а', _) => single(match place {
            Stressed => A_STRESSED,
            Pretonic => A_PRETONIC,
            Ahead | After => A_REDUCED,
            Final => A_FINAL,
            Clitic => A_CLITIC,
        }),
        ('о', _) => single(match place {
            Stressed => O_STRESSED,
            Pretonic => O_PRETONIC,
            Ahead | After | Final => O_REDUCED,
            Clitic => O_CLITIC,
        }),
        ('ё', Before::Hush) => single(O_STRESSED),
        ('ё', _) => single(YO),
        ('у', _) => single(if stressed { U } else { U_REDUCED }),
        ('ю', _) => single(if stressed { YU } else { YU_REDUCED }),
        ('ы', _) | ('и', Before::Hush) => single(Y),
        ('е', Before::Hush) => single(if stressed { E_OPEN } else { Y }),
        ('и', _) => single(match place {
            Stressed if start => I_START,
            Stressed => I,
            Final | Clitic => I_LAX,
            _ if start => I_LAX,
            _ => I_REDUCED,
        }),
        ('е', _) => single(match place {
            Stressed => E_STRESSED,
            Clitic => I_LAX,
            _ => E_REDUCED,
        }),
        _ => single(if stressed { E_OPEN } else { E_UNSTRESSED }),
    }
}

/// Voicing inside the word, from its end back: an obstruent is voiced before a voiced one other
/// than в, voiceless before a voiceless one and at the end, unless the word has no vowel (в, с,
/// к, which the next word voices); z is soft before a soft d or n.
fn assimilate(phonemes: &mut [Phoneme]) {
    let has_vowel = phonemes.iter().any(|phoneme| phoneme.is_vowel());
    for index in (0..phonemes.len()).rev() {
        let phoneme = phonemes[index];
        if !is_obstruent(phoneme) {
            continue;
        }
        let next = phonemes[index + 1..]
            .iter()
            .copied()
            .find(|next| !next.symbol(Notation::Ipa).is_empty());
        phonemes[index] = match next {
            None if has_vowel => devoice(phoneme),
            Some(next) if is_obstruent(next) && ![V, V_SOFT].contains(&next) => {
                if is_voiced(next) {
                    voice(phoneme)
                } else {
                    devoice(phoneme)
                }
            }
            _ => phoneme,
        };
        if phonemes[index] == Z && next.is_some_and(|next| [D_SOFT, N_SOFT, N_SOFT_SIGN].contains(&next)) {
            phonemes[index] = Z_SOFT;
        }
    }
}

/// How `word` (in lower case) is said with the stress on its vowel number `stress` (counting from
/// 0), or with none for a word said without stress; `None` for a letter Russian does not read.
pub(crate) fn pronounce(word: &str, stress: Option<usize>) -> Option<Pronunciation> {
    let letters: Vec<char> = respell(word).chars().collect();
    let units = units(&letters)?;
    let count = units.iter().filter(|unit| matches!(unit, Unit::Vowel { .. })).count();
    let ends_in_vowel = matches!(units.last(), Some(Unit::Vowel { .. }));
    let mut phonemes = Vec::with_capacity(units.len() + 2);
    let mut stressed_at = None;
    let mut number = 0;
    for unit in &units {
        match *unit {
            Unit::Consonant(phoneme) => phonemes.push(phoneme),
            Unit::Vowel { letter, before, start } => {
                let last = number + 1 == count && ends_in_vowel;
                let place = match stress {
                    None => Place::Clitic,
                    Some(stress) if number == stress => Place::Stressed,
                    Some(stress) if number + 1 == stress || number < stress && start => Place::Pretonic,
                    Some(stress) if number < stress => Place::Before,
                    Some(_) if last => Place::Final,
                    Some(_) => Place::After,
                };
                for phoneme in vowel(letter, before, place, start) {
                    // A stressed и at the very end is ɪ in eSpeak NG's notation (три trʲˈɪ).
                    let phoneme = if phoneme == I && last { I_FINAL } else { phoneme };
                    if place == Place::Stressed && phoneme.is_vowel() {
                        stressed_at = Some(phonemes.len());
                    }
                    phonemes.push(phoneme);
                }
                number += 1;
            }
        }
    }
    if phonemes.is_empty() {
        return None;
    }
    assimilate(&mut phonemes);
    let segments: Vec<Segment> = phonemes
        .iter()
        .enumerate()
        .map(|(at, &phoneme)| {
            let stress = if Some(at) == stressed_at {
                Stress::Primary
            } else {
                Stress::Unstressed
            };
            Segment::new(phoneme, stress)
        })
        .collect();
    Some(syllabify(&segments, legal_onset))
}

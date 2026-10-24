//! How a French word is said from its spelling, once its ending is settled (endings.rs): vowel
//! groups (eau, ai, eu, oi, ou), nasal vowels before a consonant or the end (an, in, on, un), the
//! glides of bien, nuit and fille, the readings of c, g, s, t and x, e as é, è, ə or silent by
//! what follows it, the ə dropped between single consonants (samedi samdi), and the stress on the
//! last full vowel.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::{
    endings::ending,
    phonemes::{
        A, A_BACK, A_NASAL, B, D, E, E_NASAL, E_OPEN, EU, EU_OPEN, F, G, H_ASPIRATED, I, J, K, L, M, N, NY, O, O_NASAL,
        O_OPEN, P, R, S, SCHWA, SH, T, U, U_NASAL, UI, V, W, Y, Z, ZH, legal_onset,
    },
};

/// A sound as the letters give it; `schwa` marks an ə that can drop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Unit {
    phoneme: Phoneme,
    schwa: bool,
}

const VOWELS: &str = "aàâeéèêëiîïoôuùûüyœ";

fn is_vowel(c: Option<char>) -> bool {
    c.is_some_and(|c| VOWELS.contains(c))
}

/// e, i or y, after which c is s and g is ʒ.
fn is_front(c: Option<char>) -> bool {
    matches!(c, Some('e' | 'é' | 'è' | 'ê' | 'ë' | 'i' | 'î' | 'ï' | 'y'))
}

/// The letters of a word and where the reading is.
struct Letters {
    word: Vec<char>,
    at: usize,
}

impl Letters {
    fn get(&self, offset: usize) -> Option<char> {
        self.word.get(self.at + offset).copied()
    }

    fn starts(&self, text: &str) -> bool {
        text.chars().enumerate().all(|(offset, c)| self.get(offset) == Some(c))
    }

    /// Whether the letters from `offset` on are only a final e (une, rue).
    fn final_e(&self, offset: usize) -> bool {
        self.get(offset) == Some('e') && self.get(offset + 1).is_none()
    }

    /// Whether m or n at `offset` closes the syllable: before a consonant other than m or n, or
    /// at the end.
    fn nasal_at(&self, offset: usize) -> bool {
        matches!(self.get(offset), Some('m' | 'n'))
            && match self.get(offset + 1) {
                None => true,
                Some('m' | 'n' | 'h') => false,
                next => !is_vowel(next),
            }
    }
}

/// The units of `word`, its ending already settled (endings.rs); `None` for a letter French does
/// not read.
#[expect(clippy::too_many_lines, reason = "one arm per spelling, read in order")]
fn units(word: &str) -> Option<Vec<Unit>> {
    let mut letters = Letters {
        word: word.chars().collect(),
        at: 0,
    };
    let mut out: Vec<Unit> = Vec::with_capacity(letters.word.len());
    let push = |out: &mut Vec<Unit>, phonemes: &[Phoneme]| {
        out.extend(phonemes.iter().map(|&phoneme| Unit { phoneme, schwa: false }));
    };
    while let Some(c) = letters.get(0) {
        let after_vowel = out.last().is_some_and(|unit| unit.phoneme.is_vowel());
        let step = match c {
            _ if letters.starts("eau") => {
                push(&mut out, &[O]);
                3
            }
            _ if letters.starts("aill") => {
                push(&mut out, &[A, J]);
                4
            }
            _ if letters.starts("ail") && letters.get(3).is_none() => {
                push(&mut out, &[A, J]);
                3
            }
            _ if (letters.starts("ai") || letters.starts("ei")) && letters.nasal_at(2) => {
                push(&mut out, &[E_NASAL]);
                3
            }
            _ if letters.starts("eill") => {
                push(&mut out, &[E_OPEN, J]);
                4
            }
            _ if letters.starts("eil") && letters.get(3).is_none() => {
                push(&mut out, &[E_OPEN, J]);
                3
            }
            _ if letters.starts("ay") && is_vowel(letters.get(2)) => {
                push(&mut out, &[E_OPEN, J]);
                2
            }
            _ if letters.starts("ai") || letters.starts("aî") || letters.starts("ay") || letters.starts("ei") => {
                push(&mut out, &[E_OPEN]);
                2
            }
            _ if letters.starts("euill") || letters.starts("œuill") => {
                push(&mut out, &[EU_OPEN, J]);
                5
            }
            _ if (letters.starts("euil") || letters.starts("œil"))
                && letters.get(if c == 'œ' { 3 } else { 4 }).is_none() =>
            {
                push(&mut out, &[EU_OPEN, J]);
                if c == 'œ' { 3 } else { 4 }
            }
            _ if letters.starts("eu") || letters.starts("œu") || letters.starts("eû") => {
                // ø in an open syllable and before z (deux, Europe, heureuse), œ before a consonant
                // said in the same syllable (heure, peur, neuf).
                let next = letters.get(2);
                let closed = next.is_some()
                    && !is_vowel(next)
                    && next != Some('s')
                    && (!is_vowel(letters.get(3)) || letters.final_e(3));
                push(&mut out, &[if closed { EU_OPEN } else { EU }]);
                2
            }
            'œ' => {
                push(&mut out, &[E]);
                1
            }
            _ if letters.starts("oin") && letters.nasal_at(2) => {
                push(&mut out, &[W, E_NASAL]);
                3
            }
            _ if letters.starts("oy") && is_vowel(letters.get(2)) => {
                push(&mut out, &[W, A, J]);
                2
            }
            _ if letters.starts("oi") || letters.starts("oî") || letters.starts("oy") => {
                push(&mut out, &[W, A]);
                2
            }
            _ if letters.starts("ouill") => {
                push(&mut out, &[U, J]);
                5
            }
            _ if letters.starts("ou") || letters.starts("où") || letters.starts("oû") => {
                // ou before a vowel is w (oui, jouer), but not before a final e (joue).
                let glide = is_vowel(letters.get(2)) && !letters.final_e(2);
                push(&mut out, &[if glide { W } else { U }]);
                2
            }
            _ if letters.starts("au") => {
                push(&mut out, &[O]);
                2
            }
            // The nasal vowels: a vowel before m or n that closes its syllable.
            'a' | 'â' | 'à' if letters.nasal_at(1) => {
                push(&mut out, &[A_NASAL]);
                2
            }
            'e' if letters.nasal_at(1) => {
                // -ien and -éen are ɛ̃ (bien, chien, européen); en elsewhere ɑ̃.
                let after_i = matches!(letters.word.get(letters.at.wrapping_sub(1)), Some('i' | 'é' | 'y'));
                let end = letters.get(2).is_none();
                push(&mut out, &[if after_i && end { E_NASAL } else { A_NASAL }]);
                2
            }
            // en and em starting a word before a second n or m are ɑ̃ (ennui, emmener).
            'e' if letters.at == 0 && (letters.starts("enn") || letters.starts("emm")) => {
                push(&mut out, &[A_NASAL]);
                1
            }
            'i' | 'î' | 'y' if letters.nasal_at(1) && !is_vowel(letters.get(2)) => {
                push(&mut out, &[E_NASAL]);
                2
            }
            'o' | 'ô' if letters.nasal_at(1) => {
                push(&mut out, &[O_NASAL]);
                2
            }
            // um at the end is ɔm (album, maximum); un elsewhere œ̃ (un, lundi, parfum).
            'u' if letters.starts("um") && letters.get(2).is_none() => {
                push(&mut out, &[O_OPEN, M]);
                2
            }
            'u' | 'ù' | 'û' if letters.nasal_at(1) => {
                push(&mut out, &[U_NASAL]);
                2
            }
            // ill: j after a vowel (paille, mouillé, which the groups above take), ij after a
            // consonant (fille, famille); i and y before a vowel are j (bien, yeux).
            'i' if letters.starts("ill") => {
                push(&mut out, if after_vowel { &[J] } else { &[I, J] });
                3
            }
            // After a consonant and l or r, i stays a vowel (bibliothèque, crier).
            'i' | 'y'
                if is_vowel(letters.get(1))
                    && !letters.final_e(1)
                    && !(letters.at >= 2
                        && matches!(letters.word[letters.at - 1], 'l' | 'r')
                        && letters.word[letters.at - 2] != letters.word[letters.at - 1]
                        && !is_vowel(Some(letters.word[letters.at - 2]))) =>
            {
                push(&mut out, &[J]);
                1
            }
            'i' | 'î' | 'ï' | 'y' => {
                push(&mut out, &[I]);
                1
            }
            'â' => {
                push(&mut out, &[A_BACK]);
                1
            }
            'a' | 'à' => {
                push(&mut out, &[A]);
                1
            }
            'é' => {
                push(&mut out, &[E]);
                1
            }
            'è' | 'ê' | 'ë' => {
                push(&mut out, &[E_OPEN]);
                1
            }
            'e' => {
                e(&letters, &mut out);
                1
            }
            'o' | 'ô' => {
                push(&mut out, &[o(&letters)]);
                1
            }
            // u before a vowel after a consonant is ɥ (nuit, lui, huit), but not before a final e
            // (rue).
            'u' | 'û' | 'ù' | 'ü' if is_vowel(letters.get(1)) && !letters.final_e(1) && letters.get(1) != Some('e') =>
            {
                push(&mut out, &[UI]);
                1
            }
            'u' | 'û' | 'ù' | 'ü' => {
                push(&mut out, &[Y]);
                1
            }
            _ => consonant(&letters, &mut out, after_vowel)?,
        };
        letters.at += step;
    }
    Some(out)
}

/// e by what follows it: silent at the end of a word with another vowel (porte), è before two
/// consonants, x or a final consonant (merci, elle, sel, avec), ə elsewhere (petit, demain).
fn e(letters: &Letters, out: &mut Vec<Unit>) {
    let has_vowel = out.iter().any(|unit| unit.phoneme.is_vowel());
    let next = letters.get(1);
    let after = letters.get(2);
    if next.is_none() {
        if !has_vowel {
            out.push(Unit {
                phoneme: SCHWA,
                schwa: false,
            });
        }
        return;
    }
    // ch, ph, th, gn, gu and qu count as one consonant, and so do a stop or f with l or r.
    let digraph = matches!(
        (next, after),
        (Some('c' | 'p' | 't'), Some('h')) | (Some('g'), Some('n' | 'u')) | (Some('q'), Some('u'))
    );
    let cluster =
        matches!(next, Some('b' | 'c' | 'd' | 'f' | 'g' | 'k' | 'p' | 't' | 'v')) && matches!(after, Some('l' | 'r'));
    let two_consonants = !is_vowel(next) && !is_vowel(after) && after.is_some() && !digraph && !cluster;
    let final_consonant = !is_vowel(next) && after.is_none();
    let open = if next == Some('x') || two_consonants || final_consonant {
        E_OPEN
    } else {
        SCHWA
    };
    out.push(Unit {
        phoneme: open,
        schwa: open == SCHWA,
    });
}

/// o: ɔ in a closed syllable (porte, bonne, école), o in an open one and before z (mot, rose,
/// solaire).
fn o(letters: &Letters) -> Phoneme {
    if letters.get(0) == Some('ô') {
        return O;
    }
    let next = letters.get(1);
    let after = letters.get(2);
    // ch, ph, th, gn and qu count as one consonant (bibliothèque).
    let width = if matches!(
        (next, after),
        (Some('c' | 'p' | 't'), Some('h')) | (Some('g'), Some('n')) | (Some('q'), Some('u'))
    ) {
        2
    } else {
        1
    };
    // A stop or f before l or r starts the next syllable, leaving o open (notre, noble).
    let cluster =
        matches!(next, Some('b' | 'c' | 'd' | 'f' | 'g' | 'p' | 't' | 'v')) && matches!(after, Some('l' | 'r'));
    let open = match next {
        None => true,
        Some(c) if is_vowel(Some(c)) => true,
        Some('s' | 'z') if letters.final_e(2) => true,
        Some(_) if cluster => true,
        Some(_) => is_vowel(letters.get(1 + width)) && !letters.final_e(1 + width),
    };
    if open { O } else { O_OPEN }
}

/// A consonant and how many letters it takes; `None` for a letter French does not read.
fn consonant(letters: &Letters, out: &mut Vec<Unit>, after_vowel: bool) -> Option<usize> {
    let c = letters.get(0)?;
    let next = letters.get(1);
    let after = letters.get(2);
    let single = if next == Some(c) { 2 } else { 1 };
    let previous = letters.word.get(letters.at.wrapping_sub(1)).copied();
    let (phonemes, length): (&[Phoneme], usize) = match c {
        // ch before r or l is k, as in the Greek words (chrétien, chronique, chlore).
        'c' if next == Some('h') && matches!(after, Some('r' | 'l')) => (&[K], 2),
        'c' if next == Some('h') => (&[SH], 2),
        'c' if next == Some('c') && is_front(after) => (&[K, S], 2),
        'c' if is_front(next) => (&[S], 1),
        'c' | 'k' => (&[K], single),
        'b' => (&[B], single),
        'd' => (&[D], single),
        'f' => (&[F], single),
        'g' if next == Some('n') => (&[NY], 2),
        'g' if next == Some('u') && is_front(after) => (&[G], 2),
        // ge before a, o and u is ʒ, its e silent (mangeons, Georges).
        'g' if next == Some('e') && matches!(after, Some('a' | 'â' | 'o' | 'ô' | 'u')) => (&[ZH], 2),
        'g' if is_front(next) => (&[ZH], 1),
        'g' => (&[G], single),
        'j' => (&[ZH], 1),
        'l' => (&[L], single),
        'm' => (&[M], single),
        'n' => (&[N], single),
        'p' if next == Some('h') => (&[F], 2),
        'p' => (&[P], single),
        'q' if next == Some('u') => (&[K], 2),
        'q' => (&[K], 1),
        'r' => (&[R], single),
        's' if letters.starts("sch") => (&[SH], 3),
        's' if (next == Some('c') && is_front(after)) || next == Some('s') => (&[S], 2),
        // s between vowel letters is z (maison, rose), but not after a nasal vowel (ensemble).
        's' if after_vowel && is_vowel(next) && is_vowel(previous) => (&[Z], 1),
        's' | 'ç' => (&[S], 1),
        't' if next == Some('h') => (&[T], 2),
        // -tion is sjɔ̃ (nation), but not after s or x (question).
        't' if letters.starts("tion") && !matches!(previous, Some('s' | 'x')) => (&[S], 1),
        't' => (&[T], single),
        'v' => (&[V], 1),
        'w' => (&[W], 1),
        // x: ɡz in ex- before a vowel (examen, exemple), ks elsewhere (taxi, texte).
        'x' if letters.at == 1 && letters.word[0] == 'e' && (is_vowel(next) || next == Some('h')) => (&[G, Z], 1),
        'x' => (&[K, S], 1),
        'z' => (&[Z], single),
        'h' | '\'' | '’' | '-' => (&[], 1),
        _ => return None,
    };
    out.extend(phonemes.iter().map(|&phoneme| Unit { phoneme, schwa: false }));
    Some(length)
}

/// Drops the ə between single consonants after the first syllable (samedi samdi, maintenant
/// mɛ̃tnɑ̃, appeler aple), as eSpeak NG and everyday speech do; after two consonants it stays
/// (portefeuille).
fn drop_schwas(units: &mut Vec<Unit>) {
    let mut at = 0;
    while at < units.len() {
        if units[at].schwa {
            let vowel_before = units[..at].iter().rposition(|unit| unit.phoneme.is_vowel());
            let consonant_after = units.get(at + 1).is_some_and(|unit| !unit.phoneme.is_vowel());
            if let Some(vowel) = vowel_before
                && at - vowel == 2
                && consonant_after
            {
                units.remove(at);
                continue;
            }
        }
        at += 1;
    }
}

/// How `word` is said, with its liaison consonant as a silent marker at the end (petit + ‿t) and
/// an aspirated h as one at the start; `aspirated` says whether its h is (les héros). `None` for a
/// letter French does not read.
pub(crate) fn pronounce(word: &str, aspirated: bool) -> Option<Pronunciation> {
    // oe written without its ligature reads as œ in œu and œil (oeuf, oeil).
    let lower = word.to_lowercase().replace("oeu", "œu").replace("oeil", "œil");
    let (said, liaison) = ending(&lower);
    let mut units = units(&said)?;
    drop_schwas(&mut units);
    let mut phonemes: Vec<Phoneme> = units.iter().map(|unit| unit.phoneme).collect();
    if phonemes.is_empty() {
        return None;
    }
    if aspirated {
        phonemes.insert(0, H_ASPIRATED);
    }
    phonemes.extend(liaison);
    // The stress is on the last full vowel; a word with only ə is stressed on it (le).
    let stressed = phonemes
        .iter()
        .rposition(|&phoneme| phoneme.is_vowel() && phoneme != SCHWA)
        .or_else(|| phonemes.iter().rposition(|phoneme| phoneme.is_vowel()));
    let segments: Vec<Segment> = phonemes
        .iter()
        .enumerate()
        .map(|(at, &phoneme)| {
            let stress = if Some(at) == stressed {
                Stress::Primary
            } else {
                Stress::Unstressed
            };
            Segment::new(phoneme, stress)
        })
        .collect();
    Some(syllabify(&segments, legal_onset))
}

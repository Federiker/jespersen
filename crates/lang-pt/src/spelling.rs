//! How a Portuguese word is said from its spelling: letters to sounds (digraphs, the two readings
//! of c, g, s, r and x), then nasal vowels and glides, the stress from the accents and the
//! endings, and the vowels each variety gives a syllable by where it stands from the stress.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::{
    Variety,
    phonemes::{
        A, A_AFTER_STRESS, A_BEFORE_L, A_BEFORE_NASAL, A_NASAL, A_OPENING, A_REDUCED, B, D, DJ, E, E_FINAL, E_NASAL,
        E_OPEN, E_REDUCED, E_RHYTHMIC, F, G, GLIDE_EM, GLIDE_I, GLIDE_I_NASAL_BRAZILIAN, GLIDE_I_NASAL_EUROPEAN,
        GLIDE_U, GLIDE_U_NASAL, I, I_NASAL, K, L, L_AFTER_U, L_VOCALISED, LH_BRAZILIAN, LH_EUROPEAN, M, N, NASAL_CODA,
        NASAL_CODA_LABIAL, NY, O, O_FINAL, O_NASAL, O_NASAL_REDUCED, O_OPEN, O_REDUCED, O_TILDE, OFFGLIDE_I,
        OFFGLIDE_U, OFFGLIDE_U_AFTER_O, P, R_BRAZILIAN, R_EUROPEAN, RR_BRAZILIAN, RR_EUROPEAN, S, SH, T, TAP, TAP_CODA,
        TJ, U, U_NASAL, V, Z, ZH, legal_onset,
    },
};

/// A written accent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Accent {
    None,
    /// á é í ó ú: the stress, and an open e or o.
    Acute,
    /// â ê ô: the stress, and a closed vowel.
    Circumflex,
    /// ã õ: a nasal vowel, stressed unless another vowel has an accent.
    Tilde,
    /// à: a and a fused.
    Grave,
}

/// A consonant as the letters give it, before the variety and its place in the word settle it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Consonant {
    P,
    B,
    T,
    D,
    K,
    G,
    F,
    V,
    S,
    Z,
    Sh,
    Zh,
    M,
    N,
    Nh,
    L,
    Lh,
    /// An r that is a tap between vowels, and after a consonant or at the end as the variety says.
    R,
    /// The strong r: rr, and r at the start of a word or after l, n or s.
    Rr,
}

/// A piece of a word: a vowel that is a syllable's nucleus, a glide, a nasal consonant closing a
/// syllable, or a consonant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Piece {
    Vowel {
        letter: char,
        accent: Accent,
        nasal: bool,
    },
    /// The second half of a falling diphthong (pai, meu, mão).
    OffGlide {
        letter: char,
        nasal: bool,
    },
    /// i or u before the vowel of the same syllable (quatro, pátio).
    OnGlide(char),
    /// m or n after a vowel and before a consonant or the end, which makes the vowel nasal.
    NasalCoda,
    Consonant(Consonant),
}

/// A vowel letter and its accent.
fn vowel(c: char) -> Option<(char, Accent)> {
    Some(match c {
        'a' | 'e' | 'i' | 'o' | 'u' => (c, Accent::None),
        // y, and the grave accent of the 1911 spelling on an unstressed vowel (pègadas).
        'y' | 'ì' => ('i', Accent::None),
        'è' => ('e', Accent::None),
        'ò' => ('o', Accent::None),
        'ù' => ('u', Accent::None),
        'á' => ('a', Accent::Acute),
        'é' => ('e', Accent::Acute),
        'í' => ('i', Accent::Acute),
        'ó' => ('o', Accent::Acute),
        'ú' => ('u', Accent::Acute),
        'â' => ('a', Accent::Circumflex),
        'ê' => ('e', Accent::Circumflex),
        'ô' => ('o', Accent::Circumflex),
        'ã' => ('a', Accent::Tilde),
        'õ' => ('o', Accent::Tilde),
        'à' => ('a', Accent::Grave),
        _ => return None,
    })
}

/// The pieces of `word` (in lower case) as its letters give them; `None` for a letter Portuguese
/// does not read.
fn letters(word: &[char]) -> Option<Vec<Piece>> {
    let letter = |at: usize| word.get(at).copied();
    let is_vowel = |at: usize| letter(at).and_then(vowel).is_some();
    let front = |at: usize| {
        letter(at)
            .and_then(vowel)
            .is_some_and(|(base, _)| matches!(base, 'e' | 'i'))
    };
    let mut pieces: Vec<Piece> = Vec::with_capacity(word.len());
    let consonant = |pieces: &mut Vec<Piece>, consonant| pieces.push(Piece::Consonant(consonant));
    let mut at = 0;
    while at < word.len() {
        let c = word[at];
        let mut step = 1;
        if let Some((letter, accent)) = vowel(c) {
            pieces.push(Piece::Vowel {
                letter,
                accent,
                nasal: accent == Accent::Tilde,
            });
            at += 1;
            continue;
        }
        match c {
            'b' => consonant(&mut pieces, Consonant::B),
            'c' if letter(at + 1) == Some('h') => {
                consonant(&mut pieces, Consonant::Sh);
                step = 2;
            }
            'c' if front(at + 1) => consonant(&mut pieces, Consonant::S),
            'c' | 'k' => consonant(&mut pieces, Consonant::K),
            'd' => consonant(&mut pieces, Consonant::D),
            'f' => consonant(&mut pieces, Consonant::F),
            'g' if front(at + 1) => consonant(&mut pieces, Consonant::Zh),
            'g' | 'q' => {
                consonant(&mut pieces, if c == 'g' { Consonant::G } else { Consonant::K });
                // gu and qu: the u is silent before e and i (guerra, queijo) and a glide before a
                // and o (água, quatro); ü is always said (old spelling: cinqüenta).
                match letter(at + 1) {
                    Some('u') if front(at + 2) => step = 2,
                    Some('u') if is_vowel(at + 2) => {
                        pieces.push(Piece::OnGlide('u'));
                        step = 2;
                    }
                    Some('ü') => {
                        pieces.push(Piece::OnGlide('u'));
                        step = 2;
                    }
                    _ => {}
                }
            }
            'j' => consonant(&mut pieces, Consonant::Zh),
            'l' if letter(at + 1) == Some('h') => {
                consonant(&mut pieces, Consonant::Lh);
                step = 2;
            }
            'l' => consonant(&mut pieces, Consonant::L),
            'm' => consonant(&mut pieces, Consonant::M),
            'n' if letter(at + 1) == Some('h') => {
                consonant(&mut pieces, Consonant::Nh);
                step = 2;
            }
            'n' => consonant(&mut pieces, Consonant::N),
            'p' => consonant(&mut pieces, Consonant::P),
            'r' if letter(at + 1) == Some('r') => {
                consonant(&mut pieces, Consonant::Rr);
                step = 2;
            }
            'r' if at == 0 || matches!(word[at - 1], 'l' | 'n' | 's') => consonant(&mut pieces, Consonant::Rr),
            'r' => consonant(&mut pieces, Consonant::R),
            's' if letter(at + 1) == Some('s') => {
                consonant(&mut pieces, Consonant::S);
                step = 2;
            }
            // sc before e and i, and sç, are one s (nascer, desça).
            's' if (letter(at + 1) == Some('c') && front(at + 2)) || letter(at + 1) == Some('ç') => {
                consonant(&mut pieces, Consonant::S);
                step = 2;
            }
            's' if at > 0 && is_vowel(at - 1) && is_vowel(at + 1) => consonant(&mut pieces, Consonant::Z),
            's' | 'ç' => consonant(&mut pieces, Consonant::S),
            't' => consonant(&mut pieces, Consonant::T),
            'v' => consonant(&mut pieces, Consonant::V),
            'w' => pieces.push(Piece::OnGlide('u')),
            'x' => x(word, at, &mut pieces),
            // z ends a word as s does (luz, vez).
            'z' if at + 1 == word.len() => consonant(&mut pieces, Consonant::S),
            'z' => consonant(&mut pieces, Consonant::Z),
            'h' | '\'' | '’' | '-' => {}
            _ => return None,
        }
        at += step;
    }
    Some(pieces)
}

/// x: ʃ at the start of a word, after a diphthong and between vowels (xícara, caixa, lixo), z in
/// ex- before a vowel (exame), s before a consonant (texto), ks at the end (tórax). Words that read
/// it otherwise (táxi, próximo) are respelt in jespersen's list.
fn x(word: &[char], at: usize, pieces: &mut Vec<Piece>) {
    let is_vowel = |at: usize| word.get(at).copied().and_then(vowel).is_some();
    let consonant = |consonant| Piece::Consonant(consonant);
    if at == 0 {
        pieces.push(consonant(Consonant::Sh));
    } else if at == 1 && word[0] == 'e' && is_vowel(2) {
        pieces.push(consonant(Consonant::Z));
    } else if at + 1 == word.len() {
        pieces.extend([consonant(Consonant::K), consonant(Consonant::S)]);
    } else if !is_vowel(at + 1) {
        pieces.push(consonant(Consonant::S));
    } else {
        pieces.push(consonant(Consonant::Sh));
    }
}

fn is_consonant(piece: Option<&Piece>) -> bool {
    matches!(piece, Some(Piece::Consonant(_)))
}

/// m and n close a syllable before a consonant or at the end, making the vowel before them nasal
/// (campo, tempo, bom); a final -am and -em are a nasal diphthong (falam, bem), and so are ão, ãe
/// and õe.
fn nasals(pieces: &[Piece]) -> Vec<Piece> {
    let mut out: Vec<Piece> = Vec::with_capacity(pieces.len());
    let mut at = 0;
    while at < pieces.len() {
        let piece = pieces[at];
        let next = pieces.get(at + 1);
        let after = pieces.get(at + 2);
        match piece {
            Piece::Consonant(Consonant::M | Consonant::N)
                if matches!(out.last(), Some(Piece::Vowel { .. })) && (next.is_none() || is_consonant(next)) =>
            {
                let at_end = next.is_none() || (next == Some(&Piece::Consonant(Consonant::S)) && after.is_none());
                let Some(Piece::Vowel { letter, accent, .. }) = out.last().copied() else {
                    unreachable!("matched above");
                };
                let last = out.len() - 1;
                out[last] = Piece::Vowel {
                    letter,
                    accent,
                    nasal: true,
                };
                match letter {
                    'a' if at_end && accent == Accent::None => out.push(Piece::OffGlide {
                        letter: 'u',
                        nasal: true,
                    }),
                    'e' if at_end => out.push(Piece::OffGlide {
                        letter: 'i',
                        nasal: true,
                    }),
                    _ => out.push(Piece::NasalCoda),
                }
            }
            Piece::Vowel { letter, .. }
                if matches!(
                    out.last(),
                    Some(Piece::Vowel {
                        accent: Accent::Tilde,
                        ..
                    })
                ) && matches!(letter, 'o' | 'e' | 'i') =>
            {
                out.push(Piece::OffGlide {
                    letter: if letter == 'o' { 'u' } else { 'i' },
                    nasal: true,
                });
            }
            piece => out.push(piece),
        }
        at += 1;
    }
    out
}

/// i and u after a vowel are the second half of a diphthong (pai, meu, cuidar), unless accented
/// or before nh or an l, r or z that closes their syllable (rainha, cair, juiz), or nasal (ainda).
fn offglides(mut pieces: Vec<Piece>) -> Vec<Piece> {
    for at in 1..pieces.len() {
        let Piece::Vowel {
            letter: letter @ ('i' | 'u'),
            accent: Accent::None,
            nasal: false,
        } = pieces[at]
        else {
            continue;
        };
        if !matches!(pieces[at - 1], Piece::Vowel { .. }) {
            continue;
        }
        let next = pieces.get(at + 1);
        let after = pieces.get(at + 2);
        let closed = match next {
            Some(Piece::Consonant(Consonant::Nh)) => true,
            Some(Piece::Consonant(Consonant::L | Consonant::R | Consonant::Z)) => {
                after.is_none() || is_consonant(after)
            }
            _ => false,
        };
        if !closed {
            pieces[at] = Piece::OffGlide { letter, nasal: false };
        }
    }
    pieces
}

/// Which nucleus (counted from 0) is stressed: an acute or circumflex accent, else a tilde, else
/// the last but one in words ending in a, e, o, am, em or ens (and their plurals), else the last.
fn stressed(pieces: &[Piece], word: &str, count: usize) -> usize {
    let nuclei = pieces.iter().filter_map(|piece| match piece {
        Piece::Vowel { accent, .. } => Some(*accent),
        _ => None,
    });
    let accents: Vec<Accent> = nuclei.collect();
    if let Some(at) = accents
        .iter()
        .position(|accent| matches!(accent, Accent::Acute | Accent::Circumflex))
        .or_else(|| accents.iter().position(|accent| *accent == Accent::Tilde))
    {
        return at;
    }
    let ending = word.strip_suffix('s').unwrap_or(word);
    let penultimate =
        ending.ends_with(['a', 'e', 'o']) || ending.ends_with("am") || ending.ends_with("em") || ending.ends_with("en");
    count.saturating_sub(if penultimate && count > 1 { 2 } else { 1 })
}

/// Where a vowel stands from the stress.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Place {
    Stressed,
    Before,
    After,
    Last,
}

/// The syllables (nuclei, from 0) eSpeak NG marks with a rhythmic stress: every other syllable
/// from the first, up to two before the stress (ˌunivˌeɾəsidˈadɨ), and the first before a stress on
/// the second syllable that is not the last (tˌeʎˈadʊ, but kɐfˈɛ).
fn rhythm(stressed: usize, count: usize) -> Vec<usize> {
    let mut marks: Vec<usize> = (0..stressed.saturating_sub(1)).step_by(2).collect();
    if stressed == 1 && stressed + 1 < count {
        marks.push(0);
    }
    marks
}

/// How each piece is said, as a phoneme (none for a silent one).
struct Reading {
    variety: Variety,
    /// The word's vowels are all said as unstressed ones (de dɨ, para pɐɾɐ).
    reduced: bool,
}

impl Reading {
    fn vowel(&self, pieces: &[Piece], at: usize, place: Place, rhythmic: bool) -> Phoneme {
        let Piece::Vowel { letter, accent, nasal } = pieces[at] else {
            unreachable!("only vowels");
        };
        let european = self.variety == Variety::European;
        let next = pieces.get(at + 1);
        let after = pieces.get(at + 2);
        if nasal {
            return match letter {
                'o' if accent == Accent::Tilde => O_TILDE,
                'a' => A_NASAL,
                'e' => E_NASAL,
                'i' => I_NASAL,
                'o' if european && place != Place::Stressed && accent == Accent::None => O_NASAL_REDUCED,
                'o' => O_NASAL,
                _ => U_NASAL,
            };
        }
        let coda_l = next == Some(&Piece::Consonant(Consonant::L)) && (after.is_none() || is_consonant(after));
        // A final l opens the e and o before it (papel, sol), but not an l before a consonant (olmo).
        let final_l = next == Some(&Piece::Consonant(Consonant::L)) && after.is_none();
        let before_sibilant = matches!(
            next,
            Some(Piece::Consonant(Consonant::S | Consonant::Z | Consonant::Sh))
        );
        let before_nasal = matches!(
            next,
            Some(Piece::Consonant(Consonant::M | Consonant::N | Consonant::Nh))
        );
        // A diphthong keeps its first vowel whole (eu, foi, pai), as do accented vowels.
        let whole =
            matches!(next, Some(Piece::OffGlide { .. })) || matches!(accent, Accent::Acute | Accent::Circumflex);
        let place = if whole && place != Place::Stressed {
            Place::Stressed
        } else {
            place
        };
        match (letter, place) {
            ('a', _) if accent == Accent::Grave => crate::phonemes::A_LONG,
            ('a', Place::Stressed) if accent == Accent::Circumflex => A_REDUCED,
            ('a', Place::Stressed) if accent == Accent::None && before_nasal && !whole => {
                if european {
                    A_BEFORE_NASAL
                } else {
                    A_NASAL
                }
            }
            ('a', Place::Stressed | Place::Before) if european && coda_l => A_BEFORE_L,
            ('a', Place::Before) if next == Some(&Piece::Consonant(Consonant::Nh)) => A_NASAL,
            ('a', Place::Before) if before_nasal && !european => A_OPENING,
            ('a', Place::Before) if !european => A,
            ('a', Place::Stressed) => A,
            ('a', _) if european => A_REDUCED,
            ('a', _) if nuclei(pieces) == 1 => A,
            ('a', _) => A_AFTER_STRESS,
            ('e', Place::Stressed) if accent == Accent::Acute || final_l => E_OPEN,
            // An opening e before s or x keeps its quality in eSpeak NG's notation (exame, está).
            ('e', Place::Before) if european && (rhythmic || (at == 0 && before_sibilant)) => E_RHYTHMIC,
            ('e', Place::Before | Place::After | Place::Last) if european => E_REDUCED,
            // A Brazilian function word's e before a consonant stays e (ser, nem).
            ('e', Place::Last) if !(self.reduced && next.is_some()) => E_FINAL,
            ('e', _) => E,
            ('o', Place::Stressed) if accent == Accent::Acute || final_l => O_OPEN,
            ('o', Place::Before | Place::After) if european => O_REDUCED,
            // A function word's o before a consonant is u in Portugal (por, os), but ʊ in dos and
            // nos; in Brazil o, but ʊ before s (os ʊs).
            ('o', Place::Last) if self.reduced && next.is_some() => {
                let before_s = next == Some(&Piece::Consonant(Consonant::S));
                if european && !(before_s && at > 0) {
                    O_REDUCED
                } else if before_s {
                    O_FINAL
                } else {
                    O
                }
            }
            ('o', Place::Last) => O_FINAL,
            ('o', _) => O,
            ('i', _) => I,
            // A u before nh is nasal before the stress (punhado pũɲˈadʊ).
            ('u', Place::Before) if next == Some(&Piece::Consonant(Consonant::Nh)) => U_NASAL,
            _ => U,
        }
    }

    fn consonant(&self, pieces: &[Piece], at: usize, said: &[Phoneme]) -> Phoneme {
        let Piece::Consonant(consonant) = pieces[at] else {
            unreachable!("only consonants");
        };
        let european = self.variety == Variety::European;
        let next = pieces.get(at + 1);
        let closes = next.is_none() || is_consonant(next) || next == Some(&Piece::NasalCoda);
        let voiced_next = matches!(
            next,
            Some(Piece::Consonant(
                Consonant::B
                    | Consonant::D
                    | Consonant::G
                    | Consonant::V
                    | Consonant::Z
                    | Consonant::Zh
                    | Consonant::M
                    | Consonant::N
                    | Consonant::Nh
                    | Consonant::L
                    | Consonant::Lh
                    | Consonant::R
                    | Consonant::Rr
            ))
        );
        let after_consonant = at > 0 && is_consonant(pieces.get(at - 1));
        match consonant {
            Consonant::P => P,
            Consonant::B => B,
            Consonant::T => T,
            Consonant::D => D,
            Consonant::K => K,
            Consonant::G => G,
            Consonant::F => F,
            Consonant::V => V,
            Consonant::S if closes && european => {
                if voiced_next {
                    ZH
                } else {
                    SH
                }
            }
            Consonant::S if closes && voiced_next => Z,
            Consonant::S => S,
            Consonant::Z => Z,
            Consonant::Sh => SH,
            Consonant::Zh => ZH,
            Consonant::M => M,
            Consonant::N => N,
            Consonant::Nh => NY,
            Consonant::L if closes => match said.last() {
                Some(&U) => L_AFTER_U,
                Some(&O_OPEN) if !european => L,
                _ if european => L,
                _ => L_VOCALISED,
            },
            Consonant::L => L,
            // Brazil's voice writes lh ʎ at the start of a word (lhe), lj elsewhere (filho).
            Consonant::Lh if european || at == 0 => LH_EUROPEAN,
            Consonant::Lh => LH_BRAZILIAN,
            Consonant::R if after_consonant || next.is_none() => {
                if european {
                    R_EUROPEAN
                } else {
                    R_BRAZILIAN
                }
            }
            Consonant::R if closes => TAP_CODA,
            Consonant::R => TAP,
            Consonant::Rr if european => RR_EUROPEAN,
            Consonant::Rr => RR_BRAZILIAN,
        }
    }

    fn glide(&self, pieces: &[Piece], at: usize, said: &[Phoneme]) -> Phoneme {
        match pieces[at] {
            Piece::OffGlide {
                letter: 'i',
                nasal: true,
            } => {
                // A final -em: eSpeak NG writes eɪŋ (bem); ãe and õe end in j (Brazil: ãe in y).
                if said.last() == Some(&E_NASAL) {
                    GLIDE_EM
                } else if self.variety == Variety::European || said.last() == Some(&O_TILDE) {
                    GLIDE_I_NASAL_EUROPEAN
                } else {
                    GLIDE_I_NASAL_BRAZILIAN
                }
            }
            Piece::OffGlide { letter: _, nasal: true } => GLIDE_U_NASAL,
            Piece::OffGlide { letter: 'i', .. } => OFFGLIDE_I,
            Piece::OffGlide { .. } if matches!(said.last(), Some(&O | &O_OPEN | &O_REDUCED)) => OFFGLIDE_U_AFTER_O,
            Piece::OffGlide { .. } => OFFGLIDE_U,
            Piece::OnGlide('i') => GLIDE_I,
            Piece::OnGlide(_) => GLIDE_U,
            Piece::NasalCoda => {
                let labial = matches!(pieces.get(at + 1), Some(Piece::Consonant(Consonant::P | Consonant::B)));
                if labial { NASAL_CODA_LABIAL } else { NASAL_CODA }
            }
            _ => unreachable!("only glides and nasal codas"),
        }
    }
}

/// How many vowels (syllable nuclei) `pieces` has.
fn nuclei(pieces: &[Piece]) -> usize {
    pieces
        .iter()
        .filter(|piece| matches!(piece, Piece::Vowel { .. }))
        .count()
}

/// After the stress, i and u before a vowel glide into it (pátio pˈatjʊ, história).
fn glides_after(pieces: &mut [Piece], stress: usize) {
    let mut nucleus = 0;
    for at in 0..pieces.len() {
        if let Piece::Vowel { letter, accent, nasal } = pieces[at] {
            if nucleus > stress
                && matches!(letter, 'i' | 'u')
                && accent == Accent::None
                && !nasal
                && matches!(pieces.get(at + 1), Some(Piece::Vowel { .. }))
            {
                pieces[at] = Piece::OnGlide(letter);
                continue;
            }
            nucleus += 1;
        }
    }
}

/// Brazil says t and d before an i as tʃ and dʒ (tia, cidade, leite), and the two s of exc as one
/// (excelente).
fn brazilian(said: &mut Vec<Phoneme>) {
    said.dedup_by(|this, before| *this == S && *before == S);
    for at in 0..said.len().saturating_sub(1) {
        if [I, I_NASAL, E_FINAL, GLIDE_I].contains(&said[at + 1]) {
            said[at] = match said[at] {
                T => TJ,
                D => DJ,
                other => other,
            };
        }
    }
}

/// How `word` is said in `variety`; with `reduced`, a function word's vowels are all said as
/// unstressed ones (de dɨ, para pɐɾɐ). `None` for a letter Portuguese does not read.
pub(crate) fn pronounce(word: &str, variety: Variety, reduced: bool) -> Option<Pronunciation> {
    let lower = word.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();
    let mut pieces = offglides(nasals(&letters(&chars)?));
    // Portugal drops the e of es- before a consonant in longer words (escola ʃkˈɔlɐ).
    if variety == Variety::European
        && nuclei(&pieces) >= 3
        && matches!(chars[..], ['e', 's', next, ..] if vowel(next).is_none())
    {
        pieces.remove(0);
    }
    let reading = Reading { variety, reduced };
    if nuclei(&pieces) == 0 {
        let segments: Vec<Segment> = (0..pieces.len())
            .filter(|&at| matches!(pieces[at], Piece::Consonant(_)))
            .map(|at| Segment::new(reading.consonant(&pieces, at, &[]), Stress::Unstressed))
            .collect();
        return (!segments.is_empty()).then(|| syllabify(&segments, legal_onset));
    }
    let stress = stressed(&pieces, lower.as_str(), nuclei(&pieces));
    glides_after(&mut pieces, stress);
    let total = nuclei(&pieces);
    let marks = if reduced { Vec::new() } else { rhythm(stress, total) };
    let mut said: Vec<Phoneme> = Vec::with_capacity(pieces.len());
    let mut stressed_at = None;
    let mut nucleus = 0;
    for at in 0..pieces.len() {
        let phoneme = match pieces[at] {
            Piece::Vowel { .. } => {
                let place = match nucleus {
                    _ if reduced && nucleus + 1 == total => Place::Last,
                    _ if reduced => Place::Before,
                    n if n == stress => Place::Stressed,
                    n if n < stress => Place::Before,
                    n if n + 1 == total => Place::Last,
                    _ => Place::After,
                };
                if nucleus == stress {
                    stressed_at = Some(said.len());
                }
                nucleus += 1;
                reading.vowel(&pieces, at, place, marks.contains(&(nucleus - 1)))
            }
            Piece::Consonant(_) => reading.consonant(&pieces, at, &said),
            _ => reading.glide(&pieces, at, &said),
        };
        said.push(phoneme);
    }
    if variety == Variety::Brazilian {
        brazilian(&mut said);
    }
    let segments: Vec<Segment> = said
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
    let mut pronunciation = syllabify(&segments, legal_onset);
    // Portugal's voice leaves out the rhythmic stress of a first ɐ before m or n (amigos ɐmˈiɡʊʃ,
    // caminho kɐmˈiɲʊ, but agora ˌɐɡˈɔɾɐ, trabalha tɹˌɐbˈaʎɐ).
    let first_vowel = said.iter().position(|phoneme| phoneme.is_vowel());
    let opening_a = first_vowel.is_some_and(|at| said[at] == A_REDUCED && matches!(said.get(at + 1), Some(&M | &N)));
    for mark in marks {
        if mark == 0 && stress == 1 && opening_a {
            continue;
        }
        if let Some(syllable) = pronunciation.syllables.get_mut(mark)
            && syllable.stress == Stress::Unstressed
        {
            syllable.stress = Stress::Rhythmic;
        }
    }
    Some(pronunciation)
}

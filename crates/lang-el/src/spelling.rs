//! How a Greek word is said from its spelling: the vowel digraphs (ου u, αι e, ει οι υι i), αυ ευ
//! ηυ as av ev iv before a vowel or a voiced consonant and af ef if before a voiceless one; μπ ντ
//! γκ as b d ɡ at the start of a word and mb nd ŋɡ inside it, γγ ŋɡ, τσ ts, τζ dz, ξ ks, ψ ps; κ γ
//! χ fronted to c ʝ ç before e and i; σ as z before a voiced consonant; an unstressed i-sound
//! between a consonant and a vowel as a glide (παιδιά peˈðʝa, ελιά eˈʎa, πιάνο ˈpçano); a doubled
//! consonant said once; and the stress where the accent is written, or on the last syllable but
//! one when it is not (capitals).

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::letters::{Letter, letters};

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

const A: Phoneme = vowel("a", "a");
const E: Phoneme = vowel("e", "e");
const I: Phoneme = vowel("i", "i");
const O: Phoneme = vowel("o", "o");
const U: Phoneme = vowel("u", "u");

const P: Phoneme = consonant("p", "p");
const B: Phoneme = consonant("b", "b");
const T: Phoneme = consonant("t", "t");
const D: Phoneme = consonant("d", "d");
const K: Phoneme = consonant("k", "k");
const C: Phoneme = consonant("c", "c");
const G: Phoneme = consonant("ɡ", "ɡ");
const GJ: Phoneme = consonant("ɟ", "ɟ");
const F: Phoneme = consonant("f", "f");
const V: Phoneme = consonant("v", "v");
const TH: Phoneme = consonant("θ", "θ");
const DH: Phoneme = consonant("ð", "ð");
const S: Phoneme = consonant("s", "s");
const Z: Phoneme = consonant("z", "z");
const X: Phoneme = consonant("x", "x");
const XJ: Phoneme = consonant("ç", "ç");
const GH: Phoneme = consonant("ɣ", "ɣ");
const YJ: Phoneme = consonant("ʝ", "ʝ");
const TS: Phoneme = consonant("ts", "ts");
const DZ: Phoneme = consonant("dz", "dz");
const M: Phoneme = consonant("m", "m");
const N: Phoneme = consonant("n", "n");
const NJ: Phoneme = consonant("ɲ", "ɲ");
const NG: Phoneme = consonant("ŋ", "ŋ");
const L: Phoneme = consonant("l", "l");
const LJ: Phoneme = consonant("ʎ", "ʎ");
const R: Phoneme = consonant("ɾ", "r");

/// κ, γ, χ and the stop ɡ, whose place follows the vowel after them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Velar {
    K,
    Fricative,
    X,
    Stop,
}

/// A sound of the word before the vowels after it are known.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Unit {
    /// A vowel; `glide` when it is an i-sound that may become a glide (not after a diaeresis).
    Vowel {
        phoneme: Phoneme,
        accent: bool,
        glide: bool,
    },
    /// A consonant whatever follows; `closes` when no syllable may take it with what follows (the
    /// f and v of αυ and ευ: εύ-κολος).
    Plain { phoneme: Phoneme, closes: bool },
    /// A velar, `front` when a glide has fronted it (γιατί ʝaˈti).
    Velar { velar: Velar, front: bool },
    /// σ, z before a voiced consonant.
    Sigma,
}

const fn plain(phoneme: Phoneme) -> Unit {
    Unit::Plain { phoneme, closes: false }
}

const fn velar(velar: Velar) -> Unit {
    Unit::Velar { velar, front: false }
}

fn is_vowel_letter(c: char) -> bool {
    matches!(c, 'α' | 'ε' | 'η' | 'ι' | 'ο' | 'υ' | 'ω')
}

/// The units of `letters`; `None` for a letter Greek does not read.
fn units(letters: &[Letter]) -> Option<Vec<Unit>> {
    let mut units: Vec<Unit> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let here = letters[at];
        let next = letters.get(at + 1).copied();
        // A pair of vowel letters is a digraph unless the first is accented or the second has a
        // diaeresis (τσάι, κοροϊδεύω).
        let pair = next
            .filter(|next| !here.accent && !next.diaeresis)
            .map(|next| (here.base, next));
        let vowel = |phoneme: Phoneme, accent: bool, glide: bool| Unit::Vowel { phoneme, accent, glide };
        let (new, step): (Vec<Unit>, usize) = match pair.map(|(first, next)| (first, next.base)) {
            Some(('ο', 'υ')) => (vec![vowel(U, next?.accent, false)], 2),
            Some(('α', 'ι')) => (vec![vowel(E, next?.accent, false)], 2),
            Some(('ε' | 'ο' | 'υ', 'ι')) => (vec![vowel(I, next?.accent, true)], 2),
            Some((first @ ('α' | 'ε' | 'η'), 'υ')) => {
                let quality = match first {
                    'α' => A,
                    'ε' => E,
                    _ => I,
                };
                // v before a vowel or a voiced consonant, f before a voiceless one or at the end.
                let voiced = letters
                    .get(at + 2)
                    .is_some_and(|after| is_vowel_letter(after.base) || "βγδζλμνρ".contains(after.base));
                let consonant = Unit::Plain {
                    phoneme: if voiced { V } else { F },
                    closes: true,
                };
                (vec![vowel(quality, next?.accent, false), consonant], 2)
            }
            _ => match (here.base, next.map(|next| next.base)) {
                ('α', _) => (vec![vowel(A, here.accent, false)], 1),
                ('ε', _) => (vec![vowel(E, here.accent, false)], 1),
                ('η' | 'ι' | 'υ', _) => (vec![vowel(I, here.accent, !here.diaeresis)], 1),
                ('ο' | 'ω', _) => (vec![vowel(O, here.accent, false)], 1),
                // γκ is ɡ at the start of a word and ŋɡ inside it.
                ('γ', Some('κ')) if at == 0 => (vec![velar(Velar::Stop)], 2),
                ('γ', Some('γ' | 'κ')) => (vec![plain(NG), velar(Velar::Stop)], 2),
                ('γ', Some('χ')) => (vec![plain(NG), velar(Velar::X)], 2),
                ('γ', Some('ξ')) => (vec![plain(NG), velar(Velar::K), plain(S)], 2),
                ('μ', Some('π')) if at == 0 => (vec![plain(B)], 2),
                ('μ', Some('π')) => (vec![plain(M), plain(B)], 2),
                ('ν', Some('τ')) if at == 0 => (vec![plain(D)], 2),
                ('ν', Some('τ')) => (vec![plain(N), plain(D)], 2),
                ('τ', Some('σ' | 'ς')) => (vec![plain(TS)], 2),
                ('τ', Some('ζ')) => (vec![plain(DZ)], 2),
                ('κ', _) => (vec![velar(Velar::K)], 1),
                ('γ', _) => (vec![velar(Velar::Fricative)], 1),
                ('χ', _) => (vec![velar(Velar::X)], 1),
                ('σ', _) => (vec![Unit::Sigma], 1),
                ('ξ', _) => (vec![plain(K), plain(S)], 1),
                ('ψ', _) => (vec![plain(P), plain(S)], 1),
                ('β', _) => (vec![plain(V)], 1),
                ('δ', _) => (vec![plain(DH)], 1),
                ('ζ', _) => (vec![plain(Z)], 1),
                ('θ', _) => (vec![plain(TH)], 1),
                ('λ', _) => (vec![plain(L)], 1),
                ('μ', _) => (vec![plain(M)], 1),
                ('ν', _) => (vec![plain(N)], 1),
                ('π', _) => (vec![plain(P)], 1),
                ('ρ', _) => (vec![plain(R)], 1),
                ('τ', _) => (vec![plain(T)], 1),
                ('φ', _) => (vec![plain(F)], 1),
                ('\'' | '’', _) => (Vec::new(), 1),
                _ => return None,
            },
        };
        // A doubled consonant is said once (θάλασσα, Ελλάδα).
        let doubled = step == 1 && !is_vowel_letter(here.base) && next.is_some_and(|next| next.base == here.base);
        units.extend(new);
        at += if doubled { 2 } else { step };
    }
    Some(units)
}

fn is_consonant(unit: &Unit) -> bool {
    !matches!(unit, Unit::Vowel { .. })
}

/// Whether the i-sound at `index` follows ρ and comes before o or u, where it stays a vowel
/// (αύριο ˈavɾio, κύριος, εισιτήριο; but Κυριακή ciɾʝaˈci).
fn after_r_before_back(units: &[Unit], index: usize) -> bool {
    matches!(units[index - 1], Unit::Plain { phoneme, .. } if phoneme == R)
        && matches!(units[index + 1], Unit::Vowel { phoneme, .. } if phoneme == O || phoneme == U)
}

/// Turns an unstressed i-sound between a consonant and a vowel into a glide, which the consonant
/// before it takes: a velar is fronted (γιατί), λ and ν become ʎ and ɲ (ελιά, χρόνια), μ adds ɲ
/// (μια), and any other consonant is followed by ç after a voiceless one and ʝ after a voiced one
/// (πιάνο, παιδιά).
fn glides(units: &mut Vec<Unit>) {
    let mut index = 1;
    while index + 1 < units.len() {
        let is_glide = matches!(
            units[index],
            Unit::Vowel {
                accent: false,
                glide: true,
                ..
            }
        ) && is_consonant(&units[index - 1])
            && !is_consonant(&units[index + 1])
            && !after_r_before_back(units, index);
        if !is_glide {
            index += 1;
            continue;
        }
        match units[index - 1] {
            Unit::Velar { velar, .. } => {
                units[index - 1] = Unit::Velar { velar, front: true };
                units.remove(index);
            }
            Unit::Plain { phoneme, .. } if phoneme == L || phoneme == N => {
                units[index - 1] = plain(if phoneme == L { LJ } else { NJ });
                units.remove(index);
            }
            Unit::Plain { phoneme, .. } if phoneme == M => units[index] = plain(NJ),
            Unit::Plain { phoneme, .. } if [B, D, V, DH, Z, R, DZ].contains(&phoneme) => units[index] = plain(YJ),
            _ => units[index] = plain(XJ),
        }
        index += 1;
    }
}

/// Whether `unit` is a front vowel (e or i), before which a velar is fronted.
fn is_front(unit: Option<&Unit>) -> bool {
    matches!(unit, Some(Unit::Vowel { phoneme, .. }) if *phoneme == E || *phoneme == I)
}

/// Whether `unit` is a voiced consonant, before which σ is z.
fn is_voiced(unit: Option<&Unit>) -> bool {
    match unit {
        Some(Unit::Plain { phoneme, .. }) => [B, D, V, DH, Z, M, N, NJ, L, LJ, R, YJ, DZ].contains(phoneme),
        Some(Unit::Velar { velar, .. }) => matches!(velar, Velar::Fricative | Velar::Stop),
        _ => false,
    }
}

/// Whether `cluster` can start a syllable inside a word, as it can start a Greek word: one
/// consonant; a stop or fricative before l or r (πλ, τρ, φλ, χρ); a consonant before a glide
/// (πιάνο, βιολί), mɲ (μια); s or z before a stop, f, θ, x, m, n, v or ɣ (στ, σμ, σβ); the
/// pairs πτ, κτ, φτ, χτ, θν, μν, γν, κν, πν; and s before a stop and l or r (στρ, σκλ).
fn legal_onset(cluster: &[Phoneme]) -> bool {
    let obstruent = |phoneme: &Phoneme| [P, B, T, D, K, C, G, GJ, F, V, TH, DH, X, XJ, GH, YJ].contains(phoneme);
    let sibilant = |phoneme: &Phoneme| [S, Z].contains(phoneme);
    let after_sibilant = |phoneme: &Phoneme| [P, T, K, C, F, TH, X, XJ, M, N, V, GH, YJ].contains(phoneme);
    match cluster {
        [_] => true,
        [first, second] if [L, R].contains(second) => obstruent(first),
        [_, second] if [XJ, YJ].contains(second) => true,
        [first, second] if sibilant(first) => after_sibilant(second),
        [first, second] => matches!(
            (*first, *second),
            (P | K | C | F | X | XJ, T) | (TH | M | GH | YJ | K | C | P, N) | (M, NJ)
        ),
        [first, second, third] => sibilant(first) && [P, T, K, C, F].contains(second) && [L, R].contains(third),
        _ => false,
    }
}

/// How `word` is said; `None` for a letter Greek does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let mut units = units(&letters(word))?;
    if units.is_empty() {
        return None;
    }
    glides(&mut units);
    let vowels: Vec<usize> = (0..units.len()).filter(|&index| !is_consonant(&units[index])).collect();
    let accented = vowels
        .iter()
        .copied()
        .find(|&index| matches!(units[index], Unit::Vowel { accent: true, .. }));
    let stressed = accented.or_else(|| vowels.get(vowels.len().saturating_sub(2)).copied());
    let mut segments: Vec<Segment> = Vec::with_capacity(units.len());
    let mut boundary = false;
    for (index, unit) in units.iter().enumerate() {
        let next = units.get(index + 1);
        let (phoneme, closes) = match *unit {
            Unit::Vowel { phoneme, .. } => (phoneme, false),
            Unit::Plain { phoneme, closes } => (phoneme, closes),
            Unit::Sigma => (if is_voiced(next) { Z } else { S }, false),
            Unit::Velar { velar, front } => {
                let front = front || is_front(next);
                let phoneme = match (velar, front) {
                    (Velar::K, false) => K,
                    (Velar::K, true) => C,
                    (Velar::Fricative, false) => GH,
                    (Velar::Fricative, true) => YJ,
                    (Velar::X, false) => X,
                    (Velar::X, true) => XJ,
                    (Velar::Stop, false) => G,
                    (Velar::Stop, true) => GJ,
                };
                (phoneme, false)
            }
        };
        let stress = if Some(index) == stressed {
            Stress::Primary
        } else {
            Stress::Unstressed
        };
        segments.push(Segment {
            phoneme,
            stress,
            boundary,
        });
        boundary = closes;
    }
    Some(syllabify(&segments, legal_onset))
}

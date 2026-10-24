//! How a Swahili word is said from its spelling: one letter or digraph, one sound (ch, sh, ny,
//! ng', dh, th, gh, kh), ng as ŋɡ, every vowel its own syllable, m and n syllabic before a
//! consonant they do not join (mtu, nchi, mwanamke), and the stress on the last syllable but one.
//! eSpeak NG's notation keeps to the symbols Kokoro reads (decision 48): a syllabic nasal is
//! written as the nasal alone.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

const A: Phoneme = vowel("a", "a");
const E: Phoneme = vowel("ɛ", "ɛ");
const I: Phoneme = vowel("i", "i");
const O: Phoneme = vowel("ɔ", "ɔ");
const U: Phoneme = vowel("u", "u");
/// m and n as a syllable of their own (mtu ˈm̩tu, nchi ˈn̩tʃi).
const M_SYLLABIC: Phoneme = vowel("m̩", "m");
const N_SYLLABIC: Phoneme = vowel("n̩", "n");

const B: Phoneme = consonant("b", "b");
const CH: Phoneme = consonant("tʃ", "tʃ");
const D: Phoneme = consonant("d", "d");
const DH: Phoneme = consonant("ð", "ð");
const F: Phoneme = consonant("f", "f");
const G: Phoneme = consonant("ɡ", "ɡ");
const GH: Phoneme = consonant("ɣ", "ɣ");
const H: Phoneme = consonant("h", "h");
const J: Phoneme = consonant("dʒ", "dʒ");
const K: Phoneme = consonant("k", "k");
const KH: Phoneme = consonant("x", "x");
const L: Phoneme = consonant("l", "l");
const M: Phoneme = consonant("m", "m");
const N: Phoneme = consonant("n", "n");
const NG: Phoneme = consonant("ŋ", "ŋ");
const NY: Phoneme = consonant("ɲ", "ɲ");
const P: Phoneme = consonant("p", "p");
const R: Phoneme = consonant("r", "r");
const S: Phoneme = consonant("s", "s");
const SH: Phoneme = consonant("ʃ", "ʃ");
const T: Phoneme = consonant("t", "t");
const TH: Phoneme = consonant("θ", "θ");
const V: Phoneme = consonant("v", "v");
const W: Phoneme = consonant("w", "w");
const Y: Phoneme = consonant("j", "j");
const Z: Phoneme = consonant("z", "z");

/// Whether `cluster` can start a syllable inside a word: one consonant, a consonant before w or
/// y (mbwa, kwenda), a nasal before the stop or fricative it joins (tembo, ngoma, nje, nzuri),
/// and, in loans, a stop or f before l or r (Afrika).
fn legal_onset(cluster: &[Phoneme]) -> bool {
    let glide = |phoneme: &Phoneme| [W, Y].contains(phoneme);
    let joined = |nasal: &Phoneme, next: &Phoneme| matches!((*nasal, *next), (M, B) | (N, D | J | Z) | (NG, G));
    let obstruent = |phoneme: &Phoneme| [P, B, T, D, K, G, F, V].contains(phoneme);
    match cluster {
        [_] => true,
        [_, second] if glide(second) => true,
        [first, second] if joined(first, second) => true,
        [first, second] if [L, R].contains(second) => obstruent(first),
        [first, second, third] => joined(first, second) && glide(third),
        _ => false,
    }
}

/// The sounds of `letters`, in order; `None` for a letter Swahili does not read.
fn sounds(letters: &[char]) -> Option<Vec<Phoneme>> {
    let mut sounds: Vec<Phoneme> = Vec::with_capacity(letters.len());
    let mut at = 0;
    while at < letters.len() {
        let pair: String = letters[at..letters.len().min(at + 2)].iter().collect();
        let apostrophe = matches!(letters.get(at + 2), Some('\'' | '’'));
        let (phonemes, step): (&[Phoneme], usize) = match pair.as_str() {
            // ng' is ŋ (ng'ombe), ng is ŋɡ (ngoma).
            "ng" if apostrophe => (&[NG], 3),
            "ng" => (&[NG, G], 2),
            "ny" => (&[NY], 2),
            "ch" => (&[CH], 2),
            "sh" => (&[SH], 2),
            "dh" => (&[DH], 2),
            "th" => (&[TH], 2),
            "gh" => (&[GH], 2),
            "kh" => (&[KH], 2),
            _ => (
                match letters[at] {
                    'a' | 'á' | 'à' => &[A],
                    'e' | 'é' | 'è' => &[E],
                    'i' | 'í' | 'ì' => &[I],
                    'o' | 'ó' | 'ò' => &[O],
                    'u' | 'ú' | 'ù' => &[U],
                    'b' => &[B],
                    'd' => &[D],
                    'f' => &[F],
                    'g' => &[G],
                    'h' => &[H],
                    'j' => &[J],
                    'k' | 'c' | 'q' => &[K],
                    'l' => &[L],
                    'm' => &[M],
                    'n' => &[N],
                    'p' => &[P],
                    'r' => &[R],
                    's' => &[S],
                    't' => &[T],
                    'v' => &[V],
                    'w' => &[W],
                    'x' => &[K, S],
                    'y' => &[Y],
                    'z' => &[Z],
                    '\'' | '’' => &[],
                    _ => return None,
                },
                1,
            ),
        };
        sounds.extend_from_slice(phonemes);
        at += step;
    }
    Some(sounds)
}

/// Makes m and n syllabic before a consonant they do not join: m at the start of a word before
/// any consonant but b, w and y (mtu, mvua), and inside one before a voiceless consonant
/// (mwanamke, but chumvi, zungumza); n at the start of a word before any but d, j, z and w (nchi,
/// nne); and a word's first nasal, joined or not, when the word has only one vowel (mbwa ˈm̩bwa,
/// nzi ˈn̩zi).
fn syllabic_nasals(sounds: &mut [Phoneme]) {
    const VOICELESS: [Phoneme; 10] = [P, T, K, S, SH, CH, F, TH, KH, H];
    let vowels = sounds.iter().filter(|phoneme| phoneme.is_vowel()).count();
    for index in 0..sounds.len() {
        let Some(&next) = sounds.get(index + 1) else { break };
        if next.is_vowel() || [W, Y].contains(&next) {
            continue;
        }
        let alone = index == 0 && vowels == 1;
        let m_syllabic = if index == 0 {
            next != B
        } else {
            VOICELESS.contains(&next)
        };
        match sounds[index] {
            M if alone || m_syllabic => sounds[index] = M_SYLLABIC,
            N if alone || (index == 0 && ![D, J, Z].contains(&next)) => sounds[index] = N_SYLLABIC,
            NG if alone && next == G => sounds[index] = N_SYLLABIC,
            _ => {}
        }
    }
}

/// How `word` is said, stressed on its last syllable but one; `None` for a letter Swahili does not
/// read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let letters: Vec<char> = word.to_lowercase().chars().collect();
    let mut sounds = sounds(&letters)?;
    syllabic_nasals(&mut sounds);
    let vowels: Vec<usize> = (0..sounds.len()).filter(|&index| sounds[index].is_vowel()).collect();
    let &stressed = vowels.get(vowels.len().saturating_sub(2))?;
    let segments: Vec<Segment> = sounds
        .iter()
        .enumerate()
        .map(|(index, phoneme)| {
            Segment::new(
                *phoneme,
                if index == stressed {
                    Stress::Primary
                } else {
                    Stress::Unstressed
                },
            )
        })
        .collect();
    Some(syllabify(&segments, legal_onset))
}

//! Mandarin's phonemes and how a pinyin syllable is said: its initial, glide, vowels, final nasal
//! and tone. Plain IPA follows the usual description of Standard Mandarin; misaki's notation
//! writes some of them its own way (ꭧ for zh, ɻ for r, ɨ for the apical vowels).

use jespersen_phonemes::{Phoneme, Syllable, Tone};

use crate::pinyin::{Initial, Pinyin, Rime};

const fn consonant(ipa: &'static str, misaki: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, ipa).misaki(misaki)
}

const fn vowel(ipa: &'static str, misaki: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, ipa).misaki(misaki)
}

const P: Phoneme = consonant("p", "p");
const PH: Phoneme = consonant("pʰ", "pʰ");
const M: Phoneme = consonant("m", "m");
const F: Phoneme = consonant("f", "f");
const T: Phoneme = consonant("t", "t");
const TH: Phoneme = consonant("tʰ", "tʰ");
const N: Phoneme = consonant("n", "n");
const L: Phoneme = consonant("l", "l");
const K: Phoneme = consonant("k", "k");
const KH: Phoneme = consonant("kʰ", "kʰ");
const X: Phoneme = consonant("x", "x");
const TC: Phoneme = consonant("tɕ", "ʨ");
const TCH: Phoneme = consonant("tɕʰ", "ʨʰ");
const C: Phoneme = consonant("ɕ", "ɕ");
const TS_RETROFLEX: Phoneme = consonant("ʈʂ", "ꭧ");
const TSH_RETROFLEX: Phoneme = consonant("ʈʂʰ", "ꭧʰ");
const S_RETROFLEX: Phoneme = consonant("ʂ", "ʂ");
const R: Phoneme = consonant("ʐ", "ɻ");
const TS: Phoneme = consonant("ts", "ʦ");
const TSH: Phoneme = consonant("tsʰ", "ʦʰ");
const S: Phoneme = consonant("s", "s");
const NG: Phoneme = consonant("ŋ", "ŋ");
/// The glides before a vowel: i, u and ü.
const GLIDE_I: Phoneme = consonant("j", "j");
const GLIDE_U: Phoneme = consonant("w", "w");
const GLIDE_V: Phoneme = consonant("ɥ", "ɥ");

const A: Phoneme = vowel("a", "a");
const O: Phoneme = vowel("o", "o");
const OPEN_O: Phoneme = vowel("ɔ", "ɔ");
const E_BACK: Phoneme = vowel("ɤ", "ɤ");
const SCHWA: Phoneme = vowel("ə", "ə");
const E: Phoneme = vowel("e", "e");
const OPEN_E: Phoneme = vowel("ɛ", "ɛ");
const I: Phoneme = vowel("i", "i");
const U: Phoneme = vowel("u", "u");
const V: Phoneme = vowel("y", "y");
const UPSILON: Phoneme = vowel("ʊ", "ʊ");
const ER: Phoneme = vowel("ɚ", "ɚ");
/// The apical vowel after zh, ch, sh and r, and after z, c and s.
const APICAL_RETROFLEX: Phoneme = vowel("ɻ̩", "ɨ");
const APICAL_DENTAL: Phoneme = vowel("ɹ̩", "ɨ");
/// Syllabic nasals (呣, 嗯).
const SYLLABIC_M: Phoneme = vowel("m̩", "m");
const SYLLABIC_N: Phoneme = vowel("n̩", "n");
const SYLLABIC_NG: Phoneme = vowel("ŋ̍", "ŋ");

/// The four tones' pitch (Chao's levels) and the neutral tone without its pitch, which depends
/// on the syllable before (see the sandhi module).
pub(crate) const TONES: [Tone; 5] = [
    Tone::new(&[5, 5]),
    Tone::new(&[3, 5]),
    Tone::new(&[2, 1, 4]),
    Tone::new(&[5, 1]),
    Tone::neutral(&[]),
];

/// How a syllable in pinyin is said, with its tone as written.
pub(crate) fn syllable(pinyin: Pinyin) -> Syllable {
    let initial = match pinyin.initial {
        Initial::None => None,
        Initial::B => Some(P),
        Initial::P => Some(PH),
        Initial::M => Some(M),
        Initial::F => Some(F),
        Initial::D => Some(T),
        Initial::T => Some(TH),
        Initial::N => Some(N),
        Initial::L => Some(L),
        Initial::G => Some(K),
        Initial::K => Some(KH),
        Initial::H => Some(X),
        Initial::J => Some(TC),
        Initial::Q => Some(TCH),
        Initial::X => Some(C),
        Initial::Zh => Some(TS_RETROFLEX),
        Initial::Ch => Some(TSH_RETROFLEX),
        Initial::Sh => Some(S_RETROFLEX),
        Initial::R => Some(R),
        Initial::Z => Some(TS),
        Initial::C => Some(TSH),
        Initial::S => Some(S),
    };
    let apical = if matches!(pinyin.initial, Initial::Z | Initial::C | Initial::S) {
        APICAL_DENTAL
    } else {
        APICAL_RETROFLEX
    };
    let (glide, nucleus, coda): (Option<Phoneme>, &[Phoneme], Option<Phoneme>) = match pinyin.rime {
        Rime::A => (None, &[A], None),
        // bo, po, mo, fo and lo are said with a u glide: pwo.
        Rime::O if initial.is_some() => (Some(GLIDE_U), &[O], None),
        Rime::O => (None, &[OPEN_O], None),
        Rime::E => (None, &[E_BACK], None),
        Rime::Eh => (None, &[OPEN_E], None),
        Rime::Ai => (None, &[A, I], None),
        Rime::Ei => (None, &[E, I], None),
        Rime::Ao => (None, &[A, U], None),
        Rime::Ou => (None, &[O, U], None),
        Rime::An => (None, &[A], Some(N)),
        Rime::En => (None, &[SCHWA], Some(N)),
        Rime::Ang => (None, &[A], Some(NG)),
        Rime::Eng => (None, &[SCHWA], Some(NG)),
        Rime::Ong => (None, &[UPSILON], Some(NG)),
        Rime::Er => (None, &[ER], None),
        Rime::I => (None, &[I], None),
        Rime::Apical => (None, std::slice::from_ref(&apical), None),
        Rime::Ia => (Some(GLIDE_I), &[A], None),
        Rime::Ie => (Some(GLIDE_I), &[E], None),
        Rime::Iao => (Some(GLIDE_I), &[A, U], None),
        Rime::Iou => (Some(GLIDE_I), &[O, U], None),
        Rime::Ian => (Some(GLIDE_I), &[OPEN_E], Some(N)),
        Rime::In => (None, &[I], Some(N)),
        Rime::Iang => (Some(GLIDE_I), &[A], Some(NG)),
        Rime::Ing => (None, &[I], Some(NG)),
        Rime::Iong => (Some(GLIDE_I), &[UPSILON], Some(NG)),
        Rime::Io => (Some(GLIDE_I), &[OPEN_O], None),
        Rime::Iai => (Some(GLIDE_I), &[A, I], None),
        Rime::U => (None, &[U], None),
        Rime::Ua => (Some(GLIDE_U), &[A], None),
        Rime::Uo => (Some(GLIDE_U), &[O], None),
        Rime::Uai => (Some(GLIDE_U), &[A, I], None),
        Rime::Uei => (Some(GLIDE_U), &[E, I], None),
        Rime::Uan => (Some(GLIDE_U), &[A], Some(N)),
        Rime::Uen => (Some(GLIDE_U), &[SCHWA], Some(N)),
        Rime::Uang => (Some(GLIDE_U), &[A], Some(NG)),
        Rime::Ueng => (Some(GLIDE_U), &[SCHWA], Some(NG)),
        Rime::V => (None, &[V], None),
        Rime::Ve => (Some(GLIDE_V), &[E], None),
        Rime::Van => (Some(GLIDE_V), &[OPEN_E], Some(N)),
        Rime::Vn => (None, &[V], Some(N)),
        Rime::M => (None, &[SYLLABIC_M], None),
        Rime::N => (None, &[SYLLABIC_N], None),
        Rime::Ng => (None, &[SYLLABIC_NG], None),
    };
    Syllable {
        onset: initial.into_iter().chain(glide).collect(),
        nucleus: nucleus.to_vec(),
        coda: coda.into_iter().collect(),
        tone: Some(TONES[usize::from(pinyin.tone.clamp(1, 5)) - 1]),
        ..Syllable::default()
    }
}

/// The r-colouring of erhua, which ends a syllable.
const RHOTIC: Phoneme = consonant("ɻ", "ɻ");

/// A syllable with erhua (儿 said as part of it, 哪儿 nǎr, 一点儿 yìdiǎnr): its final nasal or
/// glide gives way to the r-colouring (diǎn → diǎr, with its vowel opened: tjaɻ; kuài → kuàr), a high vowel takes a schwa
/// before it (jīn → jiēr, as said: tɕiəɻ), and the apical vowel becomes a schwa (zhī → zhēr).
pub(crate) fn with_erhua(pinyin: Pinyin) -> Syllable {
    let mut syllable = syllable(pinyin);
    match pinyin.rime {
        Rime::Ai | Rime::Ei | Rime::Uai | Rime::Uei | Rime::Iai => {
            syllable.nucleus.pop();
        }
        Rime::I | Rime::In | Rime::Ing | Rime::V | Rime::Vn => syllable.nucleus.push(SCHWA),
        Rime::Apical => syllable.nucleus = vec![SCHWA],
        Rime::Ian | Rime::Van => syllable.nucleus = vec![A],
        _ => {}
    }
    syllable.coda = vec![RHOTIC];
    syllable
}

//! Russian phonemes in plain IPA and eSpeak NG's notation, which writes the hard l as ɭ, a
//! stressed a as ɑ, ё after a soft consonant as ɵ, and keeps the letter an unstressed vowel was
//! written with (о as ʌ, а as a or ʌ, е as i).

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

/// A stressed a, ɑ in eSpeak NG's notation (сказал skazˈɑɭ).
pub(crate) const A_STRESSED: Phoneme = vowel("a", "ɑ");
/// An unstressed а: ɐ right before the stress or at the start, a in eSpeak NG's notation
/// (стакан stakˈɑn); ə elsewhere, ʌ in it (карандаш kʌrandˈɑʃ), but a at the end of a word
/// (книга knʲˈiɡa).
pub(crate) const A_PRETONIC: Phoneme = vowel("ɐ", "a");
pub(crate) const A_REDUCED: Phoneme = vowel("ə", "ʌ");
pub(crate) const A_FINAL: Phoneme = vowel("ə", "a");
/// An unstressed я, or а after ч and щ: ɪ before the end, ə at it (время vrʲˈemʲʌ); right
/// before the stress a in eSpeak NG's notation (пятнадцать pʲatnˈɑttsʌtʲ).
pub(crate) const YA_PRETONIC: Phoneme = vowel("ɪ", "a");
pub(crate) const YA_REDUCED: Phoneme = vowel("ɪ", "ʌ");
pub(crate) const YA_FINAL: Phoneme = vowel("ə", "ʌ");
/// я and ю for j and a vowel at the start, after a vowel and after ь and ъ, which eSpeak NG
/// writes as one vowel (я ˈja, поют pʌˈjut, новая nˈovʌja).
pub(crate) const YA_J: Phoneme = vowel("ja", "ja");
pub(crate) const YA_J_REDUCED: Phoneme = vowel("jɪ", "ja");
pub(crate) const YA_J_FINAL: Phoneme = vowel("jə", "ja");
pub(crate) const YU_J: Phoneme = vowel("ju", "ju");
pub(crate) const YU_J_REDUCED: Phoneme = vowel("jʊ", "ju");
pub(crate) const O_STRESSED: Phoneme = vowel("o", "o");
/// ё after a soft consonant or j, ɵ in eSpeak NG's notation (ёлка jˈɵɭka).
pub(crate) const YO: Phoneme = vowel("o", "ɵ");
/// An unstressed о: ɐ right before the stress or at the start, ə elsewhere, ʌ in eSpeak NG's
/// notation (молоко mʌɭʌkˈo).
pub(crate) const O_PRETONIC: Phoneme = vowel("ɐ", "ʌ");
pub(crate) const O_REDUCED: Phoneme = vowel("ə", "ʌ");
/// The vowel of an unstressed function word, said as written in eSpeak NG's notation (что ʃto,
/// за za).
pub(crate) const O_CLITIC: Phoneme = vowel("ɐ", "o");
pub(crate) const A_CLITIC: Phoneme = vowel("ɐ", "a");
/// на, said with ə (на столе nə stʌɭʲˈe).
pub(crate) const SCHWA: Phoneme = vowel("ə", "ə");
pub(crate) const E_STRESSED: Phoneme = vowel("e", "e");
/// A stressed е after ж, ш and ц, and э (шесть ʃˈɛstʲ, это ˈɛtʌ).
pub(crate) const E_OPEN: Phoneme = vowel("ɛ", "ɛ");
/// An unstressed е after a soft consonant, i in eSpeak NG's notation (сестра sʲistrˈɑ).
pub(crate) const E_REDUCED: Phoneme = vowel("ɪ", "i");
/// An unstressed е after j, and an unstressed и at the start or end of a word (его jɪvˈo, книги
/// knʲˈiɡʲɪ).
pub(crate) const I_LAX: Phoneme = vowel("ɪ", "ɪ");
pub(crate) const I: Phoneme = vowel("i", "i");
/// A stressed и starting a word, ɪ in eSpeak NG's notation (их ˈɪx).
pub(crate) const I_START: Phoneme = vowel("i", "ɪ");
/// An unstressed э, ɛ in eSpeak NG's notation (энергия ɛnˈɛrɡʲiɪ).
pub(crate) const E_UNSTRESSED: Phoneme = vowel("ɪ", "ɛ");
/// A stressed и at the end of a word, ɪ in eSpeak NG's notation (три trʲˈɪ).
pub(crate) const I_FINAL: Phoneme = vowel("i", "ɪ");
/// An unstressed и inside a word (читать tʃʲitˈɑtʲ).
pub(crate) const I_REDUCED: Phoneme = vowel("ɪ", "i");
/// ы, and и and an unstressed е after ж, ш and ц (жизнь ʒˈyʑn, жена ʒynˈɑ).
pub(crate) const Y: Phoneme = vowel("ɨ", "y");
pub(crate) const U: Phoneme = vowel("u", "u");
pub(crate) const U_REDUCED: Phoneme = vowel("ʊ", "u");
/// ю after a consonant, u" in eSpeak NG's notation (люблю ɭʲu"bɭʲˈu").
pub(crate) const YU: Phoneme = vowel("u", "u\"");
pub(crate) const YU_REDUCED: Phoneme = vowel("ʊ", "u\"");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const Z: Phoneme = consonant("z", "z");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const X: Phoneme = consonant("x", "x");
pub(crate) const L: Phoneme = consonant("ɫ", "ɭ");
pub(crate) const ZH: Phoneme = consonant("ʐ", "ʒ");
pub(crate) const SH: Phoneme = consonant("ʂ", "ʃ");
pub(crate) const TS: Phoneme = consonant("ts", "ts");
pub(crate) const J: Phoneme = consonant("j", "j");

pub(crate) const P_SOFT: Phoneme = consonant("pʲ", "pʲ");
pub(crate) const B_SOFT: Phoneme = consonant("bʲ", "bʲ");
pub(crate) const T_SOFT: Phoneme = consonant("tʲ", "tʲ");
pub(crate) const D_SOFT: Phoneme = consonant("dʲ", "dʲ");
pub(crate) const K_SOFT: Phoneme = consonant("kʲ", "kʲ");
pub(crate) const G_SOFT: Phoneme = consonant("ɡʲ", "ɡʲ");
pub(crate) const F_SOFT: Phoneme = consonant("fʲ", "fʲ");
pub(crate) const V_SOFT: Phoneme = consonant("vʲ", "vʲ");
pub(crate) const S_SOFT: Phoneme = consonant("sʲ", "sʲ");
/// A soft z, ʑ in eSpeak NG's notation (зима ʑimˈɑ), also softened by a soft d or n after it
/// (здесь ʑdʲˈesʲ).
pub(crate) const Z_SOFT: Phoneme = consonant("zʲ", "ʑ");
pub(crate) const M_SOFT: Phoneme = consonant("mʲ", "mʲ");
pub(crate) const N_SOFT: Phoneme = consonant("nʲ", "nʲ");
pub(crate) const R_SOFT: Phoneme = consonant("rʲ", "rʲ");
pub(crate) const X_SOFT: Phoneme = consonant("xʲ", "xʲ");
pub(crate) const L_SOFT: Phoneme = consonant("lʲ", "ɭʲ");
/// н and л softened by ь, which eSpeak NG writes hard (день dʲˈen, только tˈoɭkʌ); р
/// softened by ь before a consonant, rɪ^ in it (борьба bʌrɪ^bˈɑ); в softened by ь, v in it
/// (кровью krˈovjju), f at the end (церковь tsˈɛrkʌf).
pub(crate) const N_SOFT_SIGN: Phoneme = consonant("nʲ", "n");
pub(crate) const L_SOFT_SIGN: Phoneme = consonant("lʲ", "ɭ");
pub(crate) const R_SOFT_SIGN: Phoneme = consonant("rʲ", "rɪ^");
pub(crate) const V_SOFT_SIGN: Phoneme = consonant("vʲ", "v");
pub(crate) const F_SOFT_SIGN: Phoneme = consonant("fʲ", "f");
/// The j eSpeak NG writes for ь or ъ before я, ю and е, besides theirs (семья sʲimʲjˈja);
/// plain IPA has the one j.
pub(crate) const J_SIGN: Phoneme = consonant("", "j");
/// The л of -лнц-, not said, which eSpeak NG writes (солнце sˈoɭntsy).
pub(crate) const L_SILENT: Phoneme = consonant("", "ɭ");
/// ч and щ, always soft.
pub(crate) const CH: Phoneme = consonant("tɕ", "tʃʲ");
pub(crate) const SHCH: Phoneme = consonant("ɕː", "ɕ");

/// A word's last consonant voiced by a voiced obstruent starting the next word: in plain IPA
/// only, as eSpeak NG keeps each word's own final voicing (из дома ɪz ˈdomə, eSpeak NG ɪs dˈoma).
const VOICED_BEFORE_NEXT: [(Phoneme, Phoneme); 7] = [
    (P, consonant("b", "p")),
    (T, consonant("d", "t")),
    (K, consonant("ɡ", "k")),
    (F, consonant("v", "f")),
    (S, consonant("z", "s")),
    (SH, consonant("ʐ", "ʃ")),
    (T_SOFT, consonant("dʲ", "tʲ")),
];

/// The voiceless and voiced pairs.
const PAIRS: [(Phoneme, Phoneme); 12] = [
    (P, B),
    (T, D),
    (K, G),
    (F, V),
    (S, Z),
    (SH, ZH),
    (P_SOFT, B_SOFT),
    (T_SOFT, D_SOFT),
    (K_SOFT, G_SOFT),
    (F_SOFT, V_SOFT),
    (S_SOFT, Z_SOFT),
    (F_SOFT_SIGN, V_SOFT_SIGN),
];

/// The soft consonant of a hard one; ж, ш and ц stay hard.
pub(crate) fn soften(phoneme: Phoneme) -> Phoneme {
    const SOFT: [(Phoneme, Phoneme); 15] = [
        (P, P_SOFT),
        (B, B_SOFT),
        (T, T_SOFT),
        (D, D_SOFT),
        (K, K_SOFT),
        (G, G_SOFT),
        (F, F_SOFT),
        (V, V_SOFT),
        (S, S_SOFT),
        (Z, Z_SOFT),
        (M, M_SOFT),
        (N, N_SOFT),
        (R, R_SOFT),
        (X, X_SOFT),
        (L, L_SOFT),
    ];
    SOFT.iter()
        .find(|(hard, _)| *hard == phoneme)
        .map_or(phoneme, |(_, soft)| *soft)
}

/// Whether `phoneme` is an obstruent that voicing changes or that changes voicing.
pub(crate) fn is_obstruent(phoneme: Phoneme) -> bool {
    PAIRS
        .iter()
        .any(|(voiceless, voiced)| *voiceless == phoneme || *voiced == phoneme)
        || [TS, CH, SHCH, X, X_SOFT].contains(&phoneme)
}

/// Whether `phoneme` is a voiced obstruent.
pub(crate) fn is_voiced(phoneme: Phoneme) -> bool {
    PAIRS.iter().any(|(_, voiced)| *voiced == phoneme)
}

/// `phoneme` said voiceless (дуб dˈup).
pub(crate) fn devoice(phoneme: Phoneme) -> Phoneme {
    PAIRS
        .iter()
        .find(|(_, voiced)| *voiced == phoneme)
        .map_or(phoneme, |(voiceless, _)| *voiceless)
}

/// `phoneme` said voiced (вокзал vʌɡzˈɑɭ).
pub(crate) fn voice(phoneme: Phoneme) -> Phoneme {
    PAIRS
        .iter()
        .find(|(voiceless, _)| *voiceless == phoneme)
        .map_or(phoneme, |(_, voiced)| *voiced)
}

/// A word's last consonant voiced by the next word, in plain IPA only.
pub(crate) fn voiced_before_next(phoneme: Phoneme) -> Phoneme {
    VOICED_BEFORE_NEXT
        .iter()
        .find(|(voiceless, _)| *voiceless == phoneme)
        .map_or(phoneme, |(_, voiced)| *voiced)
}

/// Whether `cluster` can start a syllable inside a word: obstruents, then sonorants, as a word
/// can start (встреча, здравствуйте); j only on its own or after a consonant (объём).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let sonorant = |phoneme: &Phoneme| {
        [
            M,
            N,
            R,
            L,
            M_SOFT,
            N_SOFT,
            R_SOFT,
            L_SOFT,
            N_SOFT_SIGN,
            L_SOFT_SIGN,
            R_SOFT_SIGN,
            L_SILENT,
        ]
        .contains(phoneme)
    };
    let mut seen_sonorant = false;
    for (index, phoneme) in cluster.iter().enumerate() {
        if *phoneme == J {
            return index == cluster.len() - 1;
        }
        if *phoneme == J_SIGN {
            return cluster[index + 1..].iter().all(|after| *after == J) && cluster.len() - index <= 2;
        }
        if sonorant(phoneme) {
            seen_sonorant = true;
        } else if seen_sonorant {
            return false;
        }
    }
    cluster.len() <= 4
}

//! Icelandic phonemes in plain IPA and eSpeak NG's notation.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

/// a, á (au), e, i and y, í and ý (i), o, ó (ou), u (ʏ, y in eSpeak NG's notation), ú (u), ö, æ
/// (ai), au (œy), ei and ey; each short and long.
pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const A_LONG: Phoneme = vowel("aː", "aː");
pub(crate) const AU: Phoneme = vowel("au", "aʊ");
pub(crate) const AU_LONG: Phoneme = vowel("auː", "aʊː");
pub(crate) const E: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const E_LONG: Phoneme = vowel("ɛː", "ɛː");
pub(crate) const I: Phoneme = vowel("ɪ", "ɪ");
pub(crate) const I_LONG: Phoneme = vowel("ɪː", "ɪː");
pub(crate) const II: Phoneme = vowel("i", "i");
pub(crate) const II_LONG: Phoneme = vowel("iː", "iː");
pub(crate) const O: Phoneme = vowel("ɔ", "ɔ");
pub(crate) const O_LONG: Phoneme = vowel("ɔː", "ɔː");
/// o, stressed, before a g said j (bogi).
pub(crate) const OI: Phoneme = vowel("ɔi", "ɔɪ");
pub(crate) const OU: Phoneme = vowel("ou", "oʊ");
pub(crate) const OU_LONG: Phoneme = vowel("ouː", "oʊː");
pub(crate) const U: Phoneme = vowel("ʏ", "y");
pub(crate) const U_LONG: Phoneme = vowel("ʏː", "yː");
pub(crate) const UU: Phoneme = vowel("u", "u");
pub(crate) const UU_LONG: Phoneme = vowel("uː", "uː");
pub(crate) const OE: Phoneme = vowel("œ", "œ");
pub(crate) const OE_LONG: Phoneme = vowel("œː", "œː");
pub(crate) const AI: Phoneme = vowel("ai", "aɪ");
pub(crate) const AI_LONG: Phoneme = vowel("aiː", "aɪː");
pub(crate) const OEY: Phoneme = vowel("œy", "øy");
pub(crate) const OEY_LONG: Phoneme = vowel("œyː", "øyː");
pub(crate) const EI: Phoneme = vowel("ei", "eɪ");
pub(crate) const EI_LONG: Phoneme = vowel("eiː", "eɪː");
/// The ə eSpeak NG says between a stop or þ and an r (brauð bərˈøyːð, akra), not a syllable of
/// its own and nothing in plain IPA.
pub(crate) const SCHWA: Phoneme = consonant("", "ə");

/// p, t, k and the palatal k at the start of a word, aspirated (written plain in eSpeak NG's
/// notation), and b, d, g and p, t, k elsewhere, unaspirated (written b, d, ɡ, ɟ in it).
pub(crate) const P_ASP: Phoneme = consonant("pʰ", "p");
pub(crate) const T_ASP: Phoneme = consonant("tʰ", "t");
pub(crate) const K_ASP: Phoneme = consonant("kʰ", "k");
pub(crate) const C_ASP: Phoneme = consonant("cʰ", "c");
pub(crate) const P: Phoneme = consonant("p", "b");
pub(crate) const T: Phoneme = consonant("t", "d");
pub(crate) const K: Phoneme = consonant("k", "ɡ");
pub(crate) const C: Phoneme = consonant("c", "ɟ");
/// The t of ll before a voiceless l, written t (bíll bˈitl#).
pub(crate) const T_PLAIN: Phoneme = consonant("t", "t");
/// The k of x, written k (sex sˈɛks).
pub(crate) const K_PLAIN: Phoneme = consonant("k", "k");
/// The palatal k of kj inside a word, written c (kirkja).
pub(crate) const C_PLAIN: Phoneme = consonant("c", "c");
/// Preaspiration, before a doubled stop and before tl and tn (ekki ˈɛhɟɪ, litla lˈɪhdla).
pub(crate) const PREASPIRATION: Phoneme = consonant("ʰ", "h");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "ʋ");
pub(crate) const S: Phoneme = consonant("s", "s");
pub(crate) const TH: Phoneme = consonant("θ", "θ");
pub(crate) const DH: Phoneme = consonant("ð", "ð");
pub(crate) const GH: Phoneme = consonant("ɣ", "ɣ");
pub(crate) const X: Phoneme = consonant("x", "x");
pub(crate) const CC: Phoneme = consonant("ç", "ç");
pub(crate) const H: Phoneme = consonant("h", "h");
pub(crate) const J: Phoneme = consonant("j", "j");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
/// n before a palatal g or k (lengi lˈeɪɲɟɪ).
pub(crate) const NY: Phoneme = consonant("ɲ", "ɲ");
/// m, n, ŋ and ɲ voiceless before a stop, which eSpeak NG writes m#, n#, ŋ# and ɲ# (lampi
/// lˈam#bɪ, banki bˈaʊɲ#ɟɪ).
pub(crate) const M_VOICELESS: Phoneme = consonant("m̥", "m#");
pub(crate) const NG_VOICELESS: Phoneme = consonant("ŋ̊", "ŋ#");
pub(crate) const NY_VOICELESS: Phoneme = consonant("ɲ̊", "ɲ#");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const R: Phoneme = consonant("r", "r");
/// r between vowels, which eSpeak NG writes ɾr (fara fˈaːɾra).
pub(crate) const R_BETWEEN: Phoneme = consonant("r", "ɾr");
/// The voiceless n, l and r, which eSpeak NG writes n#, l# and r# (barn bˈardn#, sól sˈoʊːl#),
/// hn# after a t or a p and at the start (vatn ʋˈahdhn#, hnífur), and rr# before a voiceless stop
/// (kort kˈɔrr#d), hr# at the start (hratt).
pub(crate) const N_VOICELESS: Phoneme = consonant("n̥", "n#");
pub(crate) const N_VOICELESS_H: Phoneme = consonant("n̥", "hn#");
pub(crate) const L_VOICELESS: Phoneme = consonant("l̥", "l#");
pub(crate) const R_VOICELESS: Phoneme = consonant("r̥", "rr#");
pub(crate) const R_VOICELESS_H: Phoneme = consonant("r̥", "hr#");

/// The phoneme said again for a doubled consonant between vowels: ː in plain IPA and the
/// consonant itself in eSpeak NG's notation (mamma mˈamma, kenna cˈɛnna).
pub(crate) fn doubled(phoneme: Phoneme) -> Phoneme {
    match phoneme {
        phoneme if phoneme == M => consonant("ː", "m"),
        phoneme if phoneme == N => consonant("ː", "n"),
        phoneme if phoneme == L => consonant("ː", "l"),
        phoneme if phoneme == R => consonant("ː", "r"),
        phoneme if phoneme == S => consonant("ː", "s"),
        _ => consonant("ː", ""),
    }
}

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop, f or þ before r,
/// l, j or v, s before a stop and that pair before r or j, and a consonant before j (hjálp).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let stop = |phoneme: &Phoneme| [P_ASP, T_ASP, K_ASP, C_ASP, P, T, K, C, F, TH].contains(phoneme);
    match cluster {
        [_] => true,
        [first, second] if [R, L, V, J, R_BETWEEN].contains(second) => stop(first) || *second == J,
        [first, second] => *first == S && stop(second),
        [first, second, third] => *first == S && stop(second) && [R, J, V].contains(third),
        _ => false,
    }
}

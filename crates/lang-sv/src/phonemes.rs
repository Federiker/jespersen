//! Swedish phonemes in plain IPA and eSpeak NG's notation.

use jespersen_phonemes::Phoneme;

const fn consonant(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::consonant(ipa, espeak)
}

const fn vowel(ipa: &'static str, espeak: &'static str) -> Phoneme {
    Phoneme::vowel(ipa, espeak)
}

pub(crate) const A: Phoneme = vowel("a", "a");
pub(crate) const A_LONG: Phoneme = vowel("ɑː", "ɑː");
pub(crate) const E: Phoneme = vowel("ɛ", "ɛ");
pub(crate) const E_LONG: Phoneme = vowel("eː", "eː");
/// An unstressed e in a last syllable (pojke pˈɔjkə).
pub(crate) const SCHWA: Phoneme = vowel("ə", "ə");
pub(crate) const I: Phoneme = vowel("ɪ", "ɪ");
pub(crate) const I_LONG: Phoneme = vowel("iː", "iː");
/// o: long uː, short ɔ (bok, kort); å: long oː, short ɔ.
pub(crate) const O: Phoneme = vowel("ɔ", "ɔ");
pub(crate) const OO: Phoneme = vowel("uː", "uː");
pub(crate) const AA: Phoneme = vowel("oː", "oː");
/// u: long ʉː, which eSpeak NG writes ʉ (hus hˈʉs), short ɵ.
pub(crate) const U: Phoneme = vowel("ɵ", "ɵ");
pub(crate) const U_LONG: Phoneme = vowel("ʉː", "ʉ");
/// y: long yː, short ʏ (y in eSpeak NG's notation).
pub(crate) const Y: Phoneme = vowel("ʏ", "y");
pub(crate) const Y_LONG: Phoneme = vowel("yː", "yː");
/// ä: long ɛː, short ɛ; ö: long øː, short œ.
pub(crate) const AE_LONG: Phoneme = vowel("ɛː", "ɛː");
pub(crate) const OE: Phoneme = vowel("œ", "œ");
pub(crate) const OE_LONG: Phoneme = vowel("øː", "øː");

pub(crate) const P: Phoneme = consonant("p", "p");
pub(crate) const B: Phoneme = consonant("b", "b");
pub(crate) const T: Phoneme = consonant("t", "t");
pub(crate) const D: Phoneme = consonant("d", "d");
pub(crate) const K: Phoneme = consonant("k", "k");
pub(crate) const G: Phoneme = consonant("ɡ", "ɡ");
pub(crate) const F: Phoneme = consonant("f", "f");
pub(crate) const V: Phoneme = consonant("v", "v");
pub(crate) const S: Phoneme = consonant("s", "s");
/// The sj sound, sx in eSpeak NG's notation (sjö sxˈøː), and the tj sound (tjugo ɕˈʉɡuː).
pub(crate) const SJ: Phoneme = consonant("ɧ", "sx");
pub(crate) const TJ: Phoneme = consonant("ɕ", "ɕ");
/// rs and rt (förstå fˈœʂtoː, kort kˈɔt).
pub(crate) const RS: Phoneme = consonant("ʂ", "ʂ");
pub(crate) const RT: Phoneme = consonant("ʈ", "t");
pub(crate) const H: Phoneme = consonant("h", "h");
pub(crate) const M: Phoneme = consonant("m", "m");
pub(crate) const N: Phoneme = consonant("n", "n");
pub(crate) const NG: Phoneme = consonant("ŋ", "ŋ");
pub(crate) const L: Phoneme = consonant("l", "l");
pub(crate) const R: Phoneme = consonant("r", "r");
pub(crate) const J: Phoneme = consonant("j", "j");
/// The length of a doubled stop between vowels (vatten vˈatːən).
pub(crate) const LONG: Phoneme = consonant("ː", "ː");

// Sounds NST's words have that the rules do not tell apart: in plain IPA as NST says them, in
// eSpeak NG's notation as the rules write the same letters.
/// A short close o, written ɔ as the rules write a short o (positivt pʊsɪtˈiːft).
pub(crate) const O_CLOSE: Phoneme = vowel("ʊ", "ɔ");
/// A final unstressed close o, written uː (äro ˈɛːruː).
pub(crate) const O_FINAL: Phoneme = vowel("ʊ", "uː");
/// An unstressed i before a final consonant, written iː (varit vˈɑːriːt).
pub(crate) const I_FINAL: Phoneme = vowel("ɪ", "iː");
/// A short unstressed å (NST's o without length).
pub(crate) const AA_SHORT: Phoneme = vowel("o", "ɔ");
/// A short ʉ (förut, industri).
pub(crate) const U_CENTRAL: Phoneme = vowel("ʉ", "ʉ");
/// A short ø, written œ as the rules write a short ö; a long œː (före r), written øː.
pub(crate) const OE_CLOSE: Phoneme = vowel("ø", "œ");
pub(crate) const OE_OPEN_LONG: Phoneme = vowel("œː", "øː");
/// The retroflex consonants after a vowel, written as their letters (barn bˈɑːrn, bord), and
/// those after another retroflex, written without the r (bords).
pub(crate) const RD: Phoneme = consonant("ɖ", "rd");
pub(crate) const RN: Phoneme = consonant("ɳ", "rn");
pub(crate) const RL: Phoneme = consonant("ɭ", "rl");
pub(crate) const RD_AFTER: Phoneme = consonant("ɖ", "d");
pub(crate) const RN_AFTER: Phoneme = consonant("ɳ", "n");
pub(crate) const RL_AFTER: Phoneme = consonant("ɭ", "l");
pub(crate) const RS_AFTER: Phoneme = consonant("ʂ", "s");

/// Whether `cluster` can start a syllable inside a word: one consonant, a stop or f before l, r
/// or j, s before a stop, m, n, l or v (skola, svar), str, spr and skr, and k before v (kvinna).
pub(crate) fn legal_onset(cluster: &[Phoneme]) -> bool {
    let obstruent = |phoneme: &Phoneme| [P, B, T, D, K, G, F, V].contains(phoneme);
    match cluster {
        [_] => true,
        [first, second] if [L, R, J].contains(second) => obstruent(first),
        [first, second] if *second == V => [K, S, T, D].contains(first),
        [first, second] => *first == S && [P, T, K, M, N, L].contains(second),
        [first, second, third] => *first == S && [P, T, K].contains(second) && *third == R,
        _ => false,
    }
}

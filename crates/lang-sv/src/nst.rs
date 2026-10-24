//! Swedish words from NST's lexicon (data/derived/sv/nst.tsv, compiled by build.rs; NST's own
//! entries only, decision 42): its SAMPA turned into jespersen's phonemes, in plain IPA as NST says
//! them and in eSpeak NG's notation as the rules write the same letters.

use jespersen_lexicon::Lexicon;
use jespersen_phonemes::{Notation, Phoneme, Pronunciation, Stress, Syllable};

use crate::{
    phonemes::{
        A, A_LONG, AA, AA_SHORT, AE_LONG, B, D, E, E_LONG, F, G, H, I, I_FINAL, I_LONG, J, K, L, LONG, M, N, NG, O,
        O_CLOSE, O_FINAL, OE, OE_CLOSE, OE_LONG, OE_OPEN_LONG, OO, P, R, RD, RD_AFTER, RL, RL_AFTER, RN, RN_AFTER, RS,
        RS_AFTER, RT, S, SCHWA, SJ, T, TJ, U, U_CENTRAL, U_LONG, V, Y, Y_LONG,
    },
    sampa::{Symbol, VOWELS, symbols},
    spelling::rhythm,
};

/// Each word NST gives (in lower case, a name as written) and its transcription (compiled by
/// build.rs).
static NST: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/nst.lex"));

/// How NST says `word`, if it gives it: written with a capital, as a name first (Hans), then in
/// lower case.
#[expect(
    clippy::expect_used,
    reason = "build.rs compiled the lexicon and checked every transcription"
)]
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let lexicon = Lexicon::new(NST).expect("build.rs compiles NST's lexicon");
    let lower = word.to_lowercase();
    let values = [word, lower.as_str()]
        .into_iter()
        .find_map(|key| lexicon.get(key).expect("build.rs compiles NST's lexicon"))?;
    let sampa = std::str::from_utf8(values.first()?).expect("build.rs keeps the transcriptions in ASCII");
    let symbols = symbols(sampa).expect("build.rs checks every transcription");
    Some(pronunciation(&symbols, &lower))
}

/// The pronunciation NST's `symbols` write for `word`.
pub(crate) fn pronunciation(symbols: &[Symbol], word: &str) -> Pronunciation {
    let mut syllables: Vec<Syllable> = vec![Syllable::default()];
    // A retroflex consonant after another, or in a word with no r (kanske), has no r to write.
    let mut retroflex = !word.contains('r');
    for (index, &symbol) in symbols.iter().enumerate() {
        let next = symbols.get(index + 1).copied();
        let Some(syllable) = syllables.last_mut() else { break };
        match symbol {
            Symbol::Primary | Symbol::Secondary | Symbol::Boundary => {
                if syllable.phonemes().next().is_some() {
                    syllables.push(Syllable::default());
                }
                // NST's secondary stresses give way to eSpeak NG's rhythmic ones (rhythm).
                if symbol == Symbol::Primary
                    && let Some(syllable) = syllables.last_mut()
                {
                    syllable.stress = Stress::Primary;
                }
            }
            Symbol::Long => {}
            Symbol::Sound(sound) if VOWELS.contains(&sound) => {
                retroflex = !word.contains('r');
                let long = next == Some(Symbol::Long);
                let stressed = syllable.stress != Stress::Unstressed;
                // A short unstressed e is ə, as eSpeak NG writes it (vatten, aldeles, behövde).
                let reduced = sound == "e" && !long && !stressed;
                let vowel = if reduced { SCHWA } else { vowel(sound, long) };
                if syllable.coda.is_empty() {
                    syllable.nucleus.push(vowel);
                } else {
                    syllable.coda.push(vowel);
                }
            }
            Symbol::Sound(sound) => {
                let consonant = consonant(sound, retroflex);
                retroflex = retroflex || sound.ends_with('`');
                if syllable.nucleus.is_empty() {
                    syllable.onset.push(consonant);
                } else {
                    syllable.coda.push(consonant);
                }
            }
        }
    }
    lengthen(&mut syllables, geminates(word));
    // eSpeak NG writes a final unstressed o long (äro ˈɛːruː), and an unstressed i before a final
    // consonant but g (varit vˈɑːriːt; not fattig).
    if let Some(last) = syllables.last_mut()
        && last.stress == Stress::Unstressed
    {
        if last.coda.is_empty() && last.nucleus == [O_CLOSE] {
            last.nucleus = vec![O_FINAL];
        }
        if last.nucleus == [I] && last.coda.len() == 1 && last.coda != [G] {
            last.nucleus = vec![I_FINAL];
        }
    }
    let mut said = Pronunciation { syllables };
    rhythm(&mut said);
    said
}

/// NST's vowel `sound`, `long` or short.
fn vowel(sound: &str, long: bool) -> Phoneme {
    match (sound, long) {
        ("A" | "a", true) => A_LONG,
        ("A" | "a", false) => A,
        ("e", true) => E_LONG,
        ("E", true) => AE_LONG,
        ("e" | "E", false) => E,
        ("i", true) => I_LONG,
        ("y", true) => Y_LONG,
        ("y" | "Y", false) => Y,
        ("u0", _) => U,
        ("}", true) => U_LONG,
        ("}", false) => U_CENTRAL,
        ("u", true) => OO,
        ("U" | "u", false) => O_CLOSE,
        ("o", true) => AA,
        ("o", false) => AA_SHORT,
        ("2", true) => OE_LONG,
        ("2", false) => OE_CLOSE,
        ("9", true) => OE_OPEN_LONG,
        ("9", false) => OE,
        ("O", _) => O,
        _ => {
            if long {
                I_LONG
            } else {
                I
            }
        }
    }
}

/// NST's consonant `sound`, `after_retroflex` when it follows another retroflex one (whose r the
/// spelling has already written).
fn consonant(sound: &str, after_retroflex: bool) -> Phoneme {
    match sound {
        "p" => P,
        "b" => B,
        "t" => T,
        "t`" => RT,
        "d" => D,
        "k" => K,
        "g" => G,
        "f" => F,
        "v" => V,
        "s" => S,
        "h" => H,
        "m" => M,
        "n" => N,
        "N" => NG,
        "l" => L,
        "j" => J,
        "x\\" => SJ,
        "s'" => TJ,
        "s`" if after_retroflex => RS_AFTER,
        "s`" => RS,
        "d`" if after_retroflex => RD_AFTER,
        "d`" => RD,
        "n`" if after_retroflex => RN_AFTER,
        "n`" => RN,
        "l`" if after_retroflex => RL_AFTER,
        "l`" => RL,
        _ => R,
    }
}

/// How many doubled t and k (or ck) `word` spells between vowels, which the rules say long (vatten
/// vˈatːən, flicka; not mycket).
fn geminates(word: &str) -> usize {
    let letters: Vec<char> = word.to_lowercase().chars().collect();
    let vowel = |at: usize| letters.get(at).is_some_and(|c| "aeiouyåäö".contains(*c));
    (0..letters.len().saturating_sub(1))
        .filter(|&at| {
            let pair = (letters[at], letters[at + 1]);
            let doubled = matches!(pair, ('t', 't') | ('k', 'k'));
            let ck = pair == ('c', 'k') && letters.get(at + 2) != Some(&'e');
            (doubled || ck) && vowel(at + 2)
        })
        .count()
}

/// Marks `count` of the t and k that start a syllable after a short vowel as long, in order, as
/// the rules write a doubled one (vatten vˈatːən).
fn lengthen(syllables: &mut [Syllable], mut count: usize) {
    for index in 1..syllables.len() {
        if count == 0 {
            return;
        }
        let short = syllables[index - 1].coda.is_empty()
            && syllables[index - 1]
                .nucleus
                .last()
                .is_some_and(|vowel| !vowel.symbol(Notation::Ipa).ends_with('ː'));
        if short && matches!(syllables[index].onset[..], [T | K]) {
            syllables[index].onset.push(LONG);
            count -= 1;
        }
    }
}

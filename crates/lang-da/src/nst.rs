//! Danish words from NST's lexicon (data/derived/da/nst.tsv, compiled by build.rs; decision 41):
//! its SAMPA turned into jespersen's phonemes, with the stød in plain IPA and eSpeak NG's way of
//! writing the vowels and the letters p, t and k in its notation.

use jespersen_lexicon::Lexicon;
use jespersen_phonemes::{Phoneme, Pronunciation, Stress, Syllable};

use crate::{
    phonemes::{
        A, A_BACK, A_BACK_LONG, A_DARK, A_LONG, AA, AE, B, B_SPELT_P, D, D_SPELT_T, DH, E, E_LONG, ER, F, G, G_SPELT_K,
        H, I, I_LONG, I_LOW, J, K, L, M, N, NG, O, O_CLOSE, O_OPEN, O_OPEN_LONG, O_SHORT, OE, OE_LONG, OPEN_OE,
        OPEN_OE_LONG, P, R, R_AFTER_B, R_VOCALIC, S, SCHWA, SH, STOD, T, U, U_LONG, U_LOW, V, W, Y, Y_LONG,
    },
    sampa::{Symbol, VOWELS, symbols},
    spelling::rhythm,
};

/// Each word NST knows (in lower case) and its transcription (compiled by build.rs).
static NST: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/nst.lex"));

/// How NST says `word` (in lower case), if it knows it.
#[expect(
    clippy::expect_used,
    reason = "build.rs compiled the lexicon and checked every transcription"
)]
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let values = Lexicon::new(NST)
        .and_then(|lexicon| lexicon.get(word))
        .expect("build.rs compiles NST's lexicon")?;
    let sampa = std::str::from_utf8(values.first()?).expect("build.rs keeps the transcriptions in ASCII");
    let symbols = symbols(sampa).expect("build.rs checks every transcription");
    Some(pronunciation(&symbols, word))
}

/// The pronunciation NST's `symbols` write for `word`.
pub(crate) fn pronunciation(symbols: &[Symbol], word: &str) -> Pronunciation {
    let mut voiceless = spelt_voiceless(symbols, word).into_iter();
    let mut syllables: Vec<Syllable> = vec![Syllable::default()];
    for (index, &symbol) in symbols.iter().enumerate() {
        let next = symbols.get(index + 1).copied();
        let Some(syllable) = syllables.last_mut() else { break };
        match symbol {
            Symbol::Primary | Symbol::Secondary | Symbol::Boundary => {
                if syllable.phonemes().next().is_some() {
                    syllables.push(Syllable::default());
                }
                // NST's secondary stresses give way to eSpeak NG's rhythmic ones (rhythm), which agree
                // with it more often.
                if symbol == Symbol::Primary
                    && let Some(syllable) = syllables.last_mut()
                {
                    syllable.stress = Stress::Primary;
                }
            }
            Symbol::Long => {}
            Symbol::Stod => {
                if syllable.coda.is_empty() {
                    syllable.nucleus.push(STOD);
                } else {
                    syllable.coda.push(STOD);
                }
            }
            Symbol::Sound("6") if !syllable.nucleus.is_empty() => syllable.coda.push(R_VOCALIC),
            Symbol::Sound(sound) if VOWELS.contains(&sound) => {
                let place = Place {
                    long: next == Some(Symbol::Long),
                    stressed: syllable.stress != Stress::Unstressed,
                    before_w: next == Some(Symbol::Sound("w")),
                    at_end: !symbols[index + 1..]
                        .iter()
                        .any(|symbol| matches!(symbol, Symbol::Sound(_))),
                };
                let vowel = vowel(sound, place);
                if syllable.coda.is_empty() {
                    syllable.nucleus.push(vowel);
                } else {
                    syllable.coda.push(vowel);
                }
            }
            Symbol::Sound(sound) => {
                let spelt_voiceless = matches!(sound, "b" | "d" | "g") && voiceless.next().unwrap_or(false);
                let consonant = consonant(sound, spelt_voiceless);
                // r after b is ɐ̯ in eSpeak NG's notation (brød bɐ̯ˈʔœð).
                let consonant = if consonant == R && syllable.onset.last() == Some(&B) {
                    R_AFTER_B
                } else {
                    consonant
                };
                if syllable.nucleus.is_empty() {
                    syllable.onset.push(consonant);
                } else {
                    syllable.coda.push(consonant);
                }
            }
        }
    }
    // t between a consonant but n and an unstressed e is d in eSpeak NG's notation (præsten).
    for index in 1..syllables.len() {
        let after_consonant = syllables[index - 1]
            .coda
            .last()
            .is_some_and(|&last| !last.is_vowel() && last != N)
            || syllables[index].onset.len() > 1;
        let syllable = &mut syllables[index];
        if after_consonant
            && syllable.nucleus.first() == Some(&SCHWA)
            && let Some(last) = syllable.onset.last_mut()
            && *last == D_SPELT_T
        {
            *last = D;
        }
    }
    let mut said = Pronunciation { syllables };
    rhythm(&mut said);
    said
}

/// Where a vowel is in its word.
#[derive(Clone, Copy)]
#[expect(clippy::struct_excessive_bools, reason = "independent facts about one vowel")]
struct Place {
    long: bool,
    stressed: bool,
    before_w: bool,
    /// The word's last sound.
    at_end: bool,
}

/// NST's vowel `sound` where `place` says.
fn vowel(sound: &str, place: Place) -> Phoneme {
    let Place {
        long,
        stressed,
        before_w,
        at_end,
    } = place;
    match (sound, long) {
        ("i", false) => I,
        ("i", true) => I_LONG,
        ("e", false) => I_LOW,
        ("e", true) => E_LONG,
        ("E", false) => E,
        ("E", true) => AE,
        ("a", false) => A,
        ("a", true) => A_LONG,
        ("A", false) if stressed => A_DARK,
        ("A", false) => A_BACK,
        ("A", true) => A_BACK_LONG,
        ("y", false) => Y,
        ("y", true) => Y_LONG,
        ("2", false) => OE,
        ("2", true) => OE_LONG,
        ("9", false) => OPEN_OE,
        ("9", true) => OPEN_OE_LONG,
        ("u", false) => U,
        ("u", true) => U_LONG,
        ("o", false) => O_CLOSE,
        ("o", true) => O,
        ("O", false) => U_LOW,
        ("O", true) => AA,
        ("Q", false) => O_OPEN,
        ("Q", true) => O_OPEN_LONG,
        // 6 is ʌ with the stress (godt), ɒ before w and at the end (og, over, så), ɐ without the
        // stress at the end (efter).
        ("6", _) if stressed && (before_w || at_end) => O_OPEN,
        ("6", _) if stressed => O_SHORT,
        ("6", _) => ER,
        _ => SCHWA,
    }
}

/// NST's consonant `sound`; its b, d and g are p, t and k in eSpeak NG's notation when
/// `voiceless` says the spelling has those letters.
fn consonant(sound: &str, voiceless: bool) -> Phoneme {
    match sound {
        "p" => P,
        "t" => T,
        "k" => K,
        "b" if voiceless => B_SPELT_P,
        "d" if voiceless => D_SPELT_T,
        "g" if voiceless => G_SPELT_K,
        "b" => B,
        "d" => D,
        "g" => G,
        "f" => F,
        "v" => V,
        "s" => S,
        "s'" => SH,
        "h" => H,
        "m" => M,
        "n" => N,
        "N" => NG,
        "l" => L,
        "j" => J,
        "w" => W,
        "D" => DH,
        _ => R,
    }
}

/// Where a stop is said: 0 at the lips, 1 the teeth, 2 the soft palate.
fn place(stop: char) -> Option<u8> {
    match stop {
        'p' | 'b' => Some(0),
        't' | 'd' => Some(1),
        'k' | 'g' | 'c' | 'q' => Some(2),
        _ => None,
    }
}

/// For each unaspirated b, d and g of NST's `symbols`, in order, whether `word` spells it p, t or
/// k: NST's stops are matched in order with the spelling's stop letters of the same place, leaving
/// out a doubled letter, the d of dt and the g of ng, and if they still do not pair, a d or g after
/// a vowel (said ð, j or w); failing that, the letter wins if the word has only voiceless ones.
fn spelt_voiceless(symbols: &[Symbol], word: &str) -> Vec<bool> {
    let letters: Vec<char> = word.to_lowercase().chars().collect();
    // Each stop letter: its place, whether it is voiceless, and whether it follows a vowel.
    let mut spelt: Vec<(u8, bool, bool)> = Vec::new();
    for (at, &c) in letters.iter().enumerate() {
        let before = at.checked_sub(1).map(|before| letters[before]);
        let Some(place) = place(c) else { continue };
        let skipped = before == Some(c)
            || (c == 'd' && letters.get(at + 1) == Some(&'t'))
            || (c == 'g' && before == Some('n'))
            || (c == 'c' && matches!(letters.get(at + 1), Some('e' | 'i' | 'y' | 'h')));
        if !skipped {
            let after_vowel = before.is_some_and(|before| "aeiouyæøå".contains(before));
            spelt.push((place, matches!(c, 'p' | 't' | 'k' | 'c' | 'q'), after_vowel));
        }
    }
    let stops: Vec<(u8, bool)> = symbols
        .iter()
        .filter_map(|symbol| match symbol {
            Symbol::Sound(sound) => {
                let c = sound.chars().next()?;
                Some((place(c)?, matches!(c, 'b' | 'd' | 'g')))
            }
            _ => None,
        })
        .collect();
    let mut out = vec![false; stops.len()];
    for place in 0..3 {
        let said: Vec<usize> = (0..stops.len()).filter(|&index| stops[index].0 == place).collect();
        let all: Vec<(bool, bool)> = spelt
            .iter()
            .filter(|letter| letter.0 == place)
            .map(|&(_, voiceless, after_vowel)| (voiceless, after_vowel))
            .collect();
        let hard: Vec<(bool, bool)> = all
            .iter()
            .copied()
            .filter(|&(voiceless, after_vowel)| voiceless || !after_vowel)
            .collect();
        let paired = [&all, &hard].into_iter().find(|letters| letters.len() == said.len());
        for (order, &index) in said.iter().enumerate() {
            out[index] = match paired {
                Some(letters) => letters[order].0,
                None => all.iter().any(|letter| letter.0) && !all.iter().any(|letter| !letter.0),
            };
        }
    }
    // Only the unaspirated ones are asked about, in order.
    stops
        .iter()
        .zip(out)
        .filter(|((_, unaspirated), _)| *unaspirated)
        .map(|(_, voiceless)| voiceless)
        .collect()
}

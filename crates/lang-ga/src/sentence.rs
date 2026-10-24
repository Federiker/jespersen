use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Irish,
    numbers::is_number_word,
    phonemes::{
        A, A_BACK, D, D_PLAIN, E, G, G_BEFORE_VOWEL, N, NJ, O, OO, R, R_APPROX, RJ, SCHWA, SCHWA_A, U, legal_onset,
    },
    spelling::{pronounce, pronounce_bare},
    words::respelling,
};

/// Words said without stress in running text: the article, prepositions, particles, possessives
/// and conjunctions.
const UNSTRESSED: [&str; 31] = [
    "a", "an", "na", "sa", "san", "sna", "i", "in", "do", "de", "le", "is", "mo", "ár", "bhur", "ag", "ach", "gan",
    "gur", "nach", "chun", "nó", "um", "seo", "ná", "arsa", "ba", "ab", "n-a", "’n-a", "'n-a",
];

/// Words said with a lighter stress (eSpeak NG writes a secondary one).
const LIGHTER: [&str; 3] = ["agus", "atá", "ina"];

/// How each token (text and kind) of an Irish sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings),
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress, and their vowels are reduced.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Irish, &Reader)
}

/// How Irish says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && letter.is_uppercase()
            && let Some(name) = Irish.letter(letter)
        {
            return pronounce(name);
        }
        // ag and ar as Munster says them (Sjoestedt 1938, §§ 107–108: eg, er; but air erʹ),
        // which the spelling would give a and a slender r.
        let said = |phonemes: [Phoneme; 2]| {
            let stresses = [Stress::Primary, Stress::Unstressed];
            let segments: Vec<Segment> = phonemes
                .into_iter()
                .zip(stresses)
                .map(|(phoneme, stress)| Segment::new(phoneme, stress))
                .collect();
            Some(syllabify(&segments, legal_onset))
        };
        match lower.as_str() {
            "ag" => return said([E, G_BEFORE_VOWEL]),
            "ar" => return said([E, R]),
            _ => {}
        }
        // A short lower-case word without a vowel is an elided one written apart (d’ aois, b’
        // ainm), said as its consonants.
        pronounce(respelling(&lower).unwrap_or(&lower)).or_else(|| {
            let letters: Vec<char> = word.chars().filter(|c| c.is_alphabetic()).collect();
            (letters.len() <= 2 && letters.iter().all(|c| c.is_lowercase()))
                .then(|| pronounce_bare(word))
                .flatten()
        })
    }

    /// A word with the t, n or h a word before it adds (an t-am, i n-áit, na h-oícheanta), said
    /// as one; the parts of a number joined by hyphens said as one word with the stress on the
    /// first (fiche-a-haon fˈɪçəəheːn).
    fn say_joined(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let parts: Vec<&str> = lower.split('-').filter(|part| !part.is_empty()).collect();
        if let [prefix, _] = parts[..]
            && ["t", "n", "h"].contains(&prefix)
        {
            return pronounce(&lower);
        }
        if !parts.iter().all(|part| is_number_word(part)) {
            return None;
        }
        let mut joined: Vec<Segment> = Vec::new();
        for (index, part) in parts.iter().enumerate() {
            // The a before a unit is ə (fiche a haon).
            if *part == "a" {
                joined.push(Segment::new(SCHWA, Stress::Unstressed));
                continue;
            }
            for syllable in &self.say(part)?.syllables {
                let stress = if index == 0 {
                    syllable.stress
                } else {
                    Stress::Unstressed
                };
                for &phoneme in syllable.onset.iter().chain(&syllable.nucleus).chain(&syllable.coda) {
                    let stress = if phoneme.is_vowel() { stress } else { Stress::Unstressed };
                    joined.push(Segment::new(phoneme, stress));
                }
            }
        }
        Some(syllabify(&joined, legal_onset))
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn lighter(&self) -> &[&str] {
        &LIGHTER
    }

    /// A function word left without stress has a reduced vowel: an open or n-closed a is ə (a ə,
    /// an ən, na nə), o is ɐ (do dɐ, mo mɐ) and oː ʊ (seo ʃʊ), and its broad d plain in eSpeak
    /// NG's notation (do dɐ). A final r before a vowel is ɹ when broad or after a short vowel (ar
    /// an ˈɛɹ ən, fuar agus; but nuair a nˈuər).
    fn run(&self, words: &mut [&mut Pronunciation]) {
        // The g of ag is said before a vowel or a pause (ag ól; but ag teacht ɛ ˈtʲaxt).
        for at in 0..words.len() {
            let consonant_next = words
                .get(at + 1)
                .and_then(|next| next.phonemes().next())
                .is_some_and(|first| !first.is_vowel());
            if !consonant_next
                && let Some(last) = words[at]
                    .syllables
                    .last_mut()
                    .and_then(|syllable| syllable.coda.last_mut())
                && *last == G_BEFORE_VOWEL
            {
                *last = G;
            }
        }
        for at in 1..words.len() {
            let vowel_next = words[at].phonemes().next().is_some_and(Phoneme::is_vowel);
            if vowel_next && let Some(syllable) = words[at - 1].syllables.last_mut() {
                let short = syllable.nucleus == [E];
                if let Some(last) = syllable.coda.last_mut()
                    && (*last == R || *last == RJ && short)
                {
                    *last = R_APPROX;
                }
            }
        }
        for said in words.iter_mut() {
            if said
                .syllables
                .iter()
                .all(|syllable| syllable.stress == Stress::Unstressed)
                && said.phonemes().any(Phoneme::is_vowel)
            {
                for syllable in &mut said.syllables {
                    for consonant in &mut syllable.onset {
                        if *consonant == D {
                            *consonant = D_PLAIN;
                        }
                    }
                    let open = syllable.coda.iter().all(|c| [N, NJ, R].contains(c));
                    for vowel in &mut syllable.nucleus {
                        *vowel = match *vowel {
                            vowel if (vowel == A || vowel == A_BACK) && open => SCHWA,
                            vowel if vowel == O => SCHWA_A,
                            vowel if vowel == OO => U,
                            vowel => vowel,
                        };
                    }
                }
            }
        }
    }
}

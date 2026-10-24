use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};
use jespersen_spoken::{Locale, Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    locale::Catalan,
    numbers::is_number_word,
    phonemes::{
        A, B, B_SOFT, CH, D, D_SOFT, DZH, E, E_OPEN, F, G, G_SOFT, I, J, K, L, LATENT_K, LATENT_P, LL, M, N,
        N_LABIODENTAL, NG, NY, O, O_OPEN, P, R, R_TAP, S, SCHWA, SH, T, U, V, W, Z, ZH, legal_onset,
    },
    spelling::{pronounce, segments, softens},
    words::respelling,
};

/// Words said without stress in running text: articles, prepositions, clitic pronouns and
/// conjunctions, alone or with an elided word before them (d'en, se'n).
const UNSTRESSED: [&str; 41] = [
    "el", "la", "els", "les", "lo", "un", "uns", "de", "del", "dels", "a", "al", "als", "en", "amb", "per", "pel",
    "pels", "i", "que", "es", "se", "em", "et", "ens", "us", "li", "hi", "ho", "me", "te", "ne", "si", "d'en", "d'un",
    "d'uns", "l'un", "se'n", "me'n", "te'n", "s'en",
];

/// The pronouns that join a verb after a hyphen (menjar-se, dona-m'ho).
const ENCLITICS: [&str; 22] = [
    "lo", "la", "los", "les", "l", "ls", "me", "m", "te", "t", "se", "s", "nos", "ns", "vos", "us", "li", "hi", "ho",
    "ne", "n", "en",
];

/// How each token (text and kind) of a Catalan sentence is read, in order; `None` for a token
/// jespersen cannot read yet. Words are read from their spelling (and jespersen's list of respellings),
/// numbers, abbreviations and symbols as words. The words between punctuation marks are read
/// together: function words lose their stress, and sounds change across them.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)]) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Catalan, &Reader)
}

/// The segments of `said`, each vowel with its syllable's stress, the rest unstressed; all
/// unstressed when `stressless`.
fn segments_of(said: &Pronunciation, stressless: bool) -> impl Iterator<Item = Segment> + '_ {
    said.syllables.iter().flat_map(move |syllable| {
        syllable
            .onset
            .iter()
            .chain(&syllable.nucleus)
            .chain(&syllable.coda)
            .map(move |&phoneme| {
                let stress = if phoneme.is_vowel() && !stressless {
                    syllable.stress
                } else {
                    Stress::Unstressed
                };
                Segment::new(phoneme, stress)
            })
    })
}

/// The sound of an elided word before the word after it (l'home, d'aigua, s'acosta).
fn elided(head: &str) -> Option<Phoneme> {
    Some(match head {
        "l" => L,
        "d" => D,
        "s" => S,
        "m" => M,
        "t" => T,
        "n" => N,
        _ => return None,
    })
}

/// How Catalan says its words, for the shared reader.
struct Reader;

impl Speaker for Reader {
    /// A word from its spelling; a capital letter on its own that is not a word by its name.
    fn say(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let mut letters = word.chars();
        if let (Some(letter), None) = (letters.next(), letters.next())
            && letter.is_uppercase()
            && !"AEIOU".contains(letter)
            && let Some(name) = Catalan.letter(letter)
        {
            // A name of two words (ve doble) is said as one.
            return pronounce(&name.replace(' ', ""));
        }
        pronounce(respelling(&lower).unwrap_or(&lower))
    }

    /// A verb and the pronouns after it (menjar-se mənʒˈarsə, dona-m'ho): the verb keeps a final
    /// r its infinitive would drop (a tap before a vowel: menjar-ho mənʒˈaɾu), the pronouns are
    /// unstressed; and a number written with hyphens, its parts before the last with a lighter
    /// stress (vint-i-u bˌintiˈu, dos-cents dˌosˈens).
    fn say_joined(&self, word: &str) -> Option<Pronunciation> {
        let lower = word.to_lowercase();
        let parts: Vec<&str> = lower.split(['-', '\'', '’']).collect();
        let (first, rest) = parts.split_first()?;
        let mut joined: Vec<Segment> = Vec::new();
        if !rest.is_empty() && rest.iter().all(|pronoun| ENCLITICS.contains(pronoun)) {
            joined.extend(segments(respelling(first).unwrap_or(first), true)?);
            for pronoun in rest {
                joined.extend(segments(pronoun, false)?.into_iter().map(|segment| Segment {
                    stress: Stress::Unstressed,
                    ..segment
                }));
            }
            for at in 1..joined.len() {
                if joined[at - 1].phoneme == R && joined[at].phoneme.is_vowel() {
                    joined[at - 1].phoneme = R_TAP;
                }
            }
        } else if parts.iter().all(|part| is_number_word(part)) {
            for (index, part) in parts.iter().enumerate() {
                let lighter = if *part == "i" {
                    Some(Stress::Unstressed)
                } else {
                    (index + 1 < parts.len()).then_some(Stress::Secondary)
                };
                joined.extend(segments(part, false)?.into_iter().map(|segment| Segment {
                    stress: if let Some(lighter) = lighter {
                        segment.stress.min(lighter)
                    } else {
                        segment.stress
                    },
                    ..segment
                }));
                // The t of vint is said inside a number (vint-i-u).
                if part.ends_with("nt") && index + 1 < parts.len() {
                    joined.push(Segment::new(T, Stress::Unstressed));
                }
            }
        } else {
            return None;
        }
        Some(syllabify(&joined, legal_onset))
    }

    /// An elided article, preposition or pronoun said as the start of the word after it (l'home
    /// lˈɔmə); a verb with an enclitic pronoun after its apostrophe ends with it (posa'l pˈɔzəl).
    fn elide(&self, head: &str, word: Pronunciation) -> Option<Pronunciation> {
        let lower = head.to_lowercase();
        let mut joined: Vec<Segment> = Vec::new();
        if let Some(sound) = elided(&lower) {
            joined.push(Segment::new(sound, Stress::Unstressed));
            joined.extend(segments_of(&word, false));
        } else {
            joined.extend(segments_of(&self.say(&lower)?, UNSTRESSED.contains(&lower.as_str())));
            joined.extend(segments_of(&word, true));
        }
        Some(syllabify(&joined, legal_onset))
    }

    fn unstressed(&self) -> &[&str] {
        &UNSTRESSED
    }

    fn run(&self, words: &mut [&mut Pronunciation]) {
        for said in words.iter_mut() {
            reduce(said);
        }
        across(words);
        allophones(words);
    }
}

/// The vowels of the syllables left without stress in a run reduced: a and e to ə, o to u (de
/// də, per pəɾ).
fn reduce(said: &mut Pronunciation) {
    for syllable in &mut said.syllables {
        if syllable.stress == Stress::Unstressed {
            for vowel in &mut syllable.nucleus {
                *vowel = match *vowel {
                    vowel if [A, E, E_OPEN].contains(&vowel) => SCHWA,
                    vowel if [O, O_OPEN].contains(&vowel) => U,
                    vowel => vowel,
                };
            }
        }
    }
}

/// The sounds that meet across words: an unstressed i or u starting a word after a vowel or a glide, or
/// ending one before a vowel, is a glide (era un ˈeɾə wn, li era lj ˈeɾə); que loses its ə before a
/// vowel (que el k əl), and amb its b before a consonant (amb ells əmb ˈeʎs, amb tu əm tˈu); the c of
/// -nc and p of -mp are said only before a vowel (cinc anys sˈiŋk ˈaɲʃ), and a final r
/// there is a tap (mar endins mˈaɾ əndˈins).
fn across(words: &mut [&mut Pronunciation]) {
    let glide = |vowel: Phoneme| if vowel == I { J } else { W };
    for index in 1..words.len() {
        let before = words[index - 1].phonemes().last();
        if before.is_some_and(|before| before.is_vowel() || [J, W].contains(&before))
            && let Some(syllable) = words[index].syllables.first_mut()
            && syllable.onset.is_empty()
            && syllable.stress == Stress::Unstressed
            && let [vowel] = syllable.nucleus[..]
            && [I, U].contains(&vowel)
            && before != Some(vowel)
        {
            syllable.nucleus[0] = glide(vowel);
        }
    }
    for index in 0..words.len() {
        let vowel_after = words
            .get(index + 1)
            .and_then(|after| after.phonemes().next())
            .is_some_and(Phoneme::is_vowel);
        let said = &mut *words[index];
        if let Some(syllable) = said.syllables.last_mut()
            && let Some(&last) = syllable.coda.last()
        {
            let latent = [(LATENT_K, K), (LATENT_P, P)];
            if let Some(&(_, sound)) = latent.iter().find(|(marker, _)| *marker == last) {
                syllable.coda.pop();
                if vowel_after {
                    syllable.coda.push(sound);
                }
            } else if last == R
                && vowel_after
                && let Some(last) = syllable.coda.last_mut()
            {
                *last = R_TAP;
            }
        }
        let ending: Vec<Phoneme> = said.phonemes().collect();
        if !vowel_after {
            if ending.ends_with(&[M, B])
                && let Some(syllable) = said.syllables.last_mut()
            {
                syllable.coda.pop();
            }
        } else if ending == [K, SCHWA] {
            said.syllables[0].nucleus.clear();
        } else if let Some(syllable) = said.syllables.last_mut()
            && syllable.coda.is_empty()
            && syllable.stress == Stress::Unstressed
            && let [vowel] = syllable.nucleus[..]
            && [I, U].contains(&vowel)
        {
            syllable.nucleus[0] = glide(vowel);
        }
    }
}

/// The voiceless consonants voiced at the end of a word before a voiced consonant (un bloc de blˈɔɡ
/// ðə), the fricatives and affricates before a vowel too (vaig anar bˈadʒ ənˈa; but un pot amb
/// pˈɔt).
const VOICED: [(Phoneme, Phoneme); 7] = [(P, B), (T, D), (K, G), (CH, DZH), (SH, ZH), (S, Z), (F, V)];
const STOPS: [Phoneme; 3] = [P, T, K];

/// The allophones of running speech: s is z before a vowel or a voiced consonant (els homes, els
/// gats) and ʒ or ʃ before them (tots junts), and the other consonants ending a word are voiced
/// there too; n is m before p, b and m, ŋ before k and g and ɱ before f (un poc, un gat, en fusta),
/// and n and l are ɲ and ʎ before ʎ (un llop, el llum); b, d and g are β ð ɣ in eSpeak NG's notation
/// after a vowel, a glide, r, l (but d) and z, also across words (la boca lə βˈokə), but not before
/// a nasal (digne).
fn allophones(run: &mut [&mut Pronunciation]) {
    // Where each phoneme is: word, syllable, part (0 onset, 1 nucleus, 2 coda), index.
    let mut places: Vec<(usize, usize, usize, usize)> = Vec::new();
    for (word, said) in run.iter().enumerate() {
        for (number, syllable) in said.syllables.iter().enumerate() {
            for (part, phonemes) in [&syllable.onset, &syllable.nucleus, &syllable.coda]
                .into_iter()
                .enumerate()
            {
                places.extend((0..phonemes.len()).map(|index| (word, number, part, index)));
            }
        }
    }
    let get = |run: &[&mut Pronunciation], (word, syllable, part, index): (usize, usize, usize, usize)| -> Phoneme {
        let syllable = &run[word].syllables[syllable];
        [&syllable.onset, &syllable.nucleus, &syllable.coda][part][index]
    };
    let set = |run: &mut [&mut Pronunciation],
               (word, syllable, part, index): (usize, usize, usize, usize),
               phoneme: Phoneme| {
        let syllable = &mut run[word].syllables[syllable];
        [&mut syllable.onset, &mut syllable.nucleus, &mut syllable.coda][part][index] = phoneme;
    };
    let voiced = [B, D, G, L, LL, M, N, NY, R, R_TAP, Z, ZH, J, W];
    let nasal = [M, N, NY, NG];
    // Whether the phoneme at `at` ends its word: in the coda of the last syllable, or the onset of
    // one left without a vowel (que k).
    let word_end = |run: &[&mut Pronunciation], at: usize| {
        let (word, syllable, part, _) = places[at];
        let syllables = &run[word].syllables;
        syllable + 1 == syllables.len() && (part == 2 || syllables[syllable].nucleus.is_empty())
    };
    for at in (0..places.len().saturating_sub(1)).rev() {
        let next = get(run, places[at + 1]);
        if get(run, places[at]) == S && [ZH, SH].contains(&next) {
            set(run, places[at], next);
        } else if get(run, places[at]) == S && (voiced.contains(&next) || (word_end(run, at) && next.is_vowel())) {
            set(run, places[at], Z);
        }
    }
    for at in 1..places.len() {
        let phoneme = get(run, places[at]);
        let before = get(run, places[at - 1]);
        let next = places.get(at + 1).map(|&place| get(run, place));
        let soft_after = before.is_vowel() || [J, W, R, R_TAP, Z].contains(&before) || (before == L && phoneme != D);
        if softens(phoneme) && soft_after && !next.is_some_and(|next| nasal.contains(&next)) {
            let soft = match phoneme {
                phoneme if phoneme == B => B_SOFT,
                phoneme if phoneme == D => D_SOFT,
                _ => G_SOFT,
            };
            set(run, places[at], soft);
        }
    }
    for at in (0..places.len().saturating_sub(1)).rev() {
        let phoneme = get(run, places[at]);
        let next = get(run, places[at + 1]);
        // Stops are voiced before a voiced consonant only, the others before a vowel or a glide too.
        let next_voiced = if STOPS.contains(&phoneme) {
            (voiced.contains(&next) && ![J, W].contains(&next)) || [B_SOFT, D_SOFT, G_SOFT, V].contains(&next)
        } else {
            voiced.contains(&next) || next.is_vowel() || [B_SOFT, D_SOFT, G_SOFT, V].contains(&next)
        };
        // A word left without its vowel (que k) keeps its consonant.
        let has_vowel = run[places[at].0].phonemes().any(Phoneme::is_vowel);
        if word_end(run, at)
            && has_vowel
            && next_voiced
            && let Some(&(_, voiced)) = VOICED.iter().find(|(voiceless, _)| *voiceless == phoneme)
        {
            set(run, places[at], voiced);
        } else if phoneme == N && [P, B, M].contains(&next) {
            set(run, places[at], M);
        } else if phoneme == N && [K, G].contains(&next) {
            set(run, places[at], NG);
        } else if phoneme == N && next == F {
            set(run, places[at], N_LABIODENTAL);
        } else if [N, L].contains(&phoneme) && next == LL {
            set(run, places[at], if phoneme == N { NY } else { LL });
        }
    }
}

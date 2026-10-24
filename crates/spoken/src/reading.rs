use jespersen_phonemes::{Pronunciation, Stress};
use jespersen_text::TokenKind;

use crate::{Locale, Spoken, spoken};

/// How a token of a sentence is read: the words it is said as (none for punctuation).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenReading {
    pub words: Vec<Pronunciation>,
    /// Whether the words are a hyphenated word's parts (franco-alemán), which eSpeak NG's notation
    /// writes joined and plain IPA with hyphens.
    pub hyphenated: bool,
    /// How many tokens, from this one on, the words are for: 3.º is three (3, ., º).
    pub tokens: usize,
}

/// A language's way of saying words, for [`read_tokens`].
pub trait Speaker {
    /// How a word is said on its own, `None` when it cannot be read.
    fn say(&self, word: &str) -> Option<Pronunciation>;

    /// An elided word (`head`, without its apostrophe) said with the word after it; `None` when
    /// the language does not elide.
    fn elide(&self, _head: &str, _word: Pronunciation) -> Option<Pronunciation> {
        None
    }

    /// A hyphenated word said as one (Catalan menjar-se mənʒˈarsə); `None` to say its parts one
    /// by one.
    fn say_joined(&self, _word: &str) -> Option<Pronunciation> {
        None
    }

    /// An elided word with nothing after it (Italian po'), said on its own.
    fn elided_alone(&self, head: &str) -> Option<Pronunciation> {
        self.say(head)
    }

    /// Words said without stress in running text (articles, most prepositions).
    fn unstressed(&self) -> &[&str] {
        &[]
    }

    /// Whether `written`, a word in lower case, is said without stress in running text: by
    /// default, whether [`Self::unstressed`] lists it as written.
    fn is_unstressed(&self, written: &str) -> bool {
        self.unstressed().contains(&written)
    }

    /// Words said with a lighter stress in running text.
    fn lighter(&self) -> &[&str] {
        &[]
    }

    /// Words said as the start of the word after them, which gives them its stress (Czech
    /// prepositions: na stole ˈnastole).
    fn proclitics(&self) -> &[&str] {
        &[]
    }

    /// Whether a proclitic joins `next`, the word after it.
    fn joins(&self, _next: &Pronunciation) -> bool {
        true
    }

    /// Whether a joined proclitic takes the stress of the word after it (Czech na stole ˈnastole)
    /// or leaves it there (Slovenian v redu uˈredu).
    fn proclitic_takes_stress(&self) -> bool {
        true
    }

    /// Changes made across a run of speech once its words are said (allophones, rhythm).
    fn run(&self, _words: &mut [&mut Pronunciation]) {}
}

/// A token's words, each as written in lower case (empty for letter names, which are never
/// function words) and as said.
type TokenWords = (usize, Vec<(String, Pronunciation)>, bool);

/// How each token (text and kind) of a sentence is read, in order; `None` for a token that cannot
/// be read, or that the reading of a token before it covers. The front end says what each token
/// is ([`spoken`]); the words between punctuation marks are read together: function words lose
/// their stress, and the speaker makes its changes across the run.
pub fn read_tokens(
    tokens: &[(&str, TokenKind)],
    locale: &impl Locale,
    speaker: &impl Speaker,
) -> Vec<Option<TokenReading>> {
    let spoken = spoken(tokens, locale);
    let mut readings: Vec<Option<TokenReading>> = vec![None; tokens.len()];
    let mut start = 0;
    for end in 0..=tokens.len() {
        if end == tokens.len() || (tokens[end].1 == TokenKind::Punctuation && spoken[end] != Spoken::Covered) {
            read_run(
                &tokens[start..end],
                &spoken[start..end],
                &mut readings[start..end],
                speaker,
            );
            if end < tokens.len() {
                readings[end] = Some(TokenReading {
                    words: Vec::new(),
                    hyphenated: false,
                    tokens: 1,
                });
            }
            start = end + 1;
        }
    }
    readings
}

fn read_run(
    tokens: &[(&str, TokenKind)],
    spoken: &[Spoken],
    readings: &mut [Option<TokenReading>],
    speaker: &impl Speaker,
) {
    let mut words: Vec<TokenWords> = Vec::new();
    for (index, spoken) in spoken.iter().enumerate() {
        let said = match spoken {
            Spoken::Silent => Some((Vec::new(), false)),
            Spoken::Unreadable | Spoken::Covered => None,
            Spoken::Words(written) | Spoken::Spelt(written) => {
                let joined: Vec<Option<Pronunciation>> = written
                    .iter()
                    .map(|word| word.contains('-').then(|| speaker.say_joined(word)).flatten())
                    .collect();
                let hyphenated = tokens[index].1 == TokenKind::Word
                    && written.len() == 1
                    && written[0].contains('-')
                    && joined[0].is_none();
                say_words(written, joined, matches!(spoken, Spoken::Spelt(_)), speaker).map(|said| (said, hyphenated))
            }
        };
        if let Some((said, hyphenated)) = said {
            words.push((index, said, hyphenated));
        }
    }
    let count: usize = words.iter().map(|(_, said, _)| said.len()).sum();
    if count > 1 {
        for (written, said) in words.iter_mut().flat_map(|(_, said, _)| said.iter_mut()) {
            let stress = if speaker.is_unstressed(written) {
                Stress::Unstressed
            } else if speaker.lighter().contains(&written.as_str()) {
                Stress::Secondary
            } else {
                continue;
            };
            for syllable in &mut said.syllables {
                syllable.stress = syllable.stress.min(stress);
            }
        }
    }
    join_proclitics(&mut words, tokens, speaker);
    let mut run: Vec<&mut Pronunciation> = words
        .iter_mut()
        .flat_map(|(_, said, _)| said.iter_mut().map(|(_, said)| said))
        .collect();
    speaker.run(&mut run);
    for (index, said, hyphenated) in words {
        let covered = spoken[index + 1..]
            .iter()
            .take_while(|spoken| **spoken == Spoken::Covered)
            .count();
        readings[index] = Some(TokenReading {
            words: said.into_iter().map(|(_, said)| said).collect(),
            hyphenated,
            tokens: 1 + covered,
        });
    }
}

/// Says each proclitic that ends a token as the start of the first word of the next token, a
/// word the speaker lets it join, which loses its stress to it; the proclitic's own token is then
/// left without words.
fn join_proclitics(words: &mut [TokenWords], tokens: &[(&str, TokenKind)], speaker: &impl Speaker) {
    let proclitics = speaker.proclitics();
    for index in 0..words.len().saturating_sub(1) {
        let ends_in_proclitic = words[index]
            .1
            .last()
            .is_some_and(|(written, _)| proclitics.contains(&written.as_str()));
        let next_word = tokens[words[index + 1].0].1 == TokenKind::Word;
        let joins = words[index + 1].1.first().is_some_and(|(_, next)| speaker.joins(next));
        if !ends_in_proclitic || !next_word || !joins {
            continue;
        }
        let Some((_, mut head)) = words[index].1.pop() else {
            continue;
        };
        let (_, next) = &mut words[index + 1].1[0];
        if speaker.proclitic_takes_stress() {
            for syllable in &mut next.syllables {
                if syllable.stress == Stress::Primary {
                    syllable.stress = Stress::Unstressed;
                }
            }
            // The proclitic takes the stress, even one said without it when it stands alone.
            if let Some(first) = head.syllables.first_mut() {
                first.stress = Stress::Primary;
            }
        } else {
            for syllable in &mut head.syllables {
                syllable.stress = Stress::Unstressed;
            }
        }
        head.syllables.append(&mut next.syllables);
        *next = head;
    }
}

/// The words of a token, each with how it is written and said; an elided word (l', dell') is said
/// with the word after it, and written with it (so c'era is no form of era).
/// The written words of a token said in order: those the speaker says joined as they are (`joined`),
/// the others split at hyphens and spaces into parts.
fn say_words(
    written: &[String],
    joined: Vec<Option<Pronunciation>>,
    spelt: bool,
    speaker: &impl Speaker,
) -> Option<Vec<(String, Pronunciation)>> {
    let mut out: Vec<(String, Pronunciation)> = Vec::new();
    let mut parts: Vec<&str> = Vec::new();
    for (word, joined) in written.iter().zip(joined) {
        if let Some(said) = joined {
            out.extend(say(&parts, spelt, speaker)?);
            parts.clear();
            out.push((word.to_lowercase(), said));
        } else {
            parts.extend(word.split(['-', ' ']).filter(|part| !part.is_empty()));
        }
    }
    out.extend(say(&parts, spelt, speaker)?);
    Some(out)
}

fn say(parts: &[&str], spelt: bool, speaker: &impl Speaker) -> Option<Vec<(String, Pronunciation)>> {
    let mut out: Vec<(String, Pronunciation)> = Vec::new();
    let mut elided: Option<&str> = None;
    for part in parts {
        if let Some(head) = part.strip_suffix('\'') {
            if elided.is_some() {
                return None;
            }
            elided = Some(head);
            continue;
        }
        let said = speaker.say(part)?;
        let (written, said) = match elided.take() {
            Some(head) => (format!("{head}'{part}").to_lowercase(), speaker.elide(head, said)?),
            None if spelt => (String::new(), said),
            None => (part.to_lowercase(), said),
        };
        out.push((written, said));
    }
    if let Some(head) = elided {
        out.push((head.to_lowercase(), speaker.elided_alone(head)?));
    }
    Some(out)
}

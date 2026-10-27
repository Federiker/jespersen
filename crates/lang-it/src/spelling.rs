//! How an Italian word is said from its spelling: letters to phonemes, then stress and the open or
//! closed e and o. The spelling shows neither unless it carries an accent, so jespersen's own word
//! list (data/it/words.tsv) respells the words the rules would miss, as Italian dictionaries
//! write them: tàvolo for the stress, bène and pòrta for an open vowel, sóle for a closed one.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::{
    phonemes::{
        A, B, B_FIRST, B_LONG, CH, CH_LONG, D, D_FIRST, D_LONG, DJ, DJ_LONG, DZ, DZ_LONG, E, E_IMPERFECT, E_OPEN, F, G,
        G_FIRST, G_LONG, GLIDE_I, GLIDE_I_VOWEL, GLIDE_U, GLIDE_U_VOWEL, I, K, K_FIRST, K_LONG, L, LY, M, N, NY, O,
        O_OPEN, OFFGLIDE_I, OFFGLIDE_U, P, P_FIRST, P_LONG, R, R_SECOND, S, S_BETWEEN, SH, T, T_FIRST, T_LONG, TS,
        TS_LONG, U, V, Z, legal_onset,
    },
    words::respelling,
};

/// What a written accent says about its vowel: stressed, and for e and o open (è, ò) or closed
/// (é, ó).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Accent {
    None,
    Stressed,
    Open,
    Closed,
}

/// A letter with its accent taken off, and what the accent said.
fn base(c: char) -> (char, Accent) {
    match c {
        'à' | 'á' => ('a', Accent::Stressed),
        'è' => ('e', Accent::Open),
        'é' => ('e', Accent::Closed),
        'ì' | 'í' | 'î' => ('i', Accent::Stressed),
        'ò' => ('o', Accent::Open),
        'ó' => ('o', Accent::Closed),
        'ù' | 'ú' => ('u', Accent::Stressed),
        c => (c, Accent::None),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Sound {
    phoneme: Phoneme,
    accent: Accent,
    /// After an i that only marks a soft c, g or sc (cielo, scienza): an e here is open, as after
    /// an i glide.
    after_silent_i: bool,
}

fn vowel_letter(c: Option<char>) -> bool {
    c.is_some_and(|c| "aeiou".contains(c))
}

/// The sounds of a word in lower case, `None` for a letter Italian does not read.
#[expect(clippy::too_many_lines, reason = "one arm per letter reads best as one table")]
fn sounds(word: &str) -> Option<Vec<Sound>> {
    let letters: Vec<(char, Accent)> = word.chars().map(base).collect();
    let letter = |at: usize| letters.get(at).map(|(c, _)| *c);
    let accent = |at: usize| letters.get(at).map_or(Accent::None, |(_, accent)| *accent);
    let front = |at: usize| matches!(letter(at), Some('e' | 'i'));
    // An unaccented i that only marks c, g or sc as soft before another vowel (cia, gio, scie).
    let silent_i = |at: usize| letter(at) == Some('i') && accent(at) == Accent::None && vowel_letter(letter(at + 1));
    let mut out: Vec<Sound> = Vec::with_capacity(letters.len());
    let push = |out: &mut Vec<Sound>, phoneme| {
        out.push(Sound {
            phoneme,
            accent: Accent::None,
            after_silent_i: false,
        });
    };
    let mut after_silent_i = false;
    let mut at = 0;
    while at < letters.len() {
        let (c, written) = letters[at];
        let next = letter(at + 1);
        let doubled = next == Some(c);
        let mut step = 1;
        let mut skips_i = false;
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                let phoneme = match c {
                    'a' => A,
                    'e' => E,
                    'i' => I,
                    'o' => O,
                    _ => U,
                };
                out.push(Sound {
                    phoneme,
                    accent: written,
                    after_silent_i,
                });
            }
            'c' | 'g' if !(c == 'c' && next == Some('q')) => {
                let (hard, soft, hard_first, hard_long, soft_first, soft_long) = if c == 'c' {
                    (K, CH, K_FIRST, K_LONG, T_FIRST, CH_LONG)
                } else {
                    (G, DJ, G_FIRST, G_LONG, D_FIRST, DJ_LONG)
                };
                let start = if doubled { at + 2 } else { at + 1 };
                if c == 'g' && !doubled && next == Some('n') {
                    push(&mut out, NY);
                    step = 2;
                } else if c == 'g' && !doubled && next == Some('l') && letter(at + 2) == Some('i') {
                    // gli: ʎ, its i silent before a vowel (famiglia), said otherwise (gli).
                    push(&mut out, LY);
                    step = if vowel_letter(letter(at + 3)) && accent(at + 2) == Accent::None {
                        3
                    } else {
                        2
                    };
                } else if letter(start) == Some('h') {
                    if doubled {
                        push(&mut out, hard_first);
                        push(&mut out, hard_long);
                    } else {
                        push(&mut out, hard);
                    }
                    step = start + 1 - at;
                } else if front(start) {
                    if doubled {
                        push(&mut out, soft_first);
                        push(&mut out, soft_long);
                    } else {
                        push(&mut out, soft);
                    }
                    skips_i = silent_i(start);
                    step = start - at + usize::from(skips_i);
                } else if doubled {
                    push(&mut out, hard_first);
                    push(&mut out, hard_long);
                    step = 2;
                } else {
                    push(&mut out, hard);
                }
            }
            // cq is a long k (acqua).
            'c' if next == Some('q') => {
                push(&mut out, K_FIRST);
                push(&mut out, K_LONG);
                step = 2;
            }
            'q' | 'k' => push(&mut out, K),
            's' if next == Some('c') && front(at + 2) => {
                push(&mut out, SH);
                skips_i = silent_i(at + 2);
                step = 2 + usize::from(skips_i);
            }
            's' if doubled => {
                push(&mut out, S);
                push(&mut out, S);
                step = 2;
            }
            's' => {
                let before_vowel = vowel_letter(letters.get(at.wrapping_sub(1)).map(|(c, _)| *c)) && at > 0;
                let voiced_next = matches!(next, Some('b' | 'd' | 'g' | 'v' | 'l' | 'm' | 'n' | 'r'));
                push(
                    &mut out,
                    if voiced_next {
                        Z
                    } else if before_vowel && vowel_letter(next) {
                        S_BETWEEN
                    } else {
                        S
                    },
                );
            }
            'z' | 'ż' => {
                // ż, in jespersen's respellings, is the voiced dz (mèżżo); a word starts with dz (zio).
                let voiced = c == 'ż' || (at == 0 && letter(at + 1) != Some('z'));
                if matches!(next, Some('z' | 'ż')) {
                    push(&mut out, if voiced { D_FIRST } else { T_FIRST });
                    push(&mut out, if voiced { DZ_LONG } else { TS_LONG });
                    step = 2;
                } else {
                    push(&mut out, if voiced { DZ } else { TS });
                }
            }
            'p' | 'b' | 't' | 'd' if doubled => {
                let (first, long) = match c {
                    'p' => (P_FIRST, P_LONG),
                    'b' => (B_FIRST, B_LONG),
                    't' => (T_FIRST, T_LONG),
                    _ => (D_FIRST, D_LONG),
                };
                push(&mut out, first);
                push(&mut out, long);
                step = 2;
            }
            'r' if doubled => {
                push(&mut out, R);
                push(&mut out, R_SECOND);
                step = 2;
            }
            'p' => push(&mut out, P),
            'b' => push(&mut out, B),
            't' => push(&mut out, T),
            'd' => push(&mut out, D),
            'f' => push(&mut out, F),
            'v' => push(&mut out, V),
            'l' => push(&mut out, L),
            'm' => push(&mut out, M),
            'n' => push(&mut out, N),
            'r' => push(&mut out, R),
            // h is silent; so is |, which in jespersen's respellings marks where the words of a
            // compound join (sta|sera): an s after it begins a word, so it stays voiceless.
            'h' | '|' => {}
            'j' | 'y' => push(&mut out, I),
            'w' => push(&mut out, GLIDE_U),
            'x' => {
                push(&mut out, K);
                push(&mut out, S);
            }
            _ => return None,
        }
        // The vowel after a silent i (cie, gie, scie) is marked; any other step clears the mark.
        after_silent_i = skips_i;
        at += step;
    }
    Some(espeak_glides(glides(out)))
}

/// eSpeak NG's way with glides: an i glide after a consonant past the word's first vowel, and a u glide
/// after a consonant other than k (qu) and g (gu), written as vowels (storia stˈɔria, fuoco fʊˈɔko,
/// but piedi pjˈedɪ, quale kwˈale).
fn espeak_glides(mut sounds: Vec<Sound>) -> Vec<Sound> {
    let mut vowel_seen = false;
    for at in 0..sounds.len() {
        let before = at.checked_sub(1).map(|before| sounds[before].phoneme);
        let after_consonant = before.is_some_and(|before| !before.is_vowel());
        match sounds[at].phoneme {
            // After b the glide stays (abbiamo abːjˈamo, cambiato).
            phoneme if phoneme == GLIDE_I && after_consonant && vowel_seen && !matches!(before, Some(B | B_LONG)) => {
                sounds[at].phoneme = GLIDE_I_VOWEL;
            }
            phoneme if phoneme == GLIDE_U && after_consonant && !matches!(before, Some(K | K_LONG | G | G_LONG)) => {
                sounds[at].phoneme = GLIDE_U_VOWEL;
            }
            phoneme if phoneme.is_vowel() => vowel_seen = true,
            _ => {}
        }
    }
    sounds
}

/// Unaccented i and u before a vowel are glides (piano, uomo, quale); after one, offglides (sei,
/// poi, auto).
fn glides(mut sounds: Vec<Sound>) -> Vec<Sound> {
    let high = |sound: &Sound| sound.accent == Accent::None && (sound.phoneme == I || sound.phoneme == U);
    for at in 0..sounds.len() {
        if !high(&sounds[at]) {
            continue;
        }
        // Before an i or u accented inside the word it stays a vowel of its own (Luigi lu-ì-gi); a
        // final accent is only spelling (più, già).
        let hiatus = |next: usize| {
            next + 1 < sounds.len() && sounds[next].accent != Accent::None && [I, U].contains(&sounds[next].phoneme)
        };
        let vowel_after = sounds.get(at + 1).is_some_and(|sound| sound.phoneme.is_vowel()) && !hiatus(at + 1);
        let vowel_before = at.checked_sub(1).is_some_and(|before| {
            sounds[before].phoneme.is_vowel()
                && !(high(&sounds[before]) && sounds[before].phoneme != sounds[at].phoneme)
        });
        sounds[at].phoneme = match (sounds[at].phoneme, vowel_after, vowel_before) {
            (phoneme, true, _) if phoneme == I => GLIDE_I,
            (_, true, _) => GLIDE_U,
            (phoneme, false, true) if phoneme == I => OFFGLIDE_I,
            (_, false, true) => OFFGLIDE_U,
            (phoneme, ..) => phoneme,
        };
    }
    sounds
}

/// Endings whose words are stressed on the last syllable but two: superlatives, -abile and
/// -ibile, adjectives in -ico, ordinals in -esimo, and the imperfect and past plurals (parlavano,
/// parlarono, strinsero, dettero).
const THIRD_FROM_LAST: [&str; 32] = [
    "issimo", "issima", "issimi", "issime", "abile", "abili", "ibile", "ibili", "evole", "evoli", "ico", "ica", "ici",
    "iche", "avano", "evano", "ivano", "arono", "erono", "irono", "ssero", "rebbero", "esimo", "esima", "esimi",
    "esime", "sero", "ttero", "bbero", "llero", "ddero", "cquero",
];

/// Endings whose stressed e or o is open: ɛ in -ente, -enza, -endo, -ello; ɔ in -orio.
const OPEN_ENDINGS: [&str; 12] = [
    "ente", "enti", "ento", "enza", "enze", "endo", "ello", "ella", "elli", "elle", "orio", "oria",
];

/// How `word` is said, `None` for a word with a letter Italian does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let lower = word.to_lowercase();
    let spelt = respelling(&lower).unwrap_or(&lower);
    let sounds = sounds(spelt)?;
    let nuclei: Vec<usize> = (0..sounds.len()).filter(|&at| sounds[at].phoneme.is_vowel()).collect();
    if nuclei.is_empty() {
        let segments: Vec<Segment> = sounds
            .iter()
            .map(|sound| Segment::new(sound.phoneme, Stress::Unstressed))
            .collect();
        return (!segments.is_empty()).then(|| syllabify(&segments, legal_onset));
    }
    let written = nuclei.iter().copied().find(|&at| sounds[at].accent != Accent::None);
    let from_end = if (THIRD_FROM_LAST.iter().any(|ending| spelt.ends_with(ending)) || gerund_with_clitic(spelt))
        && nuclei.len() >= 3
    {
        3
    } else if spelt.ends_with("ai") && sounds.last().is_some_and(|sound| sound.phoneme == OFFGLIDE_I) {
        // A final -ai is stressed: the passato remoto's first person (trovai, mangiai), the
        // plurals of -aio (operai) and adverbs (assai, ormai).
        1
    } else if spelt.ends_with(|c: char| "aeiouàèéìòóù".contains(c)) {
        2
    } else {
        1
    };
    let stressed = written.unwrap_or(nuclei[nuclei.len().saturating_sub(from_end.min(nuclei.len()))]);
    let mut segments: Vec<Segment> = sounds
        .iter()
        .enumerate()
        .map(|(at, sound)| {
            Segment::new(
                sound.phoneme,
                if at == stressed {
                    Stress::Primary
                } else {
                    Stress::Unstressed
                },
            )
        })
        .collect();
    segments[stressed].phoneme = quality(&sounds, stressed, spelt, from_end == 3 && written.is_none());
    // ɲ and ʎ after the stressed vowel and before another are long (legno ˈleɲɲo, meglio ˈmɛʎʎo,
    // but falegname, sbagliavo zbaˈʎavo). Kokoro says them best so, by the owner's ear.
    for palatal in [NY, LY] {
        if segments
            .get(stressed + 1)
            .is_some_and(|segment| segment.phoneme == palatal)
            && segments
                .get(stressed + 2)
                .is_some_and(|segment| segment.phoneme.is_vowel())
        {
            segments.insert(stressed + 1, Segment::new(palatal, Stress::Unstressed));
        }
    }
    Some(syllabify(&segments, legal_onset))
}

/// The stressed vowel's quality: as its accent says; else ɛ after an i glide and ɔ after a u glide
/// (piede, buono), as the ending says, open in the last syllable but two (mèdico, pòpolo), and
/// closed otherwise.
fn quality(sounds: &[Sound], at: usize, spelt: &str, third_from_last: bool) -> Phoneme {
    let sound = sounds[at];
    let (open, closed) = match sound.phoneme {
        phoneme if phoneme == E => (E_OPEN, E),
        phoneme if phoneme == O => (O_OPEN, O),
        phoneme => return phoneme,
    };
    match sound.accent {
        // è and ò are open, and so is a final accent on e or o written plainly (caffè, però); é and
        // ó are written when it is not (perché).
        Accent::Open | Accent::Stressed => return open,
        Accent::Closed => return closed,
        Accent::None => {}
    }
    let glide_before = at.checked_sub(1).map(|before| sounds[before].phoneme);
    let i_before = matches!(glide_before, Some(GLIDE_I | GLIDE_I_VOWEL)) || sound.after_silent_i;
    let u_before = matches!(glide_before, Some(GLIDE_U | GLIDE_U_VOWEL));
    if (sound.phoneme == E && i_before) || (sound.phoneme == O && u_before) {
        return open;
    }
    let ending_open = spelt.chars().count() > 4 && OPEN_ENDINGS.iter().any(|ending| spelt.ends_with(ending));
    if ending_open || third_from_last || before_final_io(sounds, at) {
        return open;
    }
    if sound.phoneme == E && IMPERFECT.iter().any(|ending| spelt.ends_with(ending)) {
        return E_IMPERFECT;
    }
    closed
}

/// Imperfect endings, whose stressed e eSpeak NG writes open.
const IMPERFECT: [&str; 6] = ["eva", "evo", "evi", "evano", "evamo", "evate"];

/// Whether the stressed vowel at `at` is followed by consonants, an i glide or ʎ, and the word's
/// last vowel (Pinocchio, Antonio, premio, vecchio, foglio): such e and o are open.
fn before_final_io(sounds: &[Sound], at: usize) -> bool {
    let rest = &sounds[at + 1..];
    let consonants = rest.iter().take_while(|sound| !sound.phoneme.is_vowel()).count();
    let (cluster, after) = rest.split_at(consonants);
    let final_vowel = matches!(after, [last] if last.phoneme.is_vowel());
    // A consonant and an i glide (-chio, -nio), or ʎ alone (foglio).
    let glide = cluster.len() >= 2
        && cluster
            .last()
            .is_some_and(|last| [GLIDE_I, GLIDE_I_VOWEL].contains(&last.phoneme));
    let palatal = !cluster.is_empty() && cluster.iter().all(|sound| sound.phoneme == LY);
    final_vowel && (glide || palatal)
}

/// Pronouns that join the end of a verb.
const CLITICS: [&str; 11] = ["si", "ci", "mi", "ti", "vi", "lo", "la", "li", "le", "ne", "gli"];

/// Whether `word` is a gerund with a pronoun joined to it, which keeps the gerund's stress
/// (voltandosi voltàndosi, dicendolo dicèndolo).
fn gerund_with_clitic(word: &str) -> bool {
    CLITICS
        .iter()
        .filter_map(|clitic| word.strip_suffix(clitic))
        .any(|verb| verb.ends_with("ando") || verb.ends_with("endo"))
}

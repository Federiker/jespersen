//! How a Polish word is said from its spelling: its letters and digraphs (cz, sz, rz, ch, dz, dż,
//! dź), an i that only softens the consonant before it (nie, biuro, siedzi), voicing that follows
//! the last obstruent of a cluster (wszystko fʃ, prośba ʑb) and is lost at the end of a word, ą and
//! ę nasal before a fricative and a nasal consonant before a stop (ręka rɛŋka), and the stress on
//! the last syllable but one.

use jespersen_phonemes::{Phoneme, Pronunciation, Segment, Stress, syllabify};

use crate::phonemes::{
    A, B, B_BEFORE_I, B_SOFT, CH, D, DJ, DZ, DZH, E, E_NASAL, E_NASAL_SPLIT, F, F_SOFT, G, G_BEFORE_I, G_SOFT, I, J, K,
    K_SOFT, L, M, M_SOFT, M_SOFT_INSIDE, N, N_SOFT, NG, NY, NY_SPLIT, O, O_NASAL, P, P_SOFT, R, S, SH, SJ, T, TJ, TS,
    U, V, V_ALONE, V_SOFT, W, X, X_BEFORE_I, Y, Z, ZH, ZJ, is_obstruent, is_sonorant, is_voiced, with_voicing,
};

/// A sound as the letters give it, before voicing and the nasal vowels are settled.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Sound {
    phoneme: Phoneme,
    /// A v (w) or the rz sound: voiced or not by the obstruent before it, voicing none itself.
    passive: bool,
}

/// What consonant letters say: on their own, before an i that only softens them (and is then
/// silent), and before the vowel i.
struct Letters {
    plain: &'static [Phoneme],
    soft: Option<&'static [Phoneme]>,
    before_i: &'static [Phoneme],
    length: usize,
    passive: bool,
}

const fn letters(plain: &'static [Phoneme], soft: Option<&'static [Phoneme]>, before_i: &'static [Phoneme]) -> Letters {
    Letters {
        plain,
        soft,
        before_i,
        length: 1,
        passive: false,
    }
}

/// The consonant letters at the start of `rest`; `None` for a letter Polish does not read.
fn consonant(rest: &[char]) -> Option<Letters> {
    let digraph = |plain: &'static [Phoneme], soft: Option<&'static [Phoneme]>, before_i: &'static [Phoneme]| Letters {
        length: 2,
        ..letters(plain, soft, before_i)
    };
    Some(match rest {
        ['d', 'ż', ..] => digraph(&[DZH], None, &[DZH]),
        ['d', 'ź', ..] => digraph(&[DJ], None, &[DJ]),
        ['d', 'z', ..] => digraph(&[DZ], Some(&[DJ]), &[DJ]),
        ['c', 'z', ..] => digraph(&[CH], None, &[CH]),
        ['s', 'z', ..] => digraph(&[SH], None, &[SH]),
        ['r', 'z', ..] => Letters {
            passive: true,
            ..digraph(&[ZH], None, &[ZH])
        },
        ['c', 'h', ..] => digraph(&[X], Some(&[X_BEFORE_I]), &[X_BEFORE_I]),
        [letter, ..] => match letter {
            'b' => letters(&[B], Some(&[B_SOFT]), &[B_BEFORE_I]),
            'c' => letters(&[TS], Some(&[TJ]), &[TJ]),
            'ć' => letters(&[TJ], None, &[TJ]),
            'd' => letters(&[D], Some(&[D, J]), &[D]),
            'f' => letters(&[F], Some(&[F_SOFT]), &[F]),
            'g' => letters(&[G], Some(&[G_SOFT]), &[G_BEFORE_I]),
            'h' => letters(&[X], Some(&[X_BEFORE_I]), &[X_BEFORE_I]),
            'j' => letters(&[J], None, &[J]),
            'k' | 'q' => letters(&[K], Some(&[K_SOFT]), &[K]),
            'l' => letters(&[L], Some(&[L, J]), &[L]),
            'ł' => letters(&[W], None, &[W]),
            'm' => letters(&[M], Some(&[M_SOFT]), &[M]),
            'n' => letters(&[N], Some(&[N_SOFT]), &[NY]),
            'ń' => letters(&[NY], None, &[NY]),
            'p' => letters(&[P], Some(&[P_SOFT]), &[P]),
            'r' => letters(&[R], Some(&[R, J]), &[R]),
            's' => letters(&[S], Some(&[SJ]), &[SJ]),
            'ś' => letters(&[SJ], None, &[SJ]),
            't' => letters(&[T], Some(&[T, J]), &[T]),
            'w' | 'v' => Letters {
                passive: true,
                ..letters(&[V], Some(&[V_SOFT]), &[V])
            },
            'x' => letters(&[K, S], None, &[K, S]),
            'z' => letters(&[Z], Some(&[ZJ]), &[ZJ]),
            'ź' => letters(&[ZJ], None, &[ZJ]),
            'ż' => letters(&[ZH], None, &[ZH]),
            _ => return None,
        },
        [] => return None,
    })
}

/// A vowel letter's sound; ą and ę are settled once the sound after them is known.
fn vowel(letter: char) -> Option<Phoneme> {
    Some(match letter {
        'a' => A,
        'ą' => O_NASAL,
        // é, the slanted e of the old spelling (niniejszéj), is read as e today.
        'e' | 'é' => E,
        'ę' => E_NASAL,
        'i' => I,
        'o' => O,
        'ó' | 'u' => U,
        'y' => Y,
        _ => return None,
    })
}

/// The sounds of `word`'s letters.
fn sounds(word: &[char]) -> Option<Vec<Sound>> {
    let is_vowel = |at: usize| word.get(at).copied().and_then(vowel).is_some();
    let mut sounds = Vec::with_capacity(word.len());
    let mut at = 0;
    while at < word.len() {
        if let Some(phoneme) = vowel(word[at]) {
            // The u of au and eu is said as ł (auto, Europa), but not after the prefixes na- and
            // za- (nauka, zauważyć).
            let glide = word[at] == 'u'
                && at > 0
                && matches!(word[at - 1], 'a' | 'e')
                && !(at == 2 && matches!(word[0], 'n' | 'z'));
            let phoneme = if glide { W } else { phoneme };
            sounds.push(Sound {
                phoneme,
                passive: false,
            });
            at += 1;
            continue;
        }
        if matches!(word[at], '\'' | '’' | '-') {
            at += 1;
            continue;
        }
        let found = consonant(&word[at..])?;
        let inside = at > 0;
        at += found.length;
        let phonemes = if word.get(at) == Some(&'i') {
            match found.soft {
                // eSpeak NG writes mie inside a word mj (ziemie ʑˈɛmjɛ).
                Some(&[M_SOFT]) if inside && word.get(at + 1) == Some(&'e') => {
                    at += 1;
                    &[M_SOFT_INSIDE]
                }
                Some(soft) if is_vowel(at + 1) => {
                    at += 1;
                    soft
                }
                _ => found.before_i,
            }
        } else {
            found.plain
        };
        sounds.extend(phonemes.iter().map(|&phoneme| Sound {
            phoneme,
            passive: found.passive,
        }));
    }
    Some(sounds)
}

/// Voicing inside a word: in a cluster of obstruents the last that is not a v or rz gives its
/// voicing to those before it (wszystko fʃ, babcia ptɕ), a voiceless one also to a v or rz after
/// it (twój tf, przy pʃ), even across an r (trwać trf).
fn assimilate(sounds: &mut [Sound]) {
    let mut start = 0;
    while start < sounds.len() {
        let length = sounds[start..]
            .iter()
            .take_while(|sound| is_obstruent(sound.phoneme))
            .count();
        let cluster = &mut sounds[start..start + length];
        if let Some(last) = cluster.iter().rposition(|sound| !sound.passive) {
            let voiced = is_voiced(cluster[last].phoneme);
            let (before, after) = cluster.split_at_mut(last);
            for sound in before {
                sound.phoneme = with_voicing(sound.phoneme, voiced);
            }
            if !voiced {
                for sound in &mut after[1..] {
                    sound.phoneme = with_voicing(sound.phoneme, false);
                }
            }
        }
        start += length.max(1);
    }
    for index in 2..sounds.len() {
        let obstruent = sounds[index - 2].phoneme;
        if sounds[index].passive && sounds[index - 1].phoneme == R && is_obstruent(obstruent) && !is_voiced(obstruent) {
            sounds[index].phoneme = with_voicing(sounds[index].phoneme, false);
        }
    }
}

/// The phonemes of `sounds` with ą and ę settled: a vowel and a nasal consonant before a stop or
/// an affricate (ząb zɔmp, ręka rɛŋka, pięć pjɛɲtɕ), oral before l and ł, ę oral at the end of a
/// word (się), nasal vowels elsewhere (mąż, często); n is ŋ before k and g.
fn settle(sounds: &[Sound]) -> Vec<Phoneme> {
    let mut phonemes = Vec::with_capacity(sounds.len() + 2);
    for (index, sound) in sounds.iter().enumerate() {
        let next = sounds.get(index + 1).map(|sound| sound.phoneme);
        let nasal = match sound.phoneme {
            O_NASAL => O,
            E_NASAL => E,
            N if next.is_some_and(|next| [K, G, K_SOFT, G_SOFT, G_BEFORE_I].contains(&next)) => {
                phonemes.push(NG);
                continue;
            }
            phoneme => {
                // Two of the same affricate are said as one (sześćdziesiąt ʃɛʑdʑɛɕɔnt).
                if phonemes.last() != Some(&phoneme) || ![TS, DZ, CH, DZH, TJ, DJ].contains(&phoneme) {
                    phonemes.push(phoneme);
                }
                continue;
            }
        };
        let consonant = match next {
            Some(P | B | P_SOFT | B_SOFT | B_BEFORE_I) => Some(M),
            Some(T | D | TS | DZ | CH | DZH) => Some(N),
            Some(TJ | DJ) => Some(NY),
            Some(K | G | K_SOFT | G_SOFT | G_BEFORE_I) => Some(NG),
            _ => None,
        };
        // ę is oral before tn (piętnaście, dziewiętnasty).
        let before_tn = next == Some(T) && sounds.get(index + 2).is_some_and(|sound| sound.phoneme == N);
        match (consonant, next) {
            _ if sound.phoneme == E_NASAL && before_tn => phonemes.push(E),
            (Some(consonant), _) => phonemes.extend([nasal, consonant]),
            (None, Some(L | W)) => phonemes.push(nasal),
            (None, Some(SJ | ZJ)) if sound.phoneme == E_NASAL => phonemes.extend([E_NASAL_SPLIT, NY_SPLIT]),
            (None, None) if sound.phoneme == E_NASAL => phonemes.push(E),
            (None, _) => phonemes.push(sound.phoneme),
        }
    }
    phonemes
}

/// Words stressed on the third syllable from the end: Greek and Latin words in -yka and -ika, in
/// all their forms (matematyki, fizyce).
const THIRD_FROM_END: [&str; 34] = [
    "akustyka",
    "ameryka",
    "botanika",
    "dynamika",
    "elektronika",
    "estetyka",
    "fizyka",
    "fonetyka",
    "genetyka",
    "gimnastyka",
    "grafika",
    "gramatyka",
    "informatyka",
    "klinika",
    "krytyka",
    "logika",
    "logistyka",
    "matematyka",
    "mechanika",
    "muzyka",
    "optyka",
    "panika",
    "polityka",
    "praktyka",
    "problematyka",
    "publika",
    "republika",
    "robotyka",
    "semantyka",
    "statystyka",
    "taktyka",
    "technika",
    "tematyka",
    "turystyka",
];

/// How many syllables from the end the stress falls: the last but one, but the third from the end
/// in past and conditional forms (byliśmy, zrobiłaby) and words in -yka (matematyka), the fourth in
/// -libyśmy (zrobilibyśmy).
fn stress_from_end(word: &str) -> usize {
    const FOURTH: [&str; 4] = ["libyśmy", "łybyśmy", "libyście", "łybyście"];
    const THIRD: [&str; 12] = [
        "liśmy", "łyśmy", "liście", "łyście", "byśmy", "byście", "łaby", "łoby", "łyby", "liby", "łabym", "łabyś",
    ];
    if FOURTH.iter().any(|ending| word.ends_with(ending)) {
        return 4;
    }
    let yka = THIRD_FROM_END.iter().any(|lemma| {
        let stem = lemma.trim_end_matches("ka");
        word.strip_prefix(stem)
            .is_some_and(|ending| matches!(ending, "ka" | "ki" | "kę" | "ką" | "ce" | "kom" | "kami" | "kach"))
    });
    if yka || THIRD.iter().any(|ending| word.ends_with(ending)) {
        return 3;
    }
    2
}

/// Whether `cluster` can start a syllable inside a word: one consonant, or obstruents (two only if
/// the first is a fricative or the second a sibilant: sk, pʃ) and at most one sonorant after them,
/// optionally with j (kl, str, pr, but not tf or ŋk).
fn legal_onset(cluster: &[Phoneme]) -> bool {
    if cluster.len() == 1 {
        return true;
    }
    let fricative = |phoneme: Phoneme| [F, V, S, Z, SH, ZH, SJ, ZJ, X, F_SOFT, V_SOFT, X_BEFORE_I].contains(&phoneme);
    let sibilant = |phoneme: Phoneme| [S, Z, SH, ZH, SJ, ZJ].contains(&phoneme);
    let obstruents = cluster.iter().take_while(|&&phoneme| is_obstruent(phoneme)).count();
    let sonorants_fit = match &cluster[obstruents..] {
        [] => true,
        [sonorant] | [sonorant, J] => is_sonorant(*sonorant),
        _ => false,
    };
    sonorants_fit
        && match &cluster[..obstruents] {
            [_] => true,
            [first, second] => fricative(*first) || sibilant(*second),
            [first, second, third] => fricative(*first) && (sibilant(*third) || !fricative(*second)),
            _ => false,
        }
}

/// How `word` is said; `None` for a letter Polish does not read.
pub(crate) fn pronounce(word: &str) -> Option<Pronunciation> {
    let lower = word.to_lowercase();
    // The preposition w stays a v unless what follows it is voiceless (sentence.rs).
    if lower == "w" {
        return Some(syllabify(&[Segment::new(V_ALONE, Stress::Unstressed)], legal_onset));
    }
    let letters: Vec<char> = lower.chars().collect();
    let mut sounds = sounds(&letters)?;
    if sounds.is_empty() {
        return None;
    }
    assimilate(&mut sounds);
    // A word loses the voicing of its last obstruents (chleb xlɛp, Kraków krakuf); a word without
    // a vowel (z) keeps it for the word after it to settle.
    if sounds.iter().any(|sound| sound.phoneme.is_vowel()) {
        for sound in sounds.iter_mut().rev() {
            if !is_obstruent(sound.phoneme) {
                break;
            }
            sound.phoneme = with_voicing(sound.phoneme, false);
        }
    }
    let phonemes = settle(&sounds);
    let vowels: Vec<usize> = (0..phonemes.len())
        .filter(|&index| phonemes[index].is_vowel())
        .collect();
    let from_end = stress_from_end(&lower);
    let primary = vowels.len().saturating_sub(from_end);
    let stressed = vowels.get(primary).copied();
    let segments: Vec<Segment> = phonemes
        .iter()
        .enumerate()
        .map(|(index, &phoneme)| {
            let stress = if Some(index) == stressed {
                Stress::Primary
            } else {
                Stress::Unstressed
            };
            Segment::new(phoneme, stress)
        })
        .collect();
    let mut said = syllabify(&segments, legal_onset);
    // eSpeak NG's rhythm: a secondary stress on the first syllable when two or more come before
    // the stress (ˌuɲivɛrsˈɨtɛt), and two syllables after a stress further back than the last but
    // one (ɕpʲɛvˈaliɕmˌɨ, mˌatɛmˈatɨkˌa).
    if primary >= 2
        && let Some(first) = said.syllables.first_mut()
    {
        first.stress = Stress::Rhythmic;
    }
    if from_end > 2
        && stressed.is_some()
        && let Some(later) = said.syllables.get_mut(primary + 2)
    {
        later.stress = Stress::Rhythmic;
    }
    Some(said)
}

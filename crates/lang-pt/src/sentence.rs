use jespersen_phonemes::{Pronunciation, Stress, Syllable};
use jespersen_spoken::{Speaker, TokenReading};
use jespersen_text::TokenKind;

use crate::{
    Variety,
    locale::Portuguese,
    phonemes::{
        E_FINAL, GLIDE_I, GLIDE_U, I, K, L, LH_EUROPEAN, NASAL_CODA_LABIAL, O_FINAL, O_NASAL, O_REDUCED, R_BRAZILIAN,
        S, SH, TAP, Z, ZH, is_voiced,
    },
    spelling::pronounce,
    words::respelling,
};

/// Function words said with the vowels of unstressed syllables (de dɨ, que kɨ, para pɐɾɐ).
const REDUCED: [&str; 38] = [
    "o", "a", "os", "as", "de", "do", "da", "dos", "das", "no", "na", "nos", "nas", "que", "se", "me", "te", "lhe",
    "lhes", "vos", "por", "para", "num", "numa", "mas", "nem", "ou", "em", "um", "e", "seu", "ser", "la", "las", "lo",
    "los", "dum", "duma",
];
/// Words said without stress in running text: articles, most prepositions, clitic pronouns and
/// conjunctions; Brazil's voice adds com, às and ou.
const UNSTRESSED_EUROPEAN: [&str; 42] = [
    "o", "a", "os", "as", "de", "do", "da", "dos", "das", "no", "na", "nos", "nas", "ao", "aos", "que", "se", "me",
    "te", "lhes", "vos", "por", "e", "mas", "um", "num", "dum", "meu", "meus", "seu", "seus", "teu", "foi", "nós",
    "eu", "há", "la", "las", "lo", "los", "ser", "é",
];
const UNSTRESSED_BRAZILIAN: [&str; 45] = [
    "o", "a", "os", "as", "de", "do", "da", "dos", "das", "no", "na", "nos", "nas", "ao", "aos", "que", "se", "me",
    "te", "lhes", "vos", "por", "e", "mas", "um", "num", "dum", "meu", "meus", "seu", "seus", "teu", "foi", "nós",
    "eu", "há", "la", "las", "lo", "los", "ser", "é", "com", "às", "ou",
];
/// Words said with a lighter stress (eSpeak NG writes a secondary one); Brazil's voice lightens
/// more of them.
const LIGHTER_EUROPEAN: [&str; 32] = [
    "para", "uma", "à", "às", "está", "estão", "você", "tenho", "temos", "sua", "suas", "minha", "minhas", "ela",
    "ele", "elas", "eles", "numa", "duma", "este", "esse", "não", "sempre", "era", "eram", "tinha", "tinham", "aquela",
    "aquelas", "isso", "seria", "nela",
];
const LIGHTER_BRAZILIAN: [&str; 42] = [
    "para", "uma", "à", "está", "estão", "você", "tenho", "temos", "sua", "suas", "minha", "minhas", "ela", "ele",
    "elas", "eles", "numa", "duma", "este", "esse", "não", "sempre", "era", "eram", "tinha", "tinham", "aquela",
    "aquelas", "isso", "como", "até", "será", "vamos", "nosso", "nossa", "nossos", "nossas", "estes", "onde", "desde",
    "seria", "nela",
];

/// How each token (text and kind) of a Portuguese sentence is read in `variety`, in order; `None`
/// for a token jespersen cannot read yet. Words are read from their spelling (and jespersen's list of
/// respellings, ela → éla), numbers, abbreviations and symbols as words. The words between
/// punctuation marks are read together: function words lose their stress, a final s takes the
/// voicing of the word after it (os amigos uz ɐmˈiɡʊʃ), and a final o glides into a vowel.
#[must_use]
pub fn read_tokens(tokens: &[(&str, TokenKind)], variety: Variety) -> Vec<Option<TokenReading>> {
    jespersen_spoken::read_tokens(tokens, &Portuguese { variety }, &Reader { variety })
}

/// How Portuguese says its words, for the shared reader.
struct Reader {
    variety: Variety,
}

impl Speaker for Reader {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        say(word, self.variety)
    }

    fn unstressed(&self) -> &[&str] {
        match self.variety {
            Variety::European => &UNSTRESSED_EUROPEAN,
            Variety::Brazilian => &UNSTRESSED_BRAZILIAN,
        }
    }

    fn lighter(&self) -> &[&str] {
        match self.variety {
            Variety::European => &LIGHTER_EUROPEAN,
            Variety::Brazilian => &LIGHTER_BRAZILIAN,
        }
    }

    /// Across words: a final s is z before a vowel and, in Portugal ʒ, in Brazil z, before a voiced
    /// consonant (os nossos uʒ nˈɔsʊʃ); a final o before a vowel is a glide (ɡˈatw estˌa), or u in a
    /// word of one syllable, as a Brazilian final e is (dʒj ˌaveɪŋtˈuɾæs); a Brazilian final r
    /// before a vowel is a tap.
    fn run(&self, words: &mut [&mut Pronunciation]) {
        // A word said with a lighter stress loses its rhythmic ones (aquela ɐkˌɛlɐ).
        for said in words.iter_mut() {
            if !said.syllables.iter().any(|syllable| syllable.stress == Stress::Primary) {
                for syllable in &mut said.syllables {
                    if syllable.stress == Stress::Rhythmic {
                        syllable.stress = Stress::Unstressed;
                    }
                }
            }
        }
        for index in 0..words.len().saturating_sub(1) {
            let Some(next) = words[index + 1].phonemes().next() else {
                continue;
            };
            let said = &mut *words[index];
            let syllables = said.syllables.len();
            let stressed = said.syllables.iter().any(|syllable| syllable.stress == Stress::Primary);
            let Some(last) = said.syllables.last_mut() else {
                continue;
            };
            let vowel_next = next.is_vowel();
            let end = if last.coda.is_empty() && last.nucleus.is_empty() {
                &mut last.onset
            } else {
                &mut last.coda
            };
            if let Some(final_consonant) = end.last_mut() {
                if [S, SH].contains(final_consonant) && (vowel_next || is_voiced(next)) {
                    *final_consonant = if vowel_next || self.variety == Variety::Brazilian {
                        Z
                    } else {
                        ZH
                    };
                } else if *final_consonant == R_BRAZILIAN && vowel_next {
                    *final_consonant = TAP;
                }
                continue;
            }
            if vowel_next {
                glide(last, syllables, stressed, self.variety);
            }
        }
    }
}

/// A final o (or Brazilian e) before a word that starts with a vowel: the o of a stressed word of
/// more than one syllable glides (muito mwˈiŋtw, but como kˌomʊ); a Brazilian e is i after r and l
/// (sobre sˈobri, ele ˈeli), stays in que and lhe, and glides elsewhere (de dʒj, se sj, porque pˈoɾəkj).
fn glide(last: &mut Syllable, syllables: usize, stressed: bool, variety: Variety) {
    if last.nucleus == [O_FINAL] {
        if syllables > 1 && stressed {
            last.nucleus.clear();
            last.coda.insert(0, GLIDE_U);
        } else if last.onset.is_empty() {
            // The article o alone is u (o avô u ɐvˈo); do and no keep ʊ.
            last.nucleus = vec![O_REDUCED];
        }
    } else if last.nucleus == [E_FINAL] && variety == Variety::Brazilian {
        match last.onset.last() {
            Some(consonant) if [R_BRAZILIAN, TAP, L].contains(consonant) => last.nucleus = vec![I],
            Some(consonant) if syllables == 1 && [K, LH_EUROPEAN].contains(consonant) => {}
            _ => {
                last.nucleus.clear();
                last.coda.insert(0, GLIDE_I);
            }
        }
    }
}

/// How a word is said: from jespersen's respellings or its spelling; a function word with unstressed
/// vowels, and e (and) as i.
pub(crate) fn say(word: &str, variety: Variety) -> Option<Pronunciation> {
    let lower = word.to_lowercase();
    if lower == "e" {
        return pronounce("i", variety, true);
    }
    // Portugal's voice says com with its m (kˈom), a nasal o in plain IPA.
    if lower == "com" && variety == Variety::European {
        let mut said = pronounce("co", variety, false)?;
        let syllable = said.syllables.last_mut()?;
        syllable.nucleus = vec![O_NASAL];
        syllable.coda = vec![NASAL_CODA_LABIAL];
        return Some(said);
    }
    let reduced = REDUCED.contains(&lower.as_str());
    let spelt = respelling(&lower).unwrap_or(&lower);
    pronounce(spelt, variety, reduced)
}

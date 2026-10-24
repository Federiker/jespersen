use crate::{Notation, Phoneme, Stress, Tone};

/// One syllable: consonants before the nucleus, the nucleus, consonants after it.
///
/// The nucleus is empty only in a word without vowels (an interjection such as "hmm" or "psst"),
/// which is then one syllable of consonants.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Syllable {
    pub onset: Vec<Phoneme>,
    pub nucleus: Vec<Phoneme>,
    pub coda: Vec<Phoneme>,
    pub stress: Stress,
    /// Its tone, in a tone language.
    pub tone: Option<Tone>,
    /// How the language spells it, for the notations that write spellings (pinyin, zhuyin).
    pub spelling: Option<Spelling>,
}

/// How a syllable is spelt in a language's phonetic spellings, tone included: Mandarin's pinyin
/// (`háng`) and zhuyin (`ㄏㄤˊ`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Spelling {
    pub pinyin: String,
    pub zhuyin: String,
}

impl Syllable {
    /// Every phoneme of the syllable, in order.
    pub fn phonemes(&self) -> impl Iterator<Item = Phoneme> + '_ {
        self.onset.iter().chain(&self.nucleus).chain(&self.coda).copied()
    }

    fn write(&self, notation: Notation, out: &mut String) {
        match (notation, &self.spelling) {
            (Notation::Pinyin, Some(spelling)) => return out.push_str(&spelling.pinyin),
            (Notation::Zhuyin, Some(spelling)) => return out.push_str(&spelling.zhuyin),
            _ => {}
        }
        // Without a spelling, pinyin and zhuyin fall back to plain IPA.
        let notation = match notation {
            Notation::Pinyin | Notation::Zhuyin => Notation::Ipa,
            notation => notation,
        };
        let push_all = |phonemes: &[Phoneme], out: &mut String| {
            for phoneme in phonemes {
                out.push_str(phoneme.symbol(notation));
            }
        };
        let mark = self.stress.mark(notation);
        if notation == Notation::Ipa {
            out.extend(mark);
        }
        push_all(&self.onset, out);
        if notation != Notation::Ipa {
            out.extend(mark);
        }
        push_all(&self.nucleus, out);
        // misaki writes the tone's arrow after the vowels (ʂa↘ŋ), plain IPA its letters after the
        // syllable (ʂaŋ˥˩).
        if notation == Notation::Misaki {
            out.extend(self.tone.and_then(Tone::arrow));
        }
        push_all(&self.coda, out);
        if notation == Notation::Ipa
            && let Some(tone) = self.tone
        {
            tone.write_letters(out);
        }
    }
}

/// How one word is pronounced: its syllables in order.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Pronunciation {
    pub syllables: Vec<Syllable>,
}

impl Pronunciation {
    /// Every phoneme of the word, in order.
    pub fn phonemes(&self) -> impl Iterator<Item = Phoneme> + '_ {
        self.syllables.iter().flat_map(Syllable::phonemes)
    }

    /// Appends the word written in `notation` to `out`, without spaces around it. Pinyin puts an
    /// apostrophe before a syllable that starts with a, o or e inside a word (xī'ān).
    pub fn write(&self, notation: Notation, out: &mut String) {
        for (index, syllable) in self.syllables.iter().enumerate() {
            if notation == Notation::Pinyin
                && index > 0
                && syllable.spelling.as_ref().is_some_and(|spelling| {
                    spelling.pinyin.starts_with([
                        'a', 'o', 'e', 'ā', 'á', 'ǎ', 'à', 'ō', 'ó', 'ǒ', 'ò', 'ē', 'é', 'ě', 'è',
                    ])
                })
            {
                out.push('\'');
            }
            syllable.write(notation, out);
        }
    }

    /// The word written in `notation`.
    #[must_use]
    pub fn to_notation(&self, notation: Notation) -> String {
        let mut out = String::new();
        self.write(notation, &mut out);
        out
    }
}

/// How phonemes are written out. Not every language is written in every notation: each lists
/// the ones it supports.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Notation {
    /// Plain IPA: stress marks before the syllable they belong to, tone letters after it.
    #[default]
    Ipa,
    /// The IPA conventions of eSpeak NG's output, which voices such as Kokoro were trained on:
    /// its own symbol choices, and stress marks before the stressed vowel.
    Espeak,
    /// The notation of misaki, Kokoro's own front end for Chinese, which Kokoro's Chinese voices
    /// were trained on: IPA with its own symbol choices (ꭧ for zh) and an arrow for the tone after
    /// the syllable's vowels. Other languages' words inside are written as in eSpeak NG's notation.
    Misaki,
    /// Hanyu pinyin with tone marks (yínháng), for syllables that carry their spelling (Mandarin);
    /// other words are written in plain IPA.
    Pinyin,
    /// Zhuyin, or bopomofo (ㄧㄣˊㄏㄤˊ), for syllables that carry their spelling (Mandarin); other
    /// words are written in plain IPA.
    Zhuyin,
}

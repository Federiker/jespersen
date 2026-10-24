use std::{fmt, str::FromStr};

use jespersen_lexicon::LexiconError;
use jespersen_phonemes::{Notation, Pronunciation};
#[cfg(all(feature = "en", feature = "zh"))]
use jespersen_text::Script;
use jespersen_text::TokenKind;

#[cfg(feature = "en")]
use jespersen_lang_en::{BathVowel, Variety};
#[cfg(feature = "es")]
use jespersen_lang_es::Variety as SpanishVariety;
#[cfg(feature = "pt")]
use jespersen_lang_pt::Variety as PortugueseVariety;
#[cfg(feature = "zh")]
use jespersen_lang_zh::ToneSandhi;

use crate::Error;

/// A language jespersen reads. Each comes with its Cargo feature (`en` for English, `zh` for
/// Mandarin, both on by default).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Language {
    /// American English (`en-US`).
    #[cfg(feature = "en")]
    AmericanEnglish,
    /// British English (`en-GB`), with the vowel of bath, dance and path chosen
    /// (docs/decisions.md, decision 28).
    #[cfg(feature = "en")]
    BritishEnglish(BathVowel),
    /// Spanish, Castilian (`es`, `es-ES`) or Latin American (`es-419`, `es-MX`…) (decision 36).
    #[cfg(feature = "es")]
    Spanish(SpanishVariety),
    /// Finnish (`fi`).
    #[cfg(feature = "fi")]
    Finnish,
    /// Italian (`it`).
    #[cfg(feature = "it")]
    Italian,
    /// Oromo (`om`).
    #[cfg(feature = "om")]
    Oromo,
    /// Somali (`so`).
    #[cfg(feature = "so")]
    Somali,
    /// Shona (`sn`).
    #[cfg(feature = "sn")]
    Shona,
    /// Lingala (`ln`).
    #[cfg(feature = "ln")]
    Lingala,
    /// Luganda (`lg`).
    #[cfg(feature = "lg")]
    Luganda,
    /// Kirundi (`rn`).
    #[cfg(feature = "rn")]
    Kirundi,
    /// Kinyarwanda (`rw`).
    #[cfg(feature = "rw")]
    Kinyarwanda,
    /// Akan (`ak`).
    #[cfg(feature = "ak")]
    Akan,
    /// Ewe (`ee`).
    #[cfg(feature = "ee")]
    Ewe,
    /// Hausa (`ha`).
    #[cfg(feature = "ha")]
    Hausa,
    /// Igbo (`ig`).
    #[cfg(feature = "ig")]
    Igbo,
    /// Yoruba (`yo`).
    #[cfg(feature = "yo")]
    Yoruba,
    /// Greek (`el`).
    #[cfg(feature = "el")]
    Greek,
    /// Swahili (`sw`).
    #[cfg(feature = "sw")]
    Swahili,
    /// Macedonian (`mk`).
    #[cfg(feature = "mk")]
    Macedonian,
    /// Serbian (`sr`).
    #[cfg(feature = "sr")]
    Serbian,
    /// Malay (`ms`).
    #[cfg(feature = "ms")]
    Malay,
    /// Russian (`ru`).
    #[cfg(feature = "ru")]
    Russian,
    /// Irish (`ga`).
    #[cfg(feature = "ga")]
    Irish,
    /// Welsh (`cy`).
    #[cfg(feature = "cy")]
    Welsh,
    /// Icelandic (`is`).
    #[cfg(feature = "is")]
    Icelandic,
    /// Danish (`da`).
    #[cfg(feature = "da")]
    Danish,
    /// Norwegian Bokmål (`nb`).
    #[cfg(feature = "nb")]
    Norwegian,
    /// Swedish (`sv`).
    #[cfg(feature = "sv")]
    Swedish,
    /// Albanian (`sq`).
    #[cfg(feature = "sq")]
    Albanian,
    /// Lithuanian (`lt`).
    #[cfg(feature = "lt")]
    Lithuanian,
    /// Afrikaans (`af`).
    #[cfg(feature = "af")]
    Afrikaans,
    /// Latvian (`lv`).
    #[cfg(feature = "lv")]
    Latvian,
    /// Estonian (`et`).
    #[cfg(feature = "et")]
    Estonian,
    /// Hungarian (`hu`).
    #[cfg(feature = "hu")]
    Hungarian,
    /// Dutch (`nl`).
    #[cfg(feature = "nl")]
    Dutch,
    /// German (`de`).
    #[cfg(feature = "de")]
    German,
    /// Catalan (`ca`).
    #[cfg(feature = "ca")]
    Catalan,
    /// Romanian (`ro`).
    #[cfg(feature = "ro")]
    Romanian,
    /// Basque (`eu`).
    #[cfg(feature = "eu")]
    Basque,
    /// Slovenian (`sl`).
    #[cfg(feature = "sl")]
    Slovenian,
    /// Croatian (`hr`).
    #[cfg(feature = "hr")]
    Croatian,
    /// Slovak (`sk`).
    #[cfg(feature = "sk")]
    Slovak,
    /// Czech (`cs`).
    #[cfg(feature = "cs")]
    Czech,
    /// French (`fr`).
    #[cfg(feature = "fr")]
    French,
    /// Polish (`pl`).
    #[cfg(feature = "pl")]
    Polish,
    /// Portuguese, European (`pt`, `pt-PT`) or Brazilian (`pt-BR`) (decision 40).
    #[cfg(feature = "pt")]
    Portuguese(PortugueseVariety),
    /// Indonesian (`id`).
    #[cfg(feature = "id")]
    Indonesian,
    /// Turkish (`tr`).
    #[cfg(feature = "tr")]
    Turkish,
    /// Mandarin Chinese (`zh`), in Simplified or Traditional characters, with its tones as spoken
    /// or as written (docs/decisions.md, decision 31). Latin letters inside are read as American
    /// English when the `en` feature is on.
    #[cfg(feature = "zh")]
    Mandarin(ToneSandhi),
}

/// How a language read one token: the words it is said as, and how eSpeak NG's notation writes
/// them.
pub(crate) struct TokenRead {
    pub words: Vec<Pronunciation>,
    pub joined_to_next: bool,
    pub hyphenated: bool,
    /// How many tokens, from this one on, the words are for (a Chinese word of two characters is
    /// two tokens).
    pub tokens: usize,
}

impl Language {
    /// The BCP 47 tag, e.g. `en-US`.
    #[must_use]
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per language, each behind its Cargo feature"
    )]
    pub const fn tag(self) -> &'static str {
        match self {
            #[cfg(feature = "en")]
            Self::AmericanEnglish => "en-US",
            #[cfg(feature = "en")]
            Self::BritishEnglish(_) => "en-GB",
            #[cfg(feature = "es")]
            Self::Spanish(SpanishVariety::Castilian) => "es",
            #[cfg(feature = "es")]
            Self::Spanish(SpanishVariety::LatinAmerican) => "es-419",
            #[cfg(feature = "fi")]
            Self::Finnish => "fi",
            #[cfg(feature = "it")]
            Self::Italian => "it",
            #[cfg(feature = "om")]
            Self::Oromo => "om",
            #[cfg(feature = "so")]
            Self::Somali => "so",
            #[cfg(feature = "sn")]
            Self::Shona => "sn",
            #[cfg(feature = "ln")]
            Self::Lingala => "ln",
            #[cfg(feature = "lg")]
            Self::Luganda => "lg",
            #[cfg(feature = "rn")]
            Self::Kirundi => "rn",
            #[cfg(feature = "rw")]
            Self::Kinyarwanda => "rw",
            #[cfg(feature = "ak")]
            Self::Akan => "ak",
            #[cfg(feature = "ee")]
            Self::Ewe => "ee",
            #[cfg(feature = "ha")]
            Self::Hausa => "ha",
            #[cfg(feature = "ig")]
            Self::Igbo => "ig",
            #[cfg(feature = "yo")]
            Self::Yoruba => "yo",
            #[cfg(feature = "el")]
            Self::Greek => "el",
            #[cfg(feature = "sw")]
            Self::Swahili => "sw",
            #[cfg(feature = "mk")]
            Self::Macedonian => "mk",
            #[cfg(feature = "sr")]
            Self::Serbian => "sr",
            #[cfg(feature = "ms")]
            Self::Malay => "ms",
            #[cfg(feature = "ru")]
            Self::Russian => "ru",
            #[cfg(feature = "ga")]
            Self::Irish => "ga",
            #[cfg(feature = "cy")]
            Self::Welsh => "cy",
            #[cfg(feature = "is")]
            Self::Icelandic => "is",
            #[cfg(feature = "da")]
            Self::Danish => "da",
            #[cfg(feature = "nb")]
            Self::Norwegian => "nb",
            #[cfg(feature = "sv")]
            Self::Swedish => "sv",
            #[cfg(feature = "sq")]
            Self::Albanian => "sq",
            #[cfg(feature = "lt")]
            Self::Lithuanian => "lt",
            #[cfg(feature = "af")]
            Self::Afrikaans => "af",
            #[cfg(feature = "lv")]
            Self::Latvian => "lv",
            #[cfg(feature = "et")]
            Self::Estonian => "et",
            #[cfg(feature = "hu")]
            Self::Hungarian => "hu",
            #[cfg(feature = "nl")]
            Self::Dutch => "nl",
            #[cfg(feature = "de")]
            Self::German => "de",
            #[cfg(feature = "ca")]
            Self::Catalan => "ca",
            #[cfg(feature = "ro")]
            Self::Romanian => "ro",
            #[cfg(feature = "eu")]
            Self::Basque => "eu",
            #[cfg(feature = "sl")]
            Self::Slovenian => "sl",
            #[cfg(feature = "hr")]
            Self::Croatian => "hr",
            #[cfg(feature = "sk")]
            Self::Slovak => "sk",
            #[cfg(feature = "cs")]
            Self::Czech => "cs",
            #[cfg(feature = "fr")]
            Self::French => "fr",
            #[cfg(feature = "pl")]
            Self::Polish => "pl",
            #[cfg(feature = "pt")]
            Self::Portuguese(PortugueseVariety::European) => "pt",
            #[cfg(feature = "pt")]
            Self::Portuguese(PortugueseVariety::Brazilian) => "pt-BR",
            #[cfg(feature = "id")]
            Self::Indonesian => "id",
            #[cfg(feature = "tr")]
            Self::Turkish => "tr",
            #[cfg(feature = "zh")]
            Self::Mandarin(_) => "zh",
        }
    }

    /// The notations the language is written in.
    #[must_use]
    pub const fn notations(self) -> &'static [Notation] {
        match self {
            #[cfg(feature = "en")]
            Self::AmericanEnglish | Self::BritishEnglish(_) => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "es")]
            Self::Spanish(_) => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "fi")]
            Self::Finnish => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "it")]
            Self::Italian => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "om")]
            Self::Oromo => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "so")]
            Self::Somali => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "sn")]
            Self::Shona => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "ln")]
            Self::Lingala => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "lg")]
            Self::Luganda => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "rn")]
            Self::Kirundi => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "rw")]
            Self::Kinyarwanda => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "ak")]
            Self::Akan => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "ee")]
            Self::Ewe => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "ha")]
            Self::Hausa => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "ig")]
            Self::Igbo => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "yo")]
            Self::Yoruba => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "el")]
            Self::Greek => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "sw")]
            Self::Swahili => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "mk")]
            Self::Macedonian => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "sr")]
            Self::Serbian => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "ms")]
            Self::Malay => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "ru")]
            Self::Russian => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "ga")]
            Self::Irish => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "cy")]
            Self::Welsh => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "is")]
            Self::Icelandic => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "da")]
            Self::Danish => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "nb")]
            Self::Norwegian => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "sv")]
            Self::Swedish => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "sq")]
            Self::Albanian => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "lt")]
            Self::Lithuanian => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "af")]
            Self::Afrikaans => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "lv")]
            Self::Latvian => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "et")]
            Self::Estonian => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "hu")]
            Self::Hungarian => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "nl")]
            Self::Dutch => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "de")]
            Self::German => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "ca")]
            Self::Catalan => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "ro")]
            Self::Romanian => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "eu")]
            Self::Basque => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "sl")]
            Self::Slovenian => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "hr")]
            Self::Croatian => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "sk")]
            Self::Slovak => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "cs")]
            Self::Czech => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "fr")]
            Self::French => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "pl")]
            Self::Polish => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "pt")]
            Self::Portuguese(_) => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "id")]
            Self::Indonesian => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "tr")]
            Self::Turkish => &[Notation::Ipa, Notation::Espeak],
            #[cfg(feature = "zh")]
            Self::Mandarin(_) => &[Notation::Ipa, Notation::Misaki, Notation::Pinyin, Notation::Zhuyin],
        }
    }

    /// Whether `word` (without its full stop) is an abbreviation that does not end a sentence.
    #[cfg_attr(not(feature = "en"), allow(unused_variables, reason = "built without English"))]
    pub(crate) fn is_abbreviation(self, word: &str) -> bool {
        match self {
            #[cfg(feature = "en")]
            Self::AmericanEnglish | Self::BritishEnglish(_) => jespersen_lang_en::is_abbreviation(word),
            #[cfg(feature = "es")]
            Self::Spanish(_) => jespersen_lang_es::is_abbreviation(word),
            #[cfg(feature = "fi")]
            Self::Finnish => jespersen_lang_fi::is_abbreviation(word),
            #[cfg(feature = "it")]
            Self::Italian => jespersen_lang_it::is_abbreviation(word),
            #[cfg(feature = "om")]
            Self::Oromo => jespersen_lang_om::is_abbreviation(word),
            #[cfg(feature = "so")]
            Self::Somali => jespersen_lang_so::is_abbreviation(word),
            #[cfg(feature = "sn")]
            Self::Shona => jespersen_lang_sn::is_abbreviation(word),
            #[cfg(feature = "ln")]
            Self::Lingala => jespersen_lang_ln::is_abbreviation(word),
            #[cfg(feature = "lg")]
            Self::Luganda => jespersen_lang_lg::is_abbreviation(word),
            #[cfg(feature = "rn")]
            Self::Kirundi => jespersen_lang_rn::is_abbreviation(word),
            #[cfg(feature = "rw")]
            Self::Kinyarwanda => jespersen_lang_rw::is_abbreviation(word),
            #[cfg(feature = "ak")]
            Self::Akan => jespersen_lang_ak::is_abbreviation(word),
            #[cfg(feature = "ee")]
            Self::Ewe => jespersen_lang_ee::is_abbreviation(word),
            #[cfg(feature = "ha")]
            Self::Hausa => jespersen_lang_ha::is_abbreviation(word),
            #[cfg(feature = "ig")]
            Self::Igbo => jespersen_lang_ig::is_abbreviation(word),
            #[cfg(feature = "yo")]
            Self::Yoruba => jespersen_lang_yo::is_abbreviation(word),
            #[cfg(feature = "el")]
            Self::Greek => jespersen_lang_el::is_abbreviation(word),
            #[cfg(feature = "sw")]
            Self::Swahili => jespersen_lang_sw::is_abbreviation(word),
            #[cfg(feature = "mk")]
            Self::Macedonian => jespersen_lang_mk::is_abbreviation(word),
            #[cfg(feature = "sr")]
            Self::Serbian => jespersen_lang_sr::is_abbreviation(word),
            #[cfg(feature = "ms")]
            Self::Malay => jespersen_lang_ms::is_abbreviation(word),
            #[cfg(feature = "ru")]
            Self::Russian => jespersen_lang_ru::is_abbreviation(word),
            #[cfg(feature = "ga")]
            Self::Irish => jespersen_lang_ga::is_abbreviation(word),
            #[cfg(feature = "cy")]
            Self::Welsh => jespersen_lang_cy::is_abbreviation(word),
            #[cfg(feature = "is")]
            Self::Icelandic => jespersen_lang_is::is_abbreviation(word),
            #[cfg(feature = "da")]
            Self::Danish => jespersen_lang_da::is_abbreviation(word),
            #[cfg(feature = "nb")]
            Self::Norwegian => jespersen_lang_nb::is_abbreviation(word),
            #[cfg(feature = "sv")]
            Self::Swedish => jespersen_lang_sv::is_abbreviation(word),
            #[cfg(feature = "sq")]
            Self::Albanian => jespersen_lang_sq::is_abbreviation(word),
            #[cfg(feature = "lt")]
            Self::Lithuanian => jespersen_lang_lt::is_abbreviation(word),
            #[cfg(feature = "af")]
            Self::Afrikaans => jespersen_lang_af::is_abbreviation(word),
            #[cfg(feature = "lv")]
            Self::Latvian => jespersen_lang_lv::is_abbreviation(word),
            #[cfg(feature = "et")]
            Self::Estonian => jespersen_lang_et::is_abbreviation(word),
            #[cfg(feature = "hu")]
            Self::Hungarian => jespersen_lang_hu::is_abbreviation(word),
            #[cfg(feature = "nl")]
            Self::Dutch => jespersen_lang_nl::is_abbreviation(word),
            #[cfg(feature = "de")]
            Self::German => jespersen_lang_de::is_abbreviation(word),
            #[cfg(feature = "ca")]
            Self::Catalan => jespersen_lang_ca::is_abbreviation(word),
            #[cfg(feature = "ro")]
            Self::Romanian => jespersen_lang_ro::is_abbreviation(word),
            #[cfg(feature = "eu")]
            Self::Basque => jespersen_lang_eu::is_abbreviation(word),
            #[cfg(feature = "sl")]
            Self::Slovenian => jespersen_lang_sl::is_abbreviation(word),
            #[cfg(feature = "hr")]
            Self::Croatian => jespersen_lang_hr::is_abbreviation(word),
            #[cfg(feature = "sk")]
            Self::Slovak => jespersen_lang_sk::is_abbreviation(word),
            #[cfg(feature = "cs")]
            Self::Czech => jespersen_lang_cs::is_abbreviation(word),
            #[cfg(feature = "fr")]
            Self::French => jespersen_lang_fr::is_abbreviation(word),
            #[cfg(feature = "pl")]
            Self::Polish => jespersen_lang_pl::is_abbreviation(word),
            #[cfg(feature = "pt")]
            Self::Portuguese(_) => jespersen_lang_pt::is_abbreviation(word),
            #[cfg(feature = "id")]
            Self::Indonesian => jespersen_lang_id::is_abbreviation(word),
            #[cfg(feature = "tr")]
            Self::Turkish => jespersen_lang_tr::is_abbreviation(word),
            #[cfg(feature = "zh")]
            Self::Mandarin(_) => false,
        }
    }

    /// How each token (text and kind) of a sentence is read; `None` for one the language cannot
    /// read yet, or that the reading of a token before it covers.
    #[cfg_attr(
        not(any(feature = "en", feature = "zh")),
        allow(unused_variables, reason = "built without any language")
    )]
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per language, each behind its Cargo feature"
    )]
    pub(crate) fn read(self, tokens: &[(&str, TokenKind)]) -> Result<Vec<Option<TokenRead>>, LexiconError> {
        match self {
            #[cfg(feature = "en")]
            Self::AmericanEnglish => read_english(tokens, Variety::American),
            #[cfg(feature = "en")]
            Self::BritishEnglish(bath) => read_english(tokens, Variety::British(bath)),
            #[cfg(feature = "es")]
            Self::Spanish(variety) => Ok(from_spoken(jespersen_lang_es::read_tokens(tokens, variety))),
            #[cfg(feature = "fi")]
            Self::Finnish => Ok(from_spoken(jespersen_lang_fi::read_tokens(tokens))),
            #[cfg(feature = "it")]
            Self::Italian => Ok(from_spoken(jespersen_lang_it::read_tokens(tokens))),
            #[cfg(feature = "om")]
            Self::Oromo => Ok(from_spoken(jespersen_lang_om::read_tokens(tokens))),
            #[cfg(feature = "so")]
            Self::Somali => Ok(from_spoken(jespersen_lang_so::read_tokens(tokens))),
            #[cfg(feature = "sn")]
            Self::Shona => Ok(from_spoken(jespersen_lang_sn::read_tokens(tokens))),
            #[cfg(feature = "ln")]
            Self::Lingala => Ok(from_spoken(jespersen_lang_ln::read_tokens(tokens))),
            #[cfg(feature = "lg")]
            Self::Luganda => Ok(from_spoken(jespersen_lang_lg::read_tokens(tokens))),
            #[cfg(feature = "rn")]
            Self::Kirundi => Ok(from_spoken(jespersen_lang_rn::read_tokens(tokens))),
            #[cfg(feature = "rw")]
            Self::Kinyarwanda => Ok(from_spoken(jespersen_lang_rw::read_tokens(tokens))),
            #[cfg(feature = "ak")]
            Self::Akan => Ok(from_spoken(jespersen_lang_ak::read_tokens(tokens))),
            #[cfg(feature = "ee")]
            Self::Ewe => Ok(from_spoken(jespersen_lang_ee::read_tokens(tokens))),
            #[cfg(feature = "ha")]
            Self::Hausa => Ok(from_spoken(jespersen_lang_ha::read_tokens(tokens))),
            #[cfg(feature = "ig")]
            Self::Igbo => Ok(from_spoken(jespersen_lang_ig::read_tokens(tokens))),
            #[cfg(feature = "yo")]
            Self::Yoruba => Ok(from_spoken(jespersen_lang_yo::read_tokens(tokens))),
            #[cfg(feature = "el")]
            Self::Greek => Ok(from_spoken(jespersen_lang_el::read_tokens(tokens))),
            #[cfg(feature = "sw")]
            Self::Swahili => Ok(from_spoken(jespersen_lang_sw::read_tokens(tokens))),
            #[cfg(feature = "mk")]
            Self::Macedonian => Ok(from_spoken(jespersen_lang_mk::read_tokens(tokens))),
            #[cfg(feature = "sr")]
            Self::Serbian => Ok(from_spoken(jespersen_lang_sr::read_tokens(tokens))),
            #[cfg(feature = "ms")]
            Self::Malay => Ok(from_spoken(jespersen_lang_ms::read_tokens(tokens))),
            #[cfg(feature = "ru")]
            Self::Russian => Ok(from_spoken(jespersen_lang_ru::read_tokens(tokens))),
            #[cfg(feature = "ga")]
            Self::Irish => Ok(from_spoken(jespersen_lang_ga::read_tokens(tokens))),
            #[cfg(feature = "cy")]
            Self::Welsh => Ok(from_spoken(jespersen_lang_cy::read_tokens(tokens))),
            #[cfg(feature = "is")]
            Self::Icelandic => Ok(from_spoken(jespersen_lang_is::read_tokens(tokens))),
            #[cfg(feature = "da")]
            Self::Danish => Ok(from_spoken(jespersen_lang_da::read_tokens(tokens))),
            #[cfg(feature = "nb")]
            Self::Norwegian => Ok(from_spoken(jespersen_lang_nb::read_tokens(tokens))),
            #[cfg(feature = "sv")]
            Self::Swedish => Ok(from_spoken(jespersen_lang_sv::read_tokens(tokens))),
            #[cfg(feature = "sq")]
            Self::Albanian => Ok(from_spoken(jespersen_lang_sq::read_tokens(tokens))),
            #[cfg(feature = "lt")]
            Self::Lithuanian => Ok(from_spoken(jespersen_lang_lt::read_tokens(tokens))),
            #[cfg(feature = "af")]
            Self::Afrikaans => Ok(from_spoken(jespersen_lang_af::read_tokens(tokens))),
            #[cfg(feature = "lv")]
            Self::Latvian => Ok(from_spoken(jespersen_lang_lv::read_tokens(tokens))),
            #[cfg(feature = "et")]
            Self::Estonian => Ok(from_spoken(jespersen_lang_et::read_tokens(tokens))),
            #[cfg(feature = "hu")]
            Self::Hungarian => Ok(from_spoken(jespersen_lang_hu::read_tokens(tokens))),
            #[cfg(feature = "nl")]
            Self::Dutch => Ok(from_spoken(jespersen_lang_nl::read_tokens(tokens))),
            #[cfg(feature = "de")]
            Self::German => Ok(from_spoken(jespersen_lang_de::read_tokens(tokens))),
            #[cfg(feature = "ca")]
            Self::Catalan => Ok(from_spoken(jespersen_lang_ca::read_tokens(tokens))),
            #[cfg(feature = "ro")]
            Self::Romanian => Ok(from_spoken(jespersen_lang_ro::read_tokens(tokens))),
            #[cfg(feature = "eu")]
            Self::Basque => Ok(from_spoken(jespersen_lang_eu::read_tokens(tokens))),
            #[cfg(feature = "sl")]
            Self::Slovenian => Ok(from_spoken(jespersen_lang_sl::read_tokens(tokens))),
            #[cfg(feature = "hr")]
            Self::Croatian => Ok(from_spoken(jespersen_lang_hr::read_tokens(tokens))),
            #[cfg(feature = "sk")]
            Self::Slovak => Ok(from_spoken(jespersen_lang_sk::read_tokens(tokens))),
            #[cfg(feature = "cs")]
            Self::Czech => Ok(from_spoken(jespersen_lang_cs::read_tokens(tokens))),
            #[cfg(feature = "fr")]
            Self::French => Ok(from_spoken(jespersen_lang_fr::read_tokens(tokens))),
            #[cfg(feature = "pl")]
            Self::Polish => Ok(from_spoken(jespersen_lang_pl::read_tokens(tokens))),
            #[cfg(feature = "pt")]
            Self::Portuguese(variety) => Ok(from_spoken(jespersen_lang_pt::read_tokens(tokens, variety))),
            #[cfg(feature = "id")]
            Self::Indonesian => Ok(from_spoken(jespersen_lang_id::read_tokens(tokens))),
            #[cfg(feature = "tr")]
            Self::Turkish => Ok(from_spoken(jespersen_lang_tr::read_tokens(tokens))),
            #[cfg(feature = "zh")]
            Self::Mandarin(sandhi) => read_mandarin(tokens, sandhi),
        }
    }
}

#[cfg(feature = "en")]
fn read_english(tokens: &[(&str, TokenKind)], variety: Variety) -> Result<Vec<Option<TokenRead>>, LexiconError> {
    Ok(jespersen_lang_en::read_tokens(tokens, variety)?
        .into_iter()
        .map(|reading| {
            reading.map(|reading| TokenRead {
                words: reading.words,
                joined_to_next: reading.joined_to_next,
                hyphenated: reading.hyphenated,
                tokens: 1,
            })
        })
        .collect())
}

/// Mandarin, with runs of Latin words in it read as American English (decision 25).
#[cfg(feature = "zh")]
fn read_mandarin(tokens: &[(&str, TokenKind)], sandhi: ToneSandhi) -> Result<Vec<Option<TokenRead>>, LexiconError> {
    #[cfg_attr(not(feature = "en"), allow(unused_mut, reason = "English fills in the Latin words"))]
    let mut readings: Vec<Option<TokenRead>> = jespersen_lang_zh::read_tokens(tokens, sandhi)?
        .into_iter()
        .map(|reading| {
            reading.map(|reading| TokenRead {
                words: reading.words,
                joined_to_next: false,
                hyphenated: false,
                tokens: reading.tokens,
            })
        })
        .collect();
    #[cfg(feature = "en")]
    {
        let latin: Vec<bool> = (0..tokens.len())
            .map(|at| {
                readings[at].is_none()
                    && tokens[at].1 == TokenKind::Word
                    && Script::of_text(tokens[at].0) == Script::Latin
            })
            .collect();
        let mut start = 0;
        while start < tokens.len() {
            if !latin[start] {
                start += 1;
                continue;
            }
            let end = (start..tokens.len()).find(|&at| !latin[at]).unwrap_or(tokens.len());
            let english = read_english(&tokens[start..end], Variety::American)?;
            for (at, reading) in (start..end).zip(english) {
                readings[at] = reading;
            }
            start = end;
        }
    }
    Ok(readings)
}

impl FromStr for Language {
    type Err = Error;

    /// Reads a BCP 47 tag, in any case and with `-` or `_` (`en-US`, `en_gb`, `zh-Hant`). British
    /// English gets the BATH vowel by notation, Mandarin its tones as spoken.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per language, each behind its Cargo feature"
    )]
    fn from_str(tag: &str) -> Result<Self, Error> {
        match tag.to_ascii_lowercase().replace('_', "-").as_str() {
            #[cfg(feature = "en")]
            "en-us" => Ok(Self::AmericanEnglish),
            #[cfg(feature = "en")]
            "en-gb" => Ok(Self::BritishEnglish(BathVowel::default())),
            #[cfg(feature = "fi")]
            "fi" | "fi-fi" => Ok(Self::Finnish),
            #[cfg(feature = "tr")]
            "tr" | "tr-tr" => Ok(Self::Turkish),
            #[cfg(feature = "id")]
            "id" | "id-id" | "in" => Ok(Self::Indonesian),
            #[cfg(feature = "pl")]
            "pl" | "pl-pl" => Ok(Self::Polish),
            #[cfg(feature = "pt")]
            "pt" | "pt-pt" | "pt-ao" | "pt-mz" => Ok(Self::Portuguese(PortugueseVariety::European)),
            #[cfg(feature = "pt")]
            "pt-br" => Ok(Self::Portuguese(PortugueseVariety::Brazilian)),
            #[cfg(feature = "fr")]
            "fr" | "fr-fr" | "fr-be" | "fr-ca" | "fr-ch" => Ok(Self::French),
            #[cfg(feature = "cs")]
            "cs" | "cs-cz" => Ok(Self::Czech),
            #[cfg(feature = "sk")]
            "sk" | "sk-sk" => Ok(Self::Slovak),
            #[cfg(feature = "hr")]
            "hr" | "hr-hr" => Ok(Self::Croatian),
            #[cfg(feature = "sl")]
            "sl" | "sl-si" => Ok(Self::Slovenian),
            #[cfg(feature = "eu")]
            "eu" | "eu-es" => Ok(Self::Basque),
            #[cfg(feature = "ro")]
            "ro" | "ro-ro" | "ro-md" => Ok(Self::Romanian),
            #[cfg(feature = "ca")]
            "ca" | "ca-es" | "ca-ad" | "ca-fr" | "ca-it" => Ok(Self::Catalan),
            #[cfg(feature = "de")]
            "de" | "de-de" | "de-at" | "de-ch" | "de-li" | "de-lu" | "de-be" => Ok(Self::German),
            #[cfg(feature = "nl")]
            "nl" | "nl-nl" | "nl-be" | "nl-sr" => Ok(Self::Dutch),
            #[cfg(feature = "hu")]
            "hu" | "hu-hu" => Ok(Self::Hungarian),
            #[cfg(feature = "af")]
            "af" | "af-za" | "af-na" => Ok(Self::Afrikaans),
            #[cfg(feature = "lv")]
            "lv" | "lv-lv" => Ok(Self::Latvian),
            #[cfg(feature = "et")]
            "et" | "et-ee" => Ok(Self::Estonian),
            #[cfg(feature = "sv")]
            "sv" | "sv-se" | "sv-fi" => Ok(Self::Swedish),
            #[cfg(feature = "nb")]
            "nb" | "nb-no" | "no" | "no-no" => Ok(Self::Norwegian),
            #[cfg(feature = "sq")]
            "sq" | "sq-al" | "sq-xk" => Ok(Self::Albanian),
            #[cfg(feature = "lt")]
            "lt" | "lt-lt" => Ok(Self::Lithuanian),
            #[cfg(feature = "cy")]
            "cy" | "cy-gb" => Ok(Self::Welsh),
            #[cfg(feature = "is")]
            "is" | "is-is" => Ok(Self::Icelandic),
            #[cfg(feature = "da")]
            "da" | "da-dk" => Ok(Self::Danish),
            #[cfg(feature = "ga")]
            "ga" | "ga-ie" => Ok(Self::Irish),
            #[cfg(feature = "sw")]
            "sw" | "sw-tz" | "sw-ke" => Ok(Self::Swahili),
            #[cfg(feature = "el")]
            "el" | "el-gr" | "el-cy" => Ok(Self::Greek),
            #[cfg(feature = "ms")]
            "ms" | "ms-my" | "ms-bn" | "ms-sg" => Ok(Self::Malay),
            #[cfg(feature = "sr")]
            "sr" | "sr-cyrl" | "sr-latn" | "sr-rs" | "sr-cyrl-rs" | "sr-latn-rs" => Ok(Self::Serbian),
            #[cfg(feature = "mk")]
            "mk" | "mk-mk" => Ok(Self::Macedonian),
            #[cfg(feature = "so")]
            "so" | "so-so" => Ok(Self::Somali),
            #[cfg(feature = "om")]
            "om" | "om-et" => Ok(Self::Oromo),
            #[cfg(feature = "ru")]
            "ru" | "ru-ru" => Ok(Self::Russian),
            #[cfg(feature = "rw")]
            "rw" | "rw-rw" => Ok(Self::Kinyarwanda),
            #[cfg(feature = "rn")]
            "rn" | "rn-bi" => Ok(Self::Kirundi),
            #[cfg(feature = "lg")]
            "lg" | "lg-ug" => Ok(Self::Luganda),
            #[cfg(feature = "ln")]
            "ln" | "ln-cd" | "ln-cg" => Ok(Self::Lingala),
            #[cfg(feature = "sn")]
            "sn" | "sn-zw" => Ok(Self::Shona),
            #[cfg(feature = "yo")]
            "yo" | "yo-ng" | "yo-bj" => Ok(Self::Yoruba),
            #[cfg(feature = "ig")]
            "ig" | "ig-ng" => Ok(Self::Igbo),
            #[cfg(feature = "ha")]
            "ha" | "ha-ng" | "ha-ne" | "ha-latn" => Ok(Self::Hausa),
            #[cfg(feature = "ee")]
            "ee" | "ee-gh" | "ee-tg" => Ok(Self::Ewe),
            #[cfg(feature = "ak")]
            "ak" | "ak-gh" | "tw" | "twi" => Ok(Self::Akan),
            #[cfg(feature = "it")]
            "it" | "it-it" | "it-ch" | "it-sm" => Ok(Self::Italian),
            #[cfg(feature = "es")]
            "es" | "es-es" => Ok(Self::Spanish(SpanishVariety::Castilian)),
            #[cfg(feature = "es")]
            tag if tag == "es-419" || (tag.starts_with("es-") && tag.len() == 5) => {
                Ok(Self::Spanish(SpanishVariety::LatinAmerican))
            }
            #[cfg(feature = "zh")]
            "zh" | "cmn" | "zh-cn" | "zh-sg" | "zh-tw" | "zh-hans" | "zh-hant" | "cmn-hans" | "cmn-hant" => {
                Ok(Self::Mandarin(ToneSandhi::default()))
            }
            _ => Err(Error::UnsupportedLanguage(tag.to_owned())),
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.tag())
    }
}

/// The readings of a language read with the shared front end (`jespersen_spoken`).
#[cfg(any(
    feature = "es",
    feature = "fi",
    feature = "it",
    feature = "tr",
    feature = "id",
    feature = "pl",
    feature = "pt",
    feature = "fr",
    feature = "cs",
    feature = "sk",
    feature = "hr",
    feature = "sl",
    feature = "eu",
    feature = "ro",
    feature = "ca",
    feature = "de",
    feature = "nl",
    feature = "hu",
    feature = "af",
    feature = "lv",
    feature = "et",
    feature = "sv",
    feature = "nb",
    feature = "sq",
    feature = "lt",
    feature = "cy",
    feature = "is",
    feature = "da",
    feature = "ga",
    feature = "sw",
    feature = "el",
    feature = "ms",
    feature = "sr",
    feature = "mk",
    feature = "so",
    feature = "om",
    feature = "ru",
    feature = "yo",
    feature = "ig",
    feature = "ha",
    feature = "ee",
    feature = "ak"
))]
fn from_spoken(readings: Vec<Option<jespersen_spoken::TokenReading>>) -> Vec<Option<TokenRead>> {
    readings
        .into_iter()
        .map(|reading| {
            reading.map(|reading| TokenRead {
                words: reading.words,
                joined_to_next: false,
                hyphenated: reading.hyphenated,
                tokens: reading.tokens,
            })
        })
        .collect()
}

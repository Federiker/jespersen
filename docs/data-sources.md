# Data sources

Every source of language data in jespersen, with its licence checked from the
licence text itself (owner decision, 23 October 2026: permissive with
attribution, nothing share-alike). A source moves to **In use**, with its
notice, in the same commit that first uses it (`[data]`).

A source's own licence is not enough: what it was built from counts too. A
permissively licensed list derived from Wiktionary, a commercial dictionary or
eSpeak NG is rejected.

## In use

| Source | Version | Used for | Licence | Checked | Notice |
| --- | --- | --- | --- | --- | --- |
| CMUdict | commit `7479086` (24 October 2025) | American English lexicon (decision 19), and the letter-to-sound rules learned from it (`data/derived/en-us/letters.tsv`, decision 29) | BSD-style, 2 clauses | 23 October 2026 | [LICENSE](../data/upstream/cmudict/LICENSE) must ship with binaries' documentation |
| Moby Pronunciator II | Project Gutenberg #3205 | American homographs by part of speech, and words CMUdict lacks (decision 19) | Public domain | 23 October 2026 | None required; credited anyway |
| ipa-dict, Mandarin (`zh_hans`, `zh_hant`) | commit `972cd67` (22 October 2026) | Mandarin words and their readings (decision 32), converted from IPA to pinyin; sayings with commas and 18 entries missing a syllable left out | MIT for the repository; its Mandarin data credits 開放漢語詞典 (CC BY 3.0) for words of several syllables, KFCD Pinyin (CC BY 3.0) for the IPA and Unihan (Unicode License) | 23 October 2026 | [LICENSE](../data/upstream/ipa-dict/LICENSE), and the credit below for 開放漢語詞典 and 開放詞典 |
| KFCD Pinyin (開放詞典 漢語拼音轉換表) | commit `6a439be` (24 March 2021) | Turning ipa-dict's IPA back into pinyin: the table it was made with | CC BY 3.0 | 23 October 2026 | Credit 開放詞典 (kaifangcidian.com), link the licence, say what changed: [LICENSE](../data/upstream/kfcd-pinyin/LICENSE) |
| Unihan | Unicode 18.0.0 | Each Chinese character's reading on its own (`data/derived/zh/characters.tsv`): the most frequent in `kHanyuPinlu`, otherwise the first in `kMandarin` | Unicode License v3 | 23 October 2026 | [LICENSE](../data/upstream/unihan/LICENSE) must ship with copies or their documentation |
| 開放漢語字典 (kfcd/hyzd) | commit `0498775` (31 October 2020) | Example words of each reading of a character, for words ipa-dict lacks (decision 32) | CC BY 3.0 | 23 October 2026 | The credit below; [LICENSE](../data/upstream/kfcd-hyzd/LICENSE) |
| 開放詞典 繁簡體字轉換表 (kfcd/fanjian) | commit `62a67c2` (8 September 2025) | The Simplified forms of those example words, and the Traditional forms of jespersen's everyday words | CC BY 3.0 | 23 October 2026 | The credit below; [LICENSE](../data/upstream/kfcd-fanjian/LICENSE) |
| NST's Danish pronunciation lexicon (Nordisk Språkteknologi; [Språkbanken, sbr-26](https://www.nb.no/sprakbanken/ressurskatalog/oai-nb-no-sbr-26/)) | `dan030224NST.pron`, archive downloaded 24 October 2026 | Danish words before the rules (decision 41), turned into `data/derived/da/nst.tsv` (decision 43): 237 873 entries transcribed by hand at Center for Sprogteknologi, in SAMPA with stød, stress, length and syllables; garbage entries and names of several words left out. The archive has no licence file; some of its word lists (not transcriptions) were bought by NST (INSO base forms, Krak street names). | CC0 1.0: the catalogue's licence field reads "CC Zero (fri bruk)" and the CMDI record `Creative_Commons-ZERO (CC-ZERO)`; rights holder Nasjonalbiblioteket | 24 October 2026 | None required; credited anyway: "Danish pronunciations from NST's lexicon, Språkbanken at the National Library of Norway (CC0)" |
| NST's Swedish pronunciation lexicon (Nordisk Språkteknologi; [Språkbanken, sbr-22](https://www.nb.no/sprakbanken/ressurskatalog/oai-nb-no-sbr-22/)) | `swe030224NST.pron`, archive downloaded 24 October 2026 | Swedish nouns, adjectives, verbs, numbers and names before the rules (decision 42), turned into `data/derived/sv/nst.tsv` (decision 43): NST's own entries only, 35 280 of 927 167. Kept: entries transcribed by hand in NST's own projects (source field `LEX`) whose sets name no bought lexicon. Left out: the 677 266 forms NST's inflector made from the Telia lexicon's base forms (and every entry with an `INFL` source), and every entry in the Enter list (`enter_se`, bought with its transcriptions) or the Telia lexicon (`teliabaseforms_se`, `telia_personnames_se`, `telia_placenames_se`), whose transcriptions NST converted rather than wrote, as the lexicon's documentation says (section 3.3). Function words are left to the rules. | CC0 1.0, as the Danish lexicon | 24 October 2026 | None required; credited anyway: "Swedish pronunciations from NST's lexicon, Språkbanken at the National Library of Norway (CC0)" |

Their exact files and hashes are in
[data/upstream/sources.toml](../data/upstream/sources.toml).

### Credit for the Mandarin data

CC BY 3.0 asks for credit, a link to the licence and a note of changes.
Program documentation that includes jespersen with Mandarin (the `zh` feature)
should carry this text, with the Unicode and ipa-dict notices above:

> Mandarin readings from 開放漢語詞典 (by way of ipa-dict, MIT), 開放漢語字典,
> 漢語拼音轉換表 and 繁簡體字轉換表 by 開放詞典 (http://kaifangcidian.com),
> CC BY 3.0 (https://creativecommons.org/licenses/by/3.0/). Changed by
> jespersen: IPA turned back into pinyin, sayings with punctuation and entries
> whose syllables do not match their characters left out, example words read
> with their characters' readings and turned into Simplified characters, and
> some readings replaced by jespersen's own.

## Checked, usable (details)

| Source | What it is | Licence, as read | Built from | Checked |
| --- | --- | --- | --- | --- |
| [CMUdict](https://github.com/cmusphinx/cmudict) | American English, about 135 000 words, ARPAbet with stress digits | BSD-style, 2 clauses: keep the copyright notice and disclaimer in source and binary distributions. The README adds: "Use of this dictionary for any research or commercial purpose is completely unrestricted." | CMU's own hand-built dictionary and proofed sources; "entries that occur solely in copyrighted sources … are not currently included" | 23 October 2026 |
| [Moby Pronunciator II](https://en.wikipedia.org/wiki/Moby_Project) (Project Gutenberg #3205) | American English, 177 267 words and phrases, with several hundred homographs told apart by part of speech (`close/v`, `close/aj`) | "Public Domain material by grant from the author, January, 2001." | Grady Ward's own work, plus a copy of cmudict 0.1 | 23 October 2026 |
| [Unihan](https://www.unicode.org/charts/unihan.html) | Readings of every CJK character (`kMandarin`, `kHanyuPinyin`, `kTGHZ2013`, `kXHC1983`, Cantonese `kCantonese`…) | Unicode License v3: use freely, with the copyright and permission notice in the copies or their documentation | The Unicode Consortium's sources, named per field | 23 October 2026 |
| [OpenCC](https://github.com/BYVoid/OpenCC) | Traditional and Simplified conversion tables, characters and phrases | Apache-2.0 | Its contributors' own tables | 23 October 2026 |
| [NST Norwegian Bokmål lexicon](https://www.nb.no/sprakbanken/ressurskatalog/oai-nb-no-sbr-23/) (sbr-23) | 784 240 Bokmål entries in SAMPA with both tonal accents, secondary stress, length and compound splits; most inflected forms made by NST's inflector | As the Danish one: "CC Zero (fri bruk)" | Partly NST's own transcriptions, partly bought lexicons (NorKompLeks' base forms, with the inflector built on them; Onomastica's names), marked per entry by set. Usable for NST's own entries only (decision 42); tried and not used: it lowered agreement with eSpeak NG (decision 49) | 24 October 2026 |
| [Icelandic Pronunciation Dictionary for Language Technology](https://repository.clarin.is/repository/xmlui/handle/20.500.12537/331) 22.01 (iceprondict) | 65 281 Icelandic words in X-SAMPA, four regional variants, part of speech and a compound flag; no stress marked | "Creative Commons - Attribution 4.0 International (CC BY 4.0)", the legal code in the archive; credit Nikulásdóttir, Ármannsson, Bergþórsdóttir and Rögnvaldsson | Hand-revised transcriptions from the Hjal dictionary (Árni Magnússon Institute, CC BY 3.0) | 24 October 2026 |
| [LKI place-name dictionary](https://data.gov.lt/datasets/2937/) (Vietovardžių žodynas) | Lithuanian place names with their stressed forms and accent class | "Priskyrimas 4.0 Tarptautinė (CC BY 4.0)" on data.gov.lt | The Institute of the Lithuanian Language's own dictionary (2002) | 24 October 2026 |
| Wikisource transcriptions of public-domain works | Texts of public-domain books | The Wikimedia Terms of Use license under CC BY-SA only "text to which you hold the copyright"; the multilingual and Croatian Wikisources: public-domain material may be copied "without additional restrictions". A faithful transcription adds no copyright (Infopaq; Feist). | Usable text by text: an author dead for 70 years or more, a transcription of an old, scanned edition (not a modern critical one), Wikisource's notes left out. Candidates found: Agirre's Garoa (Basque, 1912), Nazor's Priče iz djetinjstva (Croatian, 1924) | 24 October 2026 |

## Rejected

| Source | Why |
| --- | --- |
| Britfone | MIT, but its British forms were taken from Wiktionary (CC BY-SA) and the Collins, Oxford, Cambridge and Macmillan dictionaries, as its README says. |
| ipa-dict `en_UK` | Derived from ipacards, GPL-3.0. (Its `en_US` is CMUdict converted to IPA; use CMUdict itself.) |
| BEEP | "Research and development use only"; inherits Oxford University Press and MRC restrictions. |
| Unisyn | Research licence only; no commercial use. |
| Combilex RP | Commercial licence for a fee. |
| CC-CEDICT | CC BY-SA 4.0 (share-alike). |
| mozillazg/phrase-pinyin-data | MIT, but merged from CC-CEDICT and zdic.net. |
| mozillazg/pinyin-data `pinyin.txt` | Mostly Unihan, merged with hand corrections and other files; use Unihan directly. |
| Wiktionary and anything built from it | CC BY-SA. |
| jieba's `dict.txt` (word list and frequencies) | The code is MIT, but the dictionary's origin is not documented; for now jespersen segments with its own lexicon. |
| THUOCL (Tsinghua) | MIT, but partly gathered from input-method dictionaries of unknown terms, and topical (place names, idioms, food…) rather than everyday words. |
| PhonItalia (Italian phonological lexicon) | Free for non-commercial research only. |
| Russian stress: StressRNN, odict.ru, russtress, Schriner's replication data (labelled CC0), GiellaLT lang-rus, ru-accent-poet, tsnorm, OpenRussian, omogre and udarenie | All built from Zaliznyak's Grammatical Dictionary (protected until 2087), Wiktionary, GPL data or undocumented sources, or themselves share-alike or non-commercial. |
| Russian stress models: ruaccent, Koziev's accentuator, Gusev's russ, Silero Stress | ruaccent is trained on the Russian National Corpus and other models' output; Koziev's model is CC BY-NC-SA; Gusev's trained on an eSpeak NG dump; Silero (MIT) does not say what it was trained on. |
| Ushakov's and Dal's Russian dictionaries | Ushakov's is protected in Russia until about 2039; Dal's is public domain but gives headwords only, with 19th-century stress. |
| German lexicons: BAS PHONOLEX, MaryTTS, Zamia-speech, ipa-dict `de`, gruut, prosodylab and MFA's `german_mfa`, kaldi-tuda-de, DWDS, LC-STAR | Closed or sold (BAS, DWDS, LC-STAR), LGPL (MaryTTS, Zamia), built from Wiktionary, eSpeak NG or CELEX (ipa-dict, gruut, prosodylab), or of undocumented origin (MFA, tuda-de's extra lexicon). |
| OpenSLR 8 (Danish) | Generated with eSpeak NG. |
| LIEPA synthesiser stress database (Lithuanian) | CC BY and BSD, but its stems were taken from the Dictionary of Modern Lithuanian (1993) and the Dictionary of International Words (1985) (Kasparaitis 2000). |
| svogunas/g2p-lt-lexicon (from LIEPA-3) | CC BY 4.0, but how its stress and tone were marked is not documented; to be asked (TODO). |
| Lithuanian accentuators: litlat-bert, kirciuokle, kirtis.info | CC BY-SA and from Wiktionary; filled from VDU's closed Kirčiuoklis; GPL. |
| ABAIR, teanglann.ie, Ó Curnáin's The Irish of Iorras Aithneach | Non-commercial or all rights reserved. Their facts may inform jespersen's own list; their data is not copied. |

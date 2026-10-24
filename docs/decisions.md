# Decisions

Owner decisions, newest last. Each says what was decided and why, so that it
is not argued again without a reason.

## Foundation (23 October 2026)

1. **Purpose.** jespersen first replaces what eSpeak NG does for manuzio: turn
   text into phonemes for Kokoro, in manuzio's narration languages, so that
   manuzio (closed source, sold) ships no GPL code. jespersen does phonetics
   only (decision 50).
2. **Phonemes only.** jespersen turns text into phonemes and does not make
   sound: Kokoro speaks. No formant or neural voice is planned.
3. **Licence.** CC0 1.0 for the code and jespersen's own data, as a starting
   point: the licence may change (for example to MIT, Apache-2.0 or a
   CC-BY data licence) whenever a source needs it, as long as jespersen stays
   usable in closed-source commercial programs. That is the hard line, so
   share-alike and copyleft stay out (owner, 23 October 2026).
4. **Strict clean room**, with eSpeak NG as a black-box oracle in development
   only; see [clean-room.md](clean-room.md).
5. **Oracle captures stay out of the repository**, in the gitignored `tmp/`
   folder; anything that reads them skips itself when they are absent.
6. **Two notations from one model.** One internal phoneme representation,
   written either in eSpeak NG's IPA conventions (so voices trained on them
   work unchanged) or as plain IPA.
7. **Languages.** English (British and American) first. Then languages with
   regular spelling (Italian, Spanish, Finnish, Polish…), harder ones last.
8. **Pronunciation.** A lexicon first, then hand-written letter-to-sound and
   stress rules per language. A small trained model for unknown words may
   come later, as an option.
9. **Full text front end.** jespersen reads raw text: sentences and clauses,
   numbers, ordinals, dates, times, currencies, abbreviations and acronyms.
10. **Data licences.** Permissive with attribution (CC0, public domain, MIT,
    BSD, Apache, CC-BY…); nothing share-alike. Each source is listed in
    [data-sources.md](data-sources.md).
11. **Data compiled in.** Each language's data is built into a compact form at
    build time and embedded; a Cargo feature per language selects them.
12. **Workspace.** `crates/*`, named `jespersen_<area>`, with a thin `jespersen`
    facade. manuzio depends on the crate and calls it in-process.
13. **Command.** `jespersen`, with subcommands and eSpeak NG-like flag names
    where they make sense; not a drop-in replacement for `espeak-ng`.
14. **Tooling.** jerome's set without Bun or Node: mise, lefthook, rustfmt,
    Clippy, nextest, cargo-llvm-cov (80%), cargo-deny, cargo-machete, typos and
    lychee; GitHub Actions and Renovate.
15. **Not on crates.io yet** (`publish = false`) while the name may change.
16. **Order.** English phonemes in manuzio, then the other languages manuzio
    needs.
17. **Mandarin is a jespersen language, as China-friendly as possible.** jespersen
    reads Chinese itself rather than leaving it to manuzio. Where Mandarin
    comes in the order and what "China-friendly" covers beyond it is decided
    when its work is shaped.
18. **The owner creates the GitHub repository and pushes**, by hand; agents
    commit locally and never push.

## English sources (23 October 2026)

19. **American English** comes from CMUdict, with Moby Pronunciator II's
    part-of-speech entries for homographs and words CMUdict lacks, after
    review (see [data-sources.md](data-sources.md)).
20. **British English** comes from jespersen's own documented rules that turn the
    American forms into British ones, plus a hand-written CC0 list of words
    that differ unpredictably. No British lexicon is usable: Britfone's
    British forms came from Wiktionary and commercial dictionaries, and a
    change of jespersen's licence would not cure that.
21. **Upstream data is committed**, pinned to an exact version with its
    SHA-256, under `data/upstream/<source>/` with its licence and notice, so
    builds work offline and every upstream change shows in a diff.
22. **Mandarin comes right after English**, before the European languages.

## Library shape (23 October 2026)

23. **Structured output with string helpers.** The library returns sentences
    of tokens, each with its span in the source text, its kind (word,
    number, punctuation…), its language and its phonemes; writers turn them
    into eSpeak NG-style or plain IPA strings.
24. **Punctuation is kept**, as tokens; a writer may drop it (as eSpeak NG
    does) or keep it (as Kokoro wants).
25. **Mixed text:** the caller names the main language; a run in another
    script (Latin inside Chinese, Chinese inside English) switches to a
    configured secondary language automatically. A change of language within
    the same script happens only when the caller marks it.
26. **English locales differ in reading, not only in sound:** British English
    reads 3/5/1999 as the 3rd of May and says "one hundred and five";
    American English reads March 5th and "one hundred five". Both recognise
    either spelling (colour, color).

## Imitation (23 October 2026)

27. **eSpeak NG's notation may imitate its systematic behaviour** (unstressed
    function words and their forms, words joined in writing, allophones), as
    rules and short lists jespersen writes and documents itself. Answers are
    never transferred in bulk into a lexicon, and plain IPA follows phonetics
    (owner decision, 23 October 2026; see [clean-room.md](clean-room.md)).
28. **The BATH vowel is an option in British English:** broad `ɑː` (RP:
    bath, dance, path) or the TRAP vowel `a`, which eSpeak NG's British
    voice writes and Kokoro's British voices learnt. By default plain IPA
    is broad and eSpeak NG's notation follows eSpeak NG; callers can choose
    either (owner decision, 23 October 2026).
29. **Unknown words are read with rules learned from the lexicon**, not
    written by hand (this refines decision 8): a trainer aligns the
    lexicon's letters with its phonemes and derives a deterministic table of
    letters in context and the phonemes they say, committed as derived data
    with the lexicon's licence and notice. No model runs at reading time
    (owner decision, 23 October 2026).

## English's finish line (23 October 2026)

30. **English is done for book narration** when these are in: homographs told
    apart by context, the front end's remaining gaps (negative numbers, units,
    URLs and e-mail addresses, more abbreviations), compound and secondary
    stress and better stress for unknown words, the glottal t and syllabic n
    of eSpeak NG's American notation, agreement measured on a long
    public-domain text in both varieties, and a Cargo feature per language.
    After that English is maintenance (more exceptions, individual words) and
    Mandarin starts (owner decision, 23 October 2026).

## Mandarin (23 October 2026)

Taken when Mandarin started, on the owner's "go ahead with Mandarin", and
confirmed by the owner the same day.

31. **Mandarin's shape.** Simplified and Traditional characters are read as
    they are, without converting one into the other, with the mainland
    standard's readings (Taiwan's own readings are not offered yet). Two
    notations: plain IPA with tone letters (`ʈʂʊŋ˥˥`), and misaki's, which
    Kokoro's Chinese voices were trained on (`ꭧʊ→ŋ`); eSpeak NG's Mandarin
    notation is not offered, since Kokoro never learnt it. Tones are spoken
    (third-tone sandhi, 一 and 不, neutral tones at their pitch; the default)
    or written as dictionaries write them (only 一 and 不 change), which is
    closer to what Kokoro learnt. Numbers are read in Chinese; Latin letters
    inside Chinese are read as American English (decision 25). Pinyin and
    zhuyin output, erhua and Cantonese come later.
32. **Mandarin sources.** Words come from ipa-dict's Mandarin lists, made
    from 開放漢語詞典 (CC BY 3.0) and turned back from IPA into pinyin with
    開放詞典's own conversion table (CC BY 3.0); each character's reading on
    its own from Unihan (Unicode License), the most frequent in its
    frequency field; jespersen's own entries and context rules (CC0) correct
    them. Text is split into the lexicon's words, fewest words first, with
    no segmenter's dictionary: jieba's does not say where it comes from.
    CC BY 3.0 asks for credit and a note of changes, which
    [data-sources.md](data-sources.md) and the data's README give.
33. **misaki is a reference, not a source.** What misaki says for manuzio's
    Mandarin sentences is compared locally (in `tmp/`), as eSpeak NG's
    output is for English. misaki is Apache-2.0, so no clean room is needed,
    but no table or code is taken from it: jespersen's syllables are written
    from Mandarin phonetics, and where misaki is wrong (很长 read zhǎng, 一共
    with 一 unchanged) jespersen follows the language.
34. **Mandarin's finish line.** Mandarin is done for book narration when
    these are in: everyday words beyond ipa-dict's (so text splits into
    words as a reader would, with their neutral tones), polyphonic
    characters read by context with erhua and verbs said twice, agreement
    measured on a long public-domain text, and pinyin and zhuyin as
    notations. Taiwan's readings come later, if ever (owner decision,
    23 October 2026).

## Spanish and Italian (23 October 2026)

35. **Spanish comes before Italian:** its spelling gives stress and sounds
    almost completely, so rules and a short exception list carry it.
36. **Spanish has two varieties as one language with an option**, as English
    has: Castilian (θ in cielo, ʎ in calle) and Latin American (seseo: s;
    yeísmo: ʝ).
37. **Italian is read with rules and jespersen's own lists**: stress on the
    last syllable but one unless written or listed, and own CC0 lists of
    common words with other stress or open ɛ and ɔ, while a clean lexicon is
    looked for.
38. **Both are written in eSpeak NG's notation, which Kokoro's Spanish and
    Italian voices learnt, and in plain IPA**, as English is, and compared
    with eSpeak NG as a black box (owner decisions, 23 October 2026).

## manuzio's languages and Portuguese (24 October 2026)

39. **manuzio's languages come first**, and among them those Kokoro has
    voices for (French and Portuguese) before the ones a foreign voice
    reads; the others follow by how regular their spelling is.
40. **Portuguese has two varieties as one language with an option**, as
    Spanish has: European (the default, as eSpeak NG's `pt` voice that
    manuzio uses today) and Brazilian, which Kokoro's Portuguese voices
    speak (owner decisions, 24 October 2026).

## The sources set aside (24 October 2026)

The sources set aside while the languages went in were checked together;
the findings are in [data-sources.md](data-sources.md).

41. **Danish is read from the NST lexicon** (CC0, every entry transcribed by
    hand), whole, before its rules.
42. **Swedish and Norwegian Bokmål are read from the NST lexicons, NST's own
    entries only:** those marked as coming from lexicons NST bought (Telia,
    NorKompLeks, Onomastica) are left out, although the National Library
    released the whole under CC0.
43. **A large upstream lexicon is committed as a trimmed text file**
    (refining decision 21): the archive is pinned by URL and SHA-256 in
    `data/upstream/sources.toml` but not committed, and a Rust mise task
    turns it into `data/derived/<language>/`, keeping only the fields
    jespersen reads. The NST files (35 to 185 MB) are too large to commit as
    published.
44. **Russian stress comes from jespersen's own rules and word list**, written
    from the standard language: no dictionary or model with inflected forms
    is clean (they come from Zaliznyak's dictionary, Wiktionary, the Russian
    National Corpus or eSpeak NG).
45. **Irish follows Munster**, the southern reading the rules already
    imitate; jespersen's own list is checked against published Munster
    descriptions.
46. **Not adopted now:** the Icelandic pronunciation dictionary (CC BY 4.0,
    usable, but without the compounds' second stress it was wanted for),
    Wikisource prose (allowed text by text, none added) and nothing for
    German or Lithuanian, which keep their rules. Questions to upstream
    projects that could clear more are listed in [TODO.md](../TODO.md)
    (owner decisions, 24 October 2026).

## manuzio's new book languages (24 October 2026)

47. **jespersen reads the new languages that are easy to read from their
    spelling:** Malay, Serbian (Cyrillic and Latin), Macedonian, Swahili and
    Greek, then Kinyarwanda, Kirundi, Luganda, Lingala, Shona, Somali,
    Oromo, Yoruba, Igbo, Hausa, Ewe and Akan. Ukrainian and Bulgarian, whose
    stress the spelling does not show, and Japanese, which needs a kanji
    dictionary, wait (owner decision, 24 October 2026).
48. **Where eSpeak NG has no voice, its notation keeps to Kokoro's
    vocabulary:** the same conventions (stress before the vowel, ː), with a
    sound Kokoro lacks (ɓ, ɗ, ejectives, ħ, ʕ) written as the nearest one it
    has and tone not written, since Kokoro is what reads that notation.
    Plain IPA writes every sound as it is. These languages are checked
    against jespersen's own expectations only.
49. **Norwegian Bokmål stays on its rules, without NST.** NST's own entries
    lowered agreement with eSpeak NG (81.1% to 73.8% on jespersen's own list,
    81.1% to 78.4% on Hamsun), because eSpeak NG reads Bokmål from its
    spelling where NST gives the spoken forms (brød, tolv, Oslo); the 3.5 MB
    is not worth it for plain IPA alone (owner decision, 24 October 2026).

## Scope (24 October 2026)

50. **jespersen does phonetics only:** text in, phonemes out. The grammar
    checker once planned after the phonemes, and the features meant to follow
    it, are dropped (owner decision, 24 October 2026).
51. **The project is called jespersen**, after Otto Jespersen (1860–1943),
    the Danish linguist who helped shape the International Phonetic
    Alphabet: with the grammar checker dropped, a phonetician's name fits
    better than the editor Max Perkins'. The crates are `jespersen_<area>`,
    the facade `jespersen` and the command `jespersen`; the name was free on
    crates.io on 24 October 2026 (owner decision).

# Clean room

jespersen does what eSpeak NG's phonemiser (GPL-3.0-or-later) does, and is
dedicated to the public domain. A faithful port of its code or data, in any
language, would be a derivative work under its licence, so jespersen is written
without it (owner decision, 23 October 2026).

## Never

- Open, read, search, quote, translate or paraphrase its source code,
  dictionaries, rule files, phoneme tables or test data, or those of a port
  (for example the `espeak-ng` Rust crate).
- Copy text from its documentation into jespersen.
- Commit its output, compile it into jespersen, or turn it into lexicon
  entries, rules or test expectations.

## Allowed

- **Running it as a black box** in development, to compare jespersen's output
  with its own and to learn its output notation from what it prints. The
  captures stay in the gitignored `tmp/` folder.
- **Imitating systematic behaviour** seen in that output (which words are
  unstressed and how they are reduced, words joined in writing, allophones),
  in eSpeak NG's notation only, as rules and short lists jespersen writes and
  documents itself (decision 27). Never a bulk transfer of answers, word by
  word, into a lexicon.
- **General phonetics and linguistics**: IPA, textbooks, grammars, published
  research, and pronunciation dictionaries whose licence allows it.
- **Data under permissive licences**, listed in
  [data-sources.md](data-sources.md).
- **Our own knowledge**, written as jespersen's own rules and lexicon entries.

## How agreement is measured

jespersen's tests check its own hand-checked expectations, which are CC0. A
separate local comparison runs a pinned eSpeak NG as a black box on a list of
texts, keeps its answers in `tmp/`, and reports how often jespersen agrees, per
language, by line and by word. It skips itself when the captures are absent,
so CI never needs them. Where jespersen and eSpeak NG disagree, jespersen's
expectation is decided from the sources above, not copied from the capture.

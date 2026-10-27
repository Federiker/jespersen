# jespersen

A text-to-phonemes library and command, written in Rust and dedicated to the
public domain, meant to do what eSpeak NG's phonemiser does without its
licence. It does phonetics only. It is named after Otto Jespersen (1860–1943),
the Danish linguist who helped shape the International Phonetic Alphabet.

**Status:** English (American and British) is done for book narration, in
plain IPA or in eSpeak NG's notation: any word (from CMUdict, from known
words, verse's elisions such as lov'd and heav'n, or with rules learned from
CMUdict), homographs by context, numbers, dates, units, money, addresses
and symbols, and function words as in running text. Mandarin is done too:
Simplified and Traditional characters, in plain IPA, in the notation Kokoro's
Chinese voices learnt, or in pinyin and zhuyin, with tone sandhi, erhua,
characters of several readings read by context, numbers in Chinese and
English words inside. Spanish (Castilian and Latin American), Portuguese
(European and Brazilian), French, Italian, Finnish, Turkish, Indonesian,
Polish, Czech, Slovak, Croatian, Slovenian, Basque, Romanian, Catalan,
German, Dutch, Hungarian, Afrikaans, Latvian, Lithuanian, Estonian,
Swedish, Norwegian Bokmål, Danish, Icelandic, Albanian, Welsh, Irish,
Swahili, Greek, Malay, Serbian (in either script), Macedonian, Somali, Oromo,
Russian, Kinyarwanda, Kirundi, Luganda, Lingala, Shona, Yoruba, Igbo, Hausa,
Ewe and Akan are read from their spelling and jespersen's own word lists
(Danish and Swedish also from NST's pronunciation lexicons), numbers and
abbreviations included, in plain IPA or eSpeak NG's notation. The other
languages come next (see [TODO.md](TODO.md)).

```sh
$ echo "Mr. Smith's water, please." | jespersen phonemes --espeak
mˈɪstɚ smˈɪθs wˈɔːɾɚ, plˈiːz.
$ echo "Mr. Smith's water, please." | jespersen phonemes --espeak -v en-GB
mˈɪstə smˈɪθs wˈɔːtə, plˈiːz.
$ echo "银行里的钱不够，他只好去问问。" | jespersen phonemes -v zh
in˧˥xaŋ˧˥ li˨˩ tɤ˦ tɕʰjɛn˧˥ pu˧˥ kou˥˩, tʰa˥˥ ʈʂɻ̩˧˥xau˨˩ tɕʰy˥˩ wən˥˩ wən˩.
$ echo "银行里的钱不够，他只好去问问。" | jespersen phonemes -v zh --misaki --tones written
i↗nxa↗ŋ li↓ tɤ ʨʰjɛ↗n pu↗ kou↘, tʰa→ ꭧɨ↓xau↓ ʨʰy↘ wə↘n wən.
$ echo "银行里的钱不够，他只好去问问。" | jespersen phonemes -v zh --pinyin
yínháng lǐ de qián bú gòu, tā zhǐhǎo qù wèn wen.
```

- **Text to phonemes.** Raw text in (numbers, dates and abbreviations
  included), IPA out: both in eSpeak NG's notation, which neural voices such
  as Kokoro were trained on, and as plain IPA. English (British and American)
  comes first, then other languages. jespersen does not produce audio.
- **Clean room.** No eSpeak NG code or data is used; see
  [docs/clean-room.md](docs/clean-room.md).

## Licence

jespersen's code and its own data are dedicated to the public domain under
[CC0 1.0](LICENSE). Language data from other sources keeps its own permissive
licence and notice, listed in [docs/data-sources.md](docs/data-sources.md).
Programs that ship Mandarin (the `zh` feature) carry the credit given there
for 開放詞典's data (CC BY 3.0).

## Setup

On a new machine, `scripts/install.sh` (macOS, Linux, Git Bash) or
`scripts\install.bat` (Windows) installs mise and the pinned tools
([mise.toml](mise.toml)), the Rust toolchain
([rust-toolchain.toml](rust-toolchain.toml)) and the Git hooks. Run it again
at any time.

`mise tasks` lists the commands; the main ones:

| Command | Does |
| --- | --- |
| `mise run check` | `cargo check` of the workspace |
| `mise run test` | The tests (nextest) |
| `mise run coverage` | The tests, failing under 80% lines or regions |
| `mise run clippy` | Clippy, warnings denied |
| `mise run fix` | Format the Rust code |
| `mise run audit` | Dependency advisories, licences and sources |
| `mise run ci` | Everything CI runs |

`cargo run -p jespersen_cli -- phonemes --help` runs the command from the
source tree.

## Local scratch data

The gitignored `tmp/` folder holds downloads, research probes, captures from
black-box comparisons and disposable scripts. Nothing in it is a build input.

## Git conventions

[.gitattributes](.gitattributes) normalises text to LF, with CRLF for Windows
batch/command scripts.

### Hooks

[Lefthook](lefthook.yml) runs three hooks, installed by the install scripts:

- **before a commit**, rustfmt formats the staged files and stages the result;
- **on the commit message**, the first line must start with a prefix below;
- **before a push**, Clippy (`mise run clippy`).

`git commit --no-verify` skips them; CI runs the same checks anyway.

### Commit messages

The first line is `[prefix] What changed`, in the imperative and UK English,
for example `[fix] Keep the stress on compounds`. One commit, one prefix: when
a change is two kinds of work, it is usually two commits. Git's own `Merge`,
`Revert` and `fixup!` messages are accepted as they are.

| Prefix | For | Why it is its own word |
| --- | --- | --- |
| `[add]` | Something new: a feature, a language, a command | The most common kind of change; its tests and docs come with it |
| `[fix]` | A bug fixed, including a wrong pronunciation | Lets a reader find when a behaviour was wrong and when it was mended |
| `[remove]` | A feature, file or dead code taken out | Removal is a decision worth finding again |
| `[refactor]` | Code reorganised with the same behaviour | Tells a reviewer that no output should change |
| `[rename]` | Files, crates or identifiers renamed or moved, nothing else | Moves make large diffs; keeping them alone keeps history readable |
| `[perf]` | Faster or smaller, same behaviour | Its measurements belong in the message |
| `[data]` | Language data added or changed: lexicons, rules, test sets | Its source and licence belong in the message and in docs/data-sources.md |
| `[docs]` | Documentation only | Never changes what jespersen does |
| `[test]` | Tests only | Adds confidence without touching the library |
| `[fmt]` | Formatting only | Large ones are listed in `.git-blame-ignore-revs` |
| `[build]` | Tooling, scripts, CI and configuration | The workshop, not the product |
| `[deps]` | Dependency updates (Renovate uses it) | Upgrades are where regressions often come from |
| `[security]` | A security fix or hardening | Easy to find when auditing |
| `[release]` | A version and its notes | Marks what shipped |
| `[revert]` | Undoing an earlier commit by hand | Says what was undone and why |
| `[init]` | The repository's first commit | Once |

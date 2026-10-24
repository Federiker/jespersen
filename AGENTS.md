# Project guidance

Read [README.md](README.md) for setup, [docs/decisions.md](docs/decisions.md)
for the owner's decisions and [docs/clean-room.md](docs/clean-room.md) before
touching anything about pronunciation rules or language data.

## Clean room (read first)

jespersen replaces eSpeak NG's phonemiser (GPL-3.0-or-later) and is dedicated to
the public domain under CC0. Its value depends on being provably free of
eSpeak NG's code and data:

- **Never open, read, quote, port or paraphrase** eSpeak NG's source code,
  dictionaries, rule files or phoneme tables, nor any port of them (for
  example the `espeak-ng` crate, or the source tree in
  `../manuzio/.cache/espeak`). Do not search them, even to "just check".
- It may only be **run as a black box** during development, to compare
  outputs. Its outputs stay in the gitignored `tmp/` folder and are never
  committed, compiled into jespersen or turned into lexicon entries.
- Every piece of language data comes from a source listed in
  [docs/data-sources.md](docs/data-sources.md) with its licence verified from
  the licence text itself. Permissive licences only (CC0, public domain, MIT,
  BSD, Apache, CC-BY, …); nothing share-alike (GPL, LGPL, CC BY-SA, ODbL…).
- When unsure whether a source or an action is allowed, stop and ask.

## Shared roadmap

- Read the root [TODO.md](TODO.md) at the start of each task to understand the
  remaining work and avoid duplicating another session's progress.
- Update relevant checklist items before finishing: add newly discovered
  work, split large tasks, note blockers and check completed items only after
  implementation and verification. A stub does not complete a feature.
- Write each TODO item as a short list of slices; they may change as the work
  shows what it needs. Tick a slice when it is done and verified; once the
  whole item is done and described in the docs, remove it from the TODO
  (document it first if it is not).
- Keep roadmap changes focused on the current task; preserve other
  contributors' entries and unresolved decisions.

## Conventions

- Use UK English in identifiers, user-facing text and documentation.
- Prefer small, cohesive crates and files. Keep Rust `lib.rs` and `mod.rs`
  files to exports only.
- Crates live in `crates/<name>` and are called `jespersen_<name>` (the folder
  omits the prefix); the `jespersen` crate is the facade other programs depend
  on, and `jespersen_cli` builds the `jespersen` command, which only calls the
  library.
- Crates must inherit package metadata, dependencies and lints from the Cargo
  workspace wherever possible.
- Put tests under each crate's `tests` directory. Unit-test modules are
  included from `src/lib.rs` with a `#[path = "../tests/unit/tests.rs"]`
  attribute.
- Keep test coverage at 80% or more of lines and regions
  (`mise run coverage`). CI fails below; new code comes with its tests.
- Language data is reviewable text in the repository; its compact binary form
  is generated at build time and never committed. Each language is a Cargo
  feature.
- Report errors explicitly; do not silently substitute empty or
  success-shaped values.
- Ask before decisions that materially change the public API, the phoneme
  output, the data licences or the licensing model.
- Until the first release, APIs need not stay backwards compatible: do not
  keep shims for older development versions.
- Prefer native Rust tooling (and `mise` tasks); do not add Docker, Python or
  Node to the build, the checks or the scripts.
- The version is set once, in the root `Cargo.toml` (`[workspace.package]`).

## Assistant conduct

- **Local notes:** if `tmp/docs/README.md` exists, read it at the start of
  each task and follow it; it holds local-only instructions that are
  deliberately not committed (`tmp/` is gitignored). Keep it up to date when
  you learn something that belongs there. If it does not exist, carry on
  without it.
- Use the gitignored root `tmp/` folder for local downloads, temporary
  scripts, oracle captures and research outputs. Do not stage it or treat its
  contents as required build inputs. Keep durable findings in tracked docs.
- Stop research helper processes when finished. Retain reusable downloads
  when useful; remove only explicitly identified obsolete scratch files.
- Ask about material uncertainties instead of guessing: anything that
  affects the phoneme output, the data and its licences, or the public API.
- When clarification or approval is needed, prefer interactive questions with
  a small set of concrete choices and a recommended option. Use the available
  question tool when supported; otherwise offer numbered choices in chat.
  Allow a custom answer and ask only questions that materially affect the work.
  This preference applies to all agents following this shared guidance.

Be courteous and constructive. If a request includes insults or abusive
language directed at the assistant, politely decline to continue that request.

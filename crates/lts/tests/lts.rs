//! Training, reading and writing rules on a toy lexicon. Phoneme codes are letters' bytes here;
//! 'K' and 'S' stand for the two phonemes of x.

use std::collections::HashMap;

use jespersen_lts::{BOUNDARY, CONTEXTS, Entry, Rule, key, predict, read_table, train, write_table};

fn entry(word: &str, phonemes: &str) -> Entry {
    Entry {
        letters: word.chars().collect(),
        phonemes: phonemes.bytes().collect(),
    }
}

/// A toy lexicon: c is k before a, o, u and s before e, i; e at the end is silent; x is k s.
fn lexicon() -> Vec<Entry> {
    vec![
        entry("cat", "kat"),
        entry("cot", "kot"),
        entry("cut", "kut"),
        entry("cell", "sel"),
        entry("city", "sity"),
        entry("cite", "sit"),
        entry("rate", "rat"),
        entry("tax", "taKS"),
        entry("box", "boKS"),
        entry("cab", "kab"),
        entry("ice", "is"),
    ]
}

fn table(rules: &[Rule]) -> HashMap<String, Vec<u8>> {
    rules
        .iter()
        .map(|rule| (rule.context.clone(), rule.phonemes.clone()))
        .collect()
}

#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn read(rules: &HashMap<String, Vec<u8>>, word: &str) -> Option<String> {
    let letters: Vec<char> = word.chars().collect();
    predict(&letters, |context| Ok::<_, ()>(rules.get(context).cloned()))
        .unwrap()
        .map(|said| String::from_utf8(said).unwrap())
}

#[test]
fn keys_put_the_letter_in_brackets_with_boundaries_beyond_the_word() {
    let letters: Vec<char> = "cat".chars().collect();
    assert_eq!(key(&letters, 0, (0, 0)), "[c]");
    assert_eq!(key(&letters, 0, (2, 1)), "##[c]a");
    assert_eq!(key(&letters, 2, (1, 2)), "a[t]##");
    assert_eq!(BOUNDARY, '#');
    assert_eq!(CONTEXTS[0], (0, 0), "the context-free rule comes first");
}

#[test]
fn learned_rules_read_the_lexicon_and_generalise() {
    let rules = train(&lexicon(), |code| code, 1);
    let table = table(&rules);
    for entry in lexicon() {
        let word: String = entry.letters.iter().collect();
        let expected = String::from_utf8(entry.phonemes).unwrap();
        assert_eq!(read(&table, &word).as_deref(), Some(expected.as_str()), "{word}");
    }
    // An unseen word: c before i.
    assert_eq!(read(&table, "cib").as_deref(), Some("sib"));
    // A letter never seen has no rule at all.
    assert_eq!(read(&table, "zap"), None);
}

#[test]
fn rules_that_add_nothing_are_pruned_and_rare_contexts_dropped() {
    let all = train(&lexicon(), |code| code, 1);
    // Every letter keeps its context-free rule; t is always t, so it needs nothing more.
    assert!(all.iter().any(|rule| rule.context == "[t]"));
    assert!(
        !all.iter()
            .any(|rule| rule.context.contains("[t]") && rule.context != "[t]")
    );
    let common = train(&lexicon(), |code| code, 3);
    assert!(common.len() < all.len());
    assert!(common.windows(2).all(|pair| pair[0] <= pair[1]), "sorted");
    assert_eq!(train(&lexicon(), |code| code, 1), all, "the same input, the same rules");
}

#[test]
fn words_that_cannot_be_aligned_are_left_out() {
    // Five phonemes from two letters cannot be aligned (at most two per letter).
    let mut entries = lexicon();
    entries.push(entry("ab", "vwxyz"));
    entries.push(entry("", ""));
    let rules = train(&entries, |code| code, 1);
    assert!(!rules.iter().any(|rule| rule.phonemes.contains(&b'z')));
}

#[test]
fn tables_round_trip_through_text() {
    let rules = train(&lexicon(), |code| code, 1);
    let text = write_table(&rules, "toy rules\nsecond line", |code| char::from(code).to_string());
    assert!(text.starts_with("# toy rules\n# second line\n"));
    assert!(text.contains("\t-\n"), "a silent letter is written -");
    let back = read_table(&text, |name| (name.len() == 1).then(|| name.as_bytes()[0])).unwrap();
    assert_eq!(back, rules);
    assert!(
        read_table("[a]\tunknown\n", |_| None)
            .unwrap_err()
            .starts_with("line 1")
    );
    assert!(read_table("no tab\n", |_| None).is_err());
    assert_eq!(read_table("#\n\n# comment\n", |_| None).unwrap(), vec![]);
}

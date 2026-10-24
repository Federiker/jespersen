//! Words spelt alike but said differently by meaning, told apart by the words around them (jespersen's
//! own rules, CC0). Most readings are CMUdict's variants; this module only chooses among them.

use crate::arpabet;

/// Words before which a homograph is a verb in its base form: to, modals, do, let's, please, and
/// subject pronouns (they record, we present).
const VERB_CUES: &[&str] = &[
    "to",
    "will",
    "would",
    "shall",
    "should",
    "can",
    "could",
    "may",
    "might",
    "must",
    "do",
    "does",
    "did",
    "don't",
    "doesn't",
    "didn't",
    "won't",
    "wouldn't",
    "can't",
    "cannot",
    "couldn't",
    "shouldn't",
    "mustn't",
    "let's",
    "please",
    "i",
    "you",
    "we",
    "they",
    "never",
    "always",
    "often",
    "not",
];

/// Subject pronouns before an -s form (she records, it lives).
const THIRD_PERSON: &[&str] = &["he", "she", "it", "who", "which", "that"];

/// Nouns and verbs that differ only by stress: the noun stresses the first syllable (a record), the
/// verb a later one (to record).
const STRESS_SHIFT: &[&str] = &[
    "address",
    "combat",
    "compound",
    "compress",
    "conduct",
    "conflict",
    "console",
    "construct",
    "content",
    "contest",
    "contract",
    "contrast",
    "convert",
    "convict",
    "decrease",
    "defect",
    "desert",
    "digest",
    "discount",
    "escort",
    "export",
    "extract",
    "impact",
    "import",
    "incline",
    "increase",
    "insert",
    "insult",
    "object",
    "perfect",
    "permit",
    "present",
    "produce",
    "progress",
    "project",
    "protest",
    "rebel",
    "recall",
    "record",
    "refund",
    "refuse",
    "reject",
    "subject",
    "survey",
    "suspect",
    "torment",
    "transfer",
    "transport",
    "upgrade",
    "upset",
];

/// The pronunciation of `key` by its context, `prev` and `next` the words around it (lexicon keys),
/// `variants` its pronunciations in the lexicon; `None` when it is no homograph jespersen tells apart
/// or the context does not decide (the lexicon's preferred reading is then used).
pub fn choose(key: &str, prev: Option<&str>, next: Option<&str>, variants: &[&[u8]]) -> Option<Vec<u8>> {
    let verb = prev.is_some_and(|prev| VERB_CUES.contains(&prev))
        || (key.ends_with('s') && prev.is_some_and(|prev| THIRD_PERSON.contains(&prev)));
    if let Some(phones) = vowel_homograph(key, prev, next, verb) {
        return encode(phones);
    }
    let base = key
        .strip_suffix('s')
        .filter(|base| STRESS_SHIFT.contains(base))
        .unwrap_or(key);
    if STRESS_SHIFT.contains(&base) && variants.len() > 1 {
        let primary = |codes: &[u8]| {
            codes
                .iter()
                .filter(|&&code| arpabet::is_vowel(usize::from(code / 3)))
                .position(|&code| code % 3 == 1)
        };
        let pick = if verb {
            // The first of equals: CMUdict lists its preferred reading first.
            variants.iter().rev().max_by_key(|codes| primary(codes))
        } else {
            variants.iter().min_by_key(|codes| primary(codes).unwrap_or(usize::MAX))
        };
        return pick.map(|codes| codes.to_vec());
    }
    if key.ends_with("ate") && variants.len() > 1 {
        // A verb's -ate is said eɪt (to separate), an adjective's or noun's ət (separate rooms).
        let ends_ey = |codes: &&&[u8]| {
            codes.len() >= 2 && usize::from(codes[codes.len() - 2] / 3) == 12 && codes[codes.len() - 1] / 3 == 30
        };
        let found = if verb {
            variants.iter().find(ends_ey)
        } else {
            variants.iter().find(|codes| !ends_ey(codes))
        };
        return found.map(|codes| codes.to_vec());
    }
    None
}

/// Homographs whose vowels or consonants differ, in ARPAbet.
#[expect(clippy::too_many_lines, reason = "a table: one homograph per arm")]
fn vowel_homograph(key: &str, prev: Option<&str>, next: Option<&str>, verb: bool) -> Option<&'static str> {
    let prev_is = |words: &[&str]| prev.is_some_and(|prev| words.contains(&prev));
    let next_is = |words: &[&str]| next.is_some_and(|next| words.contains(&next));
    let determiner = || {
        prev_is(&[
            "a", "an", "the", "this", "that", "these", "those", "my", "your", "his", "her", "its", "our", "their",
        ])
    };
    Some(match key {
        // Books are told in the past: read is rɛd unless a verb form needs riːd (to read).
        "read" => {
            if verb && !prev_is(&["i", "you", "we", "they"]) {
                "R IY1 D"
            } else {
                "R EH1 D"
            }
        }
        "lead" => {
            if prev_is(&["of", "with"]) {
                "L EH1 D"
            } else {
                "L IY1 D"
            }
        }
        "live" => {
            let adjective = determiner()
                || next_is(&[
                    "music",
                    "broadcast",
                    "wire",
                    "show",
                    "stream",
                    "coverage",
                    "ammunition",
                    "audience",
                    "bait",
                ]);
            if adjective { "L AY1 V" } else { "L IH1 V" }
        }
        "lives" => {
            if prev_is(THIRD_PERSON) {
                "L IH1 V Z"
            } else {
                "L AY1 V Z"
            }
        }
        "close" => {
            if verb {
                "K L OW1 Z"
            } else {
                "K L OW1 S"
            }
        }
        "use" => {
            if verb {
                "Y UW1 Z"
            } else {
                "Y UW1 S"
            }
        }
        "excuse" => {
            if verb {
                "IH0 K S K Y UW1 Z"
            } else {
                "IH0 K S K Y UW1 S"
            }
        }
        "abuse" => {
            if verb {
                "AH0 B Y UW1 Z"
            } else {
                "AH0 B Y UW1 S"
            }
        }
        "used" if next == Some("to") => "Y UW1 S T",
        "house" if verb => "HH AW1 Z",
        "tear" => {
            if verb {
                "T EH1 R"
            } else {
                "T IH1 R"
            }
        }
        "tears" => {
            if prev_is(THIRD_PERSON) {
                "T EH1 R Z"
            } else {
                "T IH1 R Z"
            }
        }
        "wind" => {
            if verb {
                "W AY1 N D"
            } else {
                "W IH1 N D"
            }
        }
        "wound" => {
            let past = prev_is(&["had", "has", "have", "was", "were", "been"])
                || next_is(&[
                    "up", "down", "round", "around", "through", "along", "its", "his", "her", "the", "it",
                ]);
            if past { "W AW1 N D" } else { "W UW1 N D" }
        }
        "minute" => {
            let small = prev_is(&["so", "very", "too", "most", "such"])
                || next_is(&[
                    "detail",
                    "details",
                    "particles",
                    "amount",
                    "amounts",
                    "quantity",
                    "quantities",
                    "traces",
                ]);
            if small { "M AY0 N UW1 T" } else { "M IH1 N AH0 T" }
        }
        _ => return None,
    })
}

fn encode(phones: &str) -> Option<Vec<u8>> {
    phones.split(' ').map(arpabet::encode).collect()
}

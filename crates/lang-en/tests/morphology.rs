//! Words CMUdict lacks, read from words it has.

use jespersen_lang_en::{Variety, read_sentence};
use jespersen_phonemes::Notation;

#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn ipa(word: &str) -> Option<String> {
    read_sentence(&[word], Variety::American)
        .unwrap()
        .remove(0)
        .map(|reading| reading.pronunciation.to_notation(Notation::Ipa))
}

#[test]
fn the_words_used_here_are_not_in_cmudict() {
    for word in ["frobnitzer", "quuxes", "lighthousekeeper", "unfriended", "zxqv"] {
        assert!(
            jespersen_lang_en::american(word).unwrap().is_none(),
            "{word} is in CMUdict: pick another"
        );
    }
}

#[test]
fn inflections_take_the_sound_their_stem_calls_for() {
    // A known stem with an inflection CMUdict lacks.
    assert_eq!(ipa("lighthousekeepers").as_deref(), Some("ˈlaɪtˌhaʊsˌkipɚz"));
}

#[test]
fn compounds_lower_the_second_stress() {
    assert_eq!(ipa("lighthousekeeper").as_deref(), Some("ˈlaɪtˌhaʊsˌkipɚ"));
}

#[test]
fn other_unknown_words_are_read_letter_by_letter() {
    assert_eq!(ipa("quuxes").as_deref(), Some("ˈkwuksɪz"));
    assert!(
        ipa("frobnitzer").is_some_and(|ipa| ipa.contains('ˈ')),
        "one primary stress"
    );
    assert!(ipa("Brzęczyszczykiewicz").is_some(), "accents are taken off");
    assert_eq!(ipa("жук"), None, "letters the rules do not know");
}

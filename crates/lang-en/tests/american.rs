//! American English from CMUdict: the plain IPA follows CMUdict's phonemes and stress; the eSpeak
//! NG notation adds its symbols, its stress-mark position and the allophones it writes.

use jespersen_lang_en::american;
use jespersen_phonemes::{Notation, Pronunciation};

#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn first(word: &str) -> Pronunciation {
    american(word).unwrap().unwrap().remove(0)
}

fn ipa(word: &str) -> String {
    first(word).to_notation(Notation::Ipa)
}

fn espeak(word: &str) -> String {
    first(word).to_notation(Notation::Espeak)
}

#[test]
fn plain_ipa_marks_stress_before_the_syllable() {
    assert_eq!(ipa("hello"), "həˈloʊ");
    assert_eq!(ipa("water"), "ˈwɔtɚ");
    assert_eq!(ipa("about"), "əˈbaʊt");
    assert_eq!(ipa("strength"), "ˈstɹɛŋkθ");
    assert_eq!(ipa("understand"), "ˌʌndɚˈstænd");
}

#[test]
fn espeak_notation_marks_stress_before_the_vowel_with_its_symbols() {
    assert_eq!(espeak("hello"), "həlˈoʊ");
    assert_eq!(espeak("father"), "fˈɑːðɚ");
    assert_eq!(espeak("world"), "wˈɜːld");
    assert_eq!(espeak("see"), "sˈiː");
}

#[test]
fn espeak_notation_writes_american_allophones() {
    assert_eq!(espeak("butter"), "bˈʌɾɚ");
    assert_eq!(espeak("party"), "pˈɑːɹɾi");
    assert_eq!(espeak("ladder"), "lˈædɚ");
    assert_eq!(espeak("winter"), "wˈɪntɚ");
    assert_eq!(espeak("ago"), "ɐɡˈoʊ");
    assert_eq!(espeak("attack"), "ɐtˈæk");
}

#[test]
fn case_does_not_matter_and_unknown_words_are_none() {
    assert_eq!(ipa("Hello"), ipa("HELLO"));
    assert_eq!(ipa("don’t"), ipa("don't"));
    assert!(american("qzxqzxq").unwrap().is_none());
    assert!(american("").unwrap().is_none());
}

#[test]
fn homographs_keep_every_pronunciation() {
    let read: Vec<String> = american("read")
        .unwrap()
        .unwrap()
        .iter()
        .map(|p| p.to_notation(Notation::Ipa))
        .collect();
    assert_eq!(read, ["ˈɹɛd", "ˈɹid"]);
}

#[test]
fn jespersen_own_entries_replace_cmudicts() {
    assert_eq!(ipa("thirty"), "ˈθɝti");
    assert_eq!(ipa("houses"), "ˈhaʊzɪz");
    assert_eq!(ipa("every"), "ˈɛvɹi");
}

#[test]
fn the_thought_vowel_is_kept_and_a_second_primary_stress_lowered() {
    assert_eq!(espeak("bought"), "bˈɔːt");
    assert_eq!(espeak("cough"), "kˈɔːf");
    assert_eq!(espeak("thirteen"), "θˈɜːtiːn");
    assert_eq!(ipa("nineteen"), "ˈnaɪntin", "a lowered stress keeps its full vowel");
    assert_eq!(
        espeak("dollar"),
        "dˈɑːlɚ",
        "THOUGHT only where it is spelt au, aw or ough"
    );
}

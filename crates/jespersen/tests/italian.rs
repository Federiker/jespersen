#![cfg(feature = "it")]

use jespersen::{Language, Notation, WriteOptions, phonemize, write};

#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn read(text: &str, notation: Notation) -> String {
    let options = WriteOptions {
        notation,
        drop_punctuation: false,
    };
    write(&phonemize(text, Language::Italian).unwrap(), options).unwrap()
}

#[test]
fn italian_is_read_in_both_notations() {
    assert_eq!(
        read("C'era una volta un pezzo di legno.", Notation::Espeak),
        "tʃˈɛra ˌʊna vˈɔlta ʊn pˈɛtsːo dɪ lˈeɲɲo."
    );
    assert_eq!(read("La pizza, perché no?", Notation::Ipa), "la ˈpittsa, perˌke ˈnɔ?");
}

#[test]
fn italian_tags_tokens_and_abbreviations() {
    for tag in ["it", "it-IT", "it_CH"] {
        assert_eq!(tag.parse::<Language>().unwrap(), Language::Italian, "{tag}");
    }
    assert_eq!(Language::Italian.to_string(), "it");
    assert_eq!(Language::Italian.notations(), [Notation::Ipa, Notation::Espeak]);
    let sentences = phonemize("Al 3° piano. Il Sig. Rossi parla.", Language::Italian).unwrap();
    assert_eq!(sentences.len(), 2);
    let texts: Vec<&str> = sentences[0].tokens.iter().map(|token| token.text).collect();
    assert_eq!(texts, ["Al", "3°", "piano", "."]);
}

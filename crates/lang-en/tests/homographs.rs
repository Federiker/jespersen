//! Homographs told apart by the words around them.

use jespersen_lang_en::{BathVowel, Variety, read_sentence};
use jespersen_phonemes::Notation;

/// The plain IPA of `word` (the only word read so) in `sentence`.
#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn word_in(sentence: &str, word: &str, variety: Variety) -> String {
    let words: Vec<&str> = sentence.split(' ').collect();
    let at = words.iter().position(|w| *w == word).unwrap();
    read_sentence(&words, variety).unwrap()[at]
        .as_ref()
        .unwrap()
        .pronunciation
        .to_notation(Notation::Ipa)
}

fn us(sentence: &str, word: &str) -> String {
    word_in(sentence, word, Variety::American)
}

#[test]
fn stress_shift_nouns_and_verbs() {
    assert_eq!(us("a new record", "record"), "ˈɹɛkɚd");
    assert_eq!(us("they record it", "record"), "ɹɪˈkɔɹd");
    assert_eq!(us("to record it", "record"), "ɹɪˈkɔɹd");
    assert_eq!(us("she records it", "records"), "ɹɪˈkɔɹdz");
    assert_eq!(us("the records", "records"), "ˈɹɛkɚdz");
    assert_eq!(us("a present for you", "present"), "ˈpɹɛzənt");
    assert_eq!(us("we present it", "present"), "pɹiˈzɛnt");
    assert_eq!(us("an object here", "object"), "ˈɑbdʒɛkt");
    assert_eq!(us("I object", "object"), "əbˈdʒɛkt");
}

#[test]
fn ate_words_are_said_in_full_as_verbs() {
    assert_eq!(us("two separate rooms", "separate"), "ˈsɛpɚɪt");
    assert_eq!(us("to separate them", "separate"), "ˈsɛpɚˌeɪt");
}

#[test]
fn read_is_past_unless_a_verb_form_needs_the_present() {
    assert_eq!(us("he read the letter", "read"), "ˈɹɛd");
    assert_eq!(us("I read the letter", "read"), "ˈɹɛd", "a book's past");
    assert_eq!(us("I will read it", "read"), "ˈɹid");
    assert_eq!(us("to read it", "read"), "ˈɹid");
}

#[test]
fn other_homographs() {
    assert_eq!(us("they live here", "live"), "ˈlɪv");
    assert_eq!(us("a live wire", "live"), "ˈlaɪv");
    assert_eq!(us("their lives were short", "lives"), "ˈlaɪvz");
    assert_eq!(us("she lives here", "lives"), "ˈlɪvz");
    assert_eq!(us("close the door", "close"), "ˈkloʊs", "no cue: the adjective");
    assert_eq!(us("please close the door", "close"), "ˈkloʊz");
    assert_eq!(us("the use of it", "use"), "ˈjus");
    assert_eq!(us("we use it", "use"), "ˈjuz");
    assert_eq!(us("he used to go", "used"), "ˈjust");
    assert_eq!(us("a tear fell", "tear"), "ˈtɪɹ");
    assert_eq!(us("do not tear it", "tear"), "ˈtɛɹ");
    assert_eq!(us("the wind blew", "wind"), "ˈwɪnd");
    assert_eq!(us("to wind the clock", "wind"), "ˈwaɪnd");
    assert_eq!(us("a deep wound", "wound"), "ˈwund");
    assert_eq!(us("the road wound through", "wound"), "ˈwaʊnd");
    assert_eq!(us("one minute", "minute"), "ˈmɪnət");
    assert_eq!(us("a very minute amount", "minute"), "maɪˈnut");
    assert_eq!(us("made of lead", "lead"), "ˈlɛd");
    assert_eq!(us("they lead us", "lead"), "ˈlid");
    assert_eq!(us("to house them", "house"), "ˈhaʊz");
}

#[test]
fn british_homographs_are_converted_like_other_words() {
    let british = Variety::British(BathVowel::ByNotation);
    // RP says the noun ˈrɛkɔːd; the British form of each sense is still to come (TODO.md).
    assert!(word_in("a new record", "record", british).starts_with("ˈɹɛk"));
    assert_eq!(word_in("to record it", "record", british), "ɹɪˈkɔːd");
}

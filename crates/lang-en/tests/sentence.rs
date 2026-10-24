//! Words read together in running text: function words, and "the" before a vowel.

use jespersen_lang_en::{Variety, read_sentence};
use jespersen_phonemes::Notation;

#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn read(sentence: &str, notation: Notation) -> String {
    let words: Vec<&str> = sentence.split(' ').collect();
    read_sentence(&words, Variety::American)
        .unwrap()
        .iter()
        .map(|reading| reading.as_ref().unwrap().pronunciation.to_notation(notation))
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn function_words_are_unstressed_with_full_vowels_in_espeak_notation() {
    assert_eq!(read("bread and butter", Notation::Espeak), "bɹˈɛd ænd bˈʌɾɚ");
    assert_eq!(read("the cup of tea", Notation::Espeak), "ðə kˈʌp ʌv tˈiː");
    assert_eq!(read("you were there for her", Notation::Espeak), "juː wɜː ðɛɹ fɔːɹ hɜː");
    assert_eq!(read("faster than he has", Notation::Espeak), "fˈæstɚ ðɐn hiː hɐz");
}

#[test]
fn function_words_take_their_weak_forms_in_plain_ipa() {
    assert_eq!(read("bread and butter", Notation::Ipa), "ˈbɹɛd ənd ˈbʌtɚ");
    assert_eq!(read("the cup of tea", Notation::Ipa), "ðə ˈkʌp əv ˈti");
    assert_eq!(read("you were there for her", Notation::Ipa), "ju wɚ ðɚ fɚ hɚ");
}

#[test]
fn some_words_are_only_lightly_stressed() {
    assert_eq!(read("not so", Notation::Espeak), "nˌɑːt sˌoʊ");
    assert_eq!(read("about", Notation::Espeak), "ɐbˌaʊt");
    assert_eq!(read("on into", Notation::Espeak), "ˌɔn ˌɪntʊ");
    assert_eq!(read("over", Notation::Ipa), "ˌoʊvɚ");
}

#[test]
fn the_before_a_vowel() {
    assert_eq!(read("the apple", Notation::Espeak), "ðɪ ˈæpəl");
    assert_eq!(read("the apple", Notation::Ipa), "ði ˈæpəl");
    assert_eq!(read("the pear", Notation::Espeak), "ðə pˈɛɹ");
    assert_eq!(read("The end", Notation::Espeak), "ðɪ ˈɛnd");
    assert_eq!(read("the", Notation::Espeak), "ðə");
}

#[test]
fn unknown_words_are_none_and_do_not_disturb_the_rest() {
    let read = read_sentence(&["the", "жук", "end"], Variety::American).unwrap();
    assert!(read[0].is_some() && read[1].is_none() && read[2].is_some());
    assert_eq!(
        read[0].as_ref().unwrap().pronunciation.to_notation(Notation::Espeak),
        "ðə"
    );
    assert!(read_sentence(&[], Variety::American).unwrap().is_empty());
}

#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn joins(sentence: &str) -> Vec<bool> {
    let words: Vec<&str> = sentence.split(' ').collect();
    read_sentence(&words, Variety::American)
        .unwrap()
        .iter()
        .map(|reading| reading.as_ref().unwrap().joined_to_next)
        .collect()
}

#[test]
fn some_function_words_join_the_next_word_in_espeak_notation() {
    assert_eq!(joins("in the house"), [true, false, false]);
    assert_eq!(joins("in a house"), [false, false, false]);
    assert_eq!(read("at a party", Notation::Espeak), "æɾ ə pˈɑːɹɾi");
    assert_eq!(joins("at a party"), [true, false, false]);
    assert_eq!(read("was a party", Notation::Espeak), "wʌz ɐ pˈɑːɹɾi");
    assert_eq!(read("for a while", Notation::Espeak), "fɚɹ ə wˌaɪl");
    assert_eq!(read("for a while", Notation::Ipa), "fɚ ə ˌwaɪl");
}

#[test]
fn flaps_links_and_the_vowel_of_to_across_words() {
    assert_eq!(read("it is", Notation::Espeak), "ɪɾ ɪz");
    assert_eq!(read("butter and bread", Notation::Espeak), "bˈʌɾɚɹ ænd bɹˈɛd");
    assert_eq!(read("to eat", Notation::Espeak), "tʊ ˈiːt");
    assert_eq!(read("to eat", Notation::Ipa), "tu ˈit");
}

#[test]
fn contractions_are_unstressed_or_light() {
    assert_eq!(read("she's", Notation::Espeak), "ʃiːz");
    assert_eq!(read("you'd", Notation::Espeak), "juːd");
    assert_eq!(read("isn't", Notation::Espeak), "ˌɪzənt");
    assert_eq!(read("through", Notation::Espeak), "θɹuː");
}

#[test]
fn initialisms_and_a_lone_letter_a_are_spelled() {
    assert_eq!(read("U.S.A", Notation::Espeak), "jˌuːˌɛsˈeɪ");
    assert_eq!(read("U.S.A", Notation::Ipa), "ˌjuˌɛsˈeɪ");
    assert_eq!(read("NHS", Notation::Espeak), "ˌɛnˌeɪtʃˈɛs");
    assert_eq!(
        read("the NHS", Notation::Espeak),
        "ðɪ ˌɛnˌeɪtʃˈɛs",
        "the before a spelled vowel"
    );
    assert_eq!(read("NASA", Notation::Espeak), "nˈæsə", "with vowels it is a word");
    assert_eq!(read("a", Notation::Espeak), "ˈeɪ");
    assert_eq!(read("a cat", Notation::Espeak), "ɐ kˈæt");
    assert_eq!(read("W", Notation::Ipa), "ˈdʌbəlju");
}

#[test]
fn joined_pairs_reduce_the_second_word_too() {
    assert_eq!(read("he had been there", Notation::Espeak), "hiː hɐd bɪn ðˈɛɹ");
    assert_eq!(joins("he had been there"), [false, true, false, false]);
    assert_eq!(read("it has to be done", Notation::Espeak), "ɪt hɐz tə bi dˈʌn");
    assert_eq!(read("I am here", Notation::Espeak), "aɪ ɐm hˈɪɹ");
    assert_eq!(
        read("there was a man", Notation::Espeak),
        "ðɛɹ wˌʌz ɐ mˈæn",
        "was, reduced, is not joined again"
    );
    assert_eq!(read("he did not go", Notation::Espeak), "hiː dɪd nˌɑːt ɡˌoʊ");
    assert_eq!(read("out of the house", Notation::Espeak), "ˌaʊɾ əv ðə hˈaʊs");
    assert_eq!(read("an apple", Notation::Espeak), "ɐn ˈæpəl");
    assert_eq!(read("without because upon", Notation::Espeak), "wɪðˌaʊt bɪkˈʌz əpˌɑːn");
}

#[test]
fn glottal_t_and_syllabic_n_in_american_espeak_notation() {
    assert_eq!(read("button", Notation::Espeak), "bˈʌʔn̩");
    assert_eq!(read("button", Notation::Ipa), "ˈbʌtən", "plain IPA keeps tən");
    assert_eq!(read("buttons written", Notation::Espeak), "bˈʌʔn̩z ɹˈɪʔn̩");
    assert_eq!(
        read("Latin curtain", Notation::Espeak),
        "lˈætɪn kˈɜːtən",
        "neither glottal nor flapped"
    );
    assert_eq!(read("butter", Notation::Espeak), "bˈʌɾɚ", "still flapped");
}

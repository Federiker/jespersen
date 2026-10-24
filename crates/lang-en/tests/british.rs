//! British English (RP), read from the American forms with jespersen's own rules (decision 20).

use jespersen_lang_en::{BathVowel, Variety, read_sentence, read_tokens};
use jespersen_phonemes::Notation;
use jespersen_text::TokenKind::Number;

const BRITISH: Variety = Variety::British(BathVowel::ByNotation);

#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn read_as(sentence: &str, variety: Variety, notation: Notation) -> String {
    let words: Vec<&str> = sentence.split(' ').collect();
    read_sentence(&words, variety)
        .unwrap()
        .iter()
        .map(|reading| reading.as_ref().unwrap().pronunciation.to_notation(notation))
        .collect::<Vec<_>>()
        .join(" ")
}

fn ipa(sentence: &str) -> String {
    read_as(sentence, BRITISH, Notation::Ipa)
}

fn espeak(sentence: &str) -> String {
    read_as(sentence, BRITISH, Notation::Espeak)
}

#[test]
fn lot_palm_and_thought_follow_the_spelling() {
    assert_eq!(ipa("hot"), "ˈhɒt");
    assert_eq!(ipa("watch"), "ˈwɒtʃ", "a after w");
    assert_eq!(ipa("quality"), "ˈkwɒlɪti", "a after qu");
    assert_eq!(ipa("father"), "ˈfɑːðə");
    assert_eq!(ipa("dog"), "ˈdɒɡ", "the cloth words");
    assert_eq!(ipa("thought"), "ˈθɔːt");
    assert_eq!(espeak("boat"), "bˈəʊt");
}

#[test]
fn r_is_said_only_before_a_vowel() {
    assert_eq!(ipa("car"), "ˈkɑː");
    assert_eq!(ipa("north"), "ˈnɔːθ");
    assert_eq!(espeak("near"), "nˈiə");
    assert_eq!(ipa("square"), "ˈskweə");
    assert_eq!(ipa("cure"), "ˈkjʊə");
    assert_eq!(ipa("nurse"), "ˈnɜːs");
    assert_eq!(espeak("letter"), "lˈɛtə", "no flap either");
    assert_eq!(ipa("very"), "ˈvɛɹi");
    assert_eq!(espeak("carry"), "kˈaɹi");
    assert_eq!(ipa("starry"), "ˈstɑːɹi");
    assert_eq!(ipa("story"), "ˈstɔːɹi");
    assert_eq!(ipa("series"), "ˈsɪəɹiz");
    assert_eq!(ipa("memory"), "ˈmɛməɹi");
    assert_eq!(ipa("hurry"), "ˈhʌɹi");
}

#[test]
fn r_links_to_a_following_vowel_in_espeak_notation() {
    assert_eq!(espeak("butter and bread"), "bˈʌtəɹ and bɹˈɛd");
    assert_eq!(ipa("butter and bread"), "ˈbʌtə ənd ˈbɹɛd");
    assert_eq!(espeak("far away"), "fˈɑːɹ ɐwˈeɪ");
    assert_eq!(espeak("for ever"), "fɔːɹ ˈɛvə");
    assert_eq!(espeak("butter"), "bˈʌtə", "nothing to link to");
    assert_eq!(espeak("her own"), "hɜː ˈəʊn", "her does not link");
}

#[test]
fn yod_after_t_d_n_and_th_where_spelt_u() {
    assert_eq!(ipa("tune"), "ˈtjuːn");
    assert_eq!(ipa("new"), "ˈnjuː");
    assert_eq!(ipa("duke"), "ˈdjuːk");
    assert_eq!(ipa("suit"), "ˈsuːt", "not after s");
    assert_eq!(ipa("noon"), "ˈnuːn", "not where spelt oo");
}

#[test]
fn ary_ery_ory_are_reduced() {
    assert_eq!(ipa("secretary"), "ˈsɛkɹətəɹi");
    assert_eq!(ipa("library"), "ˈlaɪbɹəɹi");
}

#[test]
fn the_bath_vowel_is_an_option() {
    assert_eq!(ipa("bath"), "ˈbɑːθ", "broad in plain IPA by default");
    assert_eq!(
        espeak("bath"),
        "bˈaθ",
        "TRAP's vowel in eSpeak NG's notation by default"
    );
    let broad = Variety::British(BathVowel::Broad);
    let flat = Variety::British(BathVowel::Flat);
    assert_eq!(read_as("dance", broad, Notation::Espeak), "dˈɑːns");
    assert_eq!(read_as("class", flat, Notation::Ipa), "ˈklæs");
    assert_eq!(ipa("after"), "ˈɑːftə");
    assert_eq!(ipa("example"), "ɪɡˈzɑːmpəl");
    assert_eq!(ipa("plant"), "ˈplɑːnt", "a listed n-t stem");
    assert_eq!(ipa("fantasy"), "ˈfæntəsi", "other n-t stays TRAP");
    assert_eq!(ipa("gas"), "ˈɡæs", "a listed exception");
    assert_eq!(ipa("traffic"), "ˈtɹæfɪk", "f before a vowel");
}

#[test]
fn jespersen_own_british_entries() {
    assert_eq!(ipa("schedule"), "ˈʃɛdjuːl");
    assert_eq!(ipa("tomato"), "təˈmɑːtəʊ");
    assert_eq!(ipa("can't"), "ˈkɑːnt");
    assert_eq!(espeak("can't"), "kˈɑːnt", "PALM, not BATH");
    assert_eq!(ipa("orange"), "ˈɒɹɪndʒ");
    assert_eq!(ipa("been"), "ˌbiːn", "still a lightly stressed function word");
}

#[test]
fn function_words_take_british_vowels() {
    assert_eq!(espeak("of"), "ɒv");
    assert_eq!(espeak("your"), "jɔː");
    assert_eq!(espeak("there are"), "ðeəɹ ɑː");
    assert_eq!(ipa("for a while"), "fə ə ˌwaɪl");
}

#[test]
fn british_spellings_and_letters() {
    assert_eq!(ipa("colour"), read_as("color", BRITISH, Notation::Ipa));
    assert_eq!(ipa("centre"), read_as("center", BRITISH, Notation::Ipa));
    assert_eq!(ipa("realise"), read_as("realize", BRITISH, Notation::Ipa));
    assert_eq!(ipa("travelled"), read_as("traveled", BRITISH, Notation::Ipa));
    assert_eq!(ipa("Z."), "ˈzɛd");
    assert_eq!(read_as("Z.", Variety::American, Notation::Ipa), "ˈzi");
}

#[test]
fn numbers_say_and() {
    let read = read_tokens(&[("105", Number)], BRITISH).unwrap().remove(0).unwrap();
    let words: Vec<String> = read.words.iter().map(|word| word.to_notation(Notation::Ipa)).collect();
    assert_eq!(words, ["ˈwʌn", "ˈhʌndɹəd", "ənd", "ˈfaɪv"]);
}

#[test]
fn one_and_because() {
    assert_eq!(espeak("one"), "wˈɒn");
    assert_eq!(ipa("one"), "ˈwʌn", "RP keeps STRUT in plain IPA");
    assert_eq!(espeak("someone"), "sˈʌmwɒn");
    assert_eq!(espeak("once"), "wˈʌns");
    assert_eq!(espeak("because"), "bɪkˈɒz");
}

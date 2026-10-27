//! Verse's elisions, a word's apostrophe for a sound it leaves out, with straight and curly
//! apostrophes. Expectations checked by hand against dictionaries' forms of the full words.

use jespersen_lang_en::{BathVowel, Variety, read_sentence};
use jespersen_phonemes::Notation;

const BRITISH: Variety = Variety::British(BathVowel::ByNotation);

#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn ipa(word: &str, variety: Variety) -> String {
    read_sentence(&[word], variety)
        .unwrap()
        .remove(0)
        .unwrap()
        .pronunciation
        .to_notation(Notation::Ipa)
}

/// Each word, straight and curly, read as expected in British and American English.
fn check(cases: &[(&str, &str, &str)]) {
    for &(word, british, american) in cases {
        for spelling in [word.to_owned(), word.replace('\'', "’")] {
            assert_eq!(ipa(&spelling, BRITISH), british, "{spelling}, British");
            assert_eq!(ipa(&spelling, Variety::American), american, "{spelling}, American");
        }
    }
}

#[test]
fn the_words_used_here_are_not_in_cmudict() {
    for word in ["dimm'd", "lov'd", "ow'st", "wander'st", "heav'n", "wand'ring", "pow'r"] {
        assert!(
            jespersen_lang_en::american(word).unwrap().is_none(),
            "{word} is in CMUdict: pick another"
        );
    }
}

#[test]
fn d_is_the_past_ending_without_its_vowel() {
    check(&[
        ("dimm'd", "ˈdɪmd", "ˈdɪmd"),
        ("lov'd", "ˈlʌvd", "ˈlʌvd"),
        ("call'd", "ˈkɔːld", "ˈkɔld"),
        ("pass'd", "ˈpɑːst", "ˈpæst"),
        ("fear'd", "ˈfɪəd", "ˈfɪɹd"),
        ("hop'd", "ˈhəʊpt", "ˈhoʊpt"),
    ]);
}

#[test]
fn st_is_the_second_person_ending_without_its_vowel() {
    check(&[
        ("ow'st", "ˈəʊst", "ˈoʊst"),
        ("com'st", "ˈkʌmst", "ˈkʌmst"),
        ("giv'st", "ˈɡɪvst", "ˈɡɪvst"),
        ("wander'st", "ˈwɒndəst", "ˈwɑndɚst"),
    ]);
}

#[test]
fn a_left_out_vowel_is_not_said() {
    check(&[
        ("heav'n", "ˈhɛvn", "ˈhɛvn"),
        ("heav'nly", "ˈhɛvnli", "ˈhɛvnli"),
        ("giv'n", "ˈɡɪvn", "ˈɡɪvn"),
        ("fall'n", "ˈfɑːln", "ˈfɑln"),
        ("sev'n", "ˈsɛvn", "ˈsɛvn"),
        ("ev'ry", "ˈɛvɹi", "ˈɛvɹi"),
        ("wand'ring", "ˈwɒndɹɪŋ", "ˈwɑndɹɪŋ"),
        ("pow'r", "ˈpaʊə", "ˈpaʊɚ"),
    ]);
}

#[test]
fn a_left_out_consonant_is_a_lexicon_entry() {
    check(&[
        ("e'en", "ˈiːn", "ˈin"),
        ("e'er", "ˈeə", "ˈɛɹ"),
        ("o'er", "ˈɔː", "ˈoʊɹ"),
        ("ne'er", "ˈneə", "ˈnɛɹ"),
        ("ta'en", "ˈteɪn", "ˈteɪn"),
        ("howe'er", "ˌhaʊˈeə", "ˌhaʊˈɛɹ"),
    ]);
}

#[test]
fn other_apostrophes_are_left_alone() {
    check(&[
        ("John's", "ˈdʒɒnz", "ˈdʒɑnz"),
        ("don't", "ˈdəʊnt", "ˈdoʊnt"),
        ("o'clock", "əˈklɒk", "əˈklɑk"),
    ]);
}

#![cfg(feature = "es")]

use jespersen::{Language, Notation, SpanishVariety, WriteOptions, phonemize, write};

#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn read(text: &str, variety: SpanishVariety, notation: Notation) -> String {
    let options = WriteOptions {
        notation,
        drop_punctuation: false,
    };
    write(&phonemize(text, Language::Spanish(variety)).unwrap(), options).unwrap()
}

#[test]
fn spanish_is_read_in_both_notations_and_varieties() {
    assert_eq!(
        read(
            "La calle y el cielo, ¿verdad?",
            SpanishVariety::Castilian,
            Notation::Espeak
        ),
        "la kˈaʎe i el θjˈelo, ¿beɾðˈad?"
    );
    assert_eq!(
        read("La calle y el cielo.", SpanishVariety::LatinAmerican, Notation::Espeak),
        "la kˈajje i el sjˈelo."
    );
    assert_eq!(
        read(
            "Rápidamente, el franco-alemán.",
            SpanishVariety::Castilian,
            Notation::Ipa
        ),
        "ˈrapidaˈmente, el ˈfɾanko-aleˈman."
    );
}

#[test]
fn an_ordinal_and_its_signs_are_one_token() {
    let sentences = phonemize("Vivo en el 3.º piso.", Language::Spanish(SpanishVariety::Castilian)).unwrap();
    let texts: Vec<&str> = sentences[0].tokens.iter().map(|token| token.text).collect();
    assert_eq!(texts, ["Vivo", "en", "el", "3.º", "piso", "."]);
}

#[test]
fn spanish_tags_and_abbreviations() {
    for (tag, variety) in [
        ("es", SpanishVariety::Castilian),
        ("es-ES", SpanishVariety::Castilian),
        ("es-419", SpanishVariety::LatinAmerican),
        ("es-MX", SpanishVariety::LatinAmerican),
        ("es_ar", SpanishVariety::LatinAmerican),
    ] {
        assert_eq!(tag.parse::<Language>().unwrap(), Language::Spanish(variety), "{tag}");
    }
    assert_eq!(Language::Spanish(SpanishVariety::LatinAmerican).to_string(), "es-419");
    assert_eq!(
        Language::Spanish(SpanishVariety::Castilian).notations(),
        [Notation::Ipa, Notation::Espeak]
    );
    // Sr. does not end the sentence.
    let sentences = phonemize(
        "Vino el Sr. García. Se fue.",
        Language::Spanish(SpanishVariety::Castilian),
    )
    .unwrap();
    assert_eq!(sentences.len(), 2);
}

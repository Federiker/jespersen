use jespersen_lang_en::is_abbreviation;

#[test]
fn titles_and_common_abbreviations() {
    for word in ["Mr", "Mrs", "Dr", "St", "Prof", "vs", "No"] {
        assert!(is_abbreviation(word), "{word}");
    }
    for word in ["Mr.", "mr", "Hello", "", "end"] {
        assert!(!is_abbreviation(word), "{word}");
    }
}

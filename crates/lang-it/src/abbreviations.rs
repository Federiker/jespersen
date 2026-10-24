/// Italian abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 26] = [
    ("Sig", "signor"),
    ("Sigg", "signori"),
    ("Dott", "dottor"),
    ("Prof", "professor"),
    ("Ing", "ingegner"),
    ("Avv", "avvocato"),
    ("Arch", "architetto"),
    ("Geom", "geometra"),
    ("Rag", "ragionier"),
    ("On", "onorevole"),
    ("Mons", "monsignor"),
    ("ecc", "eccetera"),
    ("pag", "pagina"),
    ("pagg", "pagine"),
    ("tel", "telefono"),
    ("es", "esempio"),
    ("cap", "capitolo"),
    ("vol", "volume"),
    ("sec", "secolo"),
    ("ca", "circa"),
    ("cfr", "confronta"),
    ("n", "numero"),
    ("p", "pagina"),
    ("S", "san"),
    ("Spett", "spettabile"),
    ("Egr", "egregio"),
];

/// Whether `word` (without its full stop) is an abbreviation that does not end a sentence.
#[must_use]
pub fn is_abbreviation(word: &str) -> bool {
    expansion(word).is_some()
}

/// How an abbreviation is said, `None` for a word that is not one. One listed in lower case is
/// found in any case (ecc., Ecc.).
pub(crate) fn expansion(word: &str) -> Option<&'static str> {
    let lower = word.to_lowercase();
    ABBREVIATIONS
        .iter()
        .find(|(abbreviation, _)| {
            *abbreviation == word || (abbreviation.chars().all(char::is_lowercase) && *abbreviation == lower)
        })
        .map(|(_, said)| *said)
}

/// The names of the letters, for initialisms said letter by letter (DVD di vu di).
pub(crate) fn letter_name(letter: char) -> Option<&'static str> {
    Some(match letter.to_lowercase().next()? {
        'a' => "a",
        'b' => "bi",
        'c' => "ci",
        'd' => "di",
        'e' => "e",
        'f' => "effe",
        'g' => "gi",
        'h' => "acca",
        'i' => "i",
        'j' => "i lunga",
        'k' => "cappa",
        'l' => "elle",
        'm' => "emme",
        'n' => "enne",
        'o' => "o",
        'p' => "pi",
        'q' => "cu",
        'r' => "erre",
        's' => "esse",
        't' => "ti",
        'u' => "u",
        'v' => "vu",
        'w' => "doppia vu",
        'x' => "ics",
        'y' => "ipsilon",
        'z' => "zeta",
        _ => return None,
    })
}

/// Spanish abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 34] = [
    ("Sr", "señor"),
    ("Sra", "señora"),
    ("Srta", "señorita"),
    ("Sres", "señores"),
    ("Dr", "doctor"),
    ("Dra", "doctora"),
    ("D", "don"),
    ("Dña", "doña"),
    ("Ud", "usted"),
    ("Uds", "ustedes"),
    ("Vd", "usted"),
    ("Vds", "ustedes"),
    ("etc", "etcétera"),
    ("pág", "página"),
    ("págs", "páginas"),
    ("núm", "número"),
    ("tel", "teléfono"),
    ("av", "avenida"),
    ("avda", "avenida"),
    ("Sto", "santo"),
    ("Sta", "santa"),
    ("prof", "profesor"),
    ("Lic", "licenciado"),
    ("Ing", "ingeniero"),
    ("aprox", "aproximadamente"),
    ("cap", "capítulo"),
    ("vol", "volumen"),
    ("ej", "ejemplo"),
    ("EE", "estados"),
    ("UU", "unidos"),
    ("admón", "administración"),
    ("izq", "izquierda"),
    ("dcha", "derecha"),
    ("p", "página"),
];

/// Whether `word` (without its full stop) is an abbreviation that does not end a sentence.
#[must_use]
pub fn is_abbreviation(word: &str) -> bool {
    expansion(word).is_some()
}

/// How an abbreviation is said, `None` for a word that is not one. One listed in lower case is
/// found in any case (etc., Etc.).
pub(crate) fn expansion(word: &str) -> Option<&'static str> {
    let lower = word.to_lowercase();
    ABBREVIATIONS
        .iter()
        .find(|(abbreviation, _)| {
            *abbreviation == word || (abbreviation.chars().all(char::is_lowercase) && *abbreviation == lower)
        })
        .map(|(_, said)| *said)
}

/// The names of the letters, for initialisms said letter by letter (DVD de uve de).
pub(crate) fn letter_name(letter: char) -> Option<&'static str> {
    Some(match letter.to_lowercase().next()? {
        'a' => "a",
        'b' => "be",
        'c' => "ce",
        'd' => "de",
        'e' => "e",
        'f' => "efe",
        'g' => "ge",
        'h' => "hache",
        'i' => "i",
        'j' => "jota",
        'k' => "ka",
        'l' => "ele",
        'm' => "eme",
        'n' => "ene",
        'ñ' => "eñe",
        'o' => "o",
        'p' => "pe",
        'q' => "cu",
        'r' => "erre",
        's' => "ese",
        't' => "te",
        'u' => "u",
        'v' => "uve",
        'w' => "uve doble",
        'x' => "equis",
        'y' => "ye",
        'z' => "zeta",
        _ => return None,
    })
}

/// jespersen's own exceptions (data/es/lexicon.tsv): the respelling of a word the rules read
/// wrongly (méxico → méjico).
pub(crate) fn respelling(word: &str) -> Option<&'static str> {
    const LEXICON: &str = include_str!("../../../data/es/lexicon.tsv");
    LEXICON
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| {
            let mut fields = line.split('\t');
            Some((fields.next()?, fields.next()?))
        })
        .find(|(entry, _)| *entry == word)
        .map(|(_, respelt)| respelt)
}

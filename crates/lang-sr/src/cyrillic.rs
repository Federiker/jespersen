//! Serbian Cyrillic turned into Latin letter by letter, as the two scripts correspond (љ lj, њ nj,
//! џ dž, ђ đ, ћ ć, ј j), so both are read by the same rules.

/// The Latin letters of one Cyrillic letter in lower case, `None` for a letter that is not
/// Serbian Cyrillic.
fn latin(letter: char) -> Option<&'static str> {
    Some(match letter {
        'а' => "a",
        'б' => "b",
        'в' => "v",
        'г' => "g",
        'д' => "d",
        'ђ' => "đ",
        'е' => "e",
        'ж' => "ž",
        'з' => "z",
        'и' => "i",
        'ј' => "j",
        'к' => "k",
        'л' => "l",
        'љ' => "lj",
        'м' => "m",
        'н' => "n",
        'њ' => "nj",
        'о' => "o",
        'п' => "p",
        'р' => "r",
        'с' => "s",
        'т' => "t",
        'ћ' => "ć",
        'у' => "u",
        'ф' => "f",
        'х' => "h",
        'ц' => "c",
        'ч' => "č",
        'џ' => "dž",
        'ш' => "š",
        _ => return None,
    })
}

/// `text` in lower case and in Latin letters; Latin text is only put in lower case.
pub(crate) fn to_latin(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .fold(String::with_capacity(text.len()), |mut out, letter| {
            match latin(letter) {
                Some(letters) => out.push_str(letters),
                None => out.push(letter),
            }
            out
        })
}

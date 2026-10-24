//! Letters with marks, as the spellings of West Africa write them: a dot or a vertical line below
//! (Yoruba ẹ, ọ, ṣ; Igbo ị, ụ), a dot above (Igbo ṅ), a tilde for a nasal vowel (Ewe ã, ɛ̃) and
//! tone accents, whether a letter comes precomposed (ọ) or as a base letter and combining marks
//! (o and U+0323).

/// A letter in lower case and the marks that change its sound; tone accents (acute, grave,
/// circumflex, caron, macron) and the diaeresis are dropped.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Letter {
    /// The letter without its marks.
    pub base: char,
    /// A dot or a vertical line below it.
    pub below: bool,
    /// A dot above it.
    pub above: bool,
    /// A tilde over it.
    pub tilde: bool,
}

impl Letter {
    const fn plain(base: char) -> Self {
        Self {
            base,
            below: false,
            above: false,
            tilde: false,
        }
    }
}

/// The letter a precomposed character is made from, and its marks.
fn decompose(c: char) -> Letter {
    let below = |base| Letter {
        below: true,
        ..Letter::plain(base)
    };
    let tilde = |base| Letter {
        tilde: true,
        ..Letter::plain(base)
    };
    match c {
        'á' | 'à' | 'â' | 'ǎ' | 'ā' | 'ä' => Letter::plain('a'),
        'é' | 'è' | 'ê' | 'ě' | 'ē' | 'ë' => Letter::plain('e'),
        'í' | 'ì' | 'î' | 'ǐ' | 'ī' | 'ï' => Letter::plain('i'),
        'ó' | 'ò' | 'ô' | 'ǒ' | 'ō' | 'ö' => Letter::plain('o'),
        'ú' | 'ù' | 'û' | 'ǔ' | 'ū' | 'ü' => Letter::plain('u'),
        'ń' | 'ǹ' | 'ň' => Letter::plain('n'),
        'ḿ' => Letter::plain('m'),
        'ã' => tilde('a'),
        'ẽ' => tilde('e'),
        'ĩ' => tilde('i'),
        'õ' => tilde('o'),
        'ũ' => tilde('u'),
        'ạ' => below('a'),
        'ẹ' => below('e'),
        'ị' => below('i'),
        'ọ' => below('o'),
        'ụ' => below('u'),
        'ṣ' => below('s'),
        'ṅ' => Letter {
            above: true,
            ..Letter::plain('n')
        },
        c => Letter::plain(c),
    }
}

/// `word`'s letters in lower case with their marks, however they are encoded.
#[must_use]
pub fn letters(word: &str) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::with_capacity(word.len());
    for c in word.to_lowercase().chars() {
        match (c, letters.last_mut()) {
            // Tone accents and the diaeresis.
            ('\u{300}' | '\u{301}' | '\u{302}' | '\u{304}' | '\u{308}' | '\u{30C}', Some(_)) => {}
            ('\u{303}', Some(letter)) => letter.tilde = true,
            ('\u{323}' | '\u{329}', Some(letter)) => letter.below = true,
            ('\u{307}', Some(letter)) => letter.above = true,
            (c, _) => letters.push(decompose(c)),
        }
    }
    letters
}

/// `word` in lower case without its tone accents, its other marks written the same way however
/// they came (ọ́ and o with U+0323 and U+0301 are both ọ), for comparing words with a list.
#[must_use]
pub fn plain(word: &str) -> String {
    letters(word)
        .iter()
        .map(|letter| {
            let mut written = String::from(match (letter.base, letter.below, letter.above) {
                ('a', true, _) => 'ạ',
                ('e', true, _) => 'ẹ',
                ('i', true, _) => 'ị',
                ('o', true, _) => 'ọ',
                ('u', true, _) => 'ụ',
                ('s', true, _) => 'ṣ',
                ('n', _, true) => 'ṅ',
                (base, _, _) => base,
            });
            if letter.tilde {
                written.push('\u{303}');
            }
            written
        })
        .collect()
}

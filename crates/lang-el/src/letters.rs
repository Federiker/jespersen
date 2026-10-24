//! Greek letters without their marks: monotonic and polytonic accents become one stress mark,
//! breathings and the iota subscript are dropped, and the diaeresis is kept, since it splits a
//! digraph (κοροϊδεύω).

/// A letter in lower case, and whether it carries an accent or a diaeresis.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Letter {
    pub(crate) base: char,
    pub(crate) accent: bool,
    pub(crate) diaeresis: bool,
}

const fn letter(base: char, accent: bool, diaeresis: bool) -> Letter {
    Letter {
        base,
        accent,
        diaeresis,
    }
}

/// The vowels of the rows of 16 in Greek Extended (U+1F00 to U+1F6F, then U+1F80 to U+1FAF):
/// each row holds a vowel with its breathings and accents, lower case then capitals.
const ROWS: [char; 11] = ['α', 'ε', 'η', 'ι', 'ο', 'υ', 'ω', ' ', 'α', 'η', 'ω'];

/// A letter of Greek Extended without its marks; `None` outside the block.
fn extended(code: u32) -> Option<Letter> {
    let accented = |offset: u32| offset & 7 >= 2;
    Some(match code {
        0x1F00..=0x1F6F | 0x1F80..=0x1FAF => {
            let row = usize::try_from((code - 0x1F00) / 16).ok()?;
            letter(ROWS[row], accented(code & 15), false)
        }
        0x1F70..=0x1F7D => {
            let vowels = ['α', 'ε', 'η', 'ι', 'ο', 'υ', 'ω'];
            letter(vowels[usize::try_from((code - 0x1F70) / 2).ok()?], true, false)
        }
        0x1FB0..=0x1FBC => letter(
            'α',
            matches!(code, 0x1FB2 | 0x1FB4 | 0x1FB6 | 0x1FB7 | 0x1FBA | 0x1FBB),
            false,
        ),
        0x1FC2..=0x1FC9 if matches!(code, 0x1FC8 | 0x1FC9) => letter('ε', true, false),
        0x1FC2..=0x1FCC => letter('η', code != 0x1FC3 && code != 0x1FCC, false),
        0x1FD0..=0x1FDB => letter(
            'ι',
            matches!(code, 0x1FD2 | 0x1FD3 | 0x1FD6 | 0x1FD7 | 0x1FDA | 0x1FDB),
            matches!(code, 0x1FD2 | 0x1FD3 | 0x1FD7),
        ),
        0x1FE4 | 0x1FE5 | 0x1FEC => letter('ρ', false, false),
        0x1FE0..=0x1FEB => letter(
            'υ',
            matches!(code, 0x1FE2 | 0x1FE3 | 0x1FE6 | 0x1FE7 | 0x1FEA | 0x1FEB),
            matches!(code, 0x1FE2 | 0x1FE3 | 0x1FE7),
        ),
        0x1FF8 | 0x1FF9 => letter('ο', true, false),
        0x1FF2..=0x1FFC => letter('ω', code != 0x1FF3 && code != 0x1FFC, false),
        _ => return None,
    })
}

/// `c` in lower case without its marks.
fn fold(c: char) -> Letter {
    if let Some(folded) = extended(u32::from(c)) {
        return folded;
    }
    let (base, accent, diaeresis) = match c {
        'ά' => ('α', true, false),
        'έ' => ('ε', true, false),
        'ή' => ('η', true, false),
        'ί' => ('ι', true, false),
        'ό' => ('ο', true, false),
        'ύ' => ('υ', true, false),
        'ώ' => ('ω', true, false),
        'ϊ' => ('ι', false, true),
        'ϋ' => ('υ', false, true),
        'ΐ' => ('ι', true, true),
        'ΰ' => ('υ', true, true),
        'ς' => ('σ', false, false),
        c => (c, false, false),
    };
    Letter {
        base,
        accent,
        diaeresis,
    }
}

/// The letters of `word` in lower case without their marks; combining accents (U+0300, U+0301,
/// U+0342) and diaereses (U+0308) count for the letter before them, other combining marks are
/// dropped.
pub(crate) fn letters(word: &str) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::with_capacity(word.len());
    for c in word.chars().flat_map(char::to_lowercase) {
        match (c, letters.last_mut()) {
            ('\u{300}' | '\u{301}' | '\u{342}', Some(last)) => last.accent = true,
            ('\u{308}', Some(last)) => last.diaeresis = true,
            ('\u{300}'..='\u{36F}', _) => {}
            (c, _) => letters.push(fold(c)),
        }
    }
    letters
}

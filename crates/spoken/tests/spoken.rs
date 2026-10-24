//! The shared front end and reader with a toy language: digits as words, every word a syllable
//! of its letters.

use jespersen_phonemes::{Notation, Phoneme, Pronunciation, Stress, Syllable};
use jespersen_spoken::{Gender, Locale, Speaker, Spoken, read_tokens, spoken};
use jespersen_text::TokenKind;

struct Toy;

impl Locale for Toy {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        (number < 100).then(|| format!("n{number}"))
    }
    fn decimal_word(&self, _mark: char) -> &'static str {
        "point"
    }
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        Some(format!("d{day} m{month} y{year}"))
    }
    fn range(&self) -> &'static str {
        "to"
    }
    fn minus(&self) -> &'static str {
        "minus"
    }
    fn plus(&self) -> &'static str {
        "plus"
    }
    fn percent(&self) -> &'static str {
        "percent"
    }
    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("euro", "euros")),
            _ => None,
        }
    }
    fn symbol(&self, text: &str) -> Option<&'static str> {
        (text == "&").then_some("and")
    }
    fn letter(&self, letter: char) -> Option<&'static str> {
        letter.is_ascii_uppercase().then_some("letter")
    }
    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        (word == "Dr").then_some("doctor")
    }
    fn address_sign(&self, sign: char) -> Option<&'static str> {
        (sign == '@').then_some("at")
    }
}

const SOUND: Phoneme = Phoneme::vowel("a", "a");

struct Plain;

impl Speaker for Plain {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        (!word.contains('§')).then(|| Pronunciation {
            syllables: vec![Syllable {
                nucleus: vec![SOUND; word.chars().count()],
                stress: Stress::Primary,
                ..Syllable::default()
            }],
        })
    }
    fn unstressed(&self) -> &[&str] {
        &["the"]
    }
}

fn kinds(text: &str) -> Vec<(&str, TokenKind)> {
    text.split(' ')
        .map(|token| {
            let kind = match token.chars().next() {
                Some(c) if c.is_ascii_digit() || matches!(c, '-' | '+') && token.len() > 1 => TokenKind::Number,
                Some(c) if c.is_alphabetic() => TokenKind::Word,
                Some('.' | ',') => TokenKind::Punctuation,
                _ => TokenKind::Symbol,
            };
            (token, kind)
        })
        .collect()
}

fn words(text: &str) -> Vec<Spoken> {
    spoken(&kinds(text), &Toy)
}

#[test]
fn the_default_locale_reads_times_and_ratios_with_its_range_word() {
    assert_eq!(words("10:30"), [Spoken::Words(vec!["n10".into(), "n30".into()])]);
    assert_eq!(
        words("2/3"),
        [Spoken::Words(vec!["n2".into(), "to".into(), "n3".into()])]
    );
    assert_eq!(Toy.ordinal(3, Gender::Alone), None);
    assert_eq!(Toy.gender_of("x"), Gender::Masculine);
    assert_eq!(Toy.roman(4, None, None), None);
    assert!(!Toy.elides());
    // Without ordinal signs or ordinals, 3º is no number the toy reads.
    assert_eq!(words("3º"), [Spoken::Unreadable]);
    assert_eq!(words("123"), [Spoken::Unreadable]);
}

#[test]
fn signs_money_letters_and_addresses() {
    assert_eq!(
        words("-5 €"),
        [
            Spoken::Words(vec!["minus".into(), "n5".into(), "euros".into()]),
            Spoken::Silent
        ]
    );
    assert_eq!(
        words("€ 1"),
        [Spoken::Silent, Spoken::Words(vec!["n1".into(), "euro".into()])]
    );
    assert_eq!(
        words("+1 %"),
        [
            Spoken::Words(vec!["plus".into(), "n1".into(), "percent".into()]),
            Spoken::Silent
        ]
    );
    assert_eq!(words("DVD"), [Spoken::Spelt(vec!["letter".into(); 3])]);
    assert_eq!(words("Dr"), [Spoken::Words(vec!["doctor".into()])]);
    // The toy names upper-case letters only, so the b of this address cannot be spelt.
    assert_eq!(words("a@b"), [Spoken::Unreadable]);
    assert_eq!(
        words("a@B"),
        [Spoken::Words(vec!["a".into(), "at".into(), "letter".into()])]
    );
    assert_eq!(words("& ¤"), [Spoken::Words(vec!["and".into()]), Spoken::Unreadable]);
    assert_eq!(
        words("3,05 1.250 1,250 0039"),
        [
            Spoken::Words(vec!["n3".into(), "point".into(), "n0".into(), "n5".into()]),
            Spoken::Unreadable,
            Spoken::Unreadable,
            Spoken::Words(vec!["n0".into(), "n0".into(), "n3".into(), "n9".into()]),
        ]
    );
    assert_eq!(
        words("3/5/2024"),
        [Spoken::Words(vec!["d3".into(), "m5".into(), "y2024".into()])]
    );
}

#[test]
fn runs_lose_function_word_stress_and_unreadable_tokens_are_none() {
    let tokens = kinds("the cat , § the");
    let readings = read_tokens(&tokens, &Toy, &Plain);
    let written: Vec<Option<String>> = readings
        .iter()
        .map(|reading| {
            reading.as_ref().map(|reading| {
                reading
                    .words
                    .iter()
                    .map(|word| word.to_notation(Notation::Ipa))
                    .collect::<Vec<_>>()
                    .join(" ")
            })
        })
        .collect();
    // "the" is unstressed beside another word, stressed alone in its run.
    assert_eq!(
        written,
        [
            Some("aaa".into()),
            Some("ˈaaa".into()),
            Some(String::new()),
            None,
            Some("ˈaaa".into())
        ]
    );
}

#[test]
fn dates_with_points() {
    assert_eq!(
        words("3.5.2024"),
        [Spoken::Words(vec!["d3".into(), "m5".into(), "y2024".into()])]
    );
    // Not a date: 1.250 is a thousand two hundred and fifty, which the toy does not read.
    assert_eq!(words("1.250.000"), [Spoken::Unreadable]);
}

struct PercentFirst;

impl Locale for PercentFirst {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        Toy.cardinal(number, gender)
    }
    fn decimal_word(&self, mark: char) -> &'static str {
        Toy.decimal_word(mark)
    }
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        Toy.date(day, month, year)
    }
    fn range(&self) -> &'static str {
        ""
    }
    fn minus(&self) -> &'static str {
        "minus"
    }
    fn plus(&self) -> &'static str {
        "plus"
    }
    fn percent(&self) -> &'static str {
        "percent"
    }
    fn percent_first(&self) -> bool {
        true
    }
    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        Toy.currency(sign)
    }
    /// A third form for counts ending in 2 to 4, as Polish has.
    fn currency_name(&self, sign: &str, count: Option<u64>) -> Option<&'static str> {
        let (one, many) = self.currency(sign)?;
        Some(match count {
            Some(1) => one,
            Some(count) if (2..=4).contains(&(count % 10)) => "few",
            _ => many,
        })
    }
    fn symbol(&self, text: &str) -> Option<&'static str> {
        Toy.symbol(text)
    }
    fn letter(&self, letter: char) -> Option<&'static str> {
        Toy.letter(letter)
    }
    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        Toy.abbreviation(word)
    }
    fn address_sign(&self, sign: char) -> Option<&'static str> {
        Toy.address_sign(sign)
    }
}

#[test]
fn a_percent_sign_before_its_number_and_ranges_without_a_word() {
    let tokens = [("%", TokenKind::Symbol), ("50", TokenKind::Number)];
    assert_eq!(
        spoken(&tokens, &PercentFirst),
        [Spoken::Words(vec!["percent".into()]), Spoken::Words(vec!["n50".into()])]
    );
    assert_eq!(
        spoken(&[("10-20", TokenKind::Number)], &PercentFirst),
        [Spoken::Words(vec!["n10".into(), "n20".into()])]
    );
}

#[test]
fn a_currency_in_letters_before_its_number() {
    let tokens = [("EUR", TokenKind::Word), ("5", TokenKind::Number)];
    assert_eq!(
        spoken(&tokens, &Toy),
        [Spoken::Silent, Spoken::Words(vec!["n5".into(), "euros".into()])]
    );
}

#[test]
fn the_count_picks_the_currency_name() {
    let read = |number| spoken(&[(number, TokenKind::Number), ("€", TokenKind::Symbol)], &PercentFirst);
    assert_eq!(
        read("1"),
        [Spoken::Words(vec!["n1".into(), "euro".into()]), Spoken::Silent]
    );
    assert_eq!(
        read("3"),
        [Spoken::Words(vec!["n3".into(), "few".into()]), Spoken::Silent]
    );
    assert_eq!(read("22")[0], Spoken::Words(vec!["n22".into(), "few".into()]));
    assert_eq!(
        read("5,5")[0],
        Spoken::Words(vec!["n5".into(), "point".into(), "n5".into(), "euros".into()])
    );
}

/// A speaker whose "on" is said with the word after it.
struct Proclitic;

impl Speaker for Proclitic {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        Plain.say(word)
    }
    fn proclitics(&self) -> &[&str] {
        &["on"]
    }
}

#[test]
fn a_proclitic_is_said_with_the_next_word_and_takes_its_stress() {
    let tokens = kinds("on cat on");
    let written: Vec<Vec<String>> = read_tokens(&tokens, &Toy, &Proclitic)
        .into_iter()
        .map(|reading| {
            reading
                .unwrap()
                .words
                .iter()
                .map(|word| word.to_notation(Notation::Ipa))
                .collect()
        })
        .collect();
    // The last "on" has no word after it and stays on its own.
    assert_eq!(written, [vec![], vec!["ˈaaaaa".to_owned()], vec!["ˈaa".to_owned()]]);
}

/// A speaker whose "on" joins the next word without taking its stress.
struct Unstressing;

impl Speaker for Unstressing {
    fn say(&self, word: &str) -> Option<Pronunciation> {
        Plain.say(word)
    }
    fn proclitics(&self) -> &[&str] {
        &["on"]
    }
    fn proclitic_takes_stress(&self) -> bool {
        false
    }
}

#[test]
fn a_proclitic_can_leave_the_stress_where_it_was() {
    let readings = read_tokens(&kinds("on cat"), &Toy, &Unstressing);
    let joined = readings[1].as_ref().unwrap().words[0].to_notation(Notation::Ipa);
    assert_eq!(joined, "aaˈaaa");
}

#[test]
fn letters_with_marks_are_read_however_they_are_encoded() {
    use jespersen_spoken::marks::{Letter, letters, plain};
    let dotted = Letter {
        base: 'o',
        below: true,
        above: false,
        tilde: false,
    };
    // ọ́ precomposed with a combining acute, and as o with a dot and an acute.
    assert_eq!(letters("\u{1ECD}\u{301}"), [dotted]);
    assert_eq!(letters("O\u{323}\u{301}"), [dotted]);
    assert_eq!(letters("o\u{329}"), [dotted]);
    assert!(letters("\u{1E45}")[0].above);
    assert!(letters("n\u{307}")[0].above);
    assert!(letters("\u{E3}")[0].tilde && letters("\u{25B}\u{303}")[0].tilde);
    let bases: String = letters("E\u{323}\u{300}gbo\u{323}\u{301}n")
        .iter()
        .map(|letter| letter.base)
        .collect();
    assert_eq!(bases, "egbon");
    let mark = Letter {
        base: '\u{301}',
        below: false,
        above: false,
        tilde: false,
    };
    assert_eq!(letters("\u{301}"), [mark]);
    for (written, expected) in [
        ("w\u{1ECD}\u{301}n", "w\u{1ECD}n"),
        ("wo\u{323}\u{301}n", "w\u{1ECD}n"),
        ("\u{C0}TI", "ati"),
        ("\u{1E63}\u{E9}", "\u{1E63}e"),
        ("\u{1ECB}\u{301}\u{1EE5}\u{300}\u{1EA1}", "\u{1ECB}\u{1EE5}\u{1EA1}"),
        ("n\u{307}", "\u{1E45}"),
        ("\u{25B}\u{303}", "\u{25B}\u{303}"),
        ("\u{1F9} \u{144} \u{1E3F} \u{148}", "n n m n"),
        (
            "\u{E2}\u{EA}\u{EE}\u{F4}\u{FB}\u{1CE}\u{11B}\u{1D0}\u{1D2}\u{1D4}\u{101}\u{113}\u{12B}\u{14D}\u{16B}\u{E4}\u{EB}\u{EF}\u{F6}\u{FC}",
            "aeiouaeiouaeiouaeiou",
        ),
        (
            "\u{E1}\u{E8}\u{ED}\u{F3}\u{FA}\u{E0}\u{E9}\u{EC}\u{F2}\u{F9}",
            "aeiouaeiou",
        ),
        ("\u{1EBD}\u{129}\u{F5}\u{169}", "e\u{303}i\u{303}o\u{303}u\u{303}"),
    ] {
        assert_eq!(plain(written), expected, "{written}");
    }
}

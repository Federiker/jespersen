/// How a number agrees with what it counts (Spanish veintiún años, veintiuna casas; Italian un
/// anno, una casa).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Gender {
    /// Counting on its own (uno, dos, tres).
    #[default]
    Alone,
    Masculine,
    Feminine,
}

/// A language's words for what is not written in words: numbers, dates, signs, letters and
/// abbreviations. The front end ([`crate::spoken`]) finds them in the text and asks the language
/// how to say them.
pub trait Locale {
    /// A whole number in words, agreeing with `gender`; `None` beyond what the language reads.
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String>;

    /// An ordinal in words (primero, dritte), `None` when the language reads it as a cardinal.
    fn ordinal(&self, _number: u64, _gender: Gender) -> Option<String> {
        None
    }

    /// The signs that make a number an ordinal (º, ª, °, er; 3.º), and the gender each gives.
    fn ordinal_signs(&self) -> &'static [(&'static str, Gender)] {
        &[]
    }

    /// The word for a decimal mark: `,` or `.`.
    fn decimal_word(&self, mark: char) -> &'static str;

    /// A time of day (10:30).
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        Some(format!(
            "{} {}",
            self.cardinal(hours, Gender::Alone)?,
            self.cardinal(minutes, Gender::Alone)?
        ))
    }

    /// A date from its day, month (1 to 12) and year (3/5/2024, 2024-05-03).
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String>;

    /// The word between two numbers of a range (1990-1995), and of a ratio written with a slash.
    fn range(&self) -> &'static str;
    fn ratio(&self) -> &'static str {
        self.range()
    }

    /// The words for a minus and a plus sign before a number.
    fn minus(&self) -> &'static str;
    fn plus(&self) -> &'static str;

    /// The words for a percentage after its number.
    fn percent(&self) -> &'static str;

    /// Whether the percent sign and its words come before the number (Turkish %50 yüzde elli).
    fn percent_first(&self) -> bool {
        false
    }

    /// A currency sign's name in the singular and the plural (€: euro, euros).
    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)>;

    /// A currency's name after `count` of it (`None` for a fraction): the singular for one and the
    /// plural otherwise. Languages with more forms say which (Polish 2 złote, 5 złotych).
    fn currency_name(&self, sign: &str, count: Option<u64>) -> Option<&'static str> {
        let (one, many) = self.currency(sign)?;
        Some(if count == Some(1) { one } else { many })
    }

    /// Symbols said as words (&, @, +).
    fn symbol(&self, text: &str) -> Option<&'static str>;

    /// The name of a letter, for initialisms said letter by letter.
    fn letter(&self, letter: char) -> Option<&'static str>;

    /// How an abbreviation (without its full stop) is said, `None` for a word that is not one.
    fn abbreviation(&self, word: &str) -> Option<&'static str>;

    /// The signs of a web or e-mail address said as words (@, ., /, :, -, _).
    fn address_sign(&self, sign: char) -> Option<&'static str>;

    /// The gender of the word a number counts.
    fn gender_of(&self, _word: &str) -> Gender {
        Gender::Masculine
    }

    /// A Roman numeral's reading between the words before and after it (el siglo XX, Luigi XIV),
    /// `None` when it is not read as a number there.
    fn roman(&self, _value: u64, _before: Option<&str>, _after: Option<&str>) -> Option<String> {
        None
    }

    /// Whether the language elides words with an apostrophe (l'amico, dell'acqua), read with the
    /// word after them.
    fn elides(&self) -> bool {
        false
    }

    /// The language's vowel letters in lower case, for telling initialisms that are said as words
    /// (NATO) from those spelt out (DVD).
    fn vowels(&self) -> &'static str {
        "aeiouyáéíóúàèìòùâêîôûäëïöüåæøõ"
    }
}

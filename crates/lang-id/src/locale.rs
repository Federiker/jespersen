use jespersen_spoken::{Gender, Locale};

const UNITS: [&str; 10] = [
    "nol", "satu", "dua", "tiga", "empat", "lima", "enam", "tujuh", "delapan", "sembilan",
];

const MONTHS: [&str; 12] = [
    "Januari",
    "Februari",
    "Maret",
    "April",
    "Mei",
    "Juni",
    "Juli",
    "Agustus",
    "September",
    "Oktober",
    "November",
    "Desember",
];

/// Indonesian abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 14] = [
    ("dll", "dan lain-lain"),
    ("dsb", "dan sebagainya"),
    ("dst", "dan seterusnya"),
    ("tsb", "tersebut"),
    ("jl", "jalan"),
    ("no", "nomor"),
    ("bpk", "bapak"),
    ("dr", "dokter"),
    ("prof", "profesor"),
    ("sdr", "saudara"),
    ("hlm", "halaman"),
    ("tgl", "tanggal"),
    ("yth", "yang terhormat"),
    ("kab", "kabupaten"),
];

fn expansion(word: &str) -> Option<&'static str> {
    let lower = word.to_lowercase();
    ABBREVIATIONS
        .iter()
        .find(|(abbreviation, _)| *abbreviation == lower)
        .map(|(_, said)| *said)
}

/// Whether `word` (without its full stop) is an abbreviation that does not end a sentence.
#[must_use]
pub fn is_abbreviation(word: &str) -> bool {
    expansion(word).is_some()
}

/// `number` in Indonesian words: 21 dua puluh satu, 1250 seribu dua ratus lima puluh; one hundred,
/// one thousand and one million take se- (seratus, seribu, sejuta).
pub(crate) fn cardinal(number: u64) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, name, one) in [
        (1_000_000_000, "miliar", "satu miliar"),
        (1_000_000, "juta", "sejuta"),
        (1000, "ribu", "seribu"),
    ] {
        let count = number / scale % 1000;
        if count == 1 {
            parts.push(one.to_owned());
        } else if count > 1 {
            parts.push(format!("{} {name}", below_thousand(count)));
        }
    }
    if !number.is_multiple_of(1000) {
        parts.push(below_thousand(number % 1000));
    }
    parts.join(" ")
}

fn below_thousand(number: u64) -> String {
    let digit = |value: u64| UNITS[usize::try_from(value % 10).unwrap_or(0)];
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    match hundreds {
        0 => {}
        1 => parts.push("seratus".to_owned()),
        hundreds => parts.push(format!("{} ratus", digit(hundreds))),
    }
    match (rest / 10, rest % 10) {
        (0, 0) => {}
        (0, units) => parts.push(digit(units).to_owned()),
        (1, 0) => parts.push("sepuluh".to_owned()),
        (1, 1) => parts.push("sebelas".to_owned()),
        (1, units) => parts.push(format!("{} belas", digit(units))),
        (tens, 0) => parts.push(format!("{} puluh", digit(tens))),
        (tens, units) => parts.push(format!("{} puluh {}", digit(tens), digit(units))),
    }
    parts.join(" ")
}

/// Indonesian words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Indonesian;

impl Locale for Indonesian {
    fn cardinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(cardinal(number))
    }

    /// An ordinal: ke- and the cardinal (ketiga); the first is pertama.
    fn ordinal(&self, number: u64, _gender: Gender) -> Option<String> {
        Some(if number == 1 {
            "pertama".to_owned()
        } else {
            format!("ke{}", cardinal(number))
        })
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "koma" } else { "titik" }
    }

    /// tiga Mei dua ribu dua puluh empat.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        Some(format!("{} {month} {}", cardinal(day), cardinal(year)))
    }

    fn range(&self) -> &'static str {
        "sampai"
    }

    fn ratio(&self) -> &'static str {
        "per"
    }

    fn minus(&self) -> &'static str {
        "minus"
    }

    fn plus(&self) -> &'static str {
        "plus"
    }

    fn percent(&self) -> &'static str {
        "persen"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "Rp" => Some(("rupiah", "rupiah")),
            "€" => Some(("euro", "euro")),
            "$" => Some(("dolar", "dolar")),
            "£" => Some(("pound", "pound")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("persen"),
            "&" => Some("dan"),
            "@" => Some("at"),
            "+" => Some("tambah"),
            "=" => Some("sama dengan"),
            "°" => Some("derajat"),
            "#" => Some("pagar"),
            "€" => Some("euro"),
            "$" => Some("dolar"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letter.to_lowercase().next()? {
            'a' => "a",
            'b' => "be",
            'c' => "ce",
            'd' => "de",
            'e' => "e",
            'f' => "ef",
            'g' => "ge",
            'h' => "ha",
            'i' => "i",
            'j' => "je",
            'k' => "ka",
            'l' => "el",
            'm' => "em",
            'n' => "en",
            'o' => "o",
            'p' => "pe",
            'q' => "ki",
            'r' => "er",
            's' => "es",
            't' => "te",
            'u' => "u",
            'v' => "fe",
            'w' => "we",
            'x' => "eks",
            'y' => "ye",
            'z' => "zet",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("at"),
            '.' => Some("titik"),
            '/' => Some("garis miring"),
            ':' => Some("titik dua"),
            '-' => Some("strip"),
            '_' => Some("garis bawah"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiou"
    }
}

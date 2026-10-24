use jespersen_spoken::{Gender, Locale};

use crate::{
    Variety,
    numbers::{cardinal, ordinal},
};

const MONTHS: [&str; 12] = [
    "janeiro",
    "fevereiro",
    "março",
    "abril",
    "maio",
    "junho",
    "julho",
    "agosto",
    "setembro",
    "outubro",
    "novembro",
    "dezembro",
];

/// Portuguese abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 22] = [
    ("sr", "senhor"),
    ("sra", "senhora"),
    ("srs", "senhores"),
    ("dr", "doutor"),
    ("dra", "doutora"),
    ("prof", "professor"),
    ("profa", "professora"),
    ("eng", "engenheiro"),
    ("av", "avenida"),
    ("pç", "praça"),
    ("etc", "etcétera"),
    ("pág", "página"),
    ("págs", "páginas"),
    ("tel", "telefone"),
    ("ex", "exemplo"),
    ("obs", "observação"),
    ("séc", "século"),
    ("vol", "volume"),
    ("cap", "capítulo"),
    ("aprox", "aproximadamente"),
    ("nº", "número"),
    ("n", "número"),
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
    expansion(word).is_some_and(|_| word.chars().count() > 1)
}

/// Words ending in -a that are masculine (o dia, o problema).
const MASCULINE_IN_A: [&str; 14] = [
    "dia",
    "dias",
    "problema",
    "problemas",
    "sistema",
    "sistemas",
    "tema",
    "temas",
    "mapa",
    "mapas",
    "programa",
    "programas",
    "clima",
    "planeta",
];

/// Portuguese words for numbers, dates, signs, letters and abbreviations, in `variety`.
pub(crate) struct Portuguese {
    pub(crate) variety: Variety,
}

impl Locale for Portuguese {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        Some(cardinal(number, gender == Gender::Feminine, self.variety))
    }

    fn ordinal(&self, number: u64, gender: Gender) -> Option<String> {
        ordinal(number, gender == Gender::Feminine)
    }

    fn ordinal_signs(&self) -> &'static [(&'static str, Gender)] {
        &[
            ("º", Gender::Masculine),
            ("ª", Gender::Feminine),
            ("°", Gender::Masculine),
        ]
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "vírgula" } else { "ponto" }
    }

    /// dez e trinta, oito e cinco; the full hour is dez horas.
    fn time(&self, hours: u64, minutes: u64) -> Option<String> {
        let hour = cardinal(hours, true, self.variety);
        Some(match minutes {
            0 if hours == 1 => "uma hora".to_owned(),
            0 => format!("{hour} horas"),
            _ => format!("{hour} e {}", cardinal(minutes, false, self.variety)),
        })
    }

    /// três de maio de dois mil e vinte e quatro; the first of the month is primeiro in Brazil.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        let day = if day == 1 && self.variety == Variety::Brazilian {
            "primeiro".to_owned()
        } else {
            cardinal(day, false, self.variety)
        };
        Some(format!("{day} de {month} de {}", cardinal(year, false, self.variety)))
    }

    fn range(&self) -> &'static str {
        "a"
    }

    fn ratio(&self) -> &'static str {
        "sobre"
    }

    fn minus(&self) -> &'static str {
        "menos"
    }

    fn plus(&self) -> &'static str {
        "mais"
    }

    fn percent(&self) -> &'static str {
        "por cento"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" | "EUR" => Some(("euro", "euros")),
            "$" | "USD" => Some(("dólar", "dólares")),
            "R$" | "BRL" => Some(("real", "reais")),
            "£" | "GBP" => Some(("libra", "libras")),
            _ => None,
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("por cento"),
            "&" => Some("e"),
            "@" => Some("arroba"),
            "+" => Some("mais"),
            "=" => Some("igual a"),
            "°" => Some("graus"),
            "#" => Some("cardinal"),
            "€" => Some("euros"),
            "$" => Some("dólares"),
            _ => None,
        }
    }

    fn letter(&self, letter: char) -> Option<&'static str> {
        let brazilian = self.variety == Variety::Brazilian;
        Some(match letter.to_lowercase().next()? {
            'a' => "á",
            'b' => "bê",
            'c' => "cê",
            'd' => "dê",
            'e' => "é",
            'f' => "efe",
            'g' if brazilian => "gê",
            'g' => "guê",
            'h' => "agá",
            'i' => "i",
            'j' => "jota",
            'k' if brazilian => "cá",
            'k' => "capa",
            'l' => "ele",
            'm' => "eme",
            'n' => "ene",
            'o' => "ó",
            'p' => "pê",
            'q' => "quê",
            'r' => "erre",
            's' => "esse",
            't' => "tê",
            'u' => "u",
            'v' => "vê",
            'w' => "dâblio",
            'x' => "xis",
            'y' => "ípsilon",
            'z' => "zê",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        // A single letter is an abbreviation only before its sign (n.º).
        if word.chars().count() < 2 {
            return None;
        }
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("arroba"),
            '.' => Some("ponto"),
            '/' => Some("barra"),
            ':' => Some("dois pontos"),
            '-' => Some("hífen"),
            '_' => Some("sublinhado"),
            _ => None,
        }
    }

    /// Feminine: words in -a, -ção, -dade and -agem and their plurals, but o dia, o problema.
    fn gender_of(&self, word: &str) -> Gender {
        let lower = word.to_lowercase();
        let singular = lower.strip_suffix('s').unwrap_or(&lower);
        if MASCULINE_IN_A.contains(&lower.as_str()) {
            Gender::Masculine
        } else if singular.ends_with('a')
            || lower.ends_with("ção")
            || lower.ends_with("ções")
            || singular.ends_with("dade")
            || singular.ends_with("agem")
            || lower.ends_with("agens")
        {
            Gender::Feminine
        } else {
            Gender::Masculine
        }
    }

    /// A Roman numeral after século is a number (século XX vinte); after a name it is an ordinal
    /// up to ten and a number beyond (D. João VI sexto, Luís XIV catorze).
    fn roman(&self, value: u64, before: Option<&str>, _after: Option<&str>) -> Option<String> {
        let before = before?;
        if before.to_lowercase().starts_with("séc") {
            return Some(cardinal(value, false, self.variety));
        }
        if !before.chars().next().is_some_and(char::is_uppercase) {
            return None;
        }
        if value <= 10 {
            ordinal(value, false)
        } else {
            Some(cardinal(value, false, self.variety))
        }
    }

    fn vowels(&self) -> &'static str {
        "aeiouáéíóúâêôãõà"
    }
}

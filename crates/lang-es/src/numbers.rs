//! Numbers in Spanish words: 21 veintiuno, 101 ciento uno, 1999 mil novecientos noventa y nueve.

pub(crate) use jespersen_spoken::Gender;

const UNITS: [&str; 30] = [
    "cero",
    "uno",
    "dos",
    "tres",
    "cuatro",
    "cinco",
    "seis",
    "siete",
    "ocho",
    "nueve",
    "diez",
    "once",
    "doce",
    "trece",
    "catorce",
    "quince",
    "dieciséis",
    "diecisiete",
    "dieciocho",
    "diecinueve",
    "veinte",
    "veintiuno",
    "veintidós",
    "veintitrés",
    "veinticuatro",
    "veinticinco",
    "veintiséis",
    "veintisiete",
    "veintiocho",
    "veintinueve",
];
const TENS: [&str; 10] = [
    "",
    "",
    "",
    "treinta",
    "cuarenta",
    "cincuenta",
    "sesenta",
    "setenta",
    "ochenta",
    "noventa",
];
const HUNDREDS: [&str; 10] = [
    "",
    "ciento",
    "doscientos",
    "trescientos",
    "cuatrocientos",
    "quinientos",
    "seiscientos",
    "setecientos",
    "ochocientos",
    "novecientos",
];

/// `number` in words, as Spanish says it before what it counts.
pub(crate) fn cardinal(number: u64, gender: Gender) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    let millions = number / 1_000_000;
    let thousands = number / 1000 % 1000;
    let rest = number % 1000;
    if millions > 0 {
        // Millions are masculine whatever is counted: un millón, veintiún millones.
        if millions == 1 {
            parts.push("un millón".to_owned());
        } else {
            parts.push(format!("{} millones", cardinal(millions, Gender::Masculine)));
        }
    }
    if thousands > 0 {
        // mil, not un mil; before mil the agreement is the counted noun's (veintiún mil años,
        // veintiuna mil casas is said too, but veintiún mil is the norm).
        if thousands > 1 {
            let gender = if gender == Gender::Feminine {
                Gender::Feminine
            } else {
                Gender::Masculine
            };
            parts.push(below_thousand(thousands, gender));
        }
        parts.push("mil".to_owned());
    }
    if rest > 0 {
        parts.push(below_thousand(rest, gender));
    }
    parts.join(" ")
}

fn below_thousand(number: u64, gender: Gender) -> String {
    let (hundreds, rest) = (number / 100, number % 100);
    let mut parts: Vec<String> = Vec::new();
    if hundreds > 0 {
        let word = if number == 100 {
            "cien"
        } else {
            HUNDREDS[usize::try_from(hundreds).unwrap_or(0)]
        };
        parts.push(if gender == Gender::Feminine && hundreds > 1 {
            word.replace("ientos", "ientas")
        } else {
            word.to_owned()
        });
    }
    if rest > 0 {
        let index = usize::try_from(rest).unwrap_or(0);
        let words = if rest < 30 {
            UNITS[index].to_owned()
        } else if rest % 10 == 0 {
            TENS[index / 10].to_owned()
        } else {
            format!("{} y {}", TENS[index / 10], UNITS[index % 10])
        };
        parts.push(agree(&words, gender));
    }
    parts.join(" ")
}

/// A number's last word made to agree: uno → un or una, veintiuno → veintiún or veintiuna.
fn agree(words: &str, gender: Gender) -> String {
    let (ending, masculine, feminine) = if words.ends_with("veintiuno") {
        ("veintiuno", "veintiún", "veintiuna")
    } else if words.ends_with("uno") {
        ("uno", "un", "una")
    } else {
        return words.to_owned();
    };
    let stem = &words[..words.len() - ending.len()];
    match gender {
        Gender::Alone => words.to_owned(),
        Gender::Masculine => format!("{stem}{masculine}"),
        Gender::Feminine => format!("{stem}{feminine}"),
    }
}

const ORDINALS: [&str; 11] = [
    "", "primero", "segundo", "tercero", "cuarto", "quinto", "sexto", "séptimo", "octavo", "noveno", "décimo",
];

/// An ordinal up to ten (1.º primero, 3.ª tercera; primer and tercer before a masculine noun);
/// beyond ten Spanish uses the cardinal (el siglo veinte), which `None` asks for.
pub(crate) fn ordinal(number: u64, gender: Gender) -> Option<String> {
    let word = ORDINALS
        .get(usize::try_from(number).ok()?)
        .filter(|word| !word.is_empty())?;
    Some(match gender {
        Gender::Feminine => format!("{}a", &word[..word.len() - 1]),
        Gender::Masculine if matches!(number, 1 | 3) => word[..word.len() - 1].to_owned(),
        Gender::Masculine | Gender::Alone => (*word).to_owned(),
    })
}

use jespersen_spoken::{Gender, Locale};

use crate::letters::letters;

/// 0 to 19 counting on their own (neuter); 1, 3, 4, 13 and 14 change with the gender.
const UNITS: [&str; 20] = [
    "μηδέν",
    "ένα",
    "δύο",
    "τρία",
    "τέσσερα",
    "πέντε",
    "έξι",
    "επτά",
    "οκτώ",
    "εννέα",
    "δέκα",
    "έντεκα",
    "δώδεκα",
    "δεκατρία",
    "δεκατέσσερα",
    "δεκαπέντε",
    "δεκαέξι",
    "δεκαεπτά",
    "δεκαοκτώ",
    "δεκαεννέα",
];

const TENS: [&str; 10] = [
    "",
    "δέκα",
    "είκοσι",
    "τριάντα",
    "σαράντα",
    "πενήντα",
    "εξήντα",
    "εβδομήντα",
    "ογδόντα",
    "ενενήντα",
];

/// The hundreds from 200 without their ending, which agrees with the gender (διακόσια, διακόσιοι,
/// διακόσιες).
const HUNDREDS: [&str; 10] = [
    "",
    "",
    "διακόσι",
    "τριακόσι",
    "τετρακόσι",
    "πεντακόσι",
    "εξακόσι",
    "επτακόσι",
    "οκτακόσι",
    "εννιακόσι",
];

/// The months in the genitive, as dates say them (τρεις Μαΐου).
const MONTHS: [&str; 12] = [
    "Ιανουαρίου",
    "Φεβρουαρίου",
    "Μαρτίου",
    "Απριλίου",
    "Μαΐου",
    "Ιουνίου",
    "Ιουλίου",
    "Αυγούστου",
    "Σεπτεμβρίου",
    "Οκτωβρίου",
    "Νοεμβρίου",
    "Δεκεμβρίου",
];

/// Greek abbreviations with a full stop and how they are said.
const ABBREVIATIONS: [(&str, &str); 15] = [
    ("κ", "κύριος"),
    ("κα", "κυρία"),
    ("δηλ", "δηλαδή"),
    ("κλπ", "και λοιπά"),
    ("κτλ", "και τα λοιπά"),
    ("π.χ", "παραδείγματος χάριν"),
    ("μ.χ", "μετά Χριστόν"),
    ("αρ", "αριθμός"),
    ("οδ", "οδός"),
    ("δρ", "δόκτωρ"),
    ("καθ", "καθηγητής"),
    ("σελ", "σελίδα"),
    ("τηλ", "τηλέφωνο"),
    ("βλ", "βλέπε"),
    ("χλμ", "χιλιόμετρα"),
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

/// A number below 20 agreeing with `gender`: ένας, μία, ένα; τρεις, τρία; τέσσερις, τέσσερα.
fn small(number: u64, gender: Gender) -> &'static str {
    let neuter = UNITS[usize::try_from(number).unwrap_or(0)];
    match (number, gender) {
        (1, Gender::Masculine) => "ένας",
        (1, Gender::Feminine) => "μία",
        (3, Gender::Masculine | Gender::Feminine) => "τρεις",
        (4, Gender::Masculine | Gender::Feminine) => "τέσσερις",
        (13, Gender::Masculine | Gender::Feminine) => "δεκατρείς",
        (14, Gender::Masculine | Gender::Feminine) => "δεκατέσσερις",
        _ => neuter,
    }
}

/// The ending of the hundreds and of χίλι- for `gender`.
fn ending(gender: Gender) -> &'static str {
    match gender {
        Gender::Masculine => "οι",
        Gender::Feminine => "ες",
        Gender::Alone => "α",
    }
}

fn below_thousand(number: u64, gender: Gender) -> String {
    let mut parts: Vec<String> = Vec::new();
    let (hundreds, rest) = (number / 100, number % 100);
    match hundreds {
        0 => {}
        // εκατό alone, εκατόν before more (εκατόν είκοσι).
        1 => parts.push(if rest == 0 { "εκατό" } else { "εκατόν" }.to_owned()),
        hundreds => parts.push(format!(
            "{}{}",
            HUNDREDS[usize::try_from(hundreds).unwrap_or(0)],
            ending(gender)
        )),
    }
    if rest >= 20 {
        parts.push(TENS[usize::try_from(rest / 10).unwrap_or(0)].to_owned());
        if !rest.is_multiple_of(10) {
            parts.push(small(rest % 10, gender).to_owned());
        }
    } else if rest > 0 {
        parts.push(small(rest, gender).to_owned());
    }
    parts.join(" ")
}

/// `number` in Greek words agreeing with `gender`: thousands are feminine (τρεις χιλιάδες,
/// διακόσιες χιλιάδες), millions and billions neuter (δύο εκατομμύρια).
pub(crate) fn cardinal(number: u64, gender: Gender) -> String {
    if number == 0 {
        return UNITS[0].to_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for (scale, one, many) in [
        (1_000_000_000, "ένα δισεκατομμύριο", "δισεκατομμύρια"),
        (1_000_000, "ένα εκατομμύριο", "εκατομμύρια"),
    ] {
        match number / scale % 1000 {
            0 => {}
            1 => parts.push(one.to_owned()),
            count => parts.push(format!("{} {many}", below_thousand(count, Gender::Alone))),
        }
    }
    match number / 1000 % 1000 {
        0 => {}
        1 => parts.push(format!("χίλι{}", ending(gender))),
        count => parts.push(format!("{} χιλιάδες", below_thousand(count, Gender::Feminine))),
    }
    if !number.is_multiple_of(1000) {
        parts.push(below_thousand(number % 1000, gender));
    }
    parts.join(" ")
}

/// Greek words for numbers, dates, signs, letters and abbreviations.
pub(crate) struct Greek;

impl Locale for Greek {
    fn cardinal(&self, number: u64, gender: Gender) -> Option<String> {
        Some(cardinal(number, gender))
    }

    fn decimal_word(&self, mark: char) -> &'static str {
        if mark == ',' { "κόμμα" } else { "τελεία" }
    }

    /// τρεις Μαΐου δύο χιλιάδες είκοσι τέσσερα; the first of the month is πρώτη.
    fn date(&self, day: u64, month: usize, year: u64) -> Option<String> {
        let month = MONTHS.get(month.checked_sub(1)?)?;
        let day = if day == 1 {
            "πρώτη".to_owned()
        } else {
            cardinal(day, Gender::Feminine)
        };
        Some(format!("{day} {month} {}", cardinal(year, Gender::Alone)))
    }

    fn range(&self) -> &'static str {
        "έως"
    }

    fn ratio(&self) -> &'static str {
        "προς"
    }

    fn minus(&self) -> &'static str {
        "μείον"
    }

    fn plus(&self) -> &'static str {
        "συν"
    }

    fn percent(&self) -> &'static str {
        "τοις εκατό"
    }

    fn currency(&self, sign: &str) -> Option<(&'static str, &'static str)> {
        match sign {
            "€" => Some(("ευρώ", "ευρώ")),
            "$" => Some(("δολάριο", "δολάρια")),
            "£" => Some(("λίρα", "λίρες")),
            _ => None,
        }
    }

    /// A noun's gender from its ending, for the number before it: -ος, -ης, -ας, -οι, -ους
    /// masculine; -α, -η, -ες, -ις feminine; -ο, -ι, -μα, -ματα and a stressed -ά (παιδιά)
    /// neuter.
    fn gender_of(&self, word: &str) -> Gender {
        // The letters come folded, with final ς as σ.
        let folded = letters(word);
        let plain: String = folded.iter().map(|letter| letter.base).collect();
        let stressed_last = folded.last().is_some_and(|letter| letter.accent);
        if ["οσ", "ησ", "ασ", "οι", "ουσ", "εισ"]
            .iter()
            .any(|end| plain.ends_with(end))
        {
            Gender::Masculine
        } else if plain.ends_with("μα") || plain.ends_with("ματα") || (plain.ends_with('α') && stressed_last) {
            Gender::Alone
        } else if ["α", "η", "εσ", "ισ"].iter().any(|end| plain.ends_with(end)) {
            Gender::Feminine
        } else {
            Gender::Alone
        }
    }

    fn symbol(&self, text: &str) -> Option<&'static str> {
        match text {
            "%" => Some("τοις εκατό"),
            "&" => Some("και"),
            "@" => Some("παπάκι"),
            "+" => Some("συν"),
            "=" => Some("ίσον"),
            "°" => Some("βαθμοί"),
            "#" => Some("δίεση"),
            "€" => Some("ευρώ"),
            "$" => Some("δολάριο"),
            _ => None,
        }
    }

    /// The Greek letters' names, and the Latin ones as Greeks say them (DVD ντι βι ντι).
    fn letter(&self, letter: char) -> Option<&'static str> {
        Some(match letters(&letter.to_string()).first()?.base {
            'α' => "άλφα",
            'β' => "βήτα",
            'γ' => "γάμμα",
            'δ' => "δέλτα",
            'ε' => "έψιλον",
            'ζ' => "ζήτα",
            'η' => "ήτα",
            'θ' => "θήτα",
            'ι' => "γιώτα",
            'κ' => "κάπα",
            'λ' => "λάμδα",
            'μ' => "μι",
            'ν' => "νι",
            'ξ' => "ξι",
            'ο' => "όμικρον",
            'π' | 'p' => "πι",
            'ρ' => "ρο",
            'σ' => "σίγμα",
            'τ' => "ταυ",
            'υ' => "ύψιλον",
            'φ' => "φι",
            'χ' => "χι",
            'ψ' => "ψι",
            'ω' => "ωμέγα",
            'a' => "έι",
            'b' => "μπι",
            'c' => "σι",
            'd' => "ντι",
            'e' => "ι",
            'f' => "εφ",
            'g' => "τζι",
            'h' => "έιτς",
            'i' => "άι",
            'j' => "τζέι",
            'k' => "κέι",
            'l' => "ελ",
            'm' => "εμ",
            'n' => "εν",
            'o' => "όου",
            'q' => "κιου",
            'r' => "αρ",
            's' => "ες",
            't' => "τι",
            'u' => "γιου",
            'v' => "βι",
            'w' => "ντάμπλγιου",
            'x' => "εξ",
            'y' => "γουάι",
            'z' => "ζεντ",
            _ => return None,
        })
    }

    fn abbreviation(&self, word: &str) -> Option<&'static str> {
        expansion(word)
    }

    fn address_sign(&self, sign: char) -> Option<&'static str> {
        match sign {
            '@' => Some("παπάκι"),
            '.' => Some("τελεία"),
            '/' => Some("κάθετος"),
            ':' => Some("άνω κάτω τελεία"),
            '-' => Some("παύλα"),
            '_' => Some("κάτω παύλα"),
            _ => None,
        }
    }

    fn vowels(&self) -> &'static str {
        "αεηιουωάέήίόύώϊϋΐΰ"
    }
}

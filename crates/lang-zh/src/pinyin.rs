/// One syllable in pinyin, taken apart: its initial consonant, its rime in full (the spelling
/// shortcuts undone: `xiu` is x + iou, `yu` is ü, `zhi` has the apical vowel) and its tone, 1 to 4
/// or 5 for the neutral tone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pinyin {
    pub initial: Initial,
    pub rime: Rime,
    pub tone: u8,
}

/// The consonant a syllable starts with; `None` for a syllable that starts with its vowel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Initial {
    None,
    B,
    P,
    M,
    F,
    D,
    T,
    N,
    L,
    G,
    K,
    H,
    J,
    Q,
    X,
    Zh,
    Ch,
    Sh,
    R,
    Z,
    C,
    S,
}

/// A rime, named after its full spelling (`Uei` for ui and wei, `V` for ü).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rime {
    A,
    O,
    E,
    /// ê, as in 欸.
    Eh,
    Ai,
    Ei,
    Ao,
    Ou,
    An,
    En,
    Ang,
    Eng,
    Ong,
    Er,
    I,
    /// The apical vowel of zhi, chi, shi, ri, zi, ci and si.
    Apical,
    Ia,
    Ie,
    Iao,
    Iou,
    Ian,
    In,
    Iang,
    Ing,
    Iong,
    Io,
    Iai,
    U,
    Ua,
    Uo,
    Uai,
    Uei,
    Uan,
    Uen,
    Uang,
    Ueng,
    V,
    Ve,
    Van,
    Vn,
    /// A syllabic m (呣), n (嗯) or ng (嗯), possibly after h (噷, 哼).
    M,
    N,
    Ng,
}

const INITIALS: [(&str, Initial); 21] = [
    ("zh", Initial::Zh),
    ("ch", Initial::Ch),
    ("sh", Initial::Sh),
    ("b", Initial::B),
    ("p", Initial::P),
    ("m", Initial::M),
    ("f", Initial::F),
    ("d", Initial::D),
    ("t", Initial::T),
    ("n", Initial::N),
    ("l", Initial::L),
    ("g", Initial::G),
    ("k", Initial::K),
    ("h", Initial::H),
    ("j", Initial::J),
    ("q", Initial::Q),
    ("x", Initial::X),
    ("r", Initial::R),
    ("z", Initial::Z),
    ("c", Initial::C),
    ("s", Initial::S),
];

const RIMES: [(&str, Rime); 43] = [
    ("-i", Rime::Apical),
    ("a", Rime::A),
    ("o", Rime::O),
    ("e", Rime::E),
    ("ê", Rime::Eh),
    ("ai", Rime::Ai),
    ("ei", Rime::Ei),
    ("ao", Rime::Ao),
    ("ou", Rime::Ou),
    ("an", Rime::An),
    ("en", Rime::En),
    ("ang", Rime::Ang),
    ("eng", Rime::Eng),
    ("ong", Rime::Ong),
    ("er", Rime::Er),
    ("i", Rime::I),
    ("ia", Rime::Ia),
    ("ie", Rime::Ie),
    ("iao", Rime::Iao),
    ("iou", Rime::Iou),
    ("ian", Rime::Ian),
    ("in", Rime::In),
    ("iang", Rime::Iang),
    ("ing", Rime::Ing),
    ("iong", Rime::Iong),
    ("io", Rime::Io),
    ("iai", Rime::Iai),
    ("u", Rime::U),
    ("ua", Rime::Ua),
    ("uo", Rime::Uo),
    ("uai", Rime::Uai),
    ("uei", Rime::Uei),
    ("uan", Rime::Uan),
    ("uen", Rime::Uen),
    ("uang", Rime::Uang),
    ("ueng", Rime::Ueng),
    ("v", Rime::V),
    ("ve", Rime::Ve),
    ("van", Rime::Van),
    ("vn", Rime::Vn),
    ("m", Rime::M),
    ("n", Rime::N),
    ("ng", Rime::Ng),
];

impl Pinyin {
    /// Reads a syllable in numbered pinyin (`zhong1`, `lv4`, `nue4`, `ê2`, `hm5`); without a
    /// digit the tone is neutral. `None` for something that is not a Mandarin syllable.
    #[must_use]
    pub fn parse(syllable: &str) -> Option<Self> {
        let (letters, tone) = match syllable.char_indices().last()? {
            (at, digit @ '1'..='5') => (&syllable[..at], u8::try_from(digit.to_digit(10)?).ok()?),
            _ => (syllable, 5),
        };
        let letters = letters.replace('ü', "v");
        // The syllabic nasals, alone or after h.
        let (initial, rest) = match letters.as_str() {
            "m" | "n" | "ng" => (Initial::None, letters.as_str()),
            "hm" | "hng" => (Initial::H, &letters[1..]),
            _ => INITIALS
                .iter()
                .find_map(|(spelling, initial)| letters.strip_prefix(spelling).map(|rest| (*initial, rest)))
                .filter(|(_, rest)| !rest.is_empty())
                .unwrap_or((Initial::None, letters.as_str())),
        };
        let rime = full_rime(initial, rest);
        let rime = RIMES.iter().find(|(spelling, _)| *spelling == rime)?.1;
        Some(Self { initial, rime, tone })
    }
}

/// A rime's full spelling from the one pinyin writes after `initial`.
fn full_rime(initial: Initial, written: &str) -> String {
    if initial == Initial::None {
        // y and w stand for i, ü and u at the start of a syllable.
        let full = match written {
            "yi" | "yin" | "ying" => written[1..].to_owned(),
            "you" => "iou".to_owned(),
            "wu" => "u".to_owned(),
            "wei" => "uei".to_owned(),
            "wen" => "uen".to_owned(),
            _ => {
                if let Some(rest) = written.strip_prefix("yu") {
                    format!("v{rest}")
                } else if let Some(rest) = written.strip_prefix('y') {
                    format!("i{rest}")
                } else if let Some(rest) = written.strip_prefix('w') {
                    format!("u{rest}")
                } else {
                    written.to_owned()
                }
            }
        };
        return full;
    }
    match (initial, written) {
        (Initial::Zh | Initial::Ch | Initial::Sh | Initial::R | Initial::Z | Initial::C | Initial::S, "i") => {
            "-i".to_owned()
        }
        // After j, q and x, u is ü.
        (Initial::J | Initial::Q | Initial::X, _) if written.starts_with('u') => format!("v{}", &written[1..]),
        // nüe and lüe are also written nue and lue.
        (Initial::N | Initial::L, "ue") => "ve".to_owned(),
        (_, "iu") => "iou".to_owned(),
        (_, "ui") => "uei".to_owned(),
        (_, "un") => "uen".to_owned(),
        _ => written.to_owned(),
    }
}

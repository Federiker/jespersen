use jespersen_phonemes::Notation;
use jespersen_spoken::{Gender, Locale};
use jespersen_text::TokenKind;

use crate::{
    locale::{Swedish, is_abbreviation},
    nst,
    numbers::cardinal,
    phonemes::{K, L, N, R, S, T, V, legal_onset},
    sentence::read_tokens,
    spelling::pronounce,
};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("hus", "ˈhʉːs", "hˈʉs"),
        ("skola", "ˈskuːla", "skˈuːla"),
        ("gata", "ˈɡɑːta", "ɡˈɑːta"),
        ("bok", "ˈbuːk", "bˈuːk"),
        ("far", "ˈfɑːr", "fˈɑːr"),
        ("börja", "ˈbœrja", "bˈœrja"),
        ("nation", "naˈɧuːn", "nasxˈuːn"),
        ("studera", "ˈstʉːdeːˌra", "stˈʉdeːrˌa"),
        ("dag", "ˈdɑːɡ", "dˈɑːɡ"),
        ("dagar", "ˈdɑːɡar", "dˈɑːɡar"),
        ("tjugo", "ˈɕʉːɡuː", "ɕˈʉɡuː"),
        ("hundra", "ˈhɵndra", "hˈɵndra"),
        ("kung", "ˈkɵŋ", "kˈɵŋ"),
        ("år", "ˈoːr", "ˈoːr"),
        ("vatten", "ˈvatːən", "vˈatːən"),
        ("tid", "ˈtiːd", "tˈiːd"),
        ("pojke", "ˈpɔjkə", "pˈɔjkə"),
        ("flicka", "ˈflɪkːa", "flˈɪkːa"),
        ("mycket", "ˈmʏkət", "mˈykət"),
        ("säga", "ˈsɛːɡa", "sˈɛːɡa"),
        ("kök", "ˈɕøːk", "ɕˈøːk"),
        ("bröd", "ˈbrøːd", "brˈøːd"),
        ("kvinna", "ˈkvɪna", "kvˈɪna"),
        ("fem", "ˈfɛm", "fˈɛm"),
        ("sju", "ˈɧʉː", "sxˈʉ"),
        ("sjö", "ˈɧøː", "sxˈøː"),
        ("skön", "ˈɧøːn", "sxˈøːn"),
        ("stjärna", "ˈɧɛrna", "sxˈɛrna"),
        ("kära", "ˈɕɛːra", "ɕˈɛːra"),
        ("göra", "ˈjøːra", "jˈøːra"),
        ("gäst", "ˈjɛst", "jˈɛst"),
        ("hjärta", "ˈjɛʈa", "jˈɛta"),
        ("ljus", "ˈjʉːs", "jˈʉs"),
        ("barn", "ˈbɑːrn", "bˈɑːrn"),
        ("kort", "ˈkɔʈ", "kˈɔt"),
        ("fors", "ˈfɔʂ", "fˈɔʂ"),
        ("bord", "ˈbuːrd", "bˈuːrd"),
        ("tack", "ˈtak", "tˈak"),
        ("kyrka", "ˈɕʏrka", "ɕˈyrka"),
        ("fågel", "ˈfoːɡəl", "fˈoːɡəl"),
        ("sommar", "ˈsɔmar", "sˈɔmar"),
        ("vinter", "ˈvɪntər", "vˈɪntər"),
        ("morgon", "ˈmɔrɡɔn", "mˈɔrɡɔn"),
        ("fattig", "ˈfatːɪɡ", "fˈatːɪɡ"),
        ("torget", "ˈtɔrjət", "tˈɔrjət"),
        ("berg", "ˈbɛrj", "bˈɛrj"),
        ("dimmigt", "ˈdɪmɪkt", "dˈɪmɪkt"),
        ("berätta", "bəˈrɛtːa", "bərˈɛtːa"),
        ("procent", "pruːˈsɛnt", "pruːsˈɛnt"),
        ("arbetar", "ˈarbeːˌtar", "ˈarbeːtˌar"),
        ("hej", "ˈhɛj", "hˈɛj"),
        ("miljón", "mɪlˈjuːn", "mɪljˈuːn"),
        ("idé", "iːˈdeː", "iːdˈeː"),
    ];
    let wrong: Vec<String> = cases
        .iter()
        .filter_map(|(word, ipa, espeak)| {
            let said = pronounce(word).unwrap();
            let (got_ipa, got_espeak) = (said.to_notation(Notation::Ipa), said.to_notation(Notation::Espeak));
            (got_ipa != *ipa || got_espeak != *espeak).then(|| format!("{word}: {got_ipa} {got_espeak}"))
        })
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
    assert!(pronounce("ł").is_none());
    assert!(pronounce("-").is_none());
}

/// A sentence's words in eSpeak NG's notation, read together.
#[test]
fn words_from_nst() {
    let cases = [
        // Retroflex consonants, written with their r in eSpeak NG's notation.
        ("barn", "ˈbɑːɳ", "bˈɑːrn"),
        ("bord", "ˈbuːɖ", "bˈuːrd"),
        // The stress where NST puts it.
        ("studera", "stʉːˈdeːra", "stʉdˈeːra"),
        ("polis", "pʊˈliːs", "pɔlˈiːs"),
        // A short unstressed e as ə, a doubled t long.
        ("vatten", "ˈvatːən", "vˈatːən"),
        ("fönster", "ˈfœnstər", "fˈœnstər"),
        // A final unstressed o and an i before a final consonant, long in eSpeak NG's notation.
        ("äro", "ˈɛːrʊ", "ˈɛːruː"),
        ("varit", "ˈvɑːrɪt", "vˈɑːriːt"),
    ];
    let wrong: Vec<String> = cases
        .iter()
        .filter_map(|(word, ipa, espeak)| {
            let said = nst::pronounce(word).unwrap();
            let (got_ipa, got_espeak) = (said.to_notation(Notation::Ipa), said.to_notation(Notation::Espeak));
            (got_ipa != *ipa || got_espeak != *espeak).then(|| format!("{word}: {got_ipa} {got_espeak}"))
        })
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
    // Function words are left to the rules, and a name is only a name with its capital (both
    // unstressed, as the reader takes hans for the pronoun).
    assert!(nst::pronounce("vad").is_none());
    assert!(nst::pronounce("xyzzy").is_none());
    assert_eq!(read("Hans hans"), ["hɑːns", "hans"]);
}

fn read(text: &str) -> Vec<String> {
    let tokens: Vec<(&str, TokenKind)> = text
        .split(' ')
        .map(|token| {
            let kind = if token.chars().all(|c| c.is_ascii_digit()) {
                TokenKind::Number
            } else {
                TokenKind::Word
            };
            (token, kind)
        })
        .collect();
    read_tokens(&tokens)
        .into_iter()
        .flatten()
        .flat_map(|reading| reading.words)
        .map(|said| said.to_notation(Notation::Espeak))
        .collect()
}

#[test]
fn function_words_lose_their_stress() {
    assert_eq!(read("jag har en bil"), ["jɑːɡ", "hɑːr", "ɛn", "bˈiːl"]);
    assert_eq!(read("han och sig"), ["han", "ɔk", "sˈɛj"]);
    assert_eq!(read("miljon"), ["mɪljˈuːn"]);
    assert_eq!(read("21"), ["ɕˌʉɡɔˈɛt"]);
    assert_eq!(read("101"), ["ˈɛthˈɵndra", "ˈɛt"]);
    assert_eq!(read("B"), ["bˈeː"]);
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "noll"),
        (1, "ett"),
        (13, "tretton"),
        (21, "tjugo-ett"),
        (80, "åttio"),
        (100, "ett-hundra"),
        (101, "ett-hundra ett"),
        (300, "tre-hundra"),
        (1000, "ett tusen"),
        (2024, "två tusen tjugo-fyra"),
        (1_000_000, "en miljon"),
        (3_000_000, "tre miljoner"),
        (2_000_000_000, "två miljarder"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

#[test]
fn the_locale_reads_signs_dates_and_money() {
    assert_eq!(Swedish.date(3, 5, 2024).unwrap(), "tre maj två tusen tjugo-fyra");
    assert_eq!(Swedish.date(1, 13, 2024), None);
    assert_eq!(Swedish.time(10, 30).unwrap(), "tio trettio");
    assert_eq!(Swedish.time(1, 0).unwrap(), "klockan ett");
    assert_eq!(Swedish.cardinal(1, Gender::Feminine).unwrap(), "ett");
    assert_eq!(Swedish.currency("€"), Some(("euro", "euro")));
    assert_eq!(Swedish.currency("kr"), Some(("krona", "kronor")));
    assert_eq!(Swedish.currency("¥"), None);
    let words = [
        Swedish.decimal_word(','),
        Swedish.decimal_word('.'),
        Swedish.range(),
        Swedish.ratio(),
        Swedish.minus(),
        Swedish.plus(),
        Swedish.percent(),
    ];
    assert_eq!(words, ["komma", "punkt", "till", "genom", "minus", "plus", "procent"]);
    let symbols = ["%", "&", "@", "+", "=", "°", "#", "€", "$", "§"];
    assert_eq!(symbols.iter().filter_map(|symbol| Swedish.symbol(symbol)).count(), 9);
    let letters: String = "abcdefghijklmnopqrstuvwxyzåäö"
        .chars()
        .filter_map(|letter| Swedish.letter(letter))
        .collect();
    assert!(
        letters.starts_with("abecedeeeffgehåijikå") && letters.ends_with("exysätaåäö"),
        "{letters}"
    );
    assert_eq!(Swedish.letter('1'), None);
    assert_eq!(
        "@./:-_!".chars().filter_map(|sign| Swedish.address_sign(sign)).count(),
        6
    );
    assert!(is_abbreviation("Dr") && is_abbreviation("kl") && !is_abbreviation("hus"));
    assert_eq!(Swedish.abbreviation("ca"), Some("cirka"));
    assert_eq!(Swedish.vowels(), "aeiouyåäö");
}

#[test]
fn syllables_start_as_swedish_allows() {
    assert!(legal_onset(&[T]) && legal_onset(&[S, T, R]) && legal_onset(&[K, V]) && legal_onset(&[K, L]));
    assert!(!legal_onset(&[N, T]) && !legal_onset(&[L, N, T, R]));
}

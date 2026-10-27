use jespersen_phonemes::Notation;

use crate::spelling::pronounce;

fn ipa(word: &str) -> String {
    pronounce(word).unwrap().to_notation(Notation::Ipa)
}

fn espeak(word: &str) -> String {
    pronounce(word).unwrap().to_notation(Notation::Espeak)
}

#[test]
fn words_from_their_spelling_and_the_list() {
    let cases = [
        ("cane", "ˈkane"),
        ("carne", "ˈkarne"),
        ("casa", "ˈkasa"),
        ("cosa", "ˈkɔsa"),
        ("tavolo", "ˈtavolo"),
        ("bocca", "ˈbokka"),
        ("pizza", "ˈpittsa"),
        ("zio", "ˈdzio"),
        ("gnocco", "ˈɲokko"),
        ("perché", "perˈke"),
        ("città", "tʃitˈta"),
        ("caffè", "kafˈfɛ"),
        ("però", "peˈrɔ"),
        ("famiglia", "faˈmiʎʎa"),
        ("meglio", "ˈmɛʎʎo"),
        ("migliore", "miʎˈʎore"),
        ("gli", "ˈʎi"),
        ("sciopero", "ˈʃɔpero"),
        ("scienza", "ˈʃɛntsa"),
        ("ghiaccio", "ˈɡjattʃo"),
        ("chiaro", "ˈkjaro"),
        ("giorno", "ˈdʒorno"),
        ("cielo", "ˈtʃɛlo"),
        ("uomo", "ˈwɔmo"),
        ("acqua", "ˈakkwa"),
        ("maggio", "ˈmaddʒo"),
        ("terra", "ˈtɛrra"),
        ("sbaglio", "ˈzbaʎʎo"),
        ("musica", "ˈmusika"),
        ("bellissimo", "belˈlissimo"),
        ("parlavano", "parˈlavano"),
        ("sei", "ˈsɛi̯"),
        ("auto", "ˈau̯to"),
        ("quale", "ˈkwale"),
        ("ventitré", "ventiˈtre"),
        ("importante", "imporˈtante"),
        ("mezzo", "ˈmɛddzo"),
        ("passare", "pasˈsare"),
        ("trovai", "troˈvai̯"),
        ("mangiai", "manˈdʒai̯"),
        ("abbandonai", "abbandoˈnai̯"),
        ("operai", "opeˈrai̯"),
        ("assai", "asˈsai̯"),
        ("mai", "ˈmai̯"),
        ("guai", "ˈɡwai̯"),
    ];
    let wrong: Vec<String> = cases
        .iter()
        .filter(|(word, expected)| ipa(word) != *expected)
        .map(|(word, expected)| format!("{word}: {} (expected {expected})", ipa(word)))
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn espeak_notation_writes_long_stops_with_a_length_mark() {
    assert_eq!(espeak("bocca"), "bˈokːa");
    assert_eq!(espeak("pizza"), "pˈitsːa");
    assert_eq!(espeak("terra"), "tˈɛrɾa");
    assert_eq!(espeak("casa"), "kˈaza");
    assert_eq!(espeak("bello"), "bˈɛllo");
}

#[test]
fn a_palatal_l_between_vowels_is_long_and_alone_is_not() {
    assert_eq!(espeak("meglio"), "mˈɛʎʎo");
    assert_eq!(espeak("migliore"), "miʎʎˈore");
    assert_eq!(espeak("gli"), "ʎˈi");
}

#[test]
fn an_s_starting_the_second_word_of_a_compound_stays_voiceless() {
    // Between vowels in one word the s is voiced in eSpeak NG's notation (casa kˈaza); where the
    // word list marks a compound's join it is not.
    assert_eq!(espeak("stasera"), "stasˈera");
    assert_eq!(espeak("girasole"), "dʒirasˈole");
    assert_eq!(espeak("ventisei"), "ventisˈɛi");
    assert_eq!(espeak("novantasette"), "novantasˈɛtːe");
    assert_eq!(ipa("trentasei"), "trentaˈsɛi̯");
}

use jespersen_text::{TokenKind, sentences};

use crate::{
    is_abbreviation,
    numbers::{Gender, cardinal, ordinal},
    read_tokens,
};

/// Reads `text` as the tokeniser splits it and writes it in `notation`, a space between words.
fn read(text: &str, notation: Notation) -> String {
    let mut out = Vec::new();
    for sentence in sentences(text, is_abbreviation) {
        let tokens: Vec<(&str, TokenKind)> = sentence.tokens.iter().map(|token| (token.text, token.kind)).collect();
        let mut covered = 0;
        for reading in read_tokens(&tokens) {
            match reading {
                Some(reading) => {
                    covered = reading.tokens - 1;
                    out.extend(reading.words.iter().map(|word| word.to_notation(notation)));
                }
                None if covered > 0 => covered -= 1,
                None => out.push("?".to_owned()),
            }
        }
    }
    out.join(" ")
}

#[test]
fn numbers_in_words() {
    for (number, gender, words) in [
        (0, Gender::Alone, "zero"),
        (1, Gender::Alone, "uno"),
        (1, Gender::Masculine, "un"),
        (1, Gender::Feminine, "una"),
        (17, Gender::Alone, "diciassette"),
        (21, Gender::Alone, "ventuno"),
        (23, Gender::Alone, "ventitré"),
        (28, Gender::Alone, "ventotto"),
        (100, Gender::Alone, "cento"),
        (108, Gender::Alone, "centotto"),
        (180, Gender::Alone, "centottanta"),
        (1200, Gender::Alone, "milleduecento"),
        (2024, Gender::Alone, "duemilaventiquattro"),
        (23_000, Gender::Alone, "ventitremila"),
        (1_000_000, Gender::Alone, "un milione"),
        (2_500_000, Gender::Alone, "due milioni cinquecentomila"),
        (3_000_000_000, Gender::Alone, "tre miliardi"),
        (1_000_000_000, Gender::Alone, "un miliardo"),
    ] {
        assert_eq!(cardinal(number, gender), words, "{number}");
    }
    assert_eq!(ordinal(1, false).as_deref(), Some("primo"));
    assert_eq!(ordinal(2, true).as_deref(), Some("seconda"));
    assert_eq!(ordinal(11, false).as_deref(), Some("undicesimo"));
    assert_eq!(ordinal(23, false).as_deref(), Some("ventitreesimo"));
    assert_eq!(ordinal(26, false).as_deref(), Some("ventiseiesimo"));
    assert_eq!(ordinal(20, true).as_deref(), Some("ventesima"));
    assert_eq!(ordinal(0, false), None);
}

#[test]
fn the_front_end_reads_numbers_signs_and_elisions() {
    let cases = [
        ("Ho 21 anni.", "o venˈtuno ˈanni"),
        ("Ho 1 casa e 1 cane.", "o ˌuna ˈkasa e un ˈkane"),
        ("Costa 1.200 euro.", "ˈkɔsta milledweˈtʃɛnto ˈɛu̯ro"),
        ("Pesa 3,5 chili.", "ˈpesa ˈtre ˈvirɡola ˈtʃinkwe ˈkili"),
        ("Il 50 % passa.", "il tʃinˈkwanta per ˈtʃɛnto ˈpassa"),
        ("Costa 25 €.", "ˈkɔsta ventiˈtʃinkwe ˈɛu̯ro"),
        ("Costa $5.", "ˈkɔsta ˈtʃinkwe ˈdɔllari"),
        ("Sono le 8:30.", "ˌsono le ˈɔtto e ˈmɛddza"),
        (
            "Alle 9:15 e alle 9:05.",
            "ˌalle ˈnɔve e un ˈkwarto e ˌalle ˈnɔve e ˈtʃinkwe",
        ),
        ("Alle 10:00.", "ˌalle ˈdjɛtʃi"),
        ("Il 3/5/2024.", "il ˈtre ˈmaddʒo dwemilaventiˈkwattro"),
        ("Il 1/5/2024.", "il ˈprimo ˈmaddʒo dwemilaventiˈkwattro"),
        (
            "Dal 1990-1995.",
            "dal millenovetʃentonoˈvanta a millenovetʃentonovantaˈtʃinkwe",
        ),
        ("Scese a -5 gradi.", "ˈʃese a ˈmeno ˈtʃinkwe ˈɡradi"),
        ("Salì a +3.", "saˈli a pju ˈtre"),
        ("Chiama lo 0039.", "ˈkjama lo ˈdzɛro ˈdzɛro ˈtre ˈnɔve"),
        ("Al 3° piano.", "al ˈtɛrtso ˈpjano"),
        ("La 2ª porta.", "la seˈkonda ˈpɔrta"),
        ("Il 1. ° posto.", "il ˈprimo ˈpɔsto"),
        ("Il XX secolo.", "il venˈtɛsimo ˈsɛkolo"),
        ("Luigi XIV regnò.", "luˈidʒi kwattordiˈtʃɛsimo reˈɲɔ"),
        (
            "Il Sig. Rossi e il Dott. Bianchi.",
            "il siˈɲor ˈrossi e il dotˈtor ˈbjanki",
        ),
        ("Ecc.", "etˈtʃɛtera"),
        ("La NATO e la RAI.", "la ˈnato e la ˈrai̯"),
        ("Un DVD.", "un ˈdi ˈvu ˈdi"),
        (
            "Scrivi a info@esempio.it.",
            "ˈskrivi a ˈinfo ˈkjɔttʃola eˈsɛmpjo ˈpunto ˈit",
        ),
        ("Tu & io.", "tu e ˈio"),
        ("C'era l'amico dell'acqua.", "ˈtʃɛra laˈmiko delˈlakkwa"),
        ("Un'altra volta, quest'anno.", "uˈnaltra ˈvɔlta kwesˈtanno"),
        ("Un po'.", "un ˈpɔ"),
        ("Italo-americano.", "ˈitalo ameriˈkano"),
        ("Voltandosi.", "volˈtandosi"),
        ("Strinsero.", "ˈstrinsero"),
        ("Pst!", "pst"),
        ("Un § qui.", "un ? ˈkwi"),
    ];
    let wrong: Vec<String> = cases
        .iter()
        .filter(|(text, expected)| read(text, Notation::Ipa) != *expected)
        .map(|(text, expected)| format!("{text}: {} (expected {expected})", read(text, Notation::Ipa)))
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn espeak_notation_in_running_speech() {
    let espeak = |text| read(text, Notation::Espeak);
    assert_eq!(espeak("Il cane di Rosa."), "il kˈane dɪ rˈɔza");
    assert_eq!(espeak("Il futuro di un gatto."), "il fʊtˈuro dɪ ʊn ɡˈatːo");
    assert_eq!(espeak("Inglese e legno."), "iŋɡlˈeze e lˈeɲɲo");
    assert_eq!(espeak("La storia di Pinocchio."), "la stˈɔria dɪ pinˈɔkːio");
    assert_eq!(espeak("Il fuoco e la guerra."), "il fʊˈɔko e la ɡwˈɛrɾa");
    assert_eq!(espeak("Abbiamo i piedi."), "abːjˌamo i pjˈɛdɪ");
    assert_eq!(espeak("Rideva."), "ridˈɛva");
    assert_eq!(espeak("Era la sua casa."), "ˌɛra la sˌʊa kˈaza");
    assert_eq!(espeak("Non è vero."), "non ˈɛ vˈero");
    assert_eq!(
        read("Qual era è cosa dura.", Notation::Ipa),
        "ˈkwal ˌɛra ˈɛ ˈkɔsa ˈdura"
    );
    assert_eq!(read("C'è un gatto.", Notation::Ipa), "ˈtʃɛ un ˈɡatto");
    assert_eq!(read("Il gatto e il cane.", Notation::Ipa), "il ˈɡatto e il ˈkane");
    assert_eq!(espeak("Dell'acqua."), "dellˈakːwa");
    assert_eq!(read("Rideva.", Notation::Ipa), "riˈdeva");
}

use jespersen_phonemes::Notation;

use crate::{Variety, spelling::pronounce};

fn ipa(word: &str) -> String {
    pronounce(word, Variety::Castilian).unwrap().to_notation(Notation::Ipa)
}

#[test]
fn words_from_their_spelling() {
    for (word, expected) in [
        ("perro", "ˈpero"),
        ("rosa", "ˈrosa"),
        ("carne", "ˈkaɾne"),
        ("calle", "ˈkaʎe"),
        ("cielo", "ˈθjelo"),
        ("había", "aˈbia"),
        ("agua", "ˈaɡwa"),
        ("México", "ˈmeksiko"),
        ("kilómetro", "kiˈlometɾo"),
        ("pingüino", "pinˈɡwino"),
        ("guerra", "ˈɡera"),
        ("huevo", "ˈwebo"),
        ("hielo", "ˈjelo"),
        ("examen", "ekˈsamen"),
        ("rey", "ˈrei̯"),
        ("muy", "ˈmui̯"),
        ("ciudad", "θjuˈdad"),
        ("cuidado", "kwiˈdado"),
        ("país", "paˈis"),
        ("euro", "ˈeu̯ɾo"),
        ("aire", "ˈai̯ɾe"),
        ("reina", "ˈrei̯na"),
        ("leo", "ˈleo"),
        ("papel", "paˈpel"),
        ("comer", "koˈmeɾ"),
        ("joven", "ˈxoben"),
        ("árbol", "ˈaɾbol"),
        ("chico", "ˈtʃiko"),
        ("queso", "ˈkeso"),
        ("gente", "ˈxente"),
        ("año", "ˈaɲo"),
        ("honra", "ˈonra"),
        ("acción", "akˈθjon"),
        ("y", "ˈi"),
        ("yo", "ˈʝo"),
        ("hablar", "aˈblaɾ"),
        ("atlas", "ˈatlas"),
        ("construir", "konstɾuˈiɾ"),
    ] {
        assert_eq!(ipa(word), expected, "{word}");
    }
}

use jespersen_text::{TokenKind, sentences};

use crate::{
    is_abbreviation,
    numbers::{Gender, cardinal, ordinal},
    read_tokens,
};

/// Reads `text` as the tokeniser splits it and writes it in `notation`, a space between words.
fn read(text: &str, variety: Variety, notation: Notation) -> String {
    let mut out = Vec::new();
    for sentence in sentences(text, is_abbreviation) {
        let tokens: Vec<(&str, TokenKind)> = sentence.tokens.iter().map(|token| (token.text, token.kind)).collect();
        let readings = read_tokens(&tokens, variety);
        let mut covered = 0;
        for reading in readings {
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

fn espeak(text: &str) -> String {
    read(text, Variety::Castilian, Notation::Espeak)
}

fn plain(text: &str) -> String {
    read(text, Variety::Castilian, Notation::Ipa)
}

#[test]
fn numbers_in_words() {
    for (number, gender, words) in [
        (0, Gender::Alone, "cero"),
        (1, Gender::Alone, "uno"),
        (1, Gender::Masculine, "un"),
        (1, Gender::Feminine, "una"),
        (16, Gender::Alone, "dieciséis"),
        (21, Gender::Masculine, "veintiún"),
        (21, Gender::Feminine, "veintiuna"),
        (31, Gender::Masculine, "treinta y un"),
        (100, Gender::Alone, "cien"),
        (101, Gender::Alone, "ciento uno"),
        (200, Gender::Feminine, "doscientas"),
        (555, Gender::Alone, "quinientos cincuenta y cinco"),
        (1000, Gender::Alone, "mil"),
        (2024, Gender::Alone, "dos mil veinticuatro"),
        (21_000, Gender::Alone, "veintiún mil"),
        (1_000_000, Gender::Alone, "un millón"),
        (3_500_000, Gender::Alone, "tres millones quinientos mil"),
    ] {
        assert_eq!(cardinal(number, gender), words, "{number}");
    }
    assert_eq!(ordinal(1, Gender::Masculine).as_deref(), Some("primer"));
    assert_eq!(ordinal(3, Gender::Alone).as_deref(), Some("tercero"));
    assert_eq!(ordinal(2, Gender::Feminine).as_deref(), Some("segunda"));
    assert_eq!(ordinal(11, Gender::Alone), None);
}

#[test]
fn the_front_end_reads_numbers_and_signs() {
    let cases = [
        ("Tengo 21 años.", "ˈtenɡo bei̯nˈtjun ˈaɲos"),
        ("Tengo 21 casas.", "ˈtenɡo bei̯nˈtjuna ˈkasas"),
        ("Llegaron 1.500 personas.", "ʎeˈɡaɾon ˈmil kiˈnjentas peɾˈsonas"),
        ("Pesa 3,5 kilos.", "ˈpesa ˈtɾes ˈkoma ˈθinko ˈkilos"),
        ("Mide 2.75 metros.", "ˈmide ˈdos ˈpunto seˈtenta i ˈθinko ˈmetɾos"),
        ("El 50 % aprobó.", "el θinˈkwenta poɾ ˈθjento apɾoˈbo"),
        ("Cuesta 25 €.", "ˈkwesta bei̯ntiˈθinko ˈeu̯ɾos"),
        ("Cuesta 1 €.", "ˈkwesta ˈun ˈeu̯ɾo"),
        ("Cuesta $5.", "ˈkwesta ˈθinko ˈdolaɾes"),
        ("Son las 8:30.", "ˈson las ˈotʃo i ˈmedja"),
        ("Es la 1:00.", "ˈes la ˈuna en ˈpunto"),
        ("A las 9:15 y las 9:05.", "a las ˈnwebe i ˈkwaɾto i las ˈnwebe i ˈθinko"),
        (
            "Nació el 12/10/1985.",
            "naˈθjo el ˈdoθe de okˈtubɾe de ˈmil nobeˈθjentos oˈtʃenta i ˈθinko",
        ),
        (
            "Fue el 2024-05-03.",
            "ˈfwe el ˈtɾes de ˈmaʝo de ˈdos ˈmil bei̯ntiˈkwatɾo",
        ),
        (
            "De 1990-1995.",
            "de ˈmil nobeˈθjentos noˈbenta a ˈmil nobeˈθjentos noˈbenta i ˈθinko",
        ),
        ("Bajó a -5 grados.", "baˈxo a ˈmenos ˈθinko ˈɡɾados"),
        ("Subió +3.", "suˈbjo ˈmas ˈtɾes"),
        ("Llama al 0034.", "ˈʎama al ˈθeɾo ˈθeɾo ˈtɾes ˈkwatɾo"),
        ("Vivo en el 3.º piso.", "ˈbibo en el teɾˈθeɾ ˈpiso"),
        ("Llegó 1.º.", "ʎeˈɡo pɾiˈmeɾo"),
        ("La 2.ª vez.", "la seˈɡunda ˈbeθ"),
        ("El 1.er día.", "el pɾiˈmeɾ ˈdia"),
        ("El 11.º puesto.", "el ˈonθe ˈpwesto"),
        ("El siglo XX.", "el ˈsiɡlo ˈbei̯nte"),
        ("Felipe II reinó.", "feˈlipe seˈɡundo rei̯ˈno"),
        ("Alfonso XIII.", "alˈfonso ˈtɾeθe"),
        ("El Dr. Pérez y la Sra. López.", "el dokˈtoɾ ˈpeɾeθ i la seˈɲoɾa ˈlopeθ"),
        ("Etc.", "etˈθeteɾa"),
        ("La ONU y el PSOE.", "la ˈonu i el ˈpe ˈese ˈo ˈe"),
        ("Un DVD.", "ˈun ˈde ˈube ˈde"),
        (
            "Escribe a info@ejemplo.com.",
            "esˈkɾibe a ˈinfo aˈroba eˈxemplo ˈpunto ˈkom",
        ),
        (
            "Mira www.rae.es.",
            "ˈmiɾa ˈube ˈdoble ˈube ˈdoble ˈube ˈdoble ˈpunto ˈrae ˈpunto ˈes",
        ),
        ("Tú & yo @ casa # 3.", "ˈtu i ˈʝo aˈroba ˈkasa ˈnumeɾo ˈtɾes"),
        ("Hay 1 + 1 = 2.", "ˈai̯ ˈuno ˈmas ˈuno iˈɡwal a ˈdos"),
        ("Son 5 €, 3 £ y 2 ¥.", "ˈson ˈθinko ˈeu̯ɾos ˈtɾes ˈlibɾas i ˈdos ˈʝenes"),
        ("Hace 20 °.", "ˈaθe ˈbei̯nte ˈɡɾados"),
        ("El 5 días.", "el ˈθinko ˈdias"),
        ("Cuesta 3,05.", "ˈkwesta ˈtɾes ˈkoma ˈθeɾo ˈθinko"),
        ("Unos 21 problemas.", "ˈunos bei̯nˈtjun pɾoˈblemas"),
        ("Llegan § hoy.", "ˈʎeɡan ? ˈoi̯"),
        ("El 1.2.3.", "ˈel ?"),
    ];
    let wrong: Vec<String> = cases
        .iter()
        .filter(|(text, expected)| plain(text) != *expected)
        .map(|(text, expected)| format!("{text}: {} (expected {expected})", plain(text)))
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn espeak_notation_writes_the_allophones_of_running_speech() {
    assert_eq!(espeak("El perro de Rosa come carne."), "el pˈero ðe rˈosa kˈome kˈaɾne");
    assert_eq!(espeak("Había una vez un gato."), "aβˈia ˈuna βˈeθ ˈuŋ ɡˈato");
    assert_eq!(espeak("Un beso y un paso."), "ˈum bˈeso i ˈum pˈaso");
    assert_eq!(espeak("Tengo un ángel y un gato."), "tˈɛŋɡo ˈun ˈaŋxel i ˈuŋ ɡˈato");
    assert_eq!(espeak("Usted es y usted dijo."), "ustˈeð ˈes i ustˈed ðˈixo");
    assert_eq!(espeak("Madrid."), "maðɾˈid");
    assert_eq!(espeak("Obtuso e ignorante."), "obtˈuso e ˌiɡnoɾˈante");
    assert_eq!(espeak("Absoluto."), "ˌaβsolˈuto");
    assert_eq!(espeak("Cuanto más, tras él."), "kwˌanto mˈas tɾas ˈel");
    assert_eq!(espeak("El cuidado del ruido."), "el kwiðˈaðo ðel ruˈiðo");
    assert_eq!(espeak("Luego, interior y criado."), "luˈeɣo ˌinteɾjˈoɾ i kɾiˈaðo");
    assert_eq!(espeak("El establecimiento."), "el ˌestaβlˌeθimjˈɛnto");
    assert_eq!(espeak("Rápidamente y demente."), "rˈapiðamˈente i ðemˈɛnte");
    assert_eq!(espeak("Lo puso entre amigos."), "lo pˈuso ˌentɾe amˈiɣos");
    assert_eq!(espeak("Nuestro perro."), "nwˌestɾo pˈero");
    assert_eq!(espeak("El hombre que vino."), "el ˈombɾe ke βˈino");
    assert_eq!(espeak("Y"), "ˈi");
    // Plain IPA keeps the phonemes and no rhythmic stresses.
    assert_eq!(plain("Había una vez un gato."), "aˈbia ˈuna ˈbeθ ˈun ˈɡato");
    assert_eq!(plain("El establecimiento."), "el estableθiˈmjento");
    assert_eq!(plain("Rápidamente."), "ˈrapidaˈmente");
}

#[test]
fn latin_american_spanish_has_seseo_and_yeismo() {
    let latin = |text| read(text, Variety::LatinAmerican, Notation::Espeak);
    assert_eq!(latin("La calle y el cielo."), "la kˈajje i el sjˈelo");
    assert_eq!(latin("Llegó la lluvia."), "ʝeɣˈo la ʝˈuβja");
    assert_eq!(latin("La caza y la casa."), "la kˈasa i la kˈasa");
    // ll starting a word is ʝ even after a vowel; m before f; s said once (descendencia).
    assert_eq!(latin("La llama de enfrente."), "la ʝˈama ðe emfɾˈɛnte");
    assert_eq!(espeak("De enfrente."), "de enfɾˈɛnte");
    assert_eq!(latin("Su descendencia."), "su ðˌesendˈɛnsja");
    assert_eq!(read("La calle.", Variety::LatinAmerican, Notation::Ipa), "la ˈkaʝe");
}

#[test]
fn respellings_and_unreadable_words() {
    assert_eq!(plain("México y Oaxaca."), "ˈmexiko i oaˈxaka");
    assert_eq!(plain("Franco-alemán."), "ˈfɾanko aleˈman");
    assert!(pronounce("ççç", Variety::Castilian).is_none());
    assert_eq!(ipa("pst"), "pst");
}

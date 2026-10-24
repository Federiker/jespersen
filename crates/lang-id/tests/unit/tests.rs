use jespersen_phonemes::Notation;

use crate::{locale::cardinal, spelling::pronounce};

#[test]
fn words_from_their_spelling() {
    let cases = [
        ("kucing", "ˈkutʃiŋ", "kˈutʃiŋ"),
        ("melihat", "məˈlihat", "məlˈihat"),
        ("selamat", "səˈlamat", "səlˈamat"),
        ("besar", "ˈbɛsar", "bˈɛsar"),
        ("kepulauan", "kəpuˈlau̯an", "kəpulˈaʊan"),
        ("indonesia", "indonəˈsia", "ˌindonəsˈia"),
        ("nyanyi", "ˈɲaɲi", "ɲˈaɲi"),
        ("syarat", "ˈʃarat", "çˈarat"),
        ("khusus", "ˈxusus", "xˈusus"),
        ("sore", "ˈsɔrə", "sˈɔrə"),
        ("sekolah", "səˈkolah", "səkˈolah"),
        ("kantor", "ˈkantɔr", "kˈantɔr"),
        ("bangun", "ˈbaŋun", "bˈaŋun"),
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
    assert!(pronounce("ç").is_none());
}

#[test]
fn numbers_in_words() {
    for (number, words) in [
        (0, "nol"),
        (11, "sebelas"),
        (12, "dua belas"),
        (21, "dua puluh satu"),
        (100, "seratus"),
        (1250, "seribu dua ratus lima puluh"),
        (2024, "dua ribu dua puluh empat"),
        (1_000_000, "sejuta"),
        (3_000_000_000, "tiga miliar"),
    ] {
        assert_eq!(cardinal(number), words, "{number}");
    }
}

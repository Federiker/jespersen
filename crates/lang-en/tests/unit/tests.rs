use jespersen_phonemes::{Segment, Stress};

use crate::{arpabet, onsets::legal_onset, phonemes::*, rules};

#[test]
fn every_phone_round_trips_through_its_code() {
    for (index, phone) in arpabet::PHONES.iter().enumerate() {
        let stresses: &[u8] = if arpabet::is_vowel(index) { &[0, 1, 2] } else { &[0] };
        for &stress in stresses {
            let written = if arpabet::is_vowel(index) {
                format!("{phone}{stress}")
            } else {
                (*phone).to_owned()
            };
            let code = arpabet::encode(&written).unwrap();
            assert_eq!(arpabet::decode(code), Some((index, stress)), "{written}");
            assert!(american(index, stress).is_some(), "{written}");
        }
    }
}

#[test]
fn malformed_phones_and_codes_are_refused() {
    for phone in ["", "XX", "AH", "AH3", "T1", "ah0"] {
        assert_eq!(arpabet::encode(phone), None, "{phone:?}");
    }
    // B with a stress, and past the last phone.
    assert_eq!(arpabet::decode(6 * 3 + 1), None);
    assert_eq!(arpabet::decode(39 * 3), None);
    assert_eq!(american(39, 0), None);
}

#[test]
fn stress_picks_the_vowel() {
    assert_eq!(american(2, 0), Some((SCHWA, Stress::Unstressed)));
    assert_eq!(american(2, 1), Some((STRUT, Stress::Primary)));
    assert_eq!(american(11, 0), Some((LETTER, Stress::Unstressed)));
    assert_eq!(american(11, 2), Some((NURSE, Stress::Secondary)));
    assert_eq!(american(17, 0), Some((HAPPY, Stress::Unstressed)));
    assert_eq!(american(17, 1), Some((FLEECE, Stress::Primary)));
}

fn seg(phoneme: jespersen_phonemes::Phoneme, stress: Stress) -> Segment {
    Segment::new(phoneme, stress)
}

#[test]
fn t_flaps_only_between_a_vowel_or_r_and_an_unstressed_vowel() {
    let u = Stress::Unstressed;
    let p = Stress::Primary;
    // butter: b ʌ t ɚ
    let mut butter = [seg(B, u), seg(STRUT, p), seg(T, u), seg(LETTER, u)];
    rules::american(&mut butter);
    assert_eq!(butter[2].phoneme, T_FLAP);
    // party: p ɑ ɹ t i
    let mut party = [seg(P, u), seg(LOT, p), seg(R, u), seg(T, u), seg(HAPPY, u)];
    rules::american(&mut party);
    assert_eq!(party[3].phoneme, T_FLAP);
    // attack: ə t æ k (stressed vowel after t); winter: n before t; ladder: d stays.
    let mut attack = [seg(SCHWA, u), seg(T, u), seg(TRAP, p), seg(K, u)];
    rules::initial_schwa(&mut attack);
    rules::american(&mut attack);
    assert_eq!(attack[1].phoneme, T);
    assert_eq!(attack[0].phoneme, SCHWA_OPEN);
    let mut winter = [seg(W, u), seg(KIT, p), seg(N, u), seg(T, u), seg(LETTER, u)];
    rules::american(&mut winter);
    assert_eq!(winter[3].phoneme, T);
    let mut ladder = [seg(L, u), seg(TRAP, p), seg(D, u), seg(LETTER, u)];
    rules::american(&mut ladder);
    assert_eq!(ladder[2].phoneme, D);
    rules::american(&mut []);
}

#[test]
fn english_onsets() {
    for legal in [
        &[P][..],
        &[S, T, R],
        &[S, P, L],
        &[S, K, W],
        &[K, Y],
        &[TH, R],
        &[SH, R],
        &[S, F],
        &[D, W],
    ] {
        assert!(legal_onset(legal), "{legal:?}");
    }
    for illegal in [
        &[NG][..],
        &[N, Y],
        &[T, L],
        &[L, M],
        &[S, T, L],
        &[M, P, R],
        &[T, S, R, W],
    ] {
        assert!(!legal_onset(illegal), "{illegal:?}");
    }
}

#[test]
fn reduced_kit_in_a_first_syllable_before_stress_and_in_ted_ded() {
    let (u, p) = (Stress::Unstressed, Stress::Primary);
    // behind: b ɪ h aɪ n d
    let mut behind = [seg(B, u), seg(KIT, u), seg(H, u), seg(PRICE, p), seg(N, u), seg(D, u)];
    rules::reduce_first_kit(&mut behind);
    assert_eq!(behind[1].phoneme, KIT_REDUCED);
    // planted: … t ɪ d; folded: … d ə d; but kitted's stressed first KIT stays.
    let mut planted = [seg(P, u), seg(TRAP, p), seg(N, u), seg(T, u), seg(KIT, u), seg(D, u)];
    rules::american(&mut planted);
    assert_eq!(planted[4].phoneme, KIT_REDUCED);
    let mut folded = [seg(F, u), seg(GOAT, p), seg(L, u), seg(D, u), seg(SCHWA, u), seg(D, u)];
    rules::american(&mut folded);
    assert_eq!(folded[4].phoneme, SCHWA_REDUCED);
    let mut kitten = [seg(K, u), seg(KIT, p), seg(T, u), seg(SCHWA, u), seg(N, u)];
    rules::american(&mut kitten);
    assert_eq!(kitten[1].phoneme, KIT);
    // Only a final d after t or d counts: the unstressed KIT of sending stays.
    let mut sending = [seg(S, u), seg(DRESS, p), seg(N, u), seg(D, u), seg(KIT, u), seg(NG, u)];
    rules::american(&mut sending);
    assert_eq!(sending[4].phoneme, KIT);
}

#[test]
fn closed_happy_and_linking_r_within_a_word() {
    let (u, p) = (Stress::Unstressed, Stress::Primary);
    // anything: ɛ n i θ ɪ ŋ; happy: final i stays.
    let mut anything = [
        seg(DRESS, p),
        seg(N, u),
        seg(HAPPY, u),
        seg(TH, u),
        seg(KIT, u),
        seg(NG, u),
    ];
    rules::american(&mut anything);
    assert_eq!(anything[2].phoneme, HAPPY_CLOSED);
    let mut happy = [seg(H, u), seg(TRAP, p), seg(P, u), seg(HAPPY, u)];
    rules::american(&mut happy);
    assert_eq!(happy[3].phoneme, HAPPY);
    // history: h ɪ s t ɚ i
    let mut history = [
        seg(H, u),
        seg(KIT, p),
        seg(S, u),
        seg(T, u),
        seg(LETTER, u),
        seg(HAPPY, u),
    ];
    rules::american(&mut history);
    assert_eq!(history[4].phoneme, LETTER_LINKING);
}

#[test]
fn across_words() {
    let (u, p) = (Stress::Unstressed, Stress::Primary);
    let mut butter = vec![seg(B, u), seg(STRUT, p), seg(T, u), seg(LETTER, u)];
    rules::across(&mut butter, seg(TRAP_WEAK, u));
    assert_eq!(butter[3].phoneme, LETTER_LINKING);
    let mut it = vec![seg(KIT, u), seg(T, u)];
    rules::across(&mut it, seg(KIT, u));
    assert_eq!(it[1].phoneme, T_FLAP);
    let mut it = vec![seg(KIT, u), seg(T, u)];
    rules::across(&mut it, seg(TRAP, p));
    assert_eq!(it[1].phoneme, T, "not before a stressed vowel");
    let mut it = vec![seg(KIT, u), seg(T, u)];
    rules::across(&mut it, seg(K, u));
    assert_eq!(it[1].phoneme, T, "not before a consonant");
}

/// A tiny stand-in lexicon for the morphology rules.
#[expect(clippy::unnecessary_wraps, reason = "the shape of the lookup morphology takes")]
fn fake_lookup(word: &str) -> Result<Option<Vec<Segment>>, jespersen_lexicon::LexiconError> {
    let (u, p) = (Stress::Unstressed, Stress::Primary);
    Ok(match word {
        "cat" => Some(vec![seg(K, u), seg(TRAP, p), seg(T, u)]),
        "dog" => Some(vec![seg(D, u), seg(THOUGHT, p), seg(G, u)]),
        "box" => Some(vec![seg(B, u), seg(LOT, p), seg(K, u), seg(S, u)]),
        "city" => Some(vec![seg(S, u), seg(KIT, p), seg(T, u), seg(HAPPY, u)]),
        "bake" => Some(vec![seg(B, u), seg(FACE, p), seg(K, u)]),
        "stop" => Some(vec![seg(S, u), seg(T, u), seg(LOT, p), seg(P, u)]),
        "want" => Some(vec![seg(W, u), seg(LOT, p), seg(N, u), seg(T, u)]),
        "play" => Some(vec![seg(P, u), seg(L, u), seg(FACE, p)]),
        "grey" => Some(vec![seg(G, u), seg(R, u), seg(FACE, p)]),
        _ => None,
    })
}

/// The phonemes morphology derives for `word`, as plain IPA symbols without stress.
fn derived(word: &str) -> Option<String> {
    crate::morphology::derive(word, &fake_lookup).unwrap().map(|segments| {
        segments
            .iter()
            .map(|segment| segment.phoneme.symbol(jespersen_phonemes::Notation::Ipa))
            .collect()
    })
}

#[test]
fn suffixes_take_the_sound_and_spelling_of_their_stem() {
    assert_eq!(derived("cats").as_deref(), Some("kæts"));
    assert_eq!(derived("dogs").as_deref(), Some("dɔɡz"));
    assert_eq!(derived("boxes").as_deref(), Some("bɑksɪz"));
    assert_eq!(derived("cat's").as_deref(), Some("kæts"));
    assert_eq!(derived("cities").as_deref(), Some("sɪtiz"));
    assert_eq!(derived("baked").as_deref(), Some("beɪkt"));
    assert_eq!(derived("baking").as_deref(), Some("beɪkɪŋ"));
    assert_eq!(derived("stopped").as_deref(), Some("stɑpt"));
    assert_eq!(derived("wanted").as_deref(), Some("wɑntɪd"));
    assert_eq!(derived("played").as_deref(), Some("pleɪd"));
    assert_eq!(derived("greyness").as_deref(), Some("ɡɹeɪnəs"));
    assert_eq!(derived("dogless").as_deref(), Some("dɔɡləs"));
}

#[test]
fn compounds_and_what_cannot_be_derived() {
    assert_eq!(derived("catdog").as_deref(), Some("kætdɔɡ"));
    let compound = crate::morphology::derive("catdog", &fake_lookup).unwrap().unwrap();
    assert!(compound[3].boundary, "a boundary before the second part");
    assert_eq!(
        compound[4].stress,
        Stress::Secondary,
        "the second part's stress is lowered"
    );
    assert_eq!(derived("xyzzy"), None);
    assert_eq!(derived("s"), None, "a suffix with no stem");
    assert_eq!(derived("cats's"), None);
}

#[test]
fn one_primary_stress_per_learned_word() {
    let code = |phone: &str| arpabet::encode(phone).unwrap();
    let stresses = |codes: &[u8]| codes.iter().map(|code| code % 3).collect::<Vec<_>>();
    // Two primaries: the later one becomes secondary.
    let mut two = [code("AE1"), code("T"), code("AH1")];
    arpabet::one_primary(&mut two);
    assert_eq!(stresses(&two), [1, 0, 2]);
    // None: the first secondary, else the first vowel that is not a schwa, else the first vowel.
    let mut secondary = [code("AH0"), code("IY2"), code("EH0")];
    arpabet::one_primary(&mut secondary);
    assert_eq!(stresses(&secondary), [0, 1, 0]);
    let mut full = [code("AH0"), code("T"), code("EH0")];
    arpabet::one_primary(&mut full);
    assert_eq!(stresses(&full), [0, 0, 1]);
    let mut schwas = [code("AH0"), code("AH0")];
    arpabet::one_primary(&mut schwas);
    assert_eq!(stresses(&schwas), [1, 0]);
    let mut none = [code("T"), code("K")];
    arpabet::one_primary(&mut none);
    assert_eq!(stresses(&none), [0, 0]);
}

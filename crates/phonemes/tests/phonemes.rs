use jespersen_phonemes::{
    Notation, Phoneme, PhonemeKind, Pronunciation, Segment, Spelling, Stress, Syllable, Tone, syllabify,
};

// A toy inventory, enough to exercise the model; real ones live in the language crates.
const H: Phoneme = Phoneme::consonant("h", "h");
const L: Phoneme = Phoneme::consonant("l", "l");
const S: Phoneme = Phoneme::consonant("s", "s");
const T: Phoneme = Phoneme::consonant("t", "t");
const R: Phoneme = Phoneme::consonant("r", "ɹ");
const M: Phoneme = Phoneme::consonant("m", "m");
const SCHWA: Phoneme = Phoneme::vowel("ə", "ə");
const GOAT: Phoneme = Phoneme::vowel("oʊ", "oʊ");
const STRUT: Phoneme = Phoneme::vowel("ʌ", "ʌ");
const AI: Phoneme = Phoneme::vowel("aɪ", "aɪ");

fn seg(phoneme: Phoneme) -> Segment {
    Segment::new(phoneme, Stress::Unstressed)
}

fn stressed(phoneme: Phoneme, stress: Stress) -> Segment {
    Segment::new(phoneme, stress)
}

/// Onsets of one consonant, or s + t (+ r), as a stand-in for a language's phonotactics.
fn toy_onset(cluster: &[Phoneme]) -> bool {
    matches!(cluster, [_] | [S, T] | [S, T, R] | [T, R])
}

#[test]
fn phonemes_know_their_kind_and_symbols() {
    assert_eq!(R.kind(), PhonemeKind::Consonant);
    assert!(!R.is_vowel());
    assert!(GOAT.is_vowel());
    assert_eq!(R.symbol(Notation::Ipa), "r");
    assert_eq!(R.symbol(Notation::Espeak), "ɹ");
    assert_eq!(Notation::default(), Notation::Ipa);
}

#[test]
fn stress_marks() {
    assert_eq!(Stress::Unstressed.mark(Notation::Ipa), None);
    assert_eq!(Stress::Secondary.mark(Notation::Ipa), Some('ˌ'));
    assert_eq!(Stress::Primary.mark(Notation::Espeak), Some('ˈ'));
    // A rhythmic secondary stress shows in eSpeak NG's notation only.
    assert_eq!(Stress::Rhythmic.mark(Notation::Ipa), None);
    assert_eq!(Stress::Rhythmic.mark(Notation::Espeak), Some('ˌ'));
    assert!(Stress::Rhythmic < Stress::Secondary);
    assert!(Stress::Primary > Stress::Secondary && Stress::Secondary > Stress::Unstressed);
}

#[test]
fn hello_puts_the_stress_mark_where_each_notation_wants_it() {
    let hello = syllabify(
        &[seg(H), seg(SCHWA), seg(L), stressed(GOAT, Stress::Primary)],
        toy_onset,
    );
    assert_eq!(hello.syllables.len(), 2);
    assert_eq!(hello.to_notation(Notation::Ipa), "həˈloʊ");
    assert_eq!(hello.to_notation(Notation::Espeak), "həlˈoʊ");
}

#[test]
fn consonants_between_vowels_go_to_the_longest_legal_onset() {
    // "mistrial"-like: m ɪ s t r aɪ ə l, with s t r a legal onset.
    let word = [
        seg(M),
        stressed(AI, Stress::Secondary),
        seg(S),
        seg(T),
        seg(R),
        stressed(AI, Stress::Primary),
        seg(SCHWA),
        seg(L),
    ];
    let pronunciation = syllabify(&word, toy_onset);
    let syllables = &pronunciation.syllables;
    assert_eq!(syllables.len(), 3);
    assert_eq!(
        syllables[0],
        Syllable {
            onset: vec![M],
            nucleus: vec![AI],
            coda: vec![],
            stress: Stress::Secondary,
            tone: None,
            spelling: None
        }
    );
    assert_eq!(syllables[1].onset, vec![S, T, R]);
    assert!(
        syllables[2].onset.is_empty(),
        "two vowels in a row meet without consonants"
    );
    assert_eq!(syllables[2].coda, vec![L]);
    assert_eq!(pronunciation.to_notation(Notation::Ipa), "ˌmaɪˈstraɪəl");
    assert_eq!(pronunciation.to_notation(Notation::Espeak), "mˌaɪstɹˈaɪəl");
}

#[test]
fn an_illegal_cluster_is_split_and_the_rest_goes_to_the_coda() {
    // ʌ l m ʌ: "l m" is no onset, so l closes the first syllable.
    let pronunciation = syllabify(&[seg(STRUT), seg(L), seg(M), seg(STRUT)], toy_onset);
    assert_eq!(pronunciation.syllables[0].coda, vec![L]);
    assert_eq!(pronunciation.syllables[1].onset, vec![M]);
}

#[test]
fn a_cluster_with_no_legal_onset_closes_the_syllable_before() {
    let pronunciation = syllabify(&[seg(STRUT), seg(L), seg(STRUT)], |_| false);
    assert_eq!(pronunciation.syllables[0].coda, vec![L]);
    assert!(pronunciation.syllables[1].onset.is_empty());
}

#[test]
fn words_without_vowels_are_one_syllable_of_consonants() {
    let hmm = syllabify(&[seg(H), seg(M)], toy_onset);
    assert_eq!(
        hmm,
        Pronunciation {
            syllables: vec![Syllable {
                onset: vec![H, M],
                ..Syllable::default()
            }]
        }
    );
    assert_eq!(hmm.to_notation(Notation::Espeak), "hm");
    assert_eq!(syllabify(&[], toy_onset), Pronunciation::default());
}

#[test]
fn phonemes_come_back_in_order() {
    let word = [seg(S), seg(T), stressed(GOAT, Stress::Primary), seg(R)];
    let pronunciation = syllabify(&word, toy_onset);
    assert_eq!(pronunciation.phonemes().collect::<Vec<_>>(), vec![S, T, GOAT, R]);
    let mut out = String::from("x ");
    pronunciation.write(Notation::Ipa, &mut out);
    assert_eq!(out, "x ˈstoʊr");
}

#[test]
fn onsets_do_not_reach_back_past_a_boundary() {
    // ʌ s | t oʊ: without the boundary s t would be the onset; with it, s closes the first syllable.
    let mut word = [seg(STRUT), seg(S), seg(T), stressed(GOAT, Stress::Primary)];
    assert_eq!(syllabify(&word, toy_onset).syllables[1].onset, vec![S, T]);
    word[2].boundary = true;
    let split = syllabify(&word, toy_onset);
    assert_eq!(
        (split.syllables[0].coda.clone(), split.syllables[1].onset.clone()),
        (vec![S], vec![T])
    );
    // A boundary right before the vowel leaves the onset empty.
    let mut word = [seg(STRUT), seg(S), stressed(GOAT, Stress::Primary)];
    word[2].boundary = true;
    assert!(syllabify(&word, toy_onset).syllables[1].onset.is_empty());
}

#[test]
fn misaki_writes_a_phonemes_own_symbol_or_espeaks() {
    assert_eq!(R.symbol(Notation::Misaki), "ɹ");
    let zh = Phoneme::consonant("ʈʂ", "ʈʂ").misaki("ꭧ");
    assert_eq!(
        [Notation::Ipa, Notation::Espeak, Notation::Misaki].map(|notation| zh.symbol(notation)),
        ["ʈʂ", "ʈʂ", "ꭧ"]
    );
}

#[test]
fn tones_are_written_as_letters_after_the_syllable_or_arrows_after_the_vowel() {
    let syllable = |tone| Pronunciation {
        syllables: vec![Syllable {
            onset: vec![M],
            nucleus: vec![STRUT],
            coda: vec![M],
            tone: Some(tone),
            ..Syllable::default()
        }],
    };
    for (levels, letters, arrow) in [
        (&[5, 5][..], "˥˥", "→"),
        (&[3, 5], "˧˥", "↗"),
        (&[2, 1, 4], "˨˩˦", "↓"),
        (&[2, 1], "˨˩", "↓"),
        (&[5, 1], "˥˩", "↘"),
        (&[4], "˦", "→"),
    ] {
        let word = syllable(Tone::new(levels));
        assert_eq!(word.to_notation(Notation::Ipa), format!("mʌm{letters}"));
        assert_eq!(word.to_notation(Notation::Misaki), format!("mʌ{arrow}m"));
        assert_eq!(word.to_notation(Notation::Espeak), "mʌm");
    }
    // A neutral tone has its pitch letters but no arrow.
    let neutral = Tone::neutral(&[2]);
    assert!(neutral.is_neutral() && neutral.levels() == [2]);
    assert_eq!(syllable(neutral).to_notation(Notation::Misaki), "mʌm");
    assert_eq!(syllable(neutral).to_notation(Notation::Ipa), "mʌm˨");
    assert_eq!(Tone::neutral(&[]).arrow(), None);
    assert_eq!(Tone::new(&[1]).arrow(), Some('→'));
}

#[test]
fn spellings_are_written_in_pinyin_and_zhuyin_and_ipa_stands_in_without_one() {
    let syllable = |pinyin: &str, zhuyin: &str| Syllable {
        nucleus: vec![STRUT],
        spelling: Some(Spelling {
            pinyin: pinyin.to_owned(),
            zhuyin: zhuyin.to_owned(),
        }),
        ..Syllable::default()
    };
    let xian = Pronunciation {
        syllables: vec![syllable("xī", "ㄒㄧ"), syllable("ān", "ㄢ")],
    };
    assert_eq!(xian.to_notation(Notation::Pinyin), "xī'ān");
    assert_eq!(xian.to_notation(Notation::Zhuyin), "ㄒㄧㄢ");
    assert_eq!(xian.to_notation(Notation::Ipa), "ʌʌ");
    let plain = Pronunciation {
        syllables: vec![Syllable {
            onset: vec![R],
            nucleus: vec![STRUT],
            stress: Stress::Primary,
            ..Syllable::default()
        }],
    };
    assert_eq!(plain.to_notation(Notation::Pinyin), "ˈrʌ");
    assert_eq!(plain.to_notation(Notation::Zhuyin), "ˈrʌ");
}

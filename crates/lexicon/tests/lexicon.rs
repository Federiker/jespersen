use jespersen_lexicon::{Lexicon, LexiconError, compile};

/// Keys sorted: apple, café, read, zebra. The bytes end with zebra's NUL and its empty value's
/// length byte (0).
#[expect(clippy::unwrap_used, reason = "a test helper; a failure should fail the test")]
fn sample() -> Vec<u8> {
    compile([
        ("read", vec![1, 2]),
        ("apple", vec![3]),
        ("read", vec![4, 5, 6]),
        ("zebra", vec![]),
        ("café", vec![7]),
    ])
    .unwrap()
}

#[test]
fn finds_every_key_with_its_values_in_order() {
    let bytes = sample();
    let lexicon = Lexicon::new(&bytes).unwrap();
    assert_eq!(lexicon.len(), 4);
    assert!(!lexicon.is_empty());
    assert_eq!(lexicon.get("read").unwrap(), Some(vec![&[1u8, 2][..], &[4, 5, 6][..]]));
    assert_eq!(lexicon.get("apple").unwrap(), Some(vec![&[3u8][..]]));
    assert_eq!(lexicon.get("zebra").unwrap(), Some(vec![&[][..]]));
    assert_eq!(lexicon.get("café").unwrap(), Some(vec![&[7u8][..]]));
}

#[test]
fn absent_keys_are_none() {
    let bytes = sample();
    let lexicon = Lexicon::new(&bytes).unwrap();
    for key in ["", "a", "apples", "rea", "zzz", "Read"] {
        assert_eq!(lexicon.get(key).unwrap(), None, "{key}");
    }
}

#[test]
fn an_empty_lexicon_works() {
    let bytes = compile(Vec::<(String, Vec<u8>)>::new()).unwrap();
    let lexicon = Lexicon::new(&bytes).unwrap();
    assert!(lexicon.is_empty());
    assert_eq!(lexicon.get("anything").unwrap(), None);
}

#[test]
fn bad_entries_are_refused() {
    assert_eq!(compile([("", vec![1])]), Err(LexiconError::InvalidKey(String::new())));
    assert_eq!(
        compile([("a\0b", vec![1])]),
        Err(LexiconError::InvalidKey("a\0b".into()))
    );
    assert_eq!(
        compile([("long", vec![0; 256])]),
        Err(LexiconError::ValueTooLong {
            key: "long".into(),
            len: 256
        })
    );
    assert!(compile([("max", vec![0; 255])]).is_ok());
}

#[test]
fn other_bytes_are_not_a_lexicon() {
    assert_eq!(Lexicon::new(b"").unwrap_err(), LexiconError::NotALexicon);
    assert_eq!(Lexicon::new(b"PKLX").unwrap_err(), LexiconError::NotALexicon);
    let mut other_version = sample();
    other_version[4] = 99;
    assert_eq!(Lexicon::new(&other_version).unwrap_err(), LexiconError::NotALexicon);
}

#[test]
fn damage_is_reported_not_hidden() {
    // A count larger than the offset table.
    let mut bytes = sample();
    bytes[5..9].copy_from_slice(&1000u32.to_le_bytes());
    assert_eq!(Lexicon::new(&bytes).unwrap_err(), LexiconError::Damaged);

    // The first entry's offset (apple, sorted first) pointing past the end.
    let mut bytes = sample();
    bytes[9..13].copy_from_slice(&u32::MAX.to_le_bytes());
    let lexicon = Lexicon::new(&bytes).unwrap();
    assert_eq!(lexicon.get("apple"), Err(LexiconError::Damaged));
    assert!(lexicon.get("read").unwrap().is_some(), "other entries still read");

    // The last byte is zebra's empty value's length; 200 runs past the end.
    let mut bytes = sample();
    *bytes.last_mut().unwrap() = 200;
    let lexicon = Lexicon::new(&bytes).unwrap();
    assert_eq!(lexicon.get("zebra"), Err(LexiconError::Damaged));

    // An entry without its NUL separator.
    let mut bytes = sample();
    let end = bytes.len();
    assert_eq!(bytes[end - 2..], [0, 0]);
    bytes[end - 2..].copy_from_slice(b"xx");
    let lexicon = Lexicon::new(&bytes).unwrap();
    assert_eq!(lexicon.get("zebra"), Err(LexiconError::Damaged));
}

#[test]
fn errors_explain_themselves() {
    let messages = [
        LexiconError::InvalidKey("x\0".into()).to_string(),
        LexiconError::ValueTooLong {
            key: "x".into(),
            len: 300,
        }
        .to_string(),
        LexiconError::TooLarge.to_string(),
        LexiconError::NotALexicon.to_string(),
        LexiconError::Damaged.to_string(),
    ];
    assert!(messages[0].contains("invalid lexicon key"));
    assert!(messages[1].contains("300 bytes"));
    assert!(messages.iter().all(|message| !message.is_empty()));
}

//! The trainer program's command line (training itself runs with `mise run en:letters`).

use std::process::Command;

#[test]
fn the_trainer_explains_its_usage() {
    let output = Command::new(env!("CARGO_BIN_EXE_jespersen-train-en"))
        .arg("nonsense")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("usage: jespersen-train-en evaluate | write OUT")
    );
}

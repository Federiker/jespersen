use std::{
    io::{self, BufRead, BufReader, Write},
    path::Path,
    process::{Command, Stdio},
};

/// The oracle's answers: its version line and, for each text, what it wrote.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Capture {
    pub version: String,
    pub answers: Vec<(String, String)>,
}

/// Runs an eSpeak NG front end that speaks a line protocol (its version on the first line, then
/// one line of phonemes per line of text) with `voice` and `data`, on each of `texts`.
///
/// # Errors
///
/// When the program cannot start, stops answering, or a text contains a line break.
pub fn capture(program: &Path, voice: &str, data: &Path, texts: &[String]) -> io::Result<Capture> {
    if texts.iter().any(|text| text.contains(['\n', '\r'])) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "a text contains a line break",
        ));
    }
    let mut child = Command::new(program)
        .arg(voice)
        .arg(data)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no standard input"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| io::Error::other("no standard output"))?;
    // A thread feeds the texts while this one reads the answers, so neither pipe can fill up and
    // block the other side.
    let owned = texts.to_vec();
    let writer = std::thread::spawn(move || -> io::Result<()> {
        for text in &owned {
            writeln!(stdin, "{text}")?;
        }
        Ok(())
    });
    let mut lines = BufReader::new(stdout).lines();
    let version = lines
        .next()
        .ok_or_else(|| io::Error::other("the oracle wrote nothing"))??;
    let mut answers = Vec::with_capacity(texts.len());
    for text in texts {
        let answer = lines
            .next()
            .ok_or_else(|| io::Error::other("the oracle stopped answering"))??;
        answers.push((text.clone(), answer));
    }
    writer
        .join()
        .map_err(|_| io::Error::other("the writing thread panicked"))??;
    child.wait()?;
    Ok(Capture { version, answers })
}

/// The capture as text: `# version` then one `text<TAB>answer` line per text.
#[must_use]
pub fn write_capture(capture: &Capture) -> String {
    let mut out = format!("# {}\n", capture.version);
    for (text, answer) in &capture.answers {
        out.push_str(text);
        out.push('\t');
        out.push_str(answer);
        out.push('\n');
    }
    out
}

/// Reads [`write_capture`]'s text back. Lines without a tab are skipped.
#[must_use]
pub fn read_capture(text: &str) -> Capture {
    let mut capture = Capture::default();
    for line in text.lines() {
        if let Some(version) = line.strip_prefix("# ") {
            version.clone_into(&mut capture.version);
        } else if let Some((text, answer)) = line.split_once('\t') {
            capture.answers.push((text.to_owned(), answer.to_owned()));
        }
    }
    capture
}

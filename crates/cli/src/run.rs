use std::io::{self, BufRead, Write};

use jespersen::{BathVowel, Language, Notation, ToneSandhi, WriteOptions};

/// Exit code for text jespersen could not read (an unsupported language, a word it cannot
/// pronounce yet).
const READ_ERROR: u8 = 1;
/// Exit code for a command line jespersen does not understand.
const USAGE_ERROR: u8 = 2;

/// Runs the `jespersen` command with `args` (without the program's name), reading `input` when a
/// command needs standard input, writing its output to `out` and its messages to `err`. Returns
/// the exit code: 0 on success, 1 when text could not be read, 2 for a usage error.
///
/// # Errors
///
/// When `input` cannot be read or `out` or `err` cannot be written, for example a closed pipe.
pub fn run<I, S>(args: I, input: &mut impl BufRead, out: &mut impl Write, err: &mut impl Write) -> io::Result<u8>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let args: Vec<String> = args.into_iter().map(|arg| arg.as_ref().to_owned()).collect();
    match args.first().map(String::as_str) {
        None | Some("-h" | "--help") if args.len() <= 1 => {
            write_usage(out)?;
            Ok(0)
        }
        Some("-V" | "--version") if args.len() == 1 => {
            writeln!(out, "jespersen {}", jespersen::VERSION)?;
            Ok(0)
        }
        Some("phonemes") => phonemes(&args[1..], input, out, err),
        Some(other) if args.len() == 1 => usage_error(err, &format!("unknown command {other:?}")),
        _ => usage_error(err, "too many arguments"),
    }
}

/// `jespersen phonemes`: the text's phonemes, from the arguments or line by line from `input`.
fn phonemes(args: &[String], input: &mut impl BufRead, out: &mut impl Write, err: &mut impl Write) -> io::Result<u8> {
    let mut language = Language::AmericanEnglish;
    let mut bath: Option<BathVowel> = None;
    let mut tones: Option<ToneSandhi> = None;
    let mut options = WriteOptions::default();
    let mut text: Vec<&str> = Vec::new();
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                write_phonemes_usage(out)?;
                return Ok(0);
            }
            "-v" | "--language" => {
                let Some(tag) = rest.next() else {
                    return usage_error(err, &format!("{arg} needs a language, e.g. {arg} en-US"));
                };
                language = match tag.parse() {
                    Ok(language) => language,
                    Err(error) => return read_error(err, &error.to_string()),
                };
            }
            "--bath" => {
                bath = Some(match rest.next().map(String::as_str) {
                    Some("broad") => BathVowel::Broad,
                    Some("flat") => BathVowel::Flat,
                    Some("notation") => BathVowel::ByNotation,
                    _ => return usage_error(err, "--bath needs broad, flat or notation"),
                });
            }
            "--tones" => {
                tones = Some(match rest.next().map(String::as_str) {
                    Some("spoken") => ToneSandhi::Spoken,
                    Some("written") => ToneSandhi::Written,
                    _ => return usage_error(err, "--tones needs spoken or written"),
                });
            }
            "--ipa" => options.notation = Notation::Ipa,
            "--espeak" => options.notation = Notation::Espeak,
            "--misaki" => options.notation = Notation::Misaki,
            "--pinyin" => options.notation = Notation::Pinyin,
            "--zhuyin" => options.notation = Notation::Zhuyin,
            "--no-punctuation" => options.drop_punctuation = true,
            "--" => text.extend(rest.by_ref().map(String::as_str)),
            flag if flag.starts_with('-') && flag.len() > 1 => {
                return usage_error(err, &format!("unknown option {flag:?} for `jespersen phonemes`"));
            }
            word => text.push(word),
        }
    }

    if let Some(bath) = bath {
        let Language::BritishEnglish(_) = language else {
            return usage_error(err, "--bath applies to British English (-v en-GB)");
        };
        language = Language::BritishEnglish(bath);
    }
    if let Some(tones) = tones {
        let Language::Mandarin(_) = language else {
            return usage_error(err, "--tones applies to Mandarin (-v zh)");
        };
        language = Language::Mandarin(tones);
    }
    if !text.is_empty() {
        return match read_line(&text.join(" "), language, options) {
            Ok(line) => {
                writeln!(out, "{line}")?;
                Ok(0)
            }
            Err(message) => read_error(err, &message),
        };
    }
    let mut code = 0;
    for (number, line) in input.lines().enumerate() {
        match read_line(&line?, language, options) {
            Ok(line) => writeln!(out, "{line}")?,
            Err(message) => {
                // An empty line keeps the output aligned with the input.
                writeln!(out)?;
                writeln!(err, "jespersen: line {}: {message}", number + 1)?;
                code = READ_ERROR;
            }
        }
    }
    Ok(code)
}

fn read_line(text: &str, language: Language, options: WriteOptions) -> Result<String, String> {
    let sentences = jespersen::phonemize(text, language).map_err(|error| error.to_string())?;
    jespersen::write(&sentences, options).map_err(|error| error.to_string())
}

fn write_usage(out: &mut impl Write) -> io::Result<()> {
    writeln!(
        out,
        "jespersen {}: text to phonemes, without eSpeak NG.",
        jespersen::VERSION
    )?;
    writeln!(out)?;
    writeln!(out, "Usage: jespersen <command> [options]")?;
    writeln!(out, "       jespersen [--help | --version]")?;
    writeln!(out)?;
    writeln!(out, "Commands:")?;
    writeln!(
        out,
        "  phonemes       Write text's phonemes (`jespersen phonemes --help`)"
    )?;
    writeln!(out)?;
    writeln!(out, "  -h, --help     Show this help")?;
    writeln!(out, "  -V, --version  Show the version")
}

fn write_phonemes_usage(out: &mut impl Write) -> io::Result<()> {
    writeln!(out, "Usage: jespersen phonemes [options] [TEXT...]")?;
    writeln!(out)?;
    writeln!(
        out,
        "Writes the phonemes of TEXT, or of each line of standard input, one line each."
    )?;
    writeln!(out)?;
    writeln!(
        out,
        "  -v, --language TAG  The text's language: en-US (default), en-GB, af, ak, ca, cs, cy, da, de, ee, el, es, es-419, et, eu, fi, fr, ga, ha, hr, hu, id, ig, is, it, lg, ln, lt, lv, mk, ms, nb, nl, om, pl, pt, pt-BR, rn, ro, ru, rw, sk, sl, sn, so, sq, sr, sv, sw, tr, yo or zh"
    )?;
    writeln!(
        out,
        "      --bath VOWEL    British BATH (bath, dance): broad, flat, or notation (default:"
    )?;
    writeln!(
        out,
        "                      broad in plain IPA, as eSpeak NG in its notation)"
    )?;
    writeln!(
        out,
        "      --tones TONES   Mandarin tones: spoken (default; with tone sandhi) or written"
    )?;
    writeln!(out, "      --ipa           Plain IPA (default)")?;
    writeln!(
        out,
        "      --espeak        eSpeak NG's IPA conventions, as voices such as Kokoro expect"
    )?;
    writeln!(
        out,
        "      --misaki        misaki's notation (Mandarin), as Kokoro's Chinese voices expect"
    )?;
    writeln!(out, "      --pinyin        Pinyin with tone marks (Mandarin)")?;
    writeln!(out, "      --zhuyin        Zhuyin, or bopomofo (Mandarin)")?;
    writeln!(out, "      --no-punctuation  Leave punctuation out, as eSpeak NG does")?;
    writeln!(out, "  -h, --help          Show this help")
}

fn usage_error(err: &mut impl Write, message: &str) -> io::Result<u8> {
    writeln!(err, "jespersen: {message}. Try `jespersen --help`.")?;
    Ok(USAGE_ERROR)
}

fn read_error(err: &mut impl Write, message: &str) -> io::Result<u8> {
    writeln!(err, "jespersen: {message}")?;
    Ok(READ_ERROR)
}

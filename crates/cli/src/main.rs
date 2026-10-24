use std::process::ExitCode;

fn main() -> ExitCode {
    let args = std::env::args_os()
        .skip(1)
        .map(|arg| arg.to_string_lossy().into_owned());
    let code = jespersen_cli::run(
        args,
        &mut std::io::stdin().lock(),
        &mut std::io::stdout().lock(),
        &mut std::io::stderr().lock(),
    );
    code.map_or(ExitCode::FAILURE, ExitCode::from)
}

use std::{env, fs, io, process::ExitCode};

mod lexer;

fn main() -> ExitCode {
    let mut arguments = env::args();
    let _program_name = arguments.next();

    let source_path = match arguments.next() {
        Some(source_path) => source_path,
        None => {
            eprintln!("Usage: luc <source_path>");
            return ExitCode::FAILURE;
        }
    };

    match run(source_path.as_str()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(source_path: &str) -> Result<(), io::Error> {
    let source = fs::read_to_string(source_path)?;
    let tokens = lexer::scan_tokens(&source);

    for token in tokens {
        println!("{} {}", token.kind_name(), token.lexeme());
    }

    Ok(())
}

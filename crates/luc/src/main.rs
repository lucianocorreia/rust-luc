use std::{env, fmt, fs, io, process::ExitCode};

mod ast;
mod lexer;
mod parser;

enum RunError {
    Io(io::Error),
    Lex(lexer::LexError),
    Parse(parser::ParseError),
}

impl fmt::Display for RunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunError::Io(error) => write!(formatter, "I/O error: {}", error),
            RunError::Lex(error) => write!(formatter, "Lexical error: {}", error),
            RunError::Parse(error) => write!(formatter, "Parse error: {}", error),
        }
    }
}

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

fn run(source_path: &str) -> Result<(), RunError> {
    let source = fs::read_to_string(source_path).map_err(RunError::Io)?;
    let tokens = lexer::scan_tokens(&source).map_err(RunError::Lex)?;
    let program = parser::parse(tokens).map_err(RunError::Parse)?;

    println!("PROGRAM");

    for statement in program.into_statements() {
        match statement {
            ast::Statement::Print(value) => {
                println!("  PRINT_STATEMENT '{value}'");
            }
        }
    }

    Ok(())
}

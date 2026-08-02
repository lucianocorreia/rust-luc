use std::fmt;

use crate::{
    ast::{Program, Statement},
    lexer::{Token, TokenKind},
};

pub enum ParseError {
    ExpectedPrint(Token),
    MissingString { line: usize, column: usize },
    ExpectedString(Token),
    MissingSemicolon { line: usize, column: usize },
    ExpectedSemicolon(Token),
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // ParseError::EmptyProgram => {
            //     write!(formatter, "o programa está vazio; esperado 'imprima'")
            // }
            ParseError::ExpectedPrint(token) => {
                write!(
                    formatter,
                    "linha {}, coluna {}: esperado 'imprima', encontrado {} '{}'",
                    token.line(),
                    token.column(),
                    token.kind_name(),
                    token.lexeme()
                )
            }
            ParseError::MissingString { line, column } => {
                write!(
                    formatter,
                    "linha {line}, coluna {column}: esperado STRING depois de 'imprima'"
                )
            }
            ParseError::ExpectedString(token) => {
                write!(
                    formatter,
                    "linha {}, coluna {}: esperado STRING, encontrado {} '{}'",
                    token.line(),
                    token.column(),
                    token.kind_name(),
                    token.lexeme()
                )
            }
            ParseError::MissingSemicolon { line, column } => {
                write!(
                    formatter,
                    "linha {line}, coluna {column}: esperado ';' depois da string"
                )
            }
            ParseError::ExpectedSemicolon(token) => {
                write!(
                    formatter,
                    "linha {}, coluna {}: esperado ';', encontrado {} '{}'",
                    token.line(),
                    token.column(),
                    token.kind_name(),
                    token.lexeme()
                )
            } // ParseError::UnexpectedToken(token) => {
              //     write!(
              //         formatter,
              //         "linha {}, coluna {}: token inesperado {} '{}' depois da instrução",
              //         token.line(),
              //         token.column(),
              //         token.kind_name(),
              //         token.lexeme()
              //     )
              // }
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, ParseError> {
    let mut tokens = tokens.into_iter();
    let mut statements = Vec::new();

    while let Some(print_token) = tokens.next() {
        match print_token.kind() {
            TokenKind::Print => {}
            _ => return Err(ParseError::ExpectedPrint(print_token)),
        }

        let print_line = print_token.line();
        let print_column = print_token.column();

        let string_token = match tokens.next() {
            Some(token) => token,
            None => {
                return Err(ParseError::MissingString {
                    line: print_line,
                    column: print_column,
                });
            }
        };

        match string_token.kind() {
            TokenKind::String => {}
            _ => return Err(ParseError::ExpectedString(string_token)),
        }

        let string_line = string_token.line();
        let string_column = string_token.column();

        let semicolon_token = match tokens.next() {
            Some(token) => token,
            None => {
                return Err(ParseError::MissingSemicolon {
                    line: string_line,
                    column: string_column,
                });
            }
        };

        match semicolon_token.kind() {
            TokenKind::Semicolon => {}
            _ => return Err(ParseError::ExpectedSemicolon(semicolon_token)),
        }

        let mut value = string_token.into_lexeme();
        value.remove(0);
        let _closing_quote = value.pop();

        statements.push(Statement::Print(value));
    }

    Ok(Program::new(statements))
}

#[cfg(test)]
mod tests {
    use crate::ast::{Program, Statement};

    use super::parse;

    fn parse_source(source: &str) -> Result<Program, String> {
        let tokens = match crate::lexer::scan_tokens(source) {
            Ok(tokens) => tokens,
            Err(error) => return Err(error.to_string()),
        };

        match parse(tokens) {
            Ok(program) => Ok(program),
            Err(error) => Err(error.to_string()),
        }
    }

    #[test]
    fn parses_print_statement() {
        let program = match parse_source("imprima \"Olá\";") {
            Ok(program) => program,
            Err(error) => panic!("o parser falhou: {error}"),
        };

        let statements = program.into_statements();
        assert_eq!(statements.len(), 1);

        match &statements[0] {
            Statement::Print(value) => assert_eq!(value, "Olá"),
        }
    }

    #[test]
    fn parses_multiple_statements_in_order() {
        let program = match parse_source("imprima \"a\"; imprima \"b\";") {
            Ok(program) => program,
            Err(error) => panic!("o parser falhou: {error}"),
        };

        let statements = program.into_statements();
        assert_eq!(statements.len(), 2);

        match &statements[0] {
            Statement::Print(value) => assert_eq!(value, "a"),
        }

        match &statements[1] {
            Statement::Print(value) => assert_eq!(value, "b"),
        }
    }
}

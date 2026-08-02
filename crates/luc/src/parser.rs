use std::fmt;

use crate::{
    ast::Statement,
    lexer::{Token, TokenKind},
};

pub enum ParseError {
    EmptyProgram,
    ExpectedPrint(Token),
    MissingString { line: usize, column: usize },
    ExpectedString(Token),
    UnexpectedToken(Token),
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyProgram => {
                write!(formatter, "o programa está vazio; esperado 'imprima'")
            }
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
            ParseError::UnexpectedToken(token) => {
                write!(
                    formatter,
                    "linha {}, coluna {}: token inesperado {} '{}' depois da instrução",
                    token.line(),
                    token.column(),
                    token.kind_name(),
                    token.lexeme()
                )
            }
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Statement, ParseError> {
    let mut tokens = tokens.into_iter();

    let print_token = match tokens.next() {
        Some(token) => token,
        None => return Err(ParseError::EmptyProgram),
    };

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

    if let Some(token) = tokens.next() {
        return Err(ParseError::UnexpectedToken(token));
    }

    let mut value = string_token.into_lexeme();
    value.remove(0);
    let _closing_quote = value.pop();

    Ok(Statement::Print(value))
}

#[cfg(test)]
mod tests {
    use crate::ast::Statement;

    use super::parse;

    fn parse_source(source: &str) -> Result<Statement, String> {
        let tokens = match crate::lexer::scan_tokens(source) {
            Ok(tokens) => tokens,
            Err(error) => return Err(format!("Erro de análise léxica: {error}")),
        };

        match parse(tokens) {
            Ok(statement) => Ok(statement),
            Err(error) => Err(format!("Erro de análise sintática: {error}")),
        }
    }

    #[test]
    fn parse_print_statement() {
        let statement = match parse_source("imprima \"Olá\"") {
            Ok(statement) => statement,
            Err(error) => panic!("Falha ao analisar a fonte: {error}"),
        };

        match statement {
            Statement::Print(value) => {
                assert_eq!(value, "Olá");
            }
        }
    }

    #[test]
    fn rejects_number_after_print() {
        let result = parse_source("imprima 42");

        match result {
            Ok(_) => panic!("o parser deveria rejeitar NUMBER"),
            Err(error) => assert_eq!(
                error,
                "Erro de análise sintática: linha 1, coluna 9: esperado STRING, encontrado NUMBER '42'"
            ),
        }
    }
}

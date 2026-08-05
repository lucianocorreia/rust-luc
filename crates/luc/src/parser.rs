use std::{fmt, iter::Peekable, vec::IntoIter};

use crate::{
    ast::{Expr, Program, Statement, Value},
    lexer::{Token, TokenKind},
};

pub enum ParseError {
    ExpectedPrint(Token),
    ExpectedExpression(Token),
    InvalidNumber {
        lexeme: String,
        line: usize,
        column: usize,
    },
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
            ParseError::ExpectedExpression(token) => {
                write!(
                    formatter,
                    "linha {}, coluna {}: esperada expressão, encontrado {} '{}'",
                    token.line(),
                    token.column(),
                    token.kind_name(),
                    token.lexeme()
                )
            }
            ParseError::InvalidNumber {
                lexeme,
                line,
                column,
            } => {
                write!(
                    formatter,
                    "linha {line}, coluna {column}: número inválido '{lexeme}'"
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

struct Parse {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parse {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn is_at_end(&mut self) -> bool {
        match self.tokens.peek() {
            Some(token) => match token.kind() {
                TokenKind::Eof => true,
                _ => false,
            },
            None => true,
        }
    }

    fn advance(&mut self) -> Token {
        match self.tokens.next() {
            Some(token) => token,
            None => panic!("o lexer deve produzir EOF"),
        }
    }

    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(Program::new(statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let print_token = self.advance();

        match print_token.kind() {
            TokenKind::Print => {}
            _ => return Err(ParseError::ExpectedPrint(print_token)),
        }

        let expression = self.parse_expression()?;
        let semicolon_token = self.advance();

        match semicolon_token.kind() {
            TokenKind::Semicolon => {}
            _ => return Err(ParseError::ExpectedSemicolon(semicolon_token)),
        }

        Ok(Statement::Print(expression))
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        let token = self.advance();
        let line = token.line();
        let column = token.column();

        match token.kind() {
            TokenKind::String => {
                let mut value = token.into_lexeme();
                value.remove(0);
                let _closing_quote = value.pop();
                Ok(Expr::Literal(Value::String(value)))
            }
            TokenKind::Number => {
                let lexeme = token.into_lexeme();
                let number = match lexeme.parse::<f64>() {
                    Ok(number) => number,
                    Err(_) => {
                        return Err(ParseError::InvalidNumber {
                            lexeme,
                            line,
                            column,
                        });
                    }
                };
                Ok(Expr::Literal(Value::Number(number)))
            }
            _ => Err(ParseError::ExpectedExpression(token)),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, ParseError> {
    let mut parser = Parse::new(tokens);
    parser.parse_program()
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expr, Program, Statement, Value};
    use crate::lexer::scan_tokens;

    use super::parse;

    fn parse_source(source: &str) -> Result<Program, String> {
        let tokens = scan_tokens(source).map_err(|error| error.to_string())?;
        parse(tokens).map_err(|error| error.to_string())
    }

    #[test]
    fn parses_literal_expressions() {
        let program = match parse_source("imprima \"Olá\"; imprima 42;") {
            Ok(program) => program,
            Err(error) => panic!("o parser falhou: {error}"),
        };

        let statements = program.into_statements();
        assert_eq!(statements.len(), 2);

        match &statements[0] {
            Statement::Print(Expr::Literal(Value::String(value))) => {
                assert_eq!(value, "Olá");
            }
            _ => panic!("esperada string literal"),
        }

        match &statements[1] {
            Statement::Print(Expr::Literal(Value::Number(value))) => {
                assert_eq!(*value, 42.0);
            }
            _ => panic!("esperado número literal"),
        }
    }

    #[test]
    fn rejects_missing_semicolon_at_eof() {
        let result = parse_source("imprima \"sem terminador\"");

        match result {
            Ok(_) => panic!("o parser deveria exigir ponto e vírgula"),
            Err(error) => assert_eq!(error, "linha 1, coluna 25: esperado ';', encontrado EOF ''"),
        }
    }

    #[test]
    fn rejects_missing_expression_at_eof() {
        let result = parse_source("imprima");

        match result {
            Ok(_) => panic!("o parser deveria exigir expressão"),
            Err(error) => assert_eq!(
                error,
                "linha 1, coluna 8: esperada expressão, encontrado EOF ''"
            ),
        }
    }
}

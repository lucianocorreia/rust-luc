use std::fmt;

use crate::{
    ast::{Expr, Program, Statement, Value},
    lexer::{Token, TokenKind},
};

pub enum ParseError {
    ExpectedPrint(Token),
    MissingExpression {
        line: usize,
        column: usize,
    },
    ExpectedExpression(Token),
    InvalidNumber {
        lexeme: String,
        line: usize,
        column: usize,
    },
    MissingSemicolon {
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
            ParseError::MissingExpression { line, column } => {
                write!(
                    formatter,
                    "linha {line}, coluna {column}: esperada expressão depois de 'imprima'"
                )
            }
            ParseError::ExpectedExpression(token) => {
                write!(
                    formatter,
                    "linha {}, coluna {}: esperado expressão, encontrado {} '{}'",
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

        let expression_token = match tokens.next() {
            Some(token) => token,
            None => {
                return Err(ParseError::MissingExpression {
                    line: print_line,
                    column: print_column,
                });
            }
        };

        let expression_line = expression_token.line();
        let expression_column = expression_token.column();

        let expression = match expression_token.kind() {
            TokenKind::String => {
                let mut value = expression_token.into_lexeme();
                value.remove(0);
                let _closing_quote = value.pop();
                Expr::Literal(Value::String(value))
            }
            TokenKind::Number => {
                let lexeme = expression_token.into_lexeme();
                let number = match lexeme.parse::<f64>() {
                    Ok(number) => number,
                    Err(_) => {
                        return Err(ParseError::InvalidNumber {
                            lexeme,
                            line: expression_line,
                            column: expression_column,
                        });
                    }
                };

                Expr::Literal(Value::Number(number))
            }
            _ => return Err(ParseError::ExpectedExpression(expression_token)),
        };

        let semicolon_token = match tokens.next() {
            Some(token) => token,
            None => {
                return Err(ParseError::MissingSemicolon {
                    line: expression_line,
                    column: expression_column,
                });
            }
        };

        match semicolon_token.kind() {
            TokenKind::Semicolon => {}
            _ => return Err(ParseError::ExpectedSemicolon(semicolon_token)),
        }

        statements.push(Statement::Print(expression));
    }

    Ok(Program::new(statements))
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
}

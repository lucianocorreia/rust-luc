use std::fmt;

pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

pub enum LexError {
    UnexpectedCharacter { character: char, position: Position },
    UnterminatedString { position: Position },
}

impl fmt::Display for LexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::UnexpectedCharacter {
                character,
                position,
            } => {
                write!(
                    formatter,
                    "linha {}, coluna {}: Caractere inesperado: '{character}'",
                    position.line(),
                    position.column()
                )
            }
            LexError::UnterminatedString { position } => {
                write!(
                    formatter,
                    "linha {}, coluna {}: string sem aspas de fechamento",
                    position.line(),
                    position.column()
                )
            }
        }
    }
}

pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Number,
    String,
    Identifier,
    Print,
    Eof,
    // Unknown,
}

pub struct Token {
    kind: TokenKind,
    lexeme: String,
    position: Position,
}

impl Token {
    fn new(kind: TokenKind, lexeme: String, position: Position) -> Self {
        Token {
            kind,
            lexeme,
            position,
        }
    }

    pub fn kind_name(&self) -> &str {
        match &self.kind {
            TokenKind::LeftParen => "LEFTPAREN",
            TokenKind::RightParen => "RIGHTPAREN",
            TokenKind::LeftBrace => "LEFTBRACE",
            TokenKind::RightBrace => "RIGHTBRACE",
            TokenKind::Comma => "COMMA",
            TokenKind::Dot => "DOT",
            TokenKind::Semicolon => "SEMICOLON",
            TokenKind::Plus => "PLUS",
            TokenKind::Minus => "MINUS",
            TokenKind::Star => "STAR",
            TokenKind::Slash => "SLASH",
            TokenKind::Equal => "EQUAL",
            TokenKind::EqualEqual => "EQUALEQUAL",
            TokenKind::Bang => "BANG",
            TokenKind::BangEqual => "BANGEQUAL",
            TokenKind::Less => "LESS",
            TokenKind::LessEqual => "LESSEQUAL",
            TokenKind::Greater => "GREATER",
            TokenKind::GreaterEqual => "GREATEREQUAL",
            TokenKind::Number => "NUMBER",
            TokenKind::String => "STRING",
            TokenKind::Identifier => "IDENTIFIER",
            TokenKind::Print => "PRINT",
            TokenKind::Eof => "EOF",
            // TokenKind::Unknown => "Unknown",
        }
    }

    pub fn lexeme(&self) -> &str {
        self.lexeme.as_str()
    }

    pub fn line(&self) -> usize {
        self.position.line()
    }

    pub fn column(&self) -> usize {
        self.position.column()
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn into_lexeme(self) -> String {
        self.lexeme
    }
}

fn advance_position(character: char, line: &mut usize, column: &mut usize) {
    if character == '\n' {
        *line += 1;
        *column = 1;
    } else {
        *column += 1;
    }
}

pub fn scan_tokens(source: &str) -> Result<Vec<Token>, LexError> {
    let mut line = 1;
    let mut column = 1;
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    while let Some(character) = chars.next() {
        let position = Position::new(line, column);
        advance_position(character, &mut line, &mut column);
        let token = match character {
            '(' => Some(Token::new(
                TokenKind::LeftParen,
                character.to_string(),
                position,
            )),
            ')' => Some(Token::new(
                TokenKind::RightParen,
                character.to_string(),
                position,
            )),
            '{' => Some(Token::new(
                TokenKind::LeftBrace,
                character.to_string(),
                position,
            )),
            '}' => Some(Token::new(
                TokenKind::RightBrace,
                character.to_string(),
                position,
            )),
            ',' => Some(Token::new(
                TokenKind::Comma,
                character.to_string(),
                position,
            )),
            '.' => Some(Token::new(TokenKind::Dot, character.to_string(), position)),
            ';' => Some(Token::new(
                TokenKind::Semicolon,
                character.to_string(),
                position,
            )),
            '+' => Some(Token::new(TokenKind::Plus, character.to_string(), position)),
            '-' => Some(Token::new(
                TokenKind::Minus,
                character.to_string(),
                position,
            )),
            '*' => Some(Token::new(TokenKind::Star, character.to_string(), position)),
            '/' => {
                if chars.peek() == Some(&'/') {
                    chars.next();
                    advance_position('/', &mut line, &mut column);

                    while let Some(next_character) = chars.peek() {
                        if *next_character == '\n' {
                            break;
                        }

                        let consumed_character = *next_character;
                        chars.next();
                        advance_position(consumed_character, &mut line, &mut column);
                    }

                    None
                } else {
                    Some(Token::new(
                        TokenKind::Slash,
                        character.to_string(),
                        position,
                    ))
                }
            }
            '=' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    advance_position(character, &mut line, &mut column);
                    Some(Token::new(
                        TokenKind::EqualEqual,
                        String::from("=="),
                        position,
                    ))
                } else {
                    Some(Token::new(
                        TokenKind::Equal,
                        character.to_string(),
                        position,
                    ))
                }
            }
            '!' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    advance_position(character, &mut line, &mut column);
                    Some(Token::new(
                        TokenKind::BangEqual,
                        String::from("!="),
                        position,
                    ))
                } else {
                    Some(Token::new(TokenKind::Bang, character.to_string(), position))
                }
            }
            '<' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    advance_position(character, &mut line, &mut column);
                    Some(Token::new(
                        TokenKind::LessEqual,
                        String::from("<="),
                        position,
                    ))
                } else {
                    Some(Token::new(TokenKind::Less, character.to_string(), position))
                }
            }
            '>' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    advance_position(character, &mut line, &mut column);
                    Some(Token::new(
                        TokenKind::GreaterEqual,
                        String::from(">="),
                        position,
                    ))
                } else {
                    Some(Token::new(
                        TokenKind::Greater,
                        character.to_string(),
                        position,
                    ))
                }
            }
            '"' => {
                let mut lexeme = character.to_string();
                let mut terminated = false;

                while let Some(next_character) = chars.next() {
                    lexeme.push(next_character);
                    advance_position(character, &mut line, &mut column);
                    if next_character == '"' {
                        terminated = true;
                        break;
                    }
                }

                if !terminated {
                    return Err(LexError::UnterminatedString { position });
                }

                Some(Token::new(TokenKind::String, lexeme, position))
            }
            character if character.is_ascii_digit() => {
                let mut lexeme = character.to_string();

                while let Some(next_character) = chars.peek() {
                    if next_character.is_ascii_digit() {
                        let consumed_character = *next_character;
                        lexeme.push(consumed_character);
                        chars.next();
                        advance_position(consumed_character, &mut line, &mut column);
                    } else {
                        break;
                    }
                }

                let has_fraction = if chars.peek() == Some(&'.') {
                    let mut lookahead = chars.clone();
                    lookahead.next();
                    match lookahead.peek() {
                        Some(next_character) => next_character.is_ascii_digit(),
                        None => false,
                    }
                } else {
                    false
                };

                if has_fraction {
                    chars.next();
                    lexeme.push('.');
                    advance_position('.', &mut line, &mut column);

                    while let Some(next_character) = chars.peek() {
                        if next_character.is_ascii_digit() {
                            let consumed_character = *next_character;
                            lexeme.push(consumed_character);
                            chars.next();
                            advance_position(consumed_character, &mut line, &mut column);
                        } else {
                            break;
                        }
                    }
                }

                Some(Token::new(TokenKind::Number, lexeme, position))
            }
            character if character.is_ascii_alphabetic() || character == '_' => {
                let mut lexeme = character.to_string();

                while let Some(next_character) = chars.peek() {
                    if next_character.is_ascii_alphanumeric() || *next_character == '_' {
                        let consumed_chararcter = *next_character;
                        lexeme.push(consumed_chararcter);
                        chars.next();
                        advance_position(consumed_chararcter, &mut line, &mut column);
                    } else {
                        break;
                    }
                }

                let kind = match lexeme.as_str() {
                    "imprima" => TokenKind::Print,
                    _ => TokenKind::Identifier,
                };

                Some(Token::new(kind, lexeme, position))
            }
            character if character.is_whitespace() => None,
            character => {
                return Err(LexError::UnexpectedCharacter {
                    character,
                    position,
                });
            }
        };

        if let Some(token) = token {
            tokens.push(token);
        }
    }

    tokens.push(Token::new(
        TokenKind::Eof,
        String::new(),
        Position::new(line, column),
    ));

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::scan_tokens;

    #[test]
    fn scans_print_and_string_with_positions() {
        let tokens = match scan_tokens("imprima \"Olá\"") {
            Ok(tokens) => tokens,
            Err(error) => panic!("o lexer falhou: {error}"),
        };

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind_name(), "PRINT");
        assert_eq!(tokens[0].lexeme(), "imprima");
        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[0].column(), 1);
        assert_eq!(tokens[1].kind_name(), "STRING");
        assert_eq!(tokens[1].lexeme(), "\"Olá\"");
        assert_eq!(tokens[1].line(), 1);
        assert_eq!(tokens[1].column(), 9);
        assert_eq!(tokens[2].kind_name(), "EOF");
    }

    #[test]
    fn reports_unexpected_character_position() {
        let result = scan_tokens("imprima \"ok\"\n  @");

        match result {
            Ok(_) => panic!("o lexer deveria ter falhado"),
            Err(error) => assert_eq!(
                error.to_string(),
                "linha 2, coluna 3: Caractere inesperado: '@'"
            ),
        }
    }

    #[test]
    fn reports_unterminated_string_start() {
        let result = scan_tokens("imprima \"ok\"\nimprima \"aberta");

        match result {
            Ok(_) => panic!("o lexer deveria rejeitar a string"),
            Err(error) => assert_eq!(
                error.to_string(),
                "linha 2, coluna 9: string sem aspas de fechamento"
            ),
        }
    }

    #[test]
    fn ignores_line_comments_and_preserves_nest_line_position() {
        let source = "// comentário\nimprima \"ok\" // depois\n/";
        let tokens = match scan_tokens(source) {
            Ok(tokens) => tokens,
            Err(error) => panic!("o lexer falhou: {error}"),
        };

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind_name(), "PRINT");
        assert_eq!(tokens[0].line(), 2);
        assert_eq!(tokens[0].column(), 1);
        assert_eq!(tokens[1].kind_name(), "STRING");
        assert_eq!(tokens[1].column(), 9);
        assert_eq!(tokens[2].kind_name(), "SLASH");
        assert_eq!(tokens[2].line(), 3);
        assert_eq!(tokens[2].column(), 1);
        assert_eq!(tokens[3].kind_name(), "EOF");
    }

    #[test]
    fn accepts_comment_at_end_of_file() {
        let tokens = match scan_tokens("imprima \"ok\" // sem quebra final") {
            Ok(tokens) => tokens,
            Err(error) => panic!("o lexer falhou: {error}"),
        };

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind_name(), "PRINT");
        assert_eq!(tokens[1].kind_name(), "STRING");
        assert_eq!(tokens[2].kind_name(), "EOF");
    }

    #[test]
    fn places_eof_after_last_character() {
        let tokens = match scan_tokens("imprima \"ok\";\n") {
            Ok(tokens) => tokens,
            Err(error) => panic!("o lexer falhou: {error}"),
        };

        let eof = &tokens[tokens.len() - 1];
        assert_eq!(eof.kind_name(), "EOF");
        assert_eq!(eof.lexeme(), "");
        assert_eq!(eof.line(), 2);
        assert_eq!(eof.column(), 1);
    }
}

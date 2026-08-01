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

enum TokenKind {
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
            TokenKind::LeftParen => "LeftParen",
            TokenKind::RightParen => "RightParen",
            TokenKind::LeftBrace => "LeftBrace",
            TokenKind::RightBrace => "RightBrace",
            TokenKind::Comma => "Comma",
            TokenKind::Dot => "Dot",
            TokenKind::Semicolon => "Semicolon",
            TokenKind::Plus => "Plus",
            TokenKind::Minus => "Minus",
            TokenKind::Star => "Star",
            TokenKind::Slash => "Slash",
            TokenKind::Equal => "Equal",
            TokenKind::EqualEqual => "EqualEqual",
            TokenKind::Bang => "Bang",
            TokenKind::BangEqual => "BangEqual",
            TokenKind::Less => "Less",
            TokenKind::LessEqual => "LessEqual",
            TokenKind::Greater => "Greater",
            TokenKind::GreaterEqual => "GreaterEqual",
            TokenKind::Number => "Number",
            TokenKind::String => "String",
            TokenKind::Identifier => "Identifier",
            TokenKind::Print => "Print",
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
            '/' => Some(Token::new(
                TokenKind::Slash,
                character.to_string(),
                position,
            )),
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
                    "print" => TokenKind::Print,
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

    Ok(tokens)
}

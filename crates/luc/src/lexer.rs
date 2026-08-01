use std::fmt;

pub enum LexError {
    UnexpectedCharacter(char),
    UnterminatedString,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::UnexpectedCharacter(c) => write!(f, "Unexpected character: '{}'", c),
            LexError::UnterminatedString => write!(f, "Unterminated string literal"),
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
}

impl Token {
    fn new(kind: TokenKind, lexeme: String) -> Self {
        Token { kind, lexeme }
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
}

pub fn scan_tokens(source: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    while let Some(character) = chars.next() {
        let token = match character {
            '(' => Some(Token::new(TokenKind::LeftParen, character.to_string())),
            ')' => Some(Token::new(TokenKind::RightParen, character.to_string())),
            '{' => Some(Token::new(TokenKind::LeftBrace, character.to_string())),
            '}' => Some(Token::new(TokenKind::RightBrace, character.to_string())),
            ',' => Some(Token::new(TokenKind::Comma, character.to_string())),
            '.' => Some(Token::new(TokenKind::Dot, character.to_string())),
            ';' => Some(Token::new(TokenKind::Semicolon, character.to_string())),
            '+' => Some(Token::new(TokenKind::Plus, character.to_string())),
            '-' => Some(Token::new(TokenKind::Minus, character.to_string())),
            '*' => Some(Token::new(TokenKind::Star, character.to_string())),
            '/' => Some(Token::new(TokenKind::Slash, character.to_string())),
            '=' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    Some(Token::new(TokenKind::EqualEqual, String::from("==")))
                } else {
                    Some(Token::new(TokenKind::Equal, character.to_string()))
                }
            }
            '!' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    Some(Token::new(TokenKind::BangEqual, String::from("!=")))
                } else {
                    Some(Token::new(TokenKind::Bang, character.to_string()))
                }
            }
            '<' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    Some(Token::new(TokenKind::LessEqual, String::from("<=")))
                } else {
                    Some(Token::new(TokenKind::Less, character.to_string()))
                }
            }
            '>' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    Some(Token::new(TokenKind::GreaterEqual, String::from(">=")))
                } else {
                    Some(Token::new(TokenKind::Greater, character.to_string()))
                }
            }
            '"' => {
                let mut lexeme = character.to_string();
                let mut terminated = false;

                while let Some(next_character) = chars.next() {
                    lexeme.push(next_character);
                    if next_character == '"' {
                        terminated = true;
                        break;
                    }
                }

                if !terminated {
                    return Err(LexError::UnterminatedString);
                }

                Some(Token::new(TokenKind::String, lexeme))
            }
            character if character.is_ascii_digit() => {
                let mut lexeme = character.to_string();

                while let Some(next_character) = chars.peek() {
                    if next_character.is_ascii_digit() {
                        lexeme.push(*next_character);
                        chars.next();
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

                    while let Some(next_character) = chars.peek() {
                        if next_character.is_ascii_digit() {
                            lexeme.push(*next_character);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }

                Some(Token::new(TokenKind::Number, lexeme))
            }
            character if character.is_ascii_alphabetic() || character == '_' => {
                let mut lexeme = character.to_string();

                while let Some(next_character) = chars.peek() {
                    if next_character.is_ascii_alphanumeric() || *next_character == '_' {
                        lexeme.push(*next_character);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let kind = match lexeme.as_str() {
                    "print" => TokenKind::Print,
                    _ => TokenKind::Identifier,
                };

                Some(Token::new(kind, lexeme))
            }
            character if character.is_whitespace() => None,
            character => return Err(LexError::UnexpectedCharacter(character)),
        };

        if let Some(token) = token {
            tokens.push(token);
        }
    }

    Ok(tokens)
}

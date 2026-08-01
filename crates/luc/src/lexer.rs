pub enum TokenKind {
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Unknown(char),
    // Operators
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
    // Literals
}

impl TokenKind {
    pub fn name(&self) -> &str {
        match self {
            TokenKind::LeftParen => "LEFTPAREN",
            TokenKind::RightParen => "RIGHTPAREN",
            TokenKind::LeftBrace => "LEFTBRACE",
            TokenKind::RightBrace => "RIGHTBRACE",
            TokenKind::Comma => "COMMA",
            TokenKind::Dot => "DOT",
            TokenKind::Semicolon => "SEMICOLON",
            TokenKind::Unknown(_) => "UNKNOWN",
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
        }
    }
}

pub fn scan_tokens(source: &str) -> Vec<TokenKind> {
    let mut tokens = Vec::new();
    let mut characters = source.chars().peekable();

    while let Some(c) = characters.next() {
        let token_kind = match c {
            '(' => Some(TokenKind::LeftParen),
            ')' => Some(TokenKind::RightParen),
            '{' => Some(TokenKind::LeftBrace),
            '}' => Some(TokenKind::RightBrace),
            ',' => Some(TokenKind::Comma),
            '.' => Some(TokenKind::Dot),
            ';' => Some(TokenKind::Semicolon),
            '+' => Some(TokenKind::Plus),
            '-' => Some(TokenKind::Minus),
            '*' => Some(TokenKind::Star),
            '/' => Some(TokenKind::Slash),
            '=' => {
                if characters.peek() == Some(&'=') {
                    characters.next();
                    Some(TokenKind::EqualEqual)
                } else {
                    Some(TokenKind::Equal)
                }
            }
            '!' => {
                if characters.peek() == Some(&'=') {
                    characters.next();
                    Some(TokenKind::BangEqual)
                } else {
                    Some(TokenKind::Bang)
                }
            }
            '<' => {
                if characters.peek() == Some(&'=') {
                    characters.next();
                    Some(TokenKind::LessEqual)
                } else {
                    Some(TokenKind::Less)
                }
            }
            '>' => {
                if characters.peek() == Some(&'=') {
                    characters.next();
                    Some(TokenKind::GreaterEqual)
                } else {
                    Some(TokenKind::Greater)
                }
            }
            character if character.is_whitespace() => None,
            character => Some(TokenKind::Unknown(character)),
        };

        if let Some(token) = token_kind {
            tokens.push(token);
        }
    }

    tokens
}

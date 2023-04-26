use crate::token::Token;

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

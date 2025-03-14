use crate::token::token::TokenLine;
use crate::token::{Token, TokenValue};

#[derive(Debug,Clone,PartialEq)]
pub struct TokenEOL {
    token_line: TokenLine,
}

impl TokenEOL {
    pub fn new(line_number: usize) -> Box<TokenEOL> {
        Box::new(TokenEOL { token_line: TokenLine::new(line_number), }) }
}

impl Token for TokenEOL {
    fn value(&self) -> &TokenValue {
        &TokenValue::EOL
    }


    fn line_number(&self) -> &usize {
        &self.token_line
    }

    fn clone_token(&self) -> Box<dyn Token> {
        Box::new(self.clone())
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct TokenEOF {
    token_line: TokenLine,
}


impl TokenEOF {
    pub fn new(line_number: usize) -> Box<TokenEOF> {
        Box::new(TokenEOF { token_line: TokenLine::new(line_number) }) }
}
impl Token for TokenEOF {
    fn value(&self) -> &TokenValue {
        &TokenValue::EOF
    }

    fn line_number(&self) -> &usize {
        &self.token_line
    }

    fn clone_token(&self) -> Box<dyn Token> {
        Box::new(self.clone())
    }
}
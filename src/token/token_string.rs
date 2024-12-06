use crate::token::token::TokenLine;
use crate::token::{Token, TokenValue};

#[derive(Debug)]
pub struct TokenString {
    token_base: TokenLine,
    text: TokenValue,
}


impl TokenString {
    pub fn new(line_number: usize, str: &str) -> Box<TokenString> {
        Box::new(
            TokenString {
                token_base: TokenLine::new(line_number),
                text: TokenValue::String(str.to_string()),
            }
        )
    }
}


impl Token for TokenString {
    fn value(&self) -> &TokenValue {
        &self.text
    }

    fn line_number(&self) -> &usize {
        &self.token_base
    }
}

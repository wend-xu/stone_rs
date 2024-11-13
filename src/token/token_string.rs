use crate::token::token::TokenLine;
use crate::token::{Token, TokenValue};

#[derive(Debug)]
pub struct TokenText {
    token_base: TokenLine,
    text: TokenValue,
}


impl TokenText {
    pub fn new(line_number: usize, str: &str) -> Box<TokenText> {
        Box::new(
            TokenText {
                token_base: TokenLine::new(line_number),
                text: TokenValue::TEXT(str.to_string()),
            }
        )
    }
}


impl Token for TokenText {
    fn value(&self) -> &TokenValue {
        &self.text
    }

    fn line_number(&self) -> &usize {
        &self.token_base
    }
}
